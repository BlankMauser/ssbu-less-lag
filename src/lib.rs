#![allow(warnings)]
use std::sync::atomic::{AtomicU64, AtomicU8, Ordering};
use std::sync::OnceLock;
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
use symbaker::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "nro-entry")]
use crate::compatibility::SSBUSyncHost::*;
#[cfg(all(feature = "nro-entry", feature = "host-instance"))]
use crate::compatibility::Status::{CLAIMED, PENDING};

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

#[cfg(feature = "host-instance")]
pub struct Instance;

#[cfg(feature = "host-instance")]
struct InstanceState {
    // Packed ASCII host prefix split across two u64 words.
    cached_host_prefix_lo: AtomicU64,
    cached_host_prefix_hi: AtomicU64,
    cached_host_prefix_len: AtomicU8,
}

#[cfg(feature = "host-instance")]
impl InstanceState {
    const fn new() -> Self {
        Self {
            cached_host_prefix_lo: AtomicU64::new(0),
            cached_host_prefix_hi: AtomicU64::new(0),
            cached_host_prefix_len: AtomicU8::new(0),
        }
    }
}

#[cfg(feature = "host-instance")]
static INSTANCE_STATE: OnceLock<InstanceState> = OnceLock::new();

#[cfg(feature = "host-instance")]
const PREFIX_MAX_LEN: usize = util::bytes::MAX_PREFIX_LEN;

#[cfg(all(feature = "host-instance", not(rust_analyzer)))]
const RESOLVED_PREFIX: &str = symbaker::resolved_prefix!();
#[cfg(all(feature = "host-instance", not(rust_analyzer)))]
symbaker::assert_resolved_prefix_len!(16);

#[cfg(all(feature = "host-instance", rust_analyzer))]
const RESOLVED_PREFIX: &str = "";

#[cfg(feature = "host-instance")]
fn store_cached_prefix(prefix: &str) -> bool {
    let Some((lo, hi, len)) = util::bytes::split_ascii_prefix(prefix) else {
        return false;
    };
    let state = INSTANCE_STATE.get_or_init(InstanceState::new);
    state.cached_host_prefix_lo.store(lo, Ordering::Release);
    state.cached_host_prefix_hi.store(hi, Ordering::Release);
    state.cached_host_prefix_len.store(len, Ordering::Release);
    true
}

#[cfg(feature = "host-instance")]
fn read_cached_prefix_parts() -> (u64, u64, usize) {
    match INSTANCE_STATE.get() {
        Some(state) => (
            state.cached_host_prefix_lo.load(Ordering::Acquire),
            state.cached_host_prefix_hi.load(Ordering::Acquire),
            state.cached_host_prefix_len.load(Ordering::Acquire) as usize,
        ),
        None => (0, 0, 0),
    }
}

#[cfg(feature = "host-instance")]
fn build_host_info() -> compatibility::SsbuSyncHostInfo {
    let mut info = compatibility::SsbuSyncHostInfo::empty();
    #[cfg(feature = "nro-entry")]
    {
        info.status = STATUS.load(Ordering::Acquire) as u32;
    }
    #[cfg(not(feature = "nro-entry"))]
    {
        info.status = 0;
    }

    let (lo, hi, len) = read_cached_prefix_parts();
    let capped_len = util::bytes::write_prefix_bytes(lo, hi, len, &mut info.prefix);
    info.prefix_len = capped_len as u32;
    info
}

#[cfg(feature = "host-instance")]
impl Instance {
    pub const fn new() -> Self {
        Self
    }

    pub fn init(self) -> u32 {
        let _ = INSTANCE_STATE.get_or_init(InstanceState::new);
        #[cfg(feature = "nro-entry")]
        {
            return STATUS.load(Ordering::Acquire) as u32;
        }
        #[cfg(not(feature = "nro-entry"))]
        {
            0
        }
    }

    pub fn claim_install_with_prefix(self, prefix: &str) -> u32 {
        if !store_cached_prefix(prefix) {
            return u32::MAX;
        }
        ssbusync_claim_install()
    }

    pub fn claim_install_with_resolved_prefix(self) -> u32 {
        self.claim_install_with_prefix(RESOLVED_PREFIX)
    }

    pub fn cached_prefix_len(self) -> usize {
        let (_, _, len) = read_cached_prefix_parts();
        len
    }
}

#[cfg(feature = "host-instance")]
#[symbaker]
pub extern "C" fn ssbusync_status() -> u32 {
    #[cfg(feature = "nro-entry")]
    {
        STATUS.load(Ordering::Acquire) as u32
    }
    #[cfg(not(feature = "nro-entry"))]
    {
        0
    }
}

#[cfg(feature = "host-instance")]
#[symbaker]
pub extern "C" fn init_ssbusync_instance() -> u32 {
    Instance::new().init()
}

#[cfg(feature = "host-instance")]
#[symbaker]
pub extern "C" fn ssbusync_claim_install() -> u32 {
    #[cfg(feature = "nro-entry")]
    {
        let current = STATUS.load(Ordering::Acquire);
        if current == CLAIMED {
            return current as u32;
        }

        match STATUS.compare_exchange(PENDING, CLAIMED, Ordering::AcqRel, Ordering::Acquire) {
            Ok(_) => CLAIMED as u32,
            Err(existing) => existing as u32,
        }
    }
    #[cfg(not(feature = "nro-entry"))]
    {
        0
    }
}

#[cfg(feature = "host-instance")]
#[symbaker]
pub extern "C" fn ssbusync_claim_install_resolved_prefix() -> u32 {
    Instance::new().claim_install_with_resolved_prefix()
}

#[cfg(feature = "host-instance")]
#[symbaker]
pub unsafe extern "C" fn ssbusync_claim_install_with_prefix_ptr(
    prefix_ptr: *const u8,
    prefix_len: usize,
) -> u32 {
    if prefix_len == 0 {
        return ssbusync_claim_install();
    }
    if prefix_ptr.is_null() {
        return u32::MAX;
    }
    let bytes = unsafe { core::slice::from_raw_parts(prefix_ptr, prefix_len) };
    let Ok(prefix) = core::str::from_utf8(bytes) else {
        return u32::MAX;
    };
    if !store_cached_prefix(prefix) {
        return u32::MAX;
    }
    ssbusync_claim_install()
}

/// Bootstrap symbol for clients: returns status + cached prefix in one call.
#[cfg(feature = "host-instance")]
#[no_mangle]
pub extern "C" fn ssbusync_host_info_bootstrap(
    out_info: *mut compatibility::SsbuSyncHostInfo,
) -> u32 {
    if out_info.is_null() {
        return u32::MAX;
    }
    unsafe {
        *out_info = build_host_info();
    }
    0
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
    
    #[cfg(feature = "nro-entry")]
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

#[cfg(not(feature = "nro-entry"))]
pub fn Enable_Online_Fix() {

}

#[cfg(not(feature = "nro-entry"))]
pub fn Disable_Online_Fix() {

}

#[cfg(not(feature = "nro-entry"))]
pub fn Enable_Double_Buffer() {
    let allow_buffer_swap = (!is_emulator() && SyncEnv::allow_buffer_swap());
    if allow_buffer_swap {
    start_swap_buffer(BufferMode::Double);
    } else {
        println!("[ssbusync] Swapping Buffer Mode Not Allowed!");
    }
}

#[cfg(not(feature = "nro-entry"))]
pub fn Enable_Triple_Buffer() {
    let allow_buffer_swap = (!is_emulator() && SyncEnv::allow_buffer_swap());
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
