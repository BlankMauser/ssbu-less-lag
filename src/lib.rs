#![allow(warnings)]
use std::sync::atomic::Ordering;
use std::io;
use skyline::error::*;
use skyline::nro::{self, NroInfo};
mod off_by_one;
mod pacer;
mod profiling;
mod sequencing;
mod swapchain;
mod util;
//mod vsync;
mod vsync_history;

pub mod online;
pub mod render;
pub mod compatibility;
pub use crate::util::env as SyncEnv;
pub use crate::util::file::config as Config;

use render::buffer_swap::*;
use swapchain::*;
use symbaker::symbaker;
use serde::{Deserialize, Serialize};

#[cfg(feature = "nro-entry")]
use crate::compatibility::SSBUSyncHost::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct SsbuSyncConfig {
    pub disable_vsync: bool,
    pub disable_pacer: bool,
    pub slow_pacer_bias: bool,
    pub enable_triple_buffer: bool,
    pub allow_buffer_swap: bool,
    pub smooth_ffa: bool,
    pub online_only: bool,
    pub profiling: bool,
    #[serde(skip)]
    pub emulator_check: Option<bool>,
    #[serde(skip)]
    pub override_config: bool,
}

impl Default for SsbuSyncConfig {
    fn default() -> Self {
        Self {
            disable_vsync: true,
            disable_pacer: false,
            slow_pacer_bias: false,
            enable_triple_buffer: true,
            allow_buffer_swap: false,
            smooth_ffa: false,
            online_only: false,
            profiling: false,
            emulator_check: Some(is_emulator()),
            override_config: false,
        }
    }
}

impl SsbuSyncConfig {}

pub fn emulator_status() -> bool {
    if SyncEnv::emulator_known() {
        return SyncEnv::emulator_value();
    }
    let is_emu = unsafe {
        let text_addr = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
        text_addr == 0x8504000 || text_addr == 0x80004000
    };
    SyncEnv::set_emulator_known(true);
    SyncEnv::set_emulator_value(is_emu);
    is_emu
}

pub fn is_emulator() -> bool {
    unsafe {
        let text_addr = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
        if text_addr == 0x8504000 || text_addr == 0x80004000 {
            println!("we are on Emulator");
            return true;
        } else {
            println!("we are not on Emulator");
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


/// Load or create a profile for `plugin_name` from `ssbusync.toml`.
///
/// Example TOML:
/// ```toml
/// [SsbuSync.Default]
/// profile_version = 1.0
/// disable_vsync = true
/// disable_pacer = false
/// slow_pacer_bias = false
/// enable_triple_buffer = false
/// allow_buffer_swap: false,
/// smooth_ffa: false,
/// profiling = false
///
/// [SsbuSync.HDR]
/// profile_version = 1.1
/// disable_vsync = true
/// ```
///
/// If `version` is higher than the stored `profile_version`, the profile is
/// overwritten with `defaults` and the new version is written to disk.
pub fn Get_Init_SsbuSync_Profile(
plugin_name: &str,
defaults: &SsbuSyncConfig,
version: f32,) -> io::Result<SsbuSyncConfig> {
    return Config::get_or_make_profile(plugin_name, defaults, version);
}

pub fn Install_SSBU_Sync(config: SsbuSyncConfig) {
    Config::load_or_create();
    #[cfg(feature = "nro-entry")]
    {
        Get_Init_SsbuSync_Profile("Default", &config, 0.1);
        println!("[ssbusync] Main SsbuSync Module Installing. \n");
    }

    
    let emulator = is_emulator();
    if emulator {
        println!("[ssbusync] Emulator Detected. \n");
    }
    SyncEnv::set_emulator_value(emulator);
    SyncEnv::set_allow_buffer_swap(config.allow_buffer_swap);

    if config.profiling {
        profiling::setup();
    }

    vsync_history::install(config);
    swapchain::install(config);
    off_by_one::install();
    pacer::install(config);
    
}

// #[cfg(not(feature = "nro-entry"))]
// pub fn Enable_Online_Fix() {

// }

// #[cfg(not(feature = "nro-entry"))]
// pub fn Disable_Online_Fix() {

// }

// #[cfg(not(feature = "nro-entry"))]
// pub fn Enable_Double_Buffer() {
//     let allow_buffer_swap = (!is_emulator() && SyncEnv::allow_buffer_swap());
//     if allow_buffer_swap {
//     start_swap_buffer(BufferMode::Double);
//     } else {
//         println!("[ssbusync] Swapping Buffer Mode Not Allowed!");
//     }
// }

// #[cfg(not(feature = "nro-entry"))]
// pub fn Enable_Triple_Buffer() {
//     let allow_buffer_swap = (!is_emulator() && SyncEnv::allow_buffer_swap());
//     if allow_buffer_swap {
//     start_swap_buffer(BufferMode::Triple);
//     } else {
//         println!("[ssbusync] Swapping Buffer Mode Not Allowed!");
//     }
// }

// #[cfg(not(feature = "nro-entry"))]
// pub fn Check_Buffer_Swap() {
//     render::buffer_swap::check_swap_finished();
// }

// #[cfg(feature = "nro-entry")]
// pub fn Check_Ssbusync_Disabled() -> bool {
//     // is_disabled()
// }
    
pub fn is_doubles_fix_enabled() -> bool {
    let allow_buffer_swap = (!is_emulator() && SyncEnv::allow_buffer_swap() == true);
    return allow_buffer_swap;
}

#[cfg(feature = "nro-entry")]
fn try_install() {
    if should_skip_install() {
        return;
    }
    
    if try_claim_install() {
        println!("[ssbusync] ssbusync.nro installing");
        let config = match Config::load_or_create() {
            Ok((config, Config::DefaultProfileState::Created)) => {
                println!("[ssbusync] Created new Default profile in ssbusync.toml.");
                config
            }
            Ok((config, Config::DefaultProfileState::Loaded)) => {
                println!("[ssbusync] Loaded existing Default profile.");
                config
            }
            Err(err) => {
                println!(
                    "[ssbusync] Failed to load Default profile ({}). Using built-in defaults.",
                    err
                );
                SsbuSyncConfig::default()
            }
        };
        Install_SSBU_Sync(config);
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

#[cfg(feature = "nro-entry")]
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

#[cfg(feature = "nro-entry")]
fn register_nro_hook() {
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
