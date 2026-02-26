use crate::api::{
    SsbuSyncApiV1, Status, SSBUSYNC_API_V1_BOOTSTRAP_SYMBOL, SSBUSYNC_INSTANCE_FALLBACK_SYMBOL,
    SSBUSYNC_STATUS_SYMBOL, SSBUSYNC_TRY_CLAIM_INSTALL_SYMBOL,
};
use skyline::nn::ro;

#[cfg(feature = "main-nro")]
use crate::conduct::SyncConductor;

pub(crate) fn lookup_symbol_addr(sym_nul: &[u8], caller: &str) -> Option<usize> {
    let mut addr = 0usize;
    unsafe {
        let rc = ro::LookupSymbol(&mut addr, sym_nul.as_ptr());
        println!(
            "[ssbusync] LookupSymbol caller={} sym={:?} rc={} addr=0x{:x}",
            caller, sym_nul, rc, addr
        );
        if rc == 0 && addr != 0 {
            Some(addr)
        } else {
            None
        }
    }
}

pub(crate) fn lookup_symbol_exists(sym_nul: &[u8], caller: &str) -> bool {
    lookup_symbol_addr(sym_nul, caller).is_some()
}

#[cfg(not(feature = "main-nro"))]
pub fn query_remote_status() -> Option<u32> {
    let addr = lookup_symbol_addr(SSBUSYNC_STATUS_SYMBOL, "query_remote_status")?;
    Some(unsafe {
        let func: extern "C" fn() -> u32 = core::mem::transmute(addr);
        func()
    })
}

#[cfg(not(feature = "main-nro"))]
fn query_remote_try_claim_install() -> Option<u32> {
    let addr = lookup_symbol_addr(
        SSBUSYNC_TRY_CLAIM_INSTALL_SYMBOL,
        "query_remote_try_claim_install",
    )?;
    Some(unsafe {
        let func: extern "C" fn() -> u32 = core::mem::transmute(addr);
        func()
    })
}

#[cfg(not(feature = "main-nro"))]
pub(crate) fn query_remote_api_v1() -> Option<*const SsbuSyncApiV1> {
    let addr = lookup_symbol_addr(SSBUSYNC_API_V1_BOOTSTRAP_SYMBOL, "query_remote_api_v1")
        .or_else(|| {
            lookup_symbol_addr(
                SSBUSYNC_INSTANCE_FALLBACK_SYMBOL,
                "query_remote_api_v1_fallback",
            )
        })?;
    let table_ptr = unsafe {
        let func: extern "C" fn() -> *const SsbuSyncApiV1 = core::mem::transmute(addr);
        func()
    };
    crate::util::compatibility::validate_api_v1_ptr(table_ptr)
}

#[cfg(not(feature = "main-nro"))]
pub(crate) fn client_status() -> Option<u32> {
    query_remote_status()
}

#[cfg(feature = "main-nro")]
pub fn status() -> u32 {
    SyncConductor::status() as u32
}

#[cfg(feature = "host-instance")]
pub fn claim_install() -> u32 {
    #[cfg(feature = "main-nro")]
    {
        return SyncConductor::claim_for_host();
    }
    #[cfg(not(feature = "main-nro"))]
    {
        match query_remote_status() {
            None => Status::INSTALLED as u32,
            Some(s) if s != Status::PENDING as u32 => s,
            Some(_) => query_remote_try_claim_install().unwrap_or(u32::MAX),
        }
    }
}

#[cfg(all(feature = "host-instance", not(rust_analyzer)))]
const RESOLVED_PREFIX: &str = symbaker::resolved_prefix!();
#[cfg(all(feature = "host-instance", not(rust_analyzer)))]
symbaker::assert_resolved_prefix_len!(16);
#[cfg(all(feature = "host-instance", rust_analyzer))]
const RESOLVED_PREFIX: &str = "";

#[cfg(feature = "host-instance")]
pub fn claim_install_with_prefix(prefix: &str) -> u32 {
    if prefix.len() > crate::util::bytes::MAX_PREFIX_LEN || !prefix.is_ascii() {
        return u32::MAX;
    }
    claim_install()
}

#[cfg(feature = "host-instance")]
pub fn claim_install_with_resolved_prefix() -> u32 {
    claim_install_with_prefix(RESOLVED_PREFIX)
}

#[cfg(feature = "host-instance")]
pub unsafe fn claim_install_with_prefix_ptr(prefix_ptr: *const u8, prefix_len: usize) -> u32 {
    if prefix_len == 0 {
        return claim_install();
    }
    if prefix_ptr.is_null() {
        return u32::MAX;
    }
    let bytes = unsafe { core::slice::from_raw_parts(prefix_ptr, prefix_len) };
    let Ok(prefix) = core::str::from_utf8(bytes) else {
        return u32::MAX;
    };
    claim_install_with_prefix(prefix)
}

#[cfg(all(feature = "host-instance", feature = "main-nro"))]
extern "C" fn api_status() -> u32 {
    status()
}

#[cfg(feature = "host-instance")]
static API_V1_TABLE: SsbuSyncApiV1 = SsbuSyncApiV1 {
    abi_version: crate::api::API_V1_ABI_VERSION,
    struct_size: core::mem::size_of::<SsbuSyncApiV1>() as u32,
    status_fn: {
        #[cfg(feature = "main-nro")]
        {
            Some(api_status)
        }
        #[cfg(not(feature = "main-nro"))]
        {
            None
        }
    },
    reserved_fn_0: None,
    reserved_fn_1: None,
    reserved_fn_2: None,
    reserved_fn_3: None,
    reserved_fn_4: None,
    reserved_fn_5: None,
    reserved_fn_6: None,
    reserved_fn_7: None,
};

#[cfg(feature = "host-instance")]
pub fn api_v1_ptr() -> *const SsbuSyncApiV1 {
    #[cfg(feature = "main-nro")]
    if status() == Status::PENDING as u32 {
        return core::ptr::null();
    }
    core::ptr::addr_of!(API_V1_TABLE)
}

/// Stable bootstrap symbol for API-table negotiation.
#[cfg(all(feature = "host-instance", feature = "main-nro"))]
#[no_mangle]
pub extern "C" fn ssbusync_api_v1_bootstrap() -> *const SsbuSyncApiV1 {
    api_v1_ptr()
}
