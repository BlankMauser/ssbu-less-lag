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
pub mod render;
pub mod compatibility;

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct SsbuSyncConfig {
    pub disable_vsync: bool,
    pub disable_pacer: bool,
    pub enable_triple_buffer: bool,
    pub doubles_fix: bool,
    pub emulator_check: Option<bool>,
}

impl Default for SsbuSyncConfig {
    fn default() -> Self {
        Self {
            disable_vsync: true,
            disable_pacer: false,
            enable_triple_buffer: false,
            doubles_fix: false,
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
static DOUBLES_FIX: AtomicBool = AtomicBool::new(false);
const BARRIER_MODULE_NAME: &str = "unknown";

#[cfg(feature = "disabler-symbol")]
#[no_mangle]
pub extern "C" fn ssbusync_external_disabler() -> u32 {
    1
}

fn try_claim_disabler(source: &str) -> u32 {
    if INSTALLED.load(Ordering::Acquire) {
        println!("[ssbusync] {source} too late (already installed)");
        0
    } else if DISABLER_REGISTERED
        .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
        .is_err()
    {
        println!("[ssbusync] {source} rejected (another disabler already claimed)");
        0
    } else {
        ENABLED.store(false, Ordering::Release);
        println!("[ssbusync] {source} accepted (will skip install)");
        1
    }
}

fn try_claim_external_symbol_disabler() -> bool {
    // Already claimed by any external
    if DISABLER_REGISTERED.load(Ordering::Acquire) {
        return true;
    }

    // Already Installed
    if INSTALLED.load(Ordering::Acquire) {
        return false;
    }

    if !compatibility::external_disabler_wants_disable() {
        return false;
    }

    try_claim_disabler("external disabler symbol") != 0
}

#[cfg_attr(feature = "nro-entry", no_mangle)]
pub extern "C" fn ssbusync_set_enabled(enabled: u32) {
    ENABLED.store(enabled != 0, Ordering::Release);
    if enabled == 0 {
        println!("[ssbusync] set_enabled(0) -> disabled by external module");
    } else {
        println!("[ssbusync] set_enabled(1) -> enabled");
    }
}

// External override API: call this before nro barrier
#[cfg_attr(feature = "nro-entry", no_mangle)]
pub extern "C" fn ssbusync_request_disable() -> u32 {
    // Legacy API path; keep behavior aligned with register_disabler.
    try_claim_disabler("request_disable")
}

// External disabler handshake for cross-NRO plugins
// Returns 1 if accepted before install, 0 if already installed.
#[cfg_attr(feature = "nro-entry", no_mangle)]
pub extern "C" fn ssbusync_register_disabler() -> u32 {
    try_claim_disabler("register_disabler")
}

#[cfg_attr(feature = "nro-entry", no_mangle)]
pub extern "C" fn ssbusync_is_enabled() -> u32 {
    ENABLED.load(Ordering::Acquire) as u32
}


pub fn Install_SSBU_Sync(config: SsbuSyncConfig) {
    let emulator = config.emulator_check.unwrap_or_else(is_emulator);
    if emulator {
        println!("[ssbusync] Emulator Detected. \n");
    }
    DOUBLES_FIX.store(config.doubles_fix, Ordering::Release);


    vsync_history::install();
    swapchain::install(config);
    off_by_one::install();

    // Emulator always forces pacer-disable
    if config.disable_pacer || emulator {
        pacer::install();
    }
}

pub fn Enable_Double_Buffer() {
    if !is_emulator() && DOUBLES_FIX.load(Ordering::Acquire) {
        unsafe { swapchain::enable_double_buffer(); }
    }
}

pub fn Enable_Triple_Buffer() {
    if !is_emulator() && DOUBLES_FIX.load(Ordering::Acquire) {
        unsafe { swapchain::enable_triple_buffer(); }
    }
}

fn try_install_once() {
    let _ = try_claim_external_symbol_disabler();

    if DISABLER_REGISTERED.load(Ordering::Acquire) {
        return;
    }

    if !ENABLED.load(Ordering::Acquire) {
        return;
    }

    if INSTALLED
        .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
        .is_err()
    {
        return;
    }

    println!("[ssbusync] SSBU-sync.nro installing");
    Install_SSBU_Sync(SsbuSyncConfig::default());
}

pub fn notify_nro_load(info: &NroInfo) {
    if info.name != BARRIER_MODULE_NAME {
        return;
    }

    println!("[ssbusync] unknown loaded -> evaluating install");
    try_install_once();
}

fn on_nro_load(info: &NroInfo) {
    notify_nro_load(info);
}

pub fn register_nro_hook() {
    if NRO_HOOK_REGISTERED.swap(true, Ordering::AcqRel) {
        return;
    }

    let _ = try_claim_external_symbol_disabler();

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
