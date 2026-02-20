#![allow(warnings)]
use core::sync::atomic::{AtomicBool, Ordering};
use skyline::nro::{self, NroInfo};
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
            return true;
        } else {
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
static INSTALLED: AtomicBool = AtomicBool::new(false);
static NRO_HOOK_REGISTERED: AtomicBool = AtomicBool::new(false);
static DISABLER_REGISTERED: AtomicBool = AtomicBool::new(false);
const BARRIER_MODULE_NAME: &str = "common";

#[cfg_attr(feature = "nro-entry", no_mangle)]
pub extern "C" fn ssbusync_set_enabled(enabled: u32) {
    ENABLED.store(enabled != 0, Ordering::Release);
    if enabled == 0 {
        println!("[ssbusync] set_enabled(0) -> disabled by external module");
    } else {
        println!("[ssbusync] set_enabled(1) -> enabled");
    }
}

// Optional external override API: call this before `common` to stop ssbusync auto-install.
// Returns 1 if now disabled, 0 if already installed and too late.
#[cfg_attr(feature = "nro-entry", no_mangle)]
pub extern "C" fn ssbusync_request_disable() -> u32 {
    ENABLED.store(false, Ordering::Release);
    DISABLER_REGISTERED.store(true, Ordering::Release);
    if INSTALLED.load(Ordering::Acquire) {
        println!("[ssbusync] request_disable too late (already installed)");
        0
    } else {
        println!("[ssbusync] request_disable accepted (will skip on common)");
        1
    }
}

// External disabler handshake for cross-NRO consumers.
// Returns 1 if accepted before install, 0 if already installed.
#[cfg_attr(feature = "nro-entry", no_mangle)]
pub extern "C" fn ssbusync_register_disabler() -> u32 {
    DISABLER_REGISTERED.store(true, Ordering::Release);
    ENABLED.store(false, Ordering::Release);
    if INSTALLED.load(Ordering::Acquire) {
        println!("[ssbusync] register_disabler too late (already installed)");
        0
    } else {
        println!("[ssbusync] register_disabler accepted (will skip on common)");
        1
    }
}

#[cfg_attr(feature = "nro-entry", no_mangle)]
pub extern "C" fn ssbusync_is_enabled() -> u32 {
    ENABLED.load(Ordering::Acquire) as u32
}

pub fn set_enabled(enabled: bool) {
    ssbusync_set_enabled(enabled as u32);
}

pub fn is_enabled() -> bool {
    ENABLED.load(Ordering::Acquire)
}

pub fn Install_SSBU_Sync(config: SsbuSyncConfig) {
    let emulator = config.emulator_check.unwrap_or_else(is_emulator);
    println!("[ssbusync] Installing Hooks. \n");
    if emulator {
        println!("[ssbusync] Emulator Detected. \n");
    }

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
        unsafe { swapchain::enable_double_buffer(); }
    }
}

pub fn Enable_Triple_Buffer() {
    if !is_emulator() {
        unsafe { swapchain::enable_triple_buffer(); }
    }
}

fn try_install_once() {
    if DISABLER_REGISTERED.load(Ordering::Acquire) {
        println!("[ssbusync] disabler registered; skipping install");
        return;
    }

    if !ENABLED.load(Ordering::Acquire) {
        println!("[ssbusync] disabled; skipping install");
        return;
    }

    if INSTALLED
        .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
        .is_err()
    {
        return;
    }

    println!("[ssbusync] installing hooks");
    Install_SSBU_Sync(SsbuSyncConfig::default());
}

pub fn notify_nro_load(info: &NroInfo) {
    if info.name != BARRIER_MODULE_NAME {
        return;
    }

    println!("[ssbusync] common loaded -> evaluating install");
    try_install_once();
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
            // Fallback when NRO hooks are unavailable.
            println!("[ssbusync] nro hook unavailable; installing fallback path.");
            try_install_once();
        }
    }
}

#[cfg(feature = "nro-entry")]
#[skyline::main(name = "ssbusync")]
pub fn main() {
    register_nro_hook();
}
