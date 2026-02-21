#![allow(unused)]

use crate::*;

gpu_api! {
    // DeviceBuilder
    pub static nvnDeviceBuilderSetDefaults: fn(*mut NvnDeviceBuilder);
    pub static nvnDeviceBuilderSetFlags: fn(*mut NvnDeviceBuilder, i32);
    pub static nvnDeviceBuilderGetFlags: fn(*const NvnDeviceBuilder) -> NvnDeviceFlagBits;

    // Device
    pub static nvnDeviceInitialize: fn(*mut NvnDevice, *const NvnDeviceBuilder) -> NvnBoolean;
    pub static nvnDeviceFinalize: fn(*mut NvnDevice);
    pub static nvnDeviceSetDebugLabel: fn(*mut NvnDevice, *const u8);
    pub static nvnDeviceGetProcAddress: fn(*const NvnDevice, *const u8) -> PfnNvnGenericFuncPtr;
    pub static nvnDeviceGetInteger: fn(*const NvnDevice, NvnDeviceInfo, *mut i32);
    pub static nvnDeviceGetCurrentTimestampInNanoseconds: fn(*const NvnDevice) -> u64;
    pub static nvnDeviceSetIntermediateShaderCache: fn(*mut NvnDevice, i32);
    pub static nvnDeviceGetTextureHandle: fn(*const NvnDevice, i32, i32) -> NvnTextureHandle;
    pub static nvnDeviceGetTexelFetchHandle: fn(*const NvnDevice, i32) -> NvnTextureHandle;
    pub static nvnDeviceGetImageHandle: fn(*const NvnDevice, i32) -> NvnImageHandle;
    pub static nvnDeviceInstallDebugCallback: fn(*mut NvnDevice, PfnNvnDebugCallback, *mut core::ffi::c_void, NvnBoolean);
    pub static nvnDeviceGenerateDebugDomainId: fn(*const NvnDevice, *const u8) -> NvnDebugDomainId;
    pub static nvnDeviceSetWindowOriginMode: fn(*mut NvnDevice, NvnWindowOriginMode);
    pub static nvnDeviceSetDepthMode: fn(*mut NvnDevice, NvnDepthMode);
    pub static nvnDeviceRegisterFastClearColor: fn(*mut NvnDevice, *const f32, NvnFormat) -> NvnBoolean;
    pub static nvnDeviceRegisterFastClearColori: fn(*mut NvnDevice, *const i32, NvnFormat) -> NvnBoolean;
    pub static nvnDeviceRegisterFastClearColorui: fn(*mut NvnDevice, *const u32, NvnFormat) -> NvnBoolean;
    pub static nvnDeviceRegisterFastClearDepth: fn(*mut NvnDevice, f32) -> NvnBoolean;
    pub static nvnDeviceGetWindowOriginMode: fn(*const NvnDevice) -> NvnWindowOriginMode;
    pub static nvnDeviceGetDepthMode: fn(*const NvnDevice) -> NvnDepthMode;
    pub static nvnDeviceGetTimestampInNanoseconds: fn(*const NvnDevice, *const NvnCounterData) -> u64;
    pub static nvnDeviceApplyDeferredFinalizes: fn(*mut NvnDevice, i32);
    pub static nvnDeviceFinalizeCommandHandle: fn(*mut NvnDevice, NvnCommandHandle);
    pub static nvnDeviceWalkDebugDatabase: fn(*const NvnDevice, NvnDebugObjectType, PfnNvnWalkDebugDatabaseCallback, *mut core::ffi::c_void);
    pub static nvnDeviceGetSeparateTextureHandle: fn(*const NvnDevice, i32) -> NvnSeparateTextureHandle;
    pub static nvnDeviceGetSeparateSamplerHandle: fn(*const NvnDevice, i32) -> NvnSeparateSamplerHandle;
    pub static nvnDeviceIsExternalDebuggerAttached: fn(*const NvnDevice) -> NvnBoolean;
}

// Ergonomic wrappers around resolved GPU entry points.
// These keep per-domain APIs in their own modules while using one central resolver.
crate::nvn_wrap_void!(device_builder_set_defaults(arg0: *mut NvnDeviceBuilder) => SLOT_NVN_DEVICE_BUILDER_SET_DEFAULTS);

crate::nvn_wrap_void!(device_builder_set_flags(arg0: *mut NvnDeviceBuilder, arg1: i32) => SLOT_NVN_DEVICE_BUILDER_SET_FLAGS);

crate::nvn_wrap_ret!(device_builder_get_flags(arg0: *const NvnDeviceBuilder) -> NvnDeviceFlagBits => SLOT_NVN_DEVICE_BUILDER_GET_FLAGS);

crate::nvn_wrap_ret!(device_initialize(arg0: *mut NvnDevice, arg1: *const NvnDeviceBuilder) -> NvnBoolean => SLOT_NVN_DEVICE_INITIALIZE);

crate::nvn_wrap_void!(device_finalize(arg0: *mut NvnDevice) => SLOT_NVN_DEVICE_FINALIZE);

crate::nvn_wrap_void!(device_set_debug_label(arg0: *mut NvnDevice, arg1: *const u8) => SLOT_NVN_DEVICE_SET_DEBUG_LABEL);

crate::nvn_wrap_ret!(device_get_proc_address(arg0: *const NvnDevice, arg1: *const u8) -> PfnNvnGenericFuncPtr => SLOT_NVN_DEVICE_GET_PROC_ADDRESS);

crate::nvn_wrap_void!(device_get_integer(arg0: *const NvnDevice, arg1: NvnDeviceInfo, arg2: *mut i32) => SLOT_NVN_DEVICE_GET_INTEGER);

crate::nvn_wrap_ret!(device_get_current_timestamp_in_nanoseconds(arg0: *const NvnDevice) -> u64 => SLOT_NVN_DEVICE_GET_CURRENT_TIMESTAMP_IN_NANOSECONDS);

crate::nvn_wrap_void!(device_set_intermediate_shader_cache(arg0: *mut NvnDevice, arg1: i32) => SLOT_NVN_DEVICE_SET_INTERMEDIATE_SHADER_CACHE);

crate::nvn_wrap_ret!(device_get_texture_handle(arg0: *const NvnDevice, arg1: i32, arg2: i32) -> NvnTextureHandle => SLOT_NVN_DEVICE_GET_TEXTURE_HANDLE);

crate::nvn_wrap_ret!(device_get_texel_fetch_handle(arg0: *const NvnDevice, arg1: i32) -> NvnTextureHandle => SLOT_NVN_DEVICE_GET_TEXEL_FETCH_HANDLE);

crate::nvn_wrap_ret!(device_get_image_handle(arg0: *const NvnDevice, arg1: i32) -> NvnImageHandle => SLOT_NVN_DEVICE_GET_IMAGE_HANDLE);

crate::nvn_wrap_void!(device_install_debug_callback(arg0: *mut NvnDevice, arg1: PfnNvnDebugCallback, arg2: *mut core::ffi::c_void, arg3: NvnBoolean) => SLOT_NVN_DEVICE_INSTALL_DEBUG_CALLBACK);

crate::nvn_wrap_ret!(device_generate_debug_domain_id(arg0: *const NvnDevice, arg1: *const u8) -> NvnDebugDomainId => SLOT_NVN_DEVICE_GENERATE_DEBUG_DOMAIN_ID);

crate::nvn_wrap_void!(device_set_window_origin_mode(arg0: *mut NvnDevice, arg1: NvnWindowOriginMode) => SLOT_NVN_DEVICE_SET_WINDOW_ORIGIN_MODE);

crate::nvn_wrap_void!(device_set_depth_mode(arg0: *mut NvnDevice, arg1: NvnDepthMode) => SLOT_NVN_DEVICE_SET_DEPTH_MODE);

crate::nvn_wrap_ret!(device_register_fast_clear_color(arg0: *mut NvnDevice, arg1: *const f32, arg2: NvnFormat) -> NvnBoolean => SLOT_NVN_DEVICE_REGISTER_FAST_CLEAR_COLOR);

crate::nvn_wrap_ret!(device_register_fast_clear_colori(arg0: *mut NvnDevice, arg1: *const i32, arg2: NvnFormat) -> NvnBoolean => SLOT_NVN_DEVICE_REGISTER_FAST_CLEAR_COLORI);

crate::nvn_wrap_ret!(device_register_fast_clear_colorui(arg0: *mut NvnDevice, arg1: *const u32, arg2: NvnFormat) -> NvnBoolean => SLOT_NVN_DEVICE_REGISTER_FAST_CLEAR_COLORUI);

crate::nvn_wrap_ret!(device_register_fast_clear_depth(arg0: *mut NvnDevice, arg1: f32) -> NvnBoolean => SLOT_NVN_DEVICE_REGISTER_FAST_CLEAR_DEPTH);

crate::nvn_wrap_ret!(device_get_window_origin_mode(arg0: *const NvnDevice) -> NvnWindowOriginMode => SLOT_NVN_DEVICE_GET_WINDOW_ORIGIN_MODE);

crate::nvn_wrap_ret!(device_get_depth_mode(arg0: *const NvnDevice) -> NvnDepthMode => SLOT_NVN_DEVICE_GET_DEPTH_MODE);

crate::nvn_wrap_ret!(device_get_timestamp_in_nanoseconds(arg0: *const NvnDevice, arg1: *const NvnCounterData) -> u64 => SLOT_NVN_DEVICE_GET_TIMESTAMP_IN_NANOSECONDS);

crate::nvn_wrap_void!(device_apply_deferred_finalizes(arg0: *mut NvnDevice, arg1: i32) => SLOT_NVN_DEVICE_APPLY_DEFERRED_FINALIZES);

crate::nvn_wrap_void!(device_finalize_command_handle(arg0: *mut NvnDevice, arg1: NvnCommandHandle) => SLOT_NVN_DEVICE_FINALIZE_COMMAND_HANDLE);

crate::nvn_wrap_void!(device_walk_debug_database(arg0: *const NvnDevice, arg1: NvnDebugObjectType, arg2: PfnNvnWalkDebugDatabaseCallback, arg3: *mut core::ffi::c_void) => SLOT_NVN_DEVICE_WALK_DEBUG_DATABASE);

crate::nvn_wrap_ret!(device_get_separate_texture_handle(arg0: *const NvnDevice, arg1: i32) -> NvnSeparateTextureHandle => SLOT_NVN_DEVICE_GET_SEPARATE_TEXTURE_HANDLE);

crate::nvn_wrap_ret!(device_get_separate_sampler_handle(arg0: *const NvnDevice, arg1: i32) -> NvnSeparateSamplerHandle => SLOT_NVN_DEVICE_GET_SEPARATE_SAMPLER_HANDLE);

crate::nvn_wrap_ret!(device_is_external_debugger_attached(arg0: *const NvnDevice) -> NvnBoolean => SLOT_NVN_DEVICE_IS_EXTERNAL_DEBUGGER_ATTACHED);

