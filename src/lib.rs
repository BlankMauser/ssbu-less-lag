#![allow(warnings)]
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
pub mod api;
pub mod conduct;
pub mod remote;
pub use crate::util::env as SyncEnv;
pub use crate::util::file::config as Config;

use render::buffer_swap::*;
use swapchain::*;
use symbaker::symbaker;
use serde::{Deserialize, Serialize};

#[cfg(feature = "main-nro")]
use crate::conduct::SyncConductor::*;

#[cfg(feature = "latency-slider")]
pub use local_latency_slider as LatencySlider;

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
            allow_buffer_swap: true,
            smooth_ffa: true,
            online_only: true,
            profiling: false,
            emulator_check: Some(is_emulator()),
            override_config: false,
        }
    }
}

impl SsbuSyncConfig {}

pub fn is_emulator() -> bool {
    if SyncEnv::emulator_known() {
        return SyncEnv::emulator_value();
    }
    let is_emu = unsafe {
        let text_addr = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
        text_addr == 0x8504000 || text_addr == 0x80004000
    };
    SyncEnv::set_emulator_value(is_emu);
    is_emu
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

#[cfg(any(feature = "host-instance", feature = "client"))]
pub struct Instance;

#[cfg(any(feature = "host-instance", feature = "client"))]
impl Instance {
    pub const fn new() -> Self {
        Self
    }

    pub fn Env_Flags(self) -> u32 {
        SyncEnv::flags()
    }

    #[cfg(not(feature = "main-nro"))]
    pub fn Enable_Online_Fix(self) {
        enable_online_fix();
    }

    #[cfg(not(feature = "main-nro"))]
    pub fn Disable_Online_Fix(self) {
        disable_online_fix();
    }

    #[cfg(not(feature = "main-nro"))]
    pub fn Enable_Double_Buffer(self) {
        let allow_buffer_swap = !is_emulator() && SyncEnv::allow_buffer_swap();
        if allow_buffer_swap {
            start_swap_buffer(BufferMode::Double);
        } else {
            println!("[ssbusync] Swapping Buffer Mode Not Allowed!");
        }
    }

    #[cfg(not(feature = "main-nro"))]
    pub fn Enable_Triple_Buffer(self) {
        let allow_buffer_swap = !is_emulator() && SyncEnv::allow_buffer_swap();
        if allow_buffer_swap {
            start_swap_buffer(BufferMode::Triple);
        } else {
            println!("[ssbusync] Swapping Buffer Mode Not Allowed!");
        }
    }

    #[cfg(not(feature = "main-nro"))]
    pub fn Check_Buffer_Swap(self) {
        render::buffer_swap::check_swap_finished();
    }

    #[cfg(not(feature = "main-nro"))]
    pub fn Remote_Status(self) -> Option<u32> {
        remote::client_status()
    }

}

#[cfg(feature = "host-instance")]
pub fn Try_Claim_Install() -> u32 {
    remote::claim_install()
}

#[cfg(feature = "host-instance")]
pub fn Try_Claim_Install_With_Prefix(prefix: &str) -> u32 {
    remote::claim_install_with_prefix(prefix)
}

#[cfg(feature = "host-instance")]
pub fn Try_Claim_Install_With_Resolved_Prefix() -> u32 {
    remote::claim_install_with_resolved_prefix()
}

#[cfg(feature = "main-nro")]
pub extern "C" fn ssbusync_status() -> u32 {
    remote::status()
}

#[cfg(feature = "main-nro")]
#[no_mangle]
pub extern "C" fn ssbusync_try_claim_install() -> u32 {
    claim_for_host()
}

#[cfg(feature = "host-instance")]
#[symbaker]
pub extern "C" fn ssbusync_instance() -> *const api::SsbuSyncApiV1 {
    remote::api_v1_ptr()
}

/// Fallback symbol for non-main host instances when no main ssbusync.nro exists.
#[cfg(all(feature = "host-instance", not(feature = "main-nro")))]
#[no_mangle]
pub extern "C" fn ssbusync_instance_fallback() -> *const api::SsbuSyncApiV1 {
    remote::api_v1_ptr()
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
    SyncEnv::initialize();
    
    #[cfg(feature = "main-nro")]
    println!("[ssbusync] Main SsbuSync Module Installing. \n");
    
    let emulator = config.emulator_check.unwrap_or_else(is_emulator);
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

    #[cfg(feature = "latency-slider")]
    if (emulator && config.online_only) {
        LatencySlider::Install_Latency_Slider();
    }
    
}

fn enable_online_fix() {
    
}

fn disable_online_fix() {
    
}

#[cfg(not(feature = "main-nro"))]
pub fn Enable_Online_Fix() {
    Instance::new().Enable_Online_Fix();
}

#[cfg(not(feature = "main-nro"))]
pub fn Disable_Online_Fix() {
    Instance::new().Disable_Online_Fix();
}

#[cfg(not(feature = "main-nro"))]
pub fn Enable_Double_Buffer() {
    Instance::new().Enable_Double_Buffer();
}

#[cfg(not(feature = "main-nro"))]
pub fn Enable_Triple_Buffer() {
    Instance::new().Enable_Triple_Buffer();
}

#[cfg(not(feature = "main-nro"))]
pub fn Check_Buffer_Swap() {
    Instance::new().Check_Buffer_Swap();
}

// #[cfg(feature = "main-nro")]
// pub fn Check_Ssbusync_Disabled() -> bool {
//     // is_disabled()
// }
    
pub fn is_doubles_fix_enabled() -> bool {
    let allow_buffer_swap = (!is_emulator() && SyncEnv::allow_buffer_swap() == true);
    return allow_buffer_swap;
}

#[cfg(feature = "main-nro")]
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

#[cfg(feature = "main-nro")]
fn on_nro_load(_info: &NroInfo) {
    if !should_skip_install() {
        if external_disabler() {
            set_disabled();
            println!("[ssbusync] external symbol disabler detected; skipping install.");
            return;
        }
    }
    try_install();
}

// This always returns ok right now; because skyline lol
#[cfg(feature = "main-nro")]
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

#[cfg(feature = "main-nro")]
#[skyline::main(name = "ssbusync")]
pub fn main() {
    panic_hook();
    register_nro_hook();
}
