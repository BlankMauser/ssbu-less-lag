#![allow(unused)]
use crate::*;

gpu_api! {
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


crate::nvn_wrap_ret!(sync_initialize(arg0: *mut NvnSync, arg1: *mut NvnDevice) -> NvnBoolean => SLOT_NVN_SYNC_INITIALIZE);

crate::nvn_wrap_void!(sync_finalize(arg0: *mut NvnSync) => SLOT_NVN_SYNC_FINALIZE);

crate::nvn_wrap_void!(sync_set_debug_label(arg0: *mut NvnSync, arg1: *const u8) => SLOT_NVN_SYNC_SET_DEBUG_LABEL);

crate::nvn_wrap_ret!(sync_wait(arg0: *const NvnSync, arg1: u64) -> NvnSyncWaitResult => SLOT_NVN_SYNC_WAIT);

crate::nvn_wrap_ret!(sync_initialize_from_fenced_gl_sync(arg0: *mut NvnSync, arg1: *mut NvnDevice, arg2: u64) -> NvnBoolean => SLOT_NVN_SYNC_INITIALIZE_FROM_FENCED_GL_SYNC);

crate::nvn_wrap_ret!(sync_create_gl_sync(arg0: *mut NvnSync) -> u64 => SLOT_NVN_SYNC_CREATE_GL_SYNC);

crate::nvn_wrap_void!(event_builder_set_defaults(arg0: *mut NvnEventBuilder) => SLOT_NVN_EVENT_BUILDER_SET_DEFAULTS);

crate::nvn_wrap_void!(event_builder_set_storage(arg0: *mut NvnEventBuilder, arg1: *const NvnMemoryPool, arg2: i64) => SLOT_NVN_EVENT_BUILDER_SET_STORAGE);

crate::nvn_wrap_ret!(event_builder_get_storage(arg0: *const NvnEventBuilder) -> *const core::ffi::c_void => SLOT_NVN_EVENT_BUILDER_GET_STORAGE);

crate::nvn_wrap_ret!(event_builder_get_memory_pool(arg0: *const NvnEventBuilder) -> *const NvnMemoryPool => SLOT_NVN_EVENT_BUILDER_GET_MEMORY_POOL);

crate::nvn_wrap_ret!(event_builder_get_memory_offset(arg0: *const NvnEventBuilder) -> isize => SLOT_NVN_EVENT_BUILDER_GET_MEMORY_OFFSET);

crate::nvn_wrap_ret!(event_initialize(arg0: *mut NvnEvent, arg1: *const NvnEventBuilder) -> NvnBoolean => SLOT_NVN_EVENT_INITIALIZE);

crate::nvn_wrap_void!(event_finalize(arg0: *mut NvnEvent) => SLOT_NVN_EVENT_FINALIZE);

crate::nvn_wrap_ret!(event_get_value(arg0: *const NvnEvent) -> u32 => SLOT_NVN_EVENT_GET_VALUE);

crate::nvn_wrap_void!(event_signal(arg0: *mut NvnEvent, arg1: NvnEventSignalMode, arg2: u32) => SLOT_NVN_EVENT_SIGNAL);

crate::nvn_wrap_ret!(event_get_memory_pool(arg0: *const NvnEvent) -> *const NvnMemoryPool => SLOT_NVN_EVENT_GET_MEMORY_POOL);

crate::nvn_wrap_ret!(event_get_memory_offset(arg0: *const NvnEvent) -> isize => SLOT_NVN_EVENT_GET_MEMORY_OFFSET);

