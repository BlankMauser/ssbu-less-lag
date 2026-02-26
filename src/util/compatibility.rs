#[cfg(not(feature = "main-nro"))]
use crate::api::{SsbuSyncApiHeader, SsbuSyncApiV1, API_V1_ABI_VERSION};

#[cfg(not(feature = "main-nro"))]
pub fn validate_api_v1_ptr(api_ptr: *const SsbuSyncApiV1) -> Option<*const SsbuSyncApiV1> {
    if api_ptr.is_null() {
        return None;
    }

    let header = unsafe { (api_ptr as *const SsbuSyncApiHeader).read_unaligned() };
    if header.abi_version != API_V1_ABI_VERSION {
        return None;
    }
    let host_size = header.struct_size as usize;
    if host_size < core::mem::size_of::<SsbuSyncApiHeader>() {
        return None;
    }
    Some(api_ptr)
}
