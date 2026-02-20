use core::{ffi::c_void, mem::MaybeUninit, ptr};
use core::sync::atomic::{AtomicBool, Ordering};
use skyline::nn::{
    os::{self, SleepThread},
    ro, TimeSpan,
};

type U32Fn = extern "C" fn() -> u32;
pub const VERY_LONG_TIMEOUT_MS: u32 = 15 * 60 * 1000;
pub const POST_HOOK_LOOKUP_DELAY_MS: u32 = 10;
const HANDSHAKE_STACK_SIZE: usize = 0x4000;
const HANDSHAKE_THREAD_PRIO: i32 = 16;

#[repr(align(16))]
struct AlignedStack([u8; HANDSHAKE_STACK_SIZE]);

static HANDSHAKE_STARTED: AtomicBool = AtomicBool::new(false);
static HANDSHAKE_FINISHED: AtomicBool = AtomicBool::new(false);
static mut HANDSHAKE_THREAD: MaybeUninit<os::ThreadType> = MaybeUninit::uninit();
static mut HANDSHAKE_STACK: AlignedStack = AlignedStack([0; HANDSHAKE_STACK_SIZE]);
static mut HANDSHAKE_INSTALL: Option<unsafe fn()> = None;
static mut HANDSHAKE_LOG: Option<fn(&'static str)> = None;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisableResult {
    // ssbusync not found; proceed with custom install.
    NotPresent,
    // ssbusync disabled before install started; proceed with custom install.
    Disabled,
    // ssbusync already installing; do not install another copy.
    TooLate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaitResult {
    // NROs loaded
    CommonLoaded,
    // SSBUSync Module not found
    NotPresent,
    TimedOut,
}

unsafe fn lookup_u32_fn(name: &'static [u8]) -> Option<U32Fn> {
    let mut addr: usize = 0;
    let rc = ro::LookupSymbol(&mut addr as *mut usize, name.as_ptr());
    if rc == 0 && addr != 0 {
        Some(core::mem::transmute(addr))
    } else {
        None
    }
}

pub fn try_disable_ssbusync() -> DisableResult {
    unsafe {
        let Some(request_disable) = lookup_u32_fn(b"ssbusync_request_disable\0") else {
            return DisableResult::NotPresent;
        };

        if request_disable() != 0 {
            DisableResult::Disabled
        } else {
            DisableResult::TooLate
        }
    }
}

pub fn wait_for_common_with_timeout(timeout_ms: u32) -> WaitResult {
    unsafe {
        let Some(is_common_loaded) = lookup_u32_fn(b"ssbusync_is_common_loaded\0") else {
            return WaitResult::NotPresent;
        };

        if is_common_loaded() != 0 {
            return WaitResult::CommonLoaded;
        }

        let mut waited_ms = 0;
        while waited_ms < timeout_ms {
            SleepThread(TimeSpan {
                nanoseconds: 1_000_000, // 1ms
            });
            waited_ms += 1;
            if is_common_loaded() != 0 {
                return WaitResult::CommonLoaded;
            }
        }

        WaitResult::TimedOut
    }
}

pub fn wait_for_common_with_default_timeout() -> WaitResult {
    wait_for_common_with_timeout(VERY_LONG_TIMEOUT_MS)
}

// No-timeout wait; preferred when "common" is expected to always load.
pub fn wait_for_common() -> WaitResult {
    unsafe {
        let Some(is_common_loaded) = lookup_u32_fn(b"ssbusync_is_common_loaded\0") else {
            return WaitResult::NotPresent;
        };

        while is_common_loaded() == 0 {
            SleepThread(TimeSpan {
                nanoseconds: 1_000_000, // 1ms
            });
        }

        WaitResult::CommonLoaded
    }
}

fn sleep_ms(ms: u32) {
    for _ in 0..ms {
        unsafe {
            SleepThread(TimeSpan {
                nanoseconds: 1_000_000, // 1ms
            });
        }
    }
}

extern "C" fn handshake_thread_main(_arg: *mut c_void) {
    sleep_ms(POST_HOOK_LOOKUP_DELAY_MS);

    let install = unsafe { HANDSHAKE_INSTALL };
    let log = unsafe { HANDSHAKE_LOG };

    match try_disable_ssbusync() {
        DisableResult::Disabled => {
            let _ = wait_for_common();
            if let Some(log) = log {
                log("ssbusync disabled: installing");
            }
            if let Some(install) = install {
                unsafe { install() };
            }
        }
        DisableResult::NotPresent => {
            let _ = wait_for_common();
            if let Some(log) = log {
                log("ssbusync not found: installing");
            }
            if let Some(install) = install {
                unsafe { install() };
            }
        }
        DisableResult::TooLate => {
            if let Some(log) = log {
                log("could not disable ssbusync");
            }
        }
    }

    HANDSHAKE_FINISHED.store(true, Ordering::Release);
}

fn start_handshake_thread() -> bool {
    if HANDSHAKE_STARTED
        .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
        .is_err()
    {
        return false;
    }

    unsafe {
        HANDSHAKE_THREAD.write(os::ThreadType::new());
        let thread = HANDSHAKE_THREAD.as_mut_ptr();
        let stack = HANDSHAKE_STACK.0.as_mut_ptr();
        let rc = os::CreateThread(
            thread,
            handshake_thread_main,
            ptr::null_mut(),
            stack,
            HANDSHAKE_STACK_SIZE,
            HANDSHAKE_THREAD_PRIO,
        );

        if rc != 0 {
            HANDSHAKE_STARTED.store(false, Ordering::Release);
            return false;
        }

        os::StartThread(thread);
    }

    true
}

pub fn cleanup_handshake_thread() -> bool {
    if !HANDSHAKE_STARTED.load(Ordering::Acquire) || !HANDSHAKE_FINISHED.load(Ordering::Acquire) {
        return false;
    }

    unsafe {
        let thread = HANDSHAKE_THREAD.as_mut_ptr();
        os::WaitThread(thread);
        os::DestroyThread(thread);
        HANDSHAKE_INSTALL = None;
        HANDSHAKE_LOG = None;
    }

    HANDSHAKE_FINISHED.store(false, Ordering::Release);
    HANDSHAKE_STARTED.store(false, Ordering::Release);
    true
}

// Hook-safe helper: defer symbol lookup until after the NRO callback unwinds.
pub fn spawn_disable_handshake(
    install: unsafe fn(),
    log: Option<fn(&'static str)>,
) {
    let _ = cleanup_handshake_thread();
    unsafe {
        HANDSHAKE_INSTALL = Some(install);
        HANDSHAKE_LOG = log;
    }
    let _ = start_handshake_thread();
}

// Example use from another plugin:
//
// fn disable_ssbusync_hook(info: &skyline::nro::NroInfo) {
//     if info.name != "common" {
//         return;
//     }
//
//     unsafe fn install() {
//         ssbusync::Install_SSBU_Sync(ssbusync::SsbuSyncConfig::default());
//     }
//
//     fn log_line(msg: &'static str) {
//         println!("{msg}");
//     }
//
//     ssbusync::compatibility::spawn_disable_handshake(
//         install,
//         Some(log_line),
//     );
// }
