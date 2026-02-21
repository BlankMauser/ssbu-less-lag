use core::sync::atomic::{AtomicBool, Ordering};
use skyline::nn::ro;
use skyline::nro::NroInfo;

type RegisterDisablerFn = unsafe extern "C" fn() -> u32;
type IsEnabledFn = unsafe extern "C" fn() -> u32;

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

const SSBUSYNC_REGISTER_DISABLER_SYM: &[u8] = b"ssbusync_register_disabler\0";
const SSBUSYNC_IS_ENABLED_SYM: &[u8] = b"ssbusync_is_enabled\0";
// Exported by a plugin that is NOT ssbusync.nro. If present, ssbusync.nro disables itself.
pub const SSBUSYNC_EXTERNAL_DISABLER_PROBE_SYM: &[u8] = b"ssbusync_external_disabler\0";

fn lookup_symbol_addr(sym_nul: &[u8]) -> Option<usize> {
    let mut addr = 0usize;
    unsafe {
        if ro::LookupSymbol(&mut addr, sym_nul.as_ptr()) == 0 {
            Some(addr)
        } else {
            None
        }
    }
}

fn lookup_symbol_exists(sym_nul: &[u8]) -> bool {
    matches!(lookup_symbol_addr(sym_nul), Some(addr) if addr != 0)
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
}

pub fn exported_disabler_symbol_present() -> bool {
    lookup_symbol_exists(SSBUSYNC_EXTERNAL_DISABLER_PROBE_SYM)
}

fn call_lookup_register_disabler() -> Option<u32> {
    let addr = lookup_symbol_addr(SSBUSYNC_REGISTER_DISABLER_SYM)?;
    // Use Option<fn> niche semantics: a 0 symbol address becomes None.
    let func: Option<RegisterDisablerFn> = unsafe { core::mem::transmute(addr) };
    func.map(|f| unsafe { f() })
}

fn call_lookup_is_enabled() -> Option<u32> {
    let addr = lookup_symbol_addr(SSBUSYNC_IS_ENABLED_SYM)?;
    let func: Option<IsEnabledFn> = unsafe { core::mem::transmute(addr) };
    func.map(|f| unsafe { f() })
}

pub fn disable_ssbusync_if_cached() -> DisableResult {
    if let Some(result) = call_lookup_register_disabler() {
        if result != 0 {
            return DisableResult::Disabled;
        }
        if matches!(call_lookup_is_enabled(), Some(0)) {
            // Another disabler got here first; ssbusync is already disabled.
            return DisableResult::Disabled;
        }
        return DisableResult::Indeterminate;
    }

    DisableResult::NotAvailable
}

// Tries to disable ssbusync through the explicit register_disabler API.
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
        let reg_status = observe_ssbusync_register_disabler(info);
        match disable_ssbusync_if_cached() {
            DisableResult::Disabled => {
                state.did_disable = true;
                state.decided = true;
                println!("[ssbusync-compat] ssbusync exists: disable accepted -> install custom.");
                return OverrideAction::InstallCustom;
            }
            DisableResult::Indeterminate => {
                println!(
                    "[ssbusync-compat] ssbusync detected, but disable was late/indeterminate; skipping custom install."
                );
            }
            DisableResult::NotAvailable => {
                if reg_status == CacheStatus::Missing {
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
