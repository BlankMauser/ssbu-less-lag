#![allow(warnings)]
use std::sync::atomic::{AtomicBool, Ordering};
use skyline::error::*;
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
use symbaker::symbaker;

#[cfg(feature = "nro-entry")]
use crate::compatibility::SSBUSyncHost::*;

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
    STATUS.load(Ordering::Acquire) as u32
}

fn ssbusync_install_internal(config: SsbuSyncConfig) {
    println!("ssbusync internal install");
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

#[cfg(not(feature = "nro-entry"))]
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

#[cfg(not(feature = "nro-entry"))]
pub fn Enable_Double_Buffer() {
    let allow_buffer_swap = (!is_emulator() && DOUBLES_FIX.load(Ordering::Acquire));
    if allow_buffer_swap {
    start_swap_buffer(BufferMode::Double);
    } else {
        println!("[ssbusync] Swapping Buffer Mode Not Allowed!");
    }
}

#[cfg(not(feature = "nro-entry"))]
pub fn Enable_Triple_Buffer() {
    let allow_buffer_swap = (!is_emulator() && DOUBLES_FIX.load(Ordering::Acquire));
    if allow_buffer_swap {
    start_swap_buffer(BufferMode::Triple);
    } else {
        println!("[ssbusync] Swapping Buffer Mode Not Allowed!");
    }
}

#[cfg(not(feature = "nro-entry"))]
pub fn Check_Buffer_Swap() {
    render::buffer_swap::check_swap_finished();
}

#[cfg(not(feature = "nro-entry"))]
pub fn Check_Ssbusync_Disabled() -> bool {
    is_disabled()
}
    
pub fn is_doubles_fix_enabled() -> bool {
    let allow_buffer_swap = (!is_emulator() && DOUBLES_FIX.load(Ordering::Acquire) == true);
    return allow_buffer_swap;
}

fn try_install() {
    if should_skip_install() {
        return;
    }
    
    if try_claim_install() {
        println!("[ssbusync] ssbusync.nro installing");
        ssbusync_install_internal(SsbuSyncConfig::default());
    }
}

fn panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &s[..],
                None => "Box<Any>"
            }
        };

        let err_msg = format!("thread has panicked at '{}', {}", msg, location);
        show_error(
            69,
            "Skyline plugin as panicked! Please open the details and send a screenshot to the developer, then close the game.\n",
            err_msg.as_str()
        );
    }));
}


fn on_nro_load(_info: &NroInfo) {
    if !should_skip_install() {
        if compatibility::external_disabler() {
            set_disabled();
            println!("[ssbusync] external symbol disabler detected; skipping install.");
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
    
    panic_hook();
    
    register_nro_hook();
}
