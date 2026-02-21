use core::sync::atomic::{AtomicBool, Ordering};
use skyline::nn::ro;
use skyline::nro::NroInfo;

type SetEnabledFn = extern "C" fn(u32);
type RequestDisableFn = extern "C" fn() -> u32;
type RegisterDisablerFn = extern "C" fn() -> u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheStatus {
    Ignored,
    Cached,
    Missing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisableResult {
    Disabled,
    Indeterminate,
    NotAvailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverrideAction {
    None,
    InstallCustom,
}

pub struct OverrideState {
    saw_ssbusync: bool,
    did_disable: bool,
    decided: bool,
}

impl OverrideState {
    pub const fn new() -> Self {
        Self {
            saw_ssbusync: false,
            did_disable: false,
            decided: false,
        }
    }
}

pub static CUSTOM_INSTALL_CLAIMED: AtomicBool = AtomicBool::new(false);

const SSBUSYNC_SET_ENABLED_SYM: &[u8] = b"ssbusync_set_enabled\0";
const SSBUSYNC_REQUEST_DISABLE_SYM: &[u8] = b"ssbusync_request_disable\0";
const SSBUSYNC_REGISTER_DISABLER_SYM: &[u8] = b"ssbusync_register_disabler\0";
pub const SSBUSYNC_EXTERNAL_DISABLER_PROBE_SYM: &[u8] = b"ssbusync_external_disabler\0";

fn lookup_symbol_addr(sym_nul: &[u8]) -> Option<usize> {
    let mut addr = 0usize;
    unsafe {
        if ro::LookupSymbol(&mut addr, sym_nul.as_ptr()) == 0 && addr != 0 {
            Some(addr)
        } else {
            None
        }
    }
}

fn lookup_symbol_exists(sym_nul: &[u8]) -> bool {
    lookup_symbol_addr(sym_nul).is_some()
}

pub unsafe fn observe_ssbusync_set_enabled(info: &NroInfo) -> CacheStatus {
    if info.name != "ssbusync" {
        return CacheStatus::Ignored;
    }

    if lookup_symbol_exists(SSBUSYNC_SET_ENABLED_SYM) {
        CacheStatus::Cached
    } else {
        CacheStatus::Missing
    }
}

pub unsafe fn observe_ssbusync_request_disable(info: &NroInfo) -> CacheStatus {
    if info.name != "ssbusync" {
        return CacheStatus::Ignored;
    }

    if lookup_symbol_exists(SSBUSYNC_REQUEST_DISABLE_SYM) {
        CacheStatus::Cached
    } else {
        CacheStatus::Missing
    }
}

pub unsafe fn observe_ssbusync_register_disabler(info: &NroInfo) -> CacheStatus {
    if info.name != "ssbusync" {
        return CacheStatus::Ignored;
    }

    if lookup_symbol_exists(SSBUSYNC_REGISTER_DISABLER_SYM) {
        CacheStatus::Cached
    } else {
        CacheStatus::Missing
    }
}

pub unsafe fn try_cache_ssbusync_exports_global() -> bool {
    lookup_symbol_exists(SSBUSYNC_REGISTER_DISABLER_SYM)
        || lookup_symbol_exists(SSBUSYNC_REQUEST_DISABLE_SYM)
        || lookup_symbol_exists(SSBUSYNC_SET_ENABLED_SYM)
}

pub fn external_disabler_wants_disable() -> bool {
    lookup_symbol_exists(SSBUSYNC_EXTERNAL_DISABLER_PROBE_SYM)
}

fn call_lookup_set_enabled(enabled: bool) -> bool {
    let Some(addr) = lookup_symbol_addr(SSBUSYNC_SET_ENABLED_SYM) else {
        return false;
    };

    let func: Option<SetEnabledFn> = unsafe { core::mem::transmute(addr) };
    if let Some(func) = func {
        func(enabled as u32);
        return true;
    }

    false
}

fn call_lookup_request_disable() -> Option<u32> {
    let addr = lookup_symbol_addr(SSBUSYNC_REQUEST_DISABLE_SYM)?;
    let func: Option<RequestDisableFn> = unsafe { core::mem::transmute(addr) };
    func.map(|f| f())
}

fn call_lookup_register_disabler() -> Option<u32> {
    let addr = lookup_symbol_addr(SSBUSYNC_REGISTER_DISABLER_SYM)?;
    let func: Option<RegisterDisablerFn> = unsafe { core::mem::transmute(addr) };
    func.map(|f| f())
}

pub fn disable_ssbusync_if_cached() -> DisableResult {
    if let Some(result) = call_lookup_register_disabler() {
        if result != 0 {
            return DisableResult::Disabled;
        }
        return DisableResult::Indeterminate;
    }

    if let Some(result) = call_lookup_request_disable() {
        if result != 0 {
            return DisableResult::Disabled;
        }
        return DisableResult::Indeterminate;
    }

    // Older builds may not export request_disable; this remains best-effort only.
    if call_lookup_set_enabled(false) {
        return DisableResult::Indeterminate;
    }

    DisableResult::NotAvailable
}

// Attempts to claim ssbu sync install once.
// Returns Disabled only for one disabler.
pub unsafe fn try_claim_external_disabler() -> DisableResult {
    let _ = try_cache_ssbusync_exports_global();
    disable_ssbusync_if_cached()
}

pub fn claim_custom_install_once() -> bool {
    !CUSTOM_INSTALL_CLAIMED.swap(true, Ordering::AcqRel)
}

pub fn reset_custom_install_claim() {
    CUSTOM_INSTALL_CLAIMED.store(false, Ordering::Release);
}

// High-level hook helper with built-in logging for the three common cases:
// 1) ssbusync exists, no disablers
// 2) ssbusync exists, disabler called disable
// 3) ssbusync not present, custom install should proceed
pub unsafe fn observe_and_decide_override(info: &NroInfo, state: &mut OverrideState) -> OverrideAction {
    if state.decided {
        return OverrideAction::None;
    }

    if info.name == "ssbusync" {
        state.saw_ssbusync = true;
        let set_status = observe_ssbusync_set_enabled(info);
        let req_status = observe_ssbusync_request_disable(info);
        let reg_status = observe_ssbusync_register_disabler(info);
        match disable_ssbusync_if_cached() {
            DisableResult::Disabled => {
                state.did_disable = true;
                println!("[ssbusync-compat] ssbusync exists: disable accepted.");
            }
            DisableResult::Indeterminate => {
                println!(
                    "[ssbusync-compat] ssbusync detected, but disable was late/indeterminate; skipping custom install."
                );
            }
            DisableResult::NotAvailable => {
                if set_status == CacheStatus::Missing
                    || req_status == CacheStatus::Missing
                    || reg_status == CacheStatus::Missing
                {
                    println!("[ssbusync-compat] ssbusync loaded, but expected exports are missing.");
                }
            }
        }
        return OverrideAction::None;
    }

    if info.name != "unknown" {
        return OverrideAction::None;
    }

    // If we missed ssbusync's own load callback due to load order, do a late global lookup.
    if !state.saw_ssbusync && try_cache_ssbusync_exports_global() {
        state.saw_ssbusync = true;
        match disable_ssbusync_if_cached() {
            DisableResult::Disabled => {
                state.did_disable = true;
                println!("[ssbusync-compat] ssbusync found late: disable accepted.");
            }
            DisableResult::Indeterminate => {
                println!(
                    "[ssbusync-compat] ssbusync found late, but disable was late/indeterminate; skipping custom install."
                );
            }
            DisableResult::NotAvailable => {}
        }
    }

    state.decided = true;
    if state.saw_ssbusync {
        if state.did_disable {
            println!("[ssbusync-compat] ssbusync disabled -> install custom");
            OverrideAction::InstallCustom
        } else {
            println!("[ssbusync-compat] no disablers -> ssbusync install");
            OverrideAction::None
        }
    } else {
        println!("[ssbusync-compat] ssbusync missing -> install custom");
        OverrideAction::InstallCustom
    }
}

// Guarantees InstallCustom is emitted only once.
pub unsafe fn observe_and_claim_override(info: &NroInfo, state: &mut OverrideState) -> OverrideAction {
    let action = observe_and_decide_override(info, state);
    if action == OverrideAction::InstallCustom && !claim_custom_install_once() {
        return OverrideAction::None;
    }
    action
}

// Recommended disabler flow:
//
// 1) Register exactly one NRO load hook in your plugin startup.
// 2) Do an immediate global lookup + disable attempt in startup.
// 3) Send every NRO event to observe_and_decide_override(...).
// 4) Only install custom path if action == InstallCustom.
//
// Safety notes:
// - on_nro_load runs for every NRO, but this helper ignores unrelated module names.
// - state.decided ensures the decision is only taken once.
// - Your own custom install should also be one-shot guarded.
//
// static mut OVERRIDE_STATE: ssbusync::compatibility::OverrideState =
//     ssbusync::compatibility::OverrideState::new();
//
// fn compat_init() {
//     let _ = unsafe { ssbusync::compatibility::try_claim_external_disabler() };
//
//     skyline::nro::add_hook(on_nro_load).expect("nro hook unavailable");
// }
//
// fn on_nro_load(info: &skyline::nro::NroInfo) {
//     let action = unsafe {
//         ssbusync::compatibility::observe_and_claim_override(info, &mut OVERRIDE_STATE)
//     };
//
//     if action == ssbusync::compatibility::OverrideAction::InstallCustom {
//         println!("[my-plugin] installing custom ssbusync path");
//         unsafe { ssbusync::Install_SSBU_Sync(ssbusync::SsbuSyncConfig::default()) };
//     }
// }
