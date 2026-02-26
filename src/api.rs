pub const SSBUSYNC_EXPORTED_DISABLE_SYMBOL: &[u8] = b"ssbusync_external_disabler\0";
pub const SSBUSYNC_STATUS_SYMBOL: &[u8] = b"ssbusync_status\0";
pub const SSBUSYNC_TRY_CLAIM_INSTALL_SYMBOL: &[u8] = b"ssbusync_try_claim_install\0";
pub const SSBUSYNC_API_V1_BOOTSTRAP_SYMBOL: &[u8] = b"ssbusync_api_v1_bootstrap\0";
pub const SSBUSYNC_INSTANCE_FALLBACK_SYMBOL: &[u8] = b"ssbusync_instance_fallback\0";
pub const API_V1_ABI_VERSION: u32 = 1;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SsbuSyncApiHeader {
    pub abi_version: u32,
    pub struct_size: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SsbuSyncApiV1 {
    // Bump when layout/meaning changes incompatibly.
    pub abi_version: u32,
    // Host-populated size allows newer clients to read only fields that exist.
    pub struct_size: u32,
    pub status_fn: Option<extern "C" fn() -> u32>,
    // Reserved V1 expansion slots.
    // Keep these in V1 so we can add simple no-arg `-> u32` APIs without V2.
    // If you need a different signature, add a new typed field at the end
    // (still V1-compatible as long as old fields are unchanged), or define V2.
    pub reserved_fn_0: Option<extern "C" fn() -> u32>,
    pub reserved_fn_1: Option<extern "C" fn() -> u32>,
    pub reserved_fn_2: Option<extern "C" fn() -> u32>,
    pub reserved_fn_3: Option<extern "C" fn() -> u32>,
    pub reserved_fn_4: Option<extern "C" fn() -> u32>,
    pub reserved_fn_5: Option<extern "C" fn() -> u32>,
    pub reserved_fn_6: Option<extern "C" fn() -> u32>,
    pub reserved_fn_7: Option<extern "C" fn() -> u32>,
}

// How to add a new V1 API call (no UB, backward-safe):
// 1) Append a new function pointer field to `SsbuSyncApiV1` (do not reorder/remove old fields).
// 2) Implement the host-side shim (`extern "C" fn ...`).
// 3) Fill the field in the host's `API_V1_TABLE` (or `None` when unsupported).
// 4) Add a guarded client wrapper that checks table size/field presence before calling.
// 5) Keep `API_V1_ABI_VERSION` at 1 for append-only changes.
// 6) Bump to V2 for incompatible changes.

/// Status values for the ssbusync plugin lifecycle.
pub mod Status {
    /// Initial state — not yet installed or claimed
    pub const PENDING: u8 = 0;
    /// Another plugin has claimed ownership
    pub const CLAIMED: u8 = 1;
    /// Installed and running
    pub const INSTALLED: u8 = 2;
    /// Disabled by an external disabler symbol or explicit request
    pub const DISABLED: u8 = 3;
}
