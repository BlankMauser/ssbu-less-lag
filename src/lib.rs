#![allow(warnings)]
use skyline::{nn::TimeSpan, nro::{self, NroInfo}};
use core::sync::atomic::{AtomicBool, AtomicU8, Ordering};
mod off_by_one;
mod pacer;
mod profiling;
mod sequencing;
mod swapchain;
//mod vsync;
mod vsync_history;
pub mod compatibility;

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct SsbuSyncConfig {
    pub disable_vsync: bool,
    pub disable_pacer: bool,
    pub enable_triple_buffer: bool,
    pub emulator_check: Option<bool>,
}

impl Default for SsbuSyncConfig {
    fn default() -> Self {
        Self {
            disable_vsync: true,
            disable_pacer: false,
            enable_triple_buffer: false,
            emulator_check: Some(is_emulator()),
        }
    }
}

pub fn is_emulator() -> bool {
    unsafe {
        let text_addr = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
        if text_addr == 0x8504000 || text_addr == 0x80004000 {
            // println!("we are on Emulator");
            return true;
        } else {
            // println!("we are not on Emulator");
            return false;
        }
    }
}

unsafe extern "C" {
    #[link_name = "_ZN2nn2os16GetCurrentThreadEv"]
    pub fn get_current_thread() -> u64;

    #[link_name = "_ZN2nn2os20ChangeThreadPriorityEPNS0_10ThreadTypeEi"]
    pub fn change_thread_priority(thread: u64, prio: i32);
}

static ENABLED: AtomicBool = AtomicBool::new(true);
static COMMON_LOADED: AtomicBool = AtomicBool::new(false);
static NRO_HOOK_REGISTERED: AtomicBool = AtomicBool::new(false);
// 0 = pending, 1 = installing, 2 = installed, 3 = disabled-before-install
static INSTALL_STATE: AtomicU8 = AtomicU8::new(0);
const WAIT_SLICE: TimeSpan = TimeSpan { nanoseconds: 1_000_000 }; // 1ms
const STATE_PENDING: u8 = 0;
const STATE_INSTALLING: u8 = 1;
const STATE_INSTALLED: u8 = 2;
const STATE_DISABLED: u8 = 3;

#[no_mangle]
pub extern "C" fn ssbusync_set_enabled(enabled: u32) {
    if enabled == 0 {
        let _ = ssbusync_request_disable();
        return;
    }

    if INSTALL_STATE.load(Ordering::Acquire) == STATE_DISABLED {
        return;
    }

    ENABLED.store(true, Ordering::Release);
    try_install_if_ready();
}

#[no_mangle]
pub extern "C" fn ssbusync_is_enabled() -> u32 {
    ENABLED.load(Ordering::Acquire) as u32
}

pub fn set_enabled(enabled: bool) {
    ssbusync_set_enabled(enabled as u32);
}

pub fn is_enabled() -> bool {
    ENABLED.load(Ordering::Acquire)
}

pub fn is_common_loaded() -> bool {
    COMMON_LOADED.load(Ordering::Acquire)
}

#[no_mangle]
pub extern "C" fn ssbusync_is_common_loaded() -> u32 {
    is_common_loaded() as u32
}

#[no_mangle]
pub extern "C" fn ssbusync_wait_for_common() {
    while !COMMON_LOADED.load(Ordering::Acquire) {
        unsafe { skyline::nn::os::SleepThread(WAIT_SLICE); }
    }
}

#[no_mangle]
pub extern "C" fn ssbusync_state() -> u32 {
    INSTALL_STATE.load(Ordering::Acquire) as u32
}

// Returns:
// 1 => ssbusync disabled before install (safe for external plugin to install)
// 0 => too late, install already started or completed
#[no_mangle]
pub extern "C" fn ssbusync_request_disable() -> u32 {
    ENABLED.store(false, Ordering::Release);
    match INSTALL_STATE.compare_exchange(
        STATE_PENDING,
        STATE_DISABLED,
        Ordering::AcqRel,
        Ordering::Acquire,
    ) {
        Ok(_) => {
            println!("[ssbusync] disabled by external request");
            1
        }
        Err(STATE_DISABLED) => 1,
        Err(_) => 0,
    }
}

pub fn Install_SSBU_Sync(config: SsbuSyncConfig) {
    let emulator = config.emulator_check.unwrap_or_else(is_emulator);
    println!("[ssbusync] Installing Hooks. \n");
    if emulator {println!("[ssbusync] Emulator Detected. \n"); }
    
    vsync_history::install();
    swapchain::install(config);
    off_by_one::install();

    // Emulator always forces pacer-disable
    if config.disable_pacer || emulator {
        pacer::install();
    }
}

pub fn Enable_Double_Buffer() {
    if !is_emulator() {
        swapchain::enable_double_buffer();
    }
}

pub fn Enable_Triple_Buffer() {
    if !is_emulator() {
        swapchain::enable_triple_buffer();
    }
}

fn try_install_if_ready() {
    if !COMMON_LOADED.load(Ordering::Acquire) || !ENABLED.load(Ordering::Acquire) {
        return;
    }

    if INSTALL_STATE.load(Ordering::Acquire) == STATE_DISABLED {
        return;
    }

    if INSTALL_STATE
        .compare_exchange(
            STATE_PENDING,
            STATE_INSTALLING,
            Ordering::AcqRel,
            Ordering::Acquire,
        )
        .is_err()
    {
        return;
    }

    println!("[ssbusync] installing hooks");
    Install_SSBU_Sync(SsbuSyncConfig::default());
    INSTALL_STATE.store(STATE_INSTALLED, Ordering::Release);
}

pub fn notify_nro_load(info: &NroInfo) {
    if info.name == "common" {
        COMMON_LOADED.store(true, Ordering::Release);
        try_install_if_ready();
    }
}

fn on_nro_load(info: &NroInfo) {
    notify_nro_load(info);
}

pub fn register_nro_hook() {
    if NRO_HOOK_REGISTERED.swap(true, Ordering::AcqRel) {
        return;
    }

    match nro::add_hook(on_nro_load) {
        Ok(()) => println!("[ssbusync] nro hook registered."),
        Err(_) => {
            println!("[ssbusync] nro hook unavailable; installing.");
            COMMON_LOADED.store(true, Ordering::Release);
            try_install_if_ready();
        }
    }
}

pub fn wait_for_common() {
    ssbusync_wait_for_common();
}

#[cfg(feature = "nro-entry")]
#[skyline::main(name = "ssbusync")]
pub fn main() {
    register_nro_hook();
}
