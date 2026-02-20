#![allow(unused)]

use crate::*;

nvn_func! {
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
