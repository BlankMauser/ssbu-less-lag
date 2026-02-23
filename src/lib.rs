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

use render::buffer_swap::*;
use swapchain::*;

use crate::compatibility::try_claim_install;

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct SsbuSyncConfig {
    pub disable_vsync: bool,
    pub disable_pacer: bool,
    pub slow_pacer_bias: bool,
    pub enable_triple_buffer: bool,
    pub doubles_fix: bool,
    pub profiling: bool,
    pub emulator_check: Option<bool>,
}

impl Default for SsbuSyncConfig {
    fn default() -> Self {
        Self {
            disable_vsync: true,
            disable_pacer: false,
            slow_pacer_bias: false,
            enable_triple_buffer: false,
            doubles_fix: false,
            profiling: false,
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

static DOUBLES_FIX: AtomicBool = AtomicBool::new(false);

#[cfg(feature = "disabler-symbol")]
#[no_mangle]
pub extern "C" fn ssbusync_external_disabler() -> u32 {
    1
}

#[cfg(feature = "nro-entry")]
#[no_mangle]
pub extern "C" fn ssbusync_status() -> u32 {
    compatibility::STATUS.load(Ordering::Acquire) as u32
}

pub fn Install_SSBU_Sync(config: SsbuSyncConfig) {
    let emulator = config.emulator_check.unwrap_or_else(is_emulator);
    if emulator {
        println!("[ssbusync] Emulator Detected. \n");
    }
    DOUBLES_FIX.store(config.doubles_fix, Ordering::Release);

    if config.profiling {
        profiling::setup();
    }

    vsync_history::install(config);
    swapchain::install(config);
    off_by_one::install();
    pacer::install(config);
}

pub fn Enable_Double_Buffer() {
    let allow_buffer_swap = (!is_emulator() && DOUBLES_FIX.load(Ordering::Acquire));
    if allow_buffer_swap {
    start_swap_buffer(BufferMode::Double);
    } else {
        println!("[ssbusync] Swapping Buffer Mode Not Allowed!");
    }
}

pub fn Enable_Triple_Buffer() {
    let allow_buffer_swap = (!is_emulator() && DOUBLES_FIX.load(Ordering::Acquire));
    if allow_buffer_swap {
    start_swap_buffer(BufferMode::Triple);
    } else {
        println!("[ssbusync] Swapping Buffer Mode Not Allowed!");
    }
}

pub fn Check_Buffer_Swap() {
    render::buffer_swap::check_swap_finished();
}

pub fn Check_Ssbusync_Disabled() -> bool {
    compatibility::is_disabled()
}
    
pub fn is_doubles_fix_enabled() -> bool {
    let allow_buffer_swap = (!is_emulator() && DOUBLES_FIX.load(Ordering::Acquire) == true);
    return allow_buffer_swap;
}

fn try_install() {
    if compatibility::should_skip_install() {
        return;
    }
    
    if try_claim_install() {
        println!("[ssbusync] ssbusync.nro installing");
        Install_SSBU_Sync(SsbuSyncConfig::default());
    }
}


fn on_nro_load(info: &NroInfo) {
    if !compatibility::should_skip_install() {
        if compatibility::external_disabler() {
            compatibility::set_disabled();
            println!("[ssbusync] external symbol disabler detected in main; skipping hook registration.");
            return;
        }
    }
    try_install();
}

pub fn register_nro_hook() {
    match nro::add_hook(on_nro_load) {
        Ok(()) => println!("[ssbusync] nro hook registered."),
        Err(_) => {
            // Fallback when NRO hooks are unavailable.
            println!("[ssbusync] nro hook unavailable; installing fallback path.");
            try_install();
        }
    }
}

#[cfg(feature = "nro-entry")]
#[skyline::main(name = "ssbusync")]
pub fn main() {
    if compatibility::external_disabler() {
        compatibility::set_disabled();
        println!("[ssbusync] external symbol disabler detected in main; skipping hook registration.");
        return;
    }
    register_nro_hook();
}
