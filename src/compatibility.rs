use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use skyline::nn::ro;
use skyline::nro::NroInfo;

type SetEnabledFn = extern "C" fn(u32);
type RequestDisableFn = extern "C" fn() -> u32;
type RegisterDisablerFn = extern "C" fn() -> u32;

#[repr(C)]
struct Mod0Header {
    magic: [u8; 4],
    dynamic_off: u32,
    bss_start_off: u32,
    bss_end_off: u32,
    eh_frame_hdr_start_off: u32,
    eh_frame_hdr_end_off: u32,
    module_runtime_off: u32,
}

#[repr(C)]
struct Elf64Dyn {
    d_tag: i64,
    d_val: u64,
}

#[repr(C)]
struct Elf64Sym {
    st_name: u32,
    st_info: u8,
    st_other: u8,
    st_shndx: u16,
    st_value: u64,
    st_size: u64,
}

const DT_NULL: i64 = 0;
const DT_HASH: i64 = 4;
const DT_STRTAB: i64 = 5;
const DT_SYMTAB: i64 = 6;
const DT_STRSZ: i64 = 10;

pub struct ExportCache {
    addr: AtomicUsize,
}

impl ExportCache {
    pub const fn new() -> Self {
        Self {
            addr: AtomicUsize::new(0),
        }
    }

    pub fn get(&self) -> Option<usize> {
        let addr = self.addr.load(Ordering::Acquire);
        if addr == 0 {
            None
        } else {
            Some(addr)
        }
    }

    pub fn set(&self, addr: usize) {
        if addr != 0 {
            self.addr.store(addr, Ordering::Release);
        }
    }

    pub fn clear(&self) {
        self.addr.store(0, Ordering::Release);
    }
}

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

pub static SSBUSYNC_SET_ENABLED_CACHE: ExportCache = ExportCache::new();
pub static SSBUSYNC_REQUEST_DISABLE_CACHE: ExportCache = ExportCache::new();
pub static SSBUSYNC_REGISTER_DISABLER_CACHE: ExportCache = ExportCache::new();
pub static CUSTOM_INSTALL_CLAIMED: AtomicBool = AtomicBool::new(false);

const SSBUSYNC_SET_ENABLED_SYM: &[u8] = b"ssbusync_set_enabled\0";
const SSBUSYNC_REQUEST_DISABLE_SYM: &[u8] = b"ssbusync_request_disable\0";
const SSBUSYNC_REGISTER_DISABLER_SYM: &[u8] = b"ssbusync_register_disabler\0";

unsafe fn cstr_eq(ptr: *const u8, wanted_nul: &[u8]) -> bool {
    let mut i = 0usize;
    loop {
        if i >= wanted_nul.len() {
            return false;
        }

        let a = *ptr.add(i);
        let b = wanted_nul[i];
        if a != b {
            return false;
        }
        if b == 0 {
            return true;
        }

        i += 1;
    }
}

// Resolves `sym_nul` directly from module image without nn::ro::Lookup* calls.
// `sym_nul` must be null-terminated, e.g. b"ssbusync_set_enabled\0".
pub unsafe fn resolve_export(module: &ro::Module, sym_nul: &[u8]) -> Option<usize> {
    let mut out = 0usize;
    if ro::LookupModuleSymbol(&mut out, module as *const ro::Module, sym_nul.as_ptr()) == 0 && out != 0 {
        return Some(out);
    }

    let base = module.NroPtr as *const u8;
    if base.is_null() {
        return None;
    }

    let nro = &*(base as *const ro::NroHeader);
    let mod0 = base.add(nro.mod_offset as usize) as *const Mod0Header;

    if (*mod0).magic != *b"MOD0" {
        return None;
    }

    let dyn_ptr = (mod0 as *const u8).add((*mod0).dynamic_off as usize) as *const Elf64Dyn;

    let mut symtab: *const Elf64Sym = core::ptr::null();
    let mut strtab: *const u8 = core::ptr::null();
    let mut strsz: usize = 0;
    let mut hash: *const u32 = core::ptr::null();

    for i in 0..512usize {
        let d = &*dyn_ptr.add(i);
        if d.d_tag == DT_NULL {
            break;
        }

        match d.d_tag {
            DT_SYMTAB => symtab = base.add(d.d_val as usize) as *const Elf64Sym,
            DT_STRTAB => strtab = base.add(d.d_val as usize),
            DT_STRSZ => strsz = d.d_val as usize,
            DT_HASH => hash = base.add(d.d_val as usize) as *const u32,
            _ => {}
        }
    }

    if symtab.is_null() || strtab.is_null() || hash.is_null() || strsz == 0 {
        return None;
    }

    let nchain = *hash.add(1) as usize;
    for i in 0..nchain {
        let s = &*symtab.add(i);
        let st_name = s.st_name as usize;
        if st_name >= strsz {
            continue;
        }

        if cstr_eq(strtab.add(st_name), sym_nul) {
            let addr = (base as usize).wrapping_add(s.st_value as usize);
            if addr != 0 {
                return Some(addr);
            }
        }
    }

    None
}

pub unsafe fn observe_and_cache_export(
    info: &NroInfo,
    module_name: &str,
    symbol_nul: &'static [u8],
    cache: &ExportCache,
) -> CacheStatus {
    if info.name != module_name {
        return CacheStatus::Ignored;
    }

    if cache.get().is_some() {
        return CacheStatus::Cached;
    }

    let module = info.module as *const ro::Module;
    let resolved = unsafe { resolve_export(&*module, symbol_nul) };

    if let Some(addr) = resolved {
        cache.set(addr);
        CacheStatus::Cached
    } else {
        CacheStatus::Missing
    }
}

pub unsafe fn observe_ssbusync_set_enabled(info: &NroInfo) -> CacheStatus {
    observe_and_cache_export(
        info,
        "ssbusync",
        SSBUSYNC_SET_ENABLED_SYM,
        &SSBUSYNC_SET_ENABLED_CACHE,
    )
}

pub unsafe fn observe_ssbusync_request_disable(info: &NroInfo) -> CacheStatus {
    observe_and_cache_export(
        info,
        "ssbusync",
        SSBUSYNC_REQUEST_DISABLE_SYM,
        &SSBUSYNC_REQUEST_DISABLE_CACHE,
    )
}

pub unsafe fn observe_ssbusync_register_disabler(info: &NroInfo) -> CacheStatus {
    observe_and_cache_export(
        info,
        "ssbusync",
        SSBUSYNC_REGISTER_DISABLER_SYM,
        &SSBUSYNC_REGISTER_DISABLER_CACHE,
    )
}

pub unsafe fn try_cache_ssbusync_exports_global() -> bool {
    unsafe {
        if SSBUSYNC_REGISTER_DISABLER_CACHE.get().is_none() {
            let mut addr = 0usize;
            if ro::LookupSymbol(&mut addr, SSBUSYNC_REGISTER_DISABLER_SYM.as_ptr()) == 0 && addr != 0 {
                SSBUSYNC_REGISTER_DISABLER_CACHE.set(addr);
            }
        }

        if SSBUSYNC_REQUEST_DISABLE_CACHE.get().is_none() {
            let mut addr = 0usize;
            if ro::LookupSymbol(&mut addr, SSBUSYNC_REQUEST_DISABLE_SYM.as_ptr()) == 0 && addr != 0 {
                SSBUSYNC_REQUEST_DISABLE_CACHE.set(addr);
            }
        }

        if SSBUSYNC_SET_ENABLED_CACHE.get().is_none() {
            let mut addr = 0usize;
            if ro::LookupSymbol(&mut addr, SSBUSYNC_SET_ENABLED_SYM.as_ptr()) == 0 && addr != 0 {
                SSBUSYNC_SET_ENABLED_CACHE.set(addr);
            }
        }
    }

    SSBUSYNC_REGISTER_DISABLER_CACHE.get().is_some()
        || SSBUSYNC_REQUEST_DISABLE_CACHE.get().is_some()
        || SSBUSYNC_SET_ENABLED_CACHE.get().is_some()
}

pub fn call_cached_set_enabled(cache: &ExportCache, enabled: bool) -> bool {
    let Some(addr) = cache.get() else {
        return false;
    };

    let func: Option<SetEnabledFn> = unsafe { core::mem::transmute(addr) };
    if let Some(func) = func {
        func(enabled as u32);
        return true;
    }

    false
}

pub fn call_cached_request_disable(cache: &ExportCache) -> Option<u32> {
    let Some(addr) = cache.get() else {
        return None;
    };

    let func: Option<RequestDisableFn> = unsafe { core::mem::transmute(addr) };
    func.map(|f| f())
}

pub fn call_cached_register_disabler(cache: &ExportCache) -> Option<u32> {
    let Some(addr) = cache.get() else {
        return None;
    };

    let func: Option<RegisterDisablerFn> = unsafe { core::mem::transmute(addr) };
    func.map(|f| f())
}

pub fn disable_ssbusync_if_cached() -> DisableResult {
    if let Some(result) = call_cached_register_disabler(&SSBUSYNC_REGISTER_DISABLER_CACHE) {
        if result != 0 {
            return DisableResult::Disabled;
        }
        return DisableResult::Indeterminate;
    }

    if let Some(result) = call_cached_request_disable(&SSBUSYNC_REQUEST_DISABLE_CACHE) {
        if result != 0 {
            return DisableResult::Disabled;
        }
        return DisableResult::Indeterminate;
    }

    // Older builds may not export request_disable; this remains best-effort only.
    if call_cached_set_enabled(&SSBUSYNC_SET_ENABLED_CACHE, false) {
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

// Returns true exactly once per process lifetime (or until reset_custom_install_claim()).
pub fn claim_custom_install_once() -> bool {
    !CUSTOM_INSTALL_CLAIMED.swap(true, Ordering::AcqRel)
}

// Optional for hot-reload/testing paths.
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

    if info.name != "common" {
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
