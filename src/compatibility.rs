use skyline::nn::{os::SleepThread, ro, TimeSpan};

type U32Fn = extern "C" fn() -> u32;
pub const VERY_LONG_TIMEOUT_MS: u32 = 15 * 60 * 1000;

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

// Example use from another plugin:
//
// fn on_nro_load(info: &skyline::nro::NroInfo) {
//     if info.name != "common" {
//         return;
//     }
//
//     match ssbusync::compatibility::try_disable_ssbusync() {
//         ssbusync::compatibility::DisableResult::Disabled
//         | ssbusync::compatibility::DisableResult::NotPresent => {
//             let _ = ssbusync::compatibility::wait_for_common();
//             my_plugin_install_custom_ssbusync();
//         }
//         ssbusync::compatibility::DisableResult::TooLate => {
//             // ssbusync already took over; skip custom install.
//         }
//     }
// }
