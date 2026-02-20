#![allow(unused)]
use crate::*;

nvn_func! {
    // ── Sync ──
    pub static nvnSyncInitialize: fn(*mut NvnSync, *mut NvnDevice) -> NvnBoolean;
    pub static nvnSyncFinalize: fn(*mut NvnSync);
    pub static nvnSyncSetDebugLabel: fn(*mut NvnSync, *const u8);
    pub static nvnSyncWait: fn(*const NvnSync, u64) -> NvnSyncWaitResult;
    // vtable-only
    pub static nvnSyncInitializeFromFencedGLSync: fn(*mut NvnSync, *mut NvnDevice, u64) -> NvnBoolean;
    pub static nvnSyncCreateGLSync: fn(*mut NvnSync) -> u64;

    // ── EventBuilder ──
    pub static nvnEventBuilderSetDefaults: fn(*mut NvnEventBuilder);
    pub static nvnEventBuilderSetStorage: fn(*mut NvnEventBuilder, *const NvnMemoryPool, i64);
    // vtable-only
    pub static nvnEventBuilderGetStorage: fn(*const NvnEventBuilder) -> *const core::ffi::c_void;
    pub static nvnEventBuilderGetMemoryPool: fn(*const NvnEventBuilder) -> *const NvnMemoryPool;
    pub static nvnEventBuilderGetMemoryOffset: fn(*const NvnEventBuilder) -> isize;

    // ── Event ──
    pub static nvnEventInitialize: fn(*mut NvnEvent, *const NvnEventBuilder) -> NvnBoolean;
    pub static nvnEventFinalize: fn(*mut NvnEvent);
    pub static nvnEventGetValue: fn(*const NvnEvent) -> u32;
    pub static nvnEventSignal: fn(*mut NvnEvent, NvnEventSignalMode, u32);
    // vtable-only
    pub static nvnEventGetMemoryPool: fn(*const NvnEvent) -> *const NvnMemoryPool;
    pub static nvnEventGetMemoryOffset: fn(*const NvnEvent) -> isize;
}
