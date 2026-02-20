#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
#![no_std]

pub mod device;
pub mod queue;
pub mod window;
pub mod mem;
pub mod resource;
pub mod cmdbuf;
pub mod sync;

// ── Opaque NVN object types (used only behind pointers) ──

macro_rules! opaque {
    ($($name:ident),* $(,)?) => {
        $(
            #[repr(C)]
            pub struct $name {
                _opaque: [u8; 0],
            }
        )*
    };
}

opaque!(
    NvnDeviceBuilder,
    NvnDevice,
    NvnQueueBuilder,
    NvnQueue,
    NvnWindowBuilder,
    NvnWindow,
    NvnMemoryPoolBuilder,
    NvnMemoryPool,
    NvnBufferBuilder,
    NvnBuffer,
    NvnTextureBuilder,
    NvnTexture,
    NvnTextureView,
    NvnTexturePool,
    NvnSamplerBuilder,
    NvnSampler,
    NvnSamplerPool,
    NvnProgram,
    NvnCommandBuffer,
    NvnSync,
    NvnEventBuilder,
    NvnEvent,
    NvnBlendState,
    NvnColorState,
    NvnChannelMaskState,
    NvnMultisampleState,
    NvnPolygonState,
    NvnDepthStencilState,
    NvnVertexAttribState,
    NvnVertexStreamState,
);

// ── Handle / value types ──

pub type NvnBoolean = u8;
pub type NvnTextureHandle = u64;
pub type NvnImageHandle = u64;
pub type NvnSeparateTextureHandle = u64;
pub type NvnSeparateSamplerHandle = u64;
pub type NvnBufferAddress = u64;
pub type NvnCommandHandle = u64;
pub type NvnNativeWindow = u64;
pub type NvnTextureAddress = u64;
pub type NvnDebugDomainId = u32;

// ── Enum type aliases (actual values are opaque integers) ──

pub type NvnDeviceInfo = i32;
pub type NvnDeviceFlagBits = i32;
pub type NvnFormat = i32;
pub type NvnTextureTarget = i32;
pub type NvnTextureFlags = i32;
pub type NvnTextureSwizzle = i32;
pub type NvnTextureDepthStencilMode = i32;
pub type NvnStorageClass = i32;
pub type NvnMemoryPoolFlags = i32;
pub type NvnWindowOriginMode = i32;
pub type NvnDepthMode = i32;
pub type NvnDebugObjectType = i32;
pub type NvnShaderStage = i32;
pub type NvnShaderStageBits = i32;
pub type NvnDrawPrimitive = i32;
pub type NvnIndexType = i32;
pub type NvnFace = i32;
pub type NvnFrontFace = i32;
pub type NvnPolygonMode = i32;
pub type NvnPolygonOffsetEnable = i32;
pub type NvnDepthFunc = i32;
pub type NvnStencilFunc = i32;
pub type NvnStencilOp = i32;
pub type NvnBlendFunc = i32;
pub type NvnBlendEquation = i32;
pub type NvnBlendAdvancedMode = i32;
pub type NvnBlendAdvancedOverlap = i32;
pub type NvnLogicOp = i32;
pub type NvnAlphaFunc = i32;
pub type NvnMinFilter = i32;
pub type NvnMagFilter = i32;
pub type NvnWrapMode = i32;
pub type NvnCompareMode = i32;
pub type NvnCompareFunc = i32;
pub type NvnSamplerReduction = i32;
pub type NvnCoverageModulationMode = i32;
pub type NvnTiledCacheAction = i32;
pub type NvnSyncCondition = i32;
pub type NvnSyncWaitResult = i32;
pub type NvnCounterType = i32;
pub type NvnConditionalRenderMode = i32;
pub type NvnQueueGetErrorResult = i32;
pub type NvnQueueAcquireTextureResult = i32;
pub type NvnWindowAcquireTextureResult = i32;
pub type NvnEventSignalMode = i32;
pub type NvnEventWaitMode = i32;
pub type NvnEventSignalLocation = i32;
pub type NvnViewportSwizzle = i32;

// ── Struct-like opaque types used as parameters ──

opaque!(
    NvnQueueErrorInfo,
    NvnCounterData,
    NvnShaderData,
    NvnMappingRequest,
    NvnCopyRegion,
    NvnBufferRange,
    NvnRectangle,
    NvnPackagedTextureLayout,
    NvnTextureSparseTileLayout,
    NvnDrawTextureRegion,
    NvnSubroutineLinkageMapPtr,
);

// ── Callback types ──

pub type PfnNvnGenericFuncPtr = Option<unsafe extern "C" fn()>;
pub type PfnNvnDebugCallback = Option<unsafe extern "C" fn()>;
pub type PfnNvnWalkDebugDatabaseCallback = Option<unsafe extern "C" fn()>;
pub type PfnNvnCommandBufferMemoryCallback = Option<unsafe extern "C" fn()>;

// ── Macro for declaring function pointer statics ──

#[macro_export]
macro_rules! nvn_func {
    ($(
        $(#[$meta:meta])*
        pub static $name:ident: fn($($arg:ty),* $(,)?) $(-> $ret:ty)?;
    )*) => {
        $(
            $(#[$meta])*
            pub static mut $name: Option<unsafe extern "C" fn($($arg),*) $(-> $ret)?> = None;
        )*
    };
}

// ── Initialization ──

/// Resolves all NVN function pointers using the resolver callback.
///
/// `resolver` and `get_proc` correspond to the two arguments passed to
/// the symbol-loading function found in the game binary (see nvn_vtable.txt).
///
/// `get_proc` is called as `get_proc(resolver, b"nvnFunctionName\0".as_ptr()) -> fn ptr`
pub unsafe fn init_from_resolver(
    resolver: u64,
    get_proc: unsafe extern "C" fn(u64, *const u8) -> *const (),
) {
    macro_rules! resolve {
        ($name:ident, $cname:expr) => {
            $name = core::mem::transmute(get_proc(resolver, concat!($cname, "\0").as_ptr()));
        };
    }

    // Device
    {
        use device::*;
        resolve!(nvnDeviceBuilderSetDefaults, "nvnDeviceBuilderSetDefaults");
        resolve!(nvnDeviceBuilderSetFlags, "nvnDeviceBuilderSetFlags");
        resolve!(nvnDeviceBuilderGetFlags, "nvnDeviceBuilderGetFlags");
        resolve!(nvnDeviceInitialize, "nvnDeviceInitialize");
        resolve!(nvnDeviceFinalize, "nvnDeviceFinalize");
        resolve!(nvnDeviceSetDebugLabel, "nvnDeviceSetDebugLabel");
        resolve!(nvnDeviceGetProcAddress, "nvnDeviceGetProcAddress");
        resolve!(nvnDeviceGetInteger, "nvnDeviceGetInteger");
        resolve!(nvnDeviceGetCurrentTimestampInNanoseconds, "nvnDeviceGetCurrentTimestampInNanoseconds");
        resolve!(nvnDeviceSetIntermediateShaderCache, "nvnDeviceSetIntermediateShaderCache");
        resolve!(nvnDeviceGetTextureHandle, "nvnDeviceGetTextureHandle");
        resolve!(nvnDeviceGetTexelFetchHandle, "nvnDeviceGetTexelFetchHandle");
        resolve!(nvnDeviceGetImageHandle, "nvnDeviceGetImageHandle");
        resolve!(nvnDeviceInstallDebugCallback, "nvnDeviceInstallDebugCallback");
        resolve!(nvnDeviceGenerateDebugDomainId, "nvnDeviceGenerateDebugDomainId");
        resolve!(nvnDeviceSetWindowOriginMode, "nvnDeviceSetWindowOriginMode");
        resolve!(nvnDeviceSetDepthMode, "nvnDeviceSetDepthMode");
        resolve!(nvnDeviceRegisterFastClearColor, "nvnDeviceRegisterFastClearColor");
        resolve!(nvnDeviceRegisterFastClearColori, "nvnDeviceRegisterFastClearColori");
        resolve!(nvnDeviceRegisterFastClearColorui, "nvnDeviceRegisterFastClearColorui");
        resolve!(nvnDeviceRegisterFastClearDepth, "nvnDeviceRegisterFastClearDepth");
        resolve!(nvnDeviceGetWindowOriginMode, "nvnDeviceGetWindowOriginMode");
        resolve!(nvnDeviceGetDepthMode, "nvnDeviceGetDepthMode");
        resolve!(nvnDeviceGetTimestampInNanoseconds, "nvnDeviceGetTimestampInNanoseconds");
        resolve!(nvnDeviceApplyDeferredFinalizes, "nvnDeviceApplyDeferredFinalizes");
        resolve!(nvnDeviceFinalizeCommandHandle, "nvnDeviceFinalizeCommandHandle");
        resolve!(nvnDeviceWalkDebugDatabase, "nvnDeviceWalkDebugDatabase");
        resolve!(nvnDeviceGetSeparateTextureHandle, "nvnDeviceGetSeparateTextureHandle");
        resolve!(nvnDeviceGetSeparateSamplerHandle, "nvnDeviceGetSeparateSamplerHandle");
        resolve!(nvnDeviceIsExternalDebuggerAttached, "nvnDeviceIsExternalDebuggerAttached");
    }

    // Queue
    {
        use queue::*;
        resolve!(nvnQueueGetError, "nvnQueueGetError");
        resolve!(nvnQueueGetTotalCommandMemoryUsed, "nvnQueueGetTotalCommandMemoryUsed");
        resolve!(nvnQueueGetTotalControlMemoryUsed, "nvnQueueGetTotalControlMemoryUsed");
        resolve!(nvnQueueGetTotalComputeMemoryUsed, "nvnQueueGetTotalComputeMemoryUsed");
        resolve!(nvnQueueResetMemoryUsageCounts, "nvnQueueResetMemoryUsageCounts");
        resolve!(nvnQueueBuilderSetDevice, "nvnQueueBuilderSetDevice");
        resolve!(nvnQueueBuilderSetDefaults, "nvnQueueBuilderSetDefaults");
        resolve!(nvnQueueBuilderSetFlags, "nvnQueueBuilderSetFlags");
        resolve!(nvnQueueBuilderSetCommandMemorySize, "nvnQueueBuilderSetCommandMemorySize");
        resolve!(nvnQueueBuilderSetComputeMemorySize, "nvnQueueBuilderSetComputeMemorySize");
        resolve!(nvnQueueBuilderSetControlMemorySize, "nvnQueueBuilderSetControlMemorySize");
        resolve!(nvnQueueBuilderGetQueueMemorySize, "nvnQueueBuilderGetQueueMemorySize");
        resolve!(nvnQueueBuilderSetQueueMemory, "nvnQueueBuilderSetQueueMemory");
        resolve!(nvnQueueBuilderSetCommandFlushThreshold, "nvnQueueBuilderSetCommandFlushThreshold");
        resolve!(nvnQueueBuilderSetQueuePriority, "nvnQueueBuilderSetQueuePriority");
        resolve!(nvnQueueBuilderGetQueuePriority, "nvnQueueBuilderGetQueuePriority");
        resolve!(nvnQueueBuilderGetDevice, "nvnQueueBuilderGetDevice");
        resolve!(nvnQueueBuilderGetFlags, "nvnQueueBuilderGetFlags");
        resolve!(nvnQueueBuilderGetCommandMemorySize, "nvnQueueBuilderGetCommandMemorySize");
        resolve!(nvnQueueBuilderGetComputeMemorySize, "nvnQueueBuilderGetComputeMemorySize");
        resolve!(nvnQueueBuilderGetControlMemorySize, "nvnQueueBuilderGetControlMemorySize");
        resolve!(nvnQueueBuilderGetCommandFlushThreshold, "nvnQueueBuilderGetCommandFlushThreshold");
        resolve!(nvnQueueBuilderGetMemorySize, "nvnQueueBuilderGetMemorySize");
        resolve!(nvnQueueBuilderGetMemory, "nvnQueueBuilderGetMemory");
        resolve!(nvnQueueInitialize, "nvnQueueInitialize");
        resolve!(nvnQueueFinalize, "nvnQueueFinalize");
        resolve!(nvnQueueSetDebugLabel, "nvnQueueSetDebugLabel");
        resolve!(nvnQueueSubmitCommands, "nvnQueueSubmitCommands");
        resolve!(nvnQueueFlush, "nvnQueueFlush");
        resolve!(nvnQueueFinish, "nvnQueueFinish");
        resolve!(nvnQueuePresentTexture, "nvnQueuePresentTexture");
        resolve!(nvnQueueAcquireTexture, "nvnQueueAcquireTexture");
    }

    // Window
    {
        use window::*;
        resolve!(nvnWindowBuilderSetDevice, "nvnWindowBuilderSetDevice");
        resolve!(nvnWindowBuilderSetDefaults, "nvnWindowBuilderSetDefaults");
        resolve!(nvnWindowBuilderSetNativeWindow, "nvnWindowBuilderSetNativeWindow");
        resolve!(nvnWindowBuilderSetTextures, "nvnWindowBuilderSetTextures");
        resolve!(nvnWindowBuilderSetPresentInterval, "nvnWindowBuilderSetPresentInterval");
        resolve!(nvnWindowBuilderSetNumActiveTextures, "nvnWindowBuilderSetNumActiveTextures");
        resolve!(nvnWindowBuilderGetDevice, "nvnWindowBuilderGetDevice");
        resolve!(nvnWindowBuilderGetNumTextures, "nvnWindowBuilderGetNumTextures");
        resolve!(nvnWindowBuilderGetTexture, "nvnWindowBuilderGetTexture");
        resolve!(nvnWindowBuilderGetNativeWindow, "nvnWindowBuilderGetNativeWindow");
        resolve!(nvnWindowBuilderGetPresentInterval, "nvnWindowBuilderGetPresentInterval");
        resolve!(nvnWindowBuilderGetNumActiveTextures, "nvnWindowBuilderGetNumActiveTextures");
        resolve!(nvnWindowInitialize, "nvnWindowInitialize");
        resolve!(nvnWindowFinalize, "nvnWindowFinalize");
        resolve!(nvnWindowSetDebugLabel, "nvnWindowSetDebugLabel");
        resolve!(nvnWindowAcquireTexture, "nvnWindowAcquireTexture");
        resolve!(nvnWindowGetNativeWindow, "nvnWindowGetNativeWindow");
        resolve!(nvnWindowGetPresentInterval, "nvnWindowGetPresentInterval");
        resolve!(nvnWindowSetPresentInterval, "nvnWindowSetPresentInterval");
        resolve!(nvnWindowSetCrop, "nvnWindowSetCrop");
        resolve!(nvnWindowGetCrop, "nvnWindowGetCrop");
        resolve!(nvnWindowSetNumActiveTextures, "nvnWindowSetNumActiveTextures");
        resolve!(nvnWindowGetNumActiveTextures, "nvnWindowGetNumActiveTextures");
        resolve!(nvnWindowGetNumTextures, "nvnWindowGetNumTextures");
    }

    // Memory
    {
        use mem::*;
        resolve!(nvnMemoryPoolBuilderSetDevice, "nvnMemoryPoolBuilderSetDevice");
        resolve!(nvnMemoryPoolBuilderSetDefaults, "nvnMemoryPoolBuilderSetDefaults");
        resolve!(nvnMemoryPoolBuilderSetStorage, "nvnMemoryPoolBuilderSetStorage");
        resolve!(nvnMemoryPoolBuilderSetFlags, "nvnMemoryPoolBuilderSetFlags");
        resolve!(nvnMemoryPoolBuilderGetDevice, "nvnMemoryPoolBuilderGetDevice");
        resolve!(nvnMemoryPoolBuilderGetMemory, "nvnMemoryPoolBuilderGetMemory");
        resolve!(nvnMemoryPoolBuilderGetSize, "nvnMemoryPoolBuilderGetSize");
        resolve!(nvnMemoryPoolBuilderGetFlags, "nvnMemoryPoolBuilderGetFlags");
        resolve!(nvnMemoryPoolInitialize, "nvnMemoryPoolInitialize");
        resolve!(nvnMemoryPoolSetDebugLabel, "nvnMemoryPoolSetDebugLabel");
        resolve!(nvnMemoryPoolFinalize, "nvnMemoryPoolFinalize");
        resolve!(nvnMemoryPoolMap, "nvnMemoryPoolMap");
        resolve!(nvnMemoryPoolFlushMappedRange, "nvnMemoryPoolFlushMappedRange");
        resolve!(nvnMemoryPoolInvalidateMappedRange, "nvnMemoryPoolInvalidateMappedRange");
        resolve!(nvnMemoryPoolGetBufferAddress, "nvnMemoryPoolGetBufferAddress");
        resolve!(nvnMemoryPoolMapVirtual, "nvnMemoryPoolMapVirtual");
        resolve!(nvnMemoryPoolGetSize, "nvnMemoryPoolGetSize");
        resolve!(nvnMemoryPoolGetFlags, "nvnMemoryPoolGetFlags");

        resolve!(nvnTexturePoolInitialize, "nvnTexturePoolInitialize");
        resolve!(nvnTexturePoolSetDebugLabel, "nvnTexturePoolSetDebugLabel");
        resolve!(nvnTexturePoolFinalize, "nvnTexturePoolFinalize");
        resolve!(nvnTexturePoolRegisterTexture, "nvnTexturePoolRegisterTexture");
        resolve!(nvnTexturePoolRegisterImage, "nvnTexturePoolRegisterImage");
        resolve!(nvnTexturePoolGetMemoryPool, "nvnTexturePoolGetMemoryPool");
        resolve!(nvnTexturePoolGetMemoryOffset, "nvnTexturePoolGetMemoryOffset");
        resolve!(nvnTexturePoolGetSize, "nvnTexturePoolGetSize");

        resolve!(nvnSamplerPoolInitialize, "nvnSamplerPoolInitialize");
        resolve!(nvnSamplerPoolSetDebugLabel, "nvnSamplerPoolSetDebugLabel");
        resolve!(nvnSamplerPoolFinalize, "nvnSamplerPoolFinalize");
        resolve!(nvnSamplerPoolRegisterSampler, "nvnSamplerPoolRegisterSampler");
        resolve!(nvnSamplerPoolRegisterSamplerBuilder, "nvnSamplerPoolRegisterSamplerBuilder");
        resolve!(nvnSamplerPoolGetMemoryPool, "nvnSamplerPoolGetMemoryPool");
        resolve!(nvnSamplerPoolGetMemoryOffset, "nvnSamplerPoolGetMemoryOffset");
        resolve!(nvnSamplerPoolGetSize, "nvnSamplerPoolGetSize");

        resolve!(nvnBufferBuilderSetDevice, "nvnBufferBuilderSetDevice");
        resolve!(nvnBufferBuilderSetDefaults, "nvnBufferBuilderSetDefaults");
        resolve!(nvnBufferBuilderSetStorage, "nvnBufferBuilderSetStorage");
        resolve!(nvnBufferBuilderGetDevice, "nvnBufferBuilderGetDevice");
        resolve!(nvnBufferBuilderGetMemoryPool, "nvnBufferBuilderGetMemoryPool");
        resolve!(nvnBufferBuilderGetMemoryOffset, "nvnBufferBuilderGetMemoryOffset");
        resolve!(nvnBufferBuilderGetSize, "nvnBufferBuilderGetSize");
        resolve!(nvnBufferInitialize, "nvnBufferInitialize");
        resolve!(nvnBufferSetDebugLabel, "nvnBufferSetDebugLabel");
        resolve!(nvnBufferFinalize, "nvnBufferFinalize");
        resolve!(nvnBufferMap, "nvnBufferMap");
        resolve!(nvnBufferGetAddress, "nvnBufferGetAddress");
        resolve!(nvnBufferFlushMappedRange, "nvnBufferFlushMappedRange");
        resolve!(nvnBufferInvalidateMappedRange, "nvnBufferInvalidateMappedRange");
        resolve!(nvnBufferGetMemoryPool, "nvnBufferGetMemoryPool");
        resolve!(nvnBufferGetMemoryOffset, "nvnBufferGetMemoryOffset");
        resolve!(nvnBufferGetSize, "nvnBufferGetSize");
        resolve!(nvnBufferGetDebugID, "nvnBufferGetDebugID");
    }

    // Resource
    {
        use resource::*;
        // TextureBuilder
        resolve!(nvnTextureBuilderSetDevice, "nvnTextureBuilderSetDevice");
        resolve!(nvnTextureBuilderSetDefaults, "nvnTextureBuilderSetDefaults");
        resolve!(nvnTextureBuilderSetFlags, "nvnTextureBuilderSetFlags");
        resolve!(nvnTextureBuilderSetTarget, "nvnTextureBuilderSetTarget");
        resolve!(nvnTextureBuilderSetWidth, "nvnTextureBuilderSetWidth");
        resolve!(nvnTextureBuilderSetHeight, "nvnTextureBuilderSetHeight");
        resolve!(nvnTextureBuilderSetDepth, "nvnTextureBuilderSetDepth");
        resolve!(nvnTextureBuilderSetSize1D, "nvnTextureBuilderSetSize1D");
        resolve!(nvnTextureBuilderSetSize2D, "nvnTextureBuilderSetSize2D");
        resolve!(nvnTextureBuilderSetSize3D, "nvnTextureBuilderSetSize3D");
        resolve!(nvnTextureBuilderSetLevels, "nvnTextureBuilderSetLevels");
        resolve!(nvnTextureBuilderSetFormat, "nvnTextureBuilderSetFormat");
        resolve!(nvnTextureBuilderSetSamples, "nvnTextureBuilderSetSamples");
        resolve!(nvnTextureBuilderSetSwizzle, "nvnTextureBuilderSetSwizzle");
        resolve!(nvnTextureBuilderSetDepthStencilMode, "nvnTextureBuilderSetDepthStencilMode");
        resolve!(nvnTextureBuilderGetStorageSize, "nvnTextureBuilderGetStorageSize");
        resolve!(nvnTextureBuilderGetStorageAlignment, "nvnTextureBuilderGetStorageAlignment");
        resolve!(nvnTextureBuilderSetStorage, "nvnTextureBuilderSetStorage");
        resolve!(nvnTextureBuilderSetPackagedTextureData, "nvnTextureBuilderSetPackagedTextureData");
        resolve!(nvnTextureBuilderSetPackagedTextureLayout, "nvnTextureBuilderSetPackagedTextureLayout");
        resolve!(nvnTextureBuilderSetStride, "nvnTextureBuilderSetStride");
        resolve!(nvnTextureBuilderSetGLTextureName, "nvnTextureBuilderSetGLTextureName");
        resolve!(nvnTextureBuilderGetStorageClass, "nvnTextureBuilderGetStorageClass");
        resolve!(nvnTextureBuilderGetDevice, "nvnTextureBuilderGetDevice");
        resolve!(nvnTextureBuilderGetFlags, "nvnTextureBuilderGetFlags");
        resolve!(nvnTextureBuilderGetTarget, "nvnTextureBuilderGetTarget");
        resolve!(nvnTextureBuilderGetWidth, "nvnTextureBuilderGetWidth");
        resolve!(nvnTextureBuilderGetHeight, "nvnTextureBuilderGetHeight");
        resolve!(nvnTextureBuilderGetDepth, "nvnTextureBuilderGetDepth");
        resolve!(nvnTextureBuilderGetLevels, "nvnTextureBuilderGetLevels");
        resolve!(nvnTextureBuilderGetFormat, "nvnTextureBuilderGetFormat");
        resolve!(nvnTextureBuilderGetSamples, "nvnTextureBuilderGetSamples");
        resolve!(nvnTextureBuilderGetSwizzle, "nvnTextureBuilderGetSwizzle");
        resolve!(nvnTextureBuilderGetDepthStencilMode, "nvnTextureBuilderGetDepthStencilMode");
        resolve!(nvnTextureBuilderGetPackagedTextureData, "nvnTextureBuilderGetPackagedTextureData");
        resolve!(nvnTextureBuilderGetPackagedTextureLayout, "nvnTextureBuilderGetPackagedTextureLayout");
        resolve!(nvnTextureBuilderGetStride, "nvnTextureBuilderGetStride");
        resolve!(nvnTextureBuilderGetSparseTileLayout, "nvnTextureBuilderGetSparseTileLayout");
        resolve!(nvnTextureBuilderGetGLTextureName, "nvnTextureBuilderGetGLTextureName");
        resolve!(nvnTextureBuilderGetZCullStorageSize, "nvnTextureBuilderGetZCullStorageSize");
        resolve!(nvnTextureBuilderGetMemoryPool, "nvnTextureBuilderGetMemoryPool");
        resolve!(nvnTextureBuilderGetMemoryOffset, "nvnTextureBuilderGetMemoryOffset");
        resolve!(nvnTextureBuilderGetRawStorageClass, "nvnTextureBuilderGetRawStorageClass");

        // TextureView
        resolve!(nvnTextureViewSetDefaults, "nvnTextureViewSetDefaults");
        resolve!(nvnTextureViewSetLevels, "nvnTextureViewSetLevels");
        resolve!(nvnTextureViewSetLayers, "nvnTextureViewSetLayers");
        resolve!(nvnTextureViewSetFormat, "nvnTextureViewSetFormat");
        resolve!(nvnTextureViewSetSwizzle, "nvnTextureViewSetSwizzle");
        resolve!(nvnTextureViewSetDepthStencilMode, "nvnTextureViewSetDepthStencilMode");
        resolve!(nvnTextureViewSetTarget, "nvnTextureViewSetTarget");
        resolve!(nvnTextureViewGetLevels, "nvnTextureViewGetLevels");
        resolve!(nvnTextureViewGetLayers, "nvnTextureViewGetLayers");
        resolve!(nvnTextureViewGetFormat, "nvnTextureViewGetFormat");
        resolve!(nvnTextureViewGetSwizzle, "nvnTextureViewGetSwizzle");
        resolve!(nvnTextureViewGetDepthStencilMode, "nvnTextureViewGetDepthStencilMode");
        resolve!(nvnTextureViewGetTarget, "nvnTextureViewGetTarget");
        resolve!(nvnTextureViewCompare, "nvnTextureViewCompare");

        // Texture
        resolve!(nvnTextureInitialize, "nvnTextureInitialize");
        resolve!(nvnTextureGetZCullStorageSize, "nvnTextureGetZCullStorageSize");
        resolve!(nvnTextureFinalize, "nvnTextureFinalize");
        resolve!(nvnTextureSetDebugLabel, "nvnTextureSetDebugLabel");
        resolve!(nvnTextureGetStorageClass, "nvnTextureGetStorageClass");
        resolve!(nvnTextureGetViewOffset, "nvnTextureGetViewOffset");
        resolve!(nvnTextureGetFlags, "nvnTextureGetFlags");
        resolve!(nvnTextureGetTarget, "nvnTextureGetTarget");
        resolve!(nvnTextureGetWidth, "nvnTextureGetWidth");
        resolve!(nvnTextureGetHeight, "nvnTextureGetHeight");
        resolve!(nvnTextureGetDepth, "nvnTextureGetDepth");
        resolve!(nvnTextureGetLevels, "nvnTextureGetLevels");
        resolve!(nvnTextureGetFormat, "nvnTextureGetFormat");
        resolve!(nvnTextureGetSamples, "nvnTextureGetSamples");
        resolve!(nvnTextureGetSwizzle, "nvnTextureGetSwizzle");
        resolve!(nvnTextureGetDepthStencilMode, "nvnTextureGetDepthStencilMode");
        resolve!(nvnTextureGetStride, "nvnTextureGetStride");
        resolve!(nvnTextureGetTextureAddress, "nvnTextureGetTextureAddress");
        resolve!(nvnTextureGetSparseTileLayout, "nvnTextureGetSparseTileLayout");
        resolve!(nvnTextureWriteTexels, "nvnTextureWriteTexels");
        resolve!(nvnTextureWriteTexelsStrided, "nvnTextureWriteTexelsStrided");
        resolve!(nvnTextureReadTexels, "nvnTextureReadTexels");
        resolve!(nvnTextureReadTexelsStrided, "nvnTextureReadTexelsStrided");
        resolve!(nvnTextureFlushTexels, "nvnTextureFlushTexels");
        resolve!(nvnTextureInvalidateTexels, "nvnTextureInvalidateTexels");
        resolve!(nvnTextureGetMemoryPool, "nvnTextureGetMemoryPool");
        resolve!(nvnTextureGetMemoryOffset, "nvnTextureGetMemoryOffset");
        resolve!(nvnTextureGetStorageSize, "nvnTextureGetStorageSize");
        resolve!(nvnTextureCompare, "nvnTextureCompare");
        resolve!(nvnTextureGetDebugID, "nvnTextureGetDebugID");
        resolve!(nvnTextureGetRawStorageClass, "nvnTextureGetRawStorageClass");

        // SamplerBuilder
        resolve!(nvnSamplerBuilderSetDevice, "nvnSamplerBuilderSetDevice");
        resolve!(nvnSamplerBuilderSetDefaults, "nvnSamplerBuilderSetDefaults");
        resolve!(nvnSamplerBuilderSetMinMagFilter, "nvnSamplerBuilderSetMinMagFilter");
        resolve!(nvnSamplerBuilderSetWrapMode, "nvnSamplerBuilderSetWrapMode");
        resolve!(nvnSamplerBuilderSetLodClamp, "nvnSamplerBuilderSetLodClamp");
        resolve!(nvnSamplerBuilderSetLodBias, "nvnSamplerBuilderSetLodBias");
        resolve!(nvnSamplerBuilderSetCompare, "nvnSamplerBuilderSetCompare");
        resolve!(nvnSamplerBuilderSetBorderColor, "nvnSamplerBuilderSetBorderColor");
        resolve!(nvnSamplerBuilderSetBorderColori, "nvnSamplerBuilderSetBorderColori");
        resolve!(nvnSamplerBuilderSetBorderColorui, "nvnSamplerBuilderSetBorderColorui");
        resolve!(nvnSamplerBuilderSetMaxAnisotropy, "nvnSamplerBuilderSetMaxAnisotropy");
        resolve!(nvnSamplerBuilderSetReductionFilter, "nvnSamplerBuilderSetReductionFilter");
        resolve!(nvnSamplerBuilderSetLodSnap, "nvnSamplerBuilderSetLodSnap");
        resolve!(nvnSamplerBuilderGetDevice, "nvnSamplerBuilderGetDevice");
        resolve!(nvnSamplerBuilderGetMinMagFilter, "nvnSamplerBuilderGetMinMagFilter");
        resolve!(nvnSamplerBuilderGetWrapMode, "nvnSamplerBuilderGetWrapMode");
        resolve!(nvnSamplerBuilderGetLodClamp, "nvnSamplerBuilderGetLodClamp");
        resolve!(nvnSamplerBuilderGetLodBias, "nvnSamplerBuilderGetLodBias");
        resolve!(nvnSamplerBuilderGetCompare, "nvnSamplerBuilderGetCompare");
        resolve!(nvnSamplerBuilderGetBorderColor, "nvnSamplerBuilderGetBorderColor");
        resolve!(nvnSamplerBuilderGetBorderColori, "nvnSamplerBuilderGetBorderColori");
        resolve!(nvnSamplerBuilderGetBorderColorui, "nvnSamplerBuilderGetBorderColorui");
        resolve!(nvnSamplerBuilderGetMaxAnisotropy, "nvnSamplerBuilderGetMaxAnisotropy");
        resolve!(nvnSamplerBuilderGetReductionFilter, "nvnSamplerBuilderGetReductionFilter");
        resolve!(nvnSamplerBuilderGetLodSnap, "nvnSamplerBuilderGetLodSnap");

        // Sampler
        resolve!(nvnSamplerInitialize, "nvnSamplerInitialize");
        resolve!(nvnSamplerFinalize, "nvnSamplerFinalize");
        resolve!(nvnSamplerSetDebugLabel, "nvnSamplerSetDebugLabel");
        resolve!(nvnSamplerGetMinMagFilter, "nvnSamplerGetMinMagFilter");
        resolve!(nvnSamplerGetWrapMode, "nvnSamplerGetWrapMode");
        resolve!(nvnSamplerGetLodClamp, "nvnSamplerGetLodClamp");
        resolve!(nvnSamplerGetLodBias, "nvnSamplerGetLodBias");
        resolve!(nvnSamplerGetCompare, "nvnSamplerGetCompare");
        resolve!(nvnSamplerGetBorderColor, "nvnSamplerGetBorderColor");
        resolve!(nvnSamplerGetBorderColori, "nvnSamplerGetBorderColori");
        resolve!(nvnSamplerGetBorderColorui, "nvnSamplerGetBorderColorui");
        resolve!(nvnSamplerGetMaxAnisotropy, "nvnSamplerGetMaxAnisotropy");
        resolve!(nvnSamplerGetReductionFilter, "nvnSamplerGetReductionFilter");
        resolve!(nvnSamplerCompare, "nvnSamplerCompare");
        resolve!(nvnSamplerGetDebugID, "nvnSamplerGetDebugID");

        // Program
        resolve!(nvnProgramInitialize, "nvnProgramInitialize");
        resolve!(nvnProgramFinalize, "nvnProgramFinalize");
        resolve!(nvnProgramSetDebugLabel, "nvnProgramSetDebugLabel");
        resolve!(nvnProgramSetShaders, "nvnProgramSetShaders");
        resolve!(nvnProgramSetShadersExt, "nvnProgramSetShadersExt");
        resolve!(nvnProgramSetSampleShading, "nvnProgramSetSampleShading");
        resolve!(nvnProgramSetSubroutineLinkage, "nvnProgramSetSubroutineLinkage");

        // BlendState
        resolve!(nvnBlendStateSetDefaults, "nvnBlendStateSetDefaults");
        resolve!(nvnBlendStateSetBlendTarget, "nvnBlendStateSetBlendTarget");
        resolve!(nvnBlendStateSetBlendFunc, "nvnBlendStateSetBlendFunc");
        resolve!(nvnBlendStateSetBlendEquation, "nvnBlendStateSetBlendEquation");
        resolve!(nvnBlendStateSetAdvancedMode, "nvnBlendStateSetAdvancedMode");
        resolve!(nvnBlendStateSetAdvancedOverlap, "nvnBlendStateSetAdvancedOverlap");
        resolve!(nvnBlendStateSetAdvancedPremultipliedSrc, "nvnBlendStateSetAdvancedPremultipliedSrc");
        resolve!(nvnBlendStateSetAdvancedNormalizedDst, "nvnBlendStateSetAdvancedNormalizedDst");
        resolve!(nvnBlendStateGetBlendTarget, "nvnBlendStateGetBlendTarget");
        resolve!(nvnBlendStateGetBlendFunc, "nvnBlendStateGetBlendFunc");
        resolve!(nvnBlendStateGetBlendEquation, "nvnBlendStateGetBlendEquation");
        resolve!(nvnBlendStateGetAdvancedMode, "nvnBlendStateGetAdvancedMode");
        resolve!(nvnBlendStateGetAdvancedOverlap, "nvnBlendStateGetAdvancedOverlap");
        resolve!(nvnBlendStateGetAdvancedPremultipliedSrc, "nvnBlendStateGetAdvancedPremultipliedSrc");
        resolve!(nvnBlendStateGetAdvancedNormalizedDst, "nvnBlendStateGetAdvancedNormalizedDst");

        // ColorState
        resolve!(nvnColorStateSetDefaults, "nvnColorStateSetDefaults");
        resolve!(nvnColorStateSetBlendEnable, "nvnColorStateSetBlendEnable");
        resolve!(nvnColorStateSetLogicOp, "nvnColorStateSetLogicOp");
        resolve!(nvnColorStateSetAlphaTest, "nvnColorStateSetAlphaTest");
        resolve!(nvnColorStateGetBlendEnable, "nvnColorStateGetBlendEnable");
        resolve!(nvnColorStateGetLogicOp, "nvnColorStateGetLogicOp");
        resolve!(nvnColorStateGetAlphaTest, "nvnColorStateGetAlphaTest");

        // ChannelMaskState
        resolve!(nvnChannelMaskStateSetDefaults, "nvnChannelMaskStateSetDefaults");
        resolve!(nvnChannelMaskStateSetChannelMask, "nvnChannelMaskStateSetChannelMask");
        resolve!(nvnChannelMaskStateGetChannelMask, "nvnChannelMaskStateGetChannelMask");

        // MultisampleState
        resolve!(nvnMultisampleStateSetDefaults, "nvnMultisampleStateSetDefaults");
        resolve!(nvnMultisampleStateSetMultisampleEnable, "nvnMultisampleStateSetMultisampleEnable");
        resolve!(nvnMultisampleStateSetSamples, "nvnMultisampleStateSetSamples");
        resolve!(nvnMultisampleStateSetAlphaToCoverageEnable, "nvnMultisampleStateSetAlphaToCoverageEnable");
        resolve!(nvnMultisampleStateSetAlphaToCoverageDither, "nvnMultisampleStateSetAlphaToCoverageDither");
        resolve!(nvnMultisampleStateGetMultisampleEnable, "nvnMultisampleStateGetMultisampleEnable");
        resolve!(nvnMultisampleStateGetSamples, "nvnMultisampleStateGetSamples");
        resolve!(nvnMultisampleStateGetAlphaToCoverageEnable, "nvnMultisampleStateGetAlphaToCoverageEnable");
        resolve!(nvnMultisampleStateGetAlphaToCoverageDither, "nvnMultisampleStateGetAlphaToCoverageDither");
        resolve!(nvnMultisampleStateSetRasterSamples, "nvnMultisampleStateSetRasterSamples");
        resolve!(nvnMultisampleStateGetRasterSamples, "nvnMultisampleStateGetRasterSamples");
        resolve!(nvnMultisampleStateSetCoverageModulationMode, "nvnMultisampleStateSetCoverageModulationMode");
        resolve!(nvnMultisampleStateGetCoverageModulationMode, "nvnMultisampleStateGetCoverageModulationMode");
        resolve!(nvnMultisampleStateSetCoverageToColorEnable, "nvnMultisampleStateSetCoverageToColorEnable");
        resolve!(nvnMultisampleStateGetCoverageToColorEnable, "nvnMultisampleStateGetCoverageToColorEnable");
        resolve!(nvnMultisampleStateSetCoverageToColorOutput, "nvnMultisampleStateSetCoverageToColorOutput");
        resolve!(nvnMultisampleStateGetCoverageToColorOutput, "nvnMultisampleStateGetCoverageToColorOutput");
        resolve!(nvnMultisampleStateSetSampleLocationsEnable, "nvnMultisampleStateSetSampleLocationsEnable");
        resolve!(nvnMultisampleStateGetSampleLocationsEnable, "nvnMultisampleStateGetSampleLocationsEnable");
        resolve!(nvnMultisampleStateGetSampleLocationsGrid, "nvnMultisampleStateGetSampleLocationsGrid");
        resolve!(nvnMultisampleStateSetSampleLocationsGridEnable, "nvnMultisampleStateSetSampleLocationsGridEnable");
        resolve!(nvnMultisampleStateGetSampleLocationsGridEnable, "nvnMultisampleStateGetSampleLocationsGridEnable");
        resolve!(nvnMultisampleStateSetSampleLocations, "nvnMultisampleStateSetSampleLocations");

        // PolygonState
        resolve!(nvnPolygonStateSetDefaults, "nvnPolygonStateSetDefaults");
        resolve!(nvnPolygonStateSetCullFace, "nvnPolygonStateSetCullFace");
        resolve!(nvnPolygonStateSetFrontFace, "nvnPolygonStateSetFrontFace");
        resolve!(nvnPolygonStateSetPolygonMode, "nvnPolygonStateSetPolygonMode");
        resolve!(nvnPolygonStateSetPolygonOffsetEnables, "nvnPolygonStateSetPolygonOffsetEnables");
        resolve!(nvnPolygonStateGetCullFace, "nvnPolygonStateGetCullFace");
        resolve!(nvnPolygonStateGetFrontFace, "nvnPolygonStateGetFrontFace");
        resolve!(nvnPolygonStateGetPolygonMode, "nvnPolygonStateGetPolygonMode");
        resolve!(nvnPolygonStateGetPolygonOffsetEnables, "nvnPolygonStateGetPolygonOffsetEnables");

        // DepthStencilState
        resolve!(nvnDepthStencilStateSetDefaults, "nvnDepthStencilStateSetDefaults");
        resolve!(nvnDepthStencilStateSetDepthTestEnable, "nvnDepthStencilStateSetDepthTestEnable");
        resolve!(nvnDepthStencilStateSetDepthWriteEnable, "nvnDepthStencilStateSetDepthWriteEnable");
        resolve!(nvnDepthStencilStateSetDepthFunc, "nvnDepthStencilStateSetDepthFunc");
        resolve!(nvnDepthStencilStateSetStencilTestEnable, "nvnDepthStencilStateSetStencilTestEnable");
        resolve!(nvnDepthStencilStateSetStencilFunc, "nvnDepthStencilStateSetStencilFunc");
        resolve!(nvnDepthStencilStateSetStencilOp, "nvnDepthStencilStateSetStencilOp");
        resolve!(nvnDepthStencilStateGetDepthTestEnable, "nvnDepthStencilStateGetDepthTestEnable");
        resolve!(nvnDepthStencilStateGetDepthWriteEnable, "nvnDepthStencilStateGetDepthWriteEnable");
        resolve!(nvnDepthStencilStateGetDepthFunc, "nvnDepthStencilStateGetDepthFunc");
        resolve!(nvnDepthStencilStateGetStencilTestEnable, "nvnDepthStencilStateGetStencilTestEnable");
        resolve!(nvnDepthStencilStateGetStencilFunc, "nvnDepthStencilStateGetStencilFunc");
        resolve!(nvnDepthStencilStateGetStencilOp, "nvnDepthStencilStateGetStencilOp");

        // VertexAttribState
        resolve!(nvnVertexAttribStateSetDefaults, "nvnVertexAttribStateSetDefaults");
        resolve!(nvnVertexAttribStateSetFormat, "nvnVertexAttribStateSetFormat");
        resolve!(nvnVertexAttribStateSetStreamIndex, "nvnVertexAttribStateSetStreamIndex");
        resolve!(nvnVertexAttribStateGetFormat, "nvnVertexAttribStateGetFormat");
        resolve!(nvnVertexAttribStateGetStreamIndex, "nvnVertexAttribStateGetStreamIndex");

        // VertexStreamState
        resolve!(nvnVertexStreamStateSetDefaults, "nvnVertexStreamStateSetDefaults");
        resolve!(nvnVertexStreamStateSetStride, "nvnVertexStreamStateSetStride");
        resolve!(nvnVertexStreamStateSetDivisor, "nvnVertexStreamStateSetDivisor");
        resolve!(nvnVertexStreamStateGetStride, "nvnVertexStreamStateGetStride");
        resolve!(nvnVertexStreamStateGetDivisor, "nvnVertexStreamStateGetDivisor");
    }

    // CommandBuffer
    {
        use cmdbuf::*;
        resolve!(nvnCommandBufferInitialize, "nvnCommandBufferInitialize");
        resolve!(nvnCommandBufferFinalize, "nvnCommandBufferFinalize");
        resolve!(nvnCommandBufferSetDebugLabel, "nvnCommandBufferSetDebugLabel");
        resolve!(nvnCommandBufferSetMemoryCallback, "nvnCommandBufferSetMemoryCallback");
        resolve!(nvnCommandBufferSetMemoryCallbackData, "nvnCommandBufferSetMemoryCallbackData");
        resolve!(nvnCommandBufferSetCommandMemoryCallbackEnabled, "nvnCommandBufferSetCommandMemoryCallbackEnabled");
        resolve!(nvnCommandBufferAddCommandMemory, "nvnCommandBufferAddCommandMemory");
        resolve!(nvnCommandBufferAddControlMemory, "nvnCommandBufferAddControlMemory");
        resolve!(nvnCommandBufferGetCommandMemorySize, "nvnCommandBufferGetCommandMemorySize");
        resolve!(nvnCommandBufferGetCommandMemoryUsed, "nvnCommandBufferGetCommandMemoryUsed");
        resolve!(nvnCommandBufferGetCommandMemoryFree, "nvnCommandBufferGetCommandMemoryFree");
        resolve!(nvnCommandBufferGetControlMemorySize, "nvnCommandBufferGetControlMemorySize");
        resolve!(nvnCommandBufferGetControlMemoryUsed, "nvnCommandBufferGetControlMemoryUsed");
        resolve!(nvnCommandBufferGetControlMemoryFree, "nvnCommandBufferGetControlMemoryFree");
        resolve!(nvnCommandBufferBeginRecording, "nvnCommandBufferBeginRecording");
        resolve!(nvnCommandBufferEndRecording, "nvnCommandBufferEndRecording");
        resolve!(nvnCommandBufferCallCommands, "nvnCommandBufferCallCommands");
        resolve!(nvnCommandBufferCopyCommands, "nvnCommandBufferCopyCommands");
        resolve!(nvnCommandBufferBindBlendState, "nvnCommandBufferBindBlendState");
        resolve!(nvnCommandBufferBindChannelMaskState, "nvnCommandBufferBindChannelMaskState");
        resolve!(nvnCommandBufferBindColorState, "nvnCommandBufferBindColorState");
        resolve!(nvnCommandBufferBindMultisampleState, "nvnCommandBufferBindMultisampleState");
        resolve!(nvnCommandBufferBindPolygonState, "nvnCommandBufferBindPolygonState");
        resolve!(nvnCommandBufferBindDepthStencilState, "nvnCommandBufferBindDepthStencilState");
        resolve!(nvnCommandBufferBindVertexAttribState, "nvnCommandBufferBindVertexAttribState");
        resolve!(nvnCommandBufferBindVertexStreamState, "nvnCommandBufferBindVertexStreamState");
        resolve!(nvnCommandBufferBindProgram, "nvnCommandBufferBindProgram");
        resolve!(nvnCommandBufferBindVertexBuffer, "nvnCommandBufferBindVertexBuffer");
        resolve!(nvnCommandBufferBindVertexBuffers, "nvnCommandBufferBindVertexBuffers");
        resolve!(nvnCommandBufferBindUniformBuffer, "nvnCommandBufferBindUniformBuffer");
        resolve!(nvnCommandBufferBindUniformBuffers, "nvnCommandBufferBindUniformBuffers");
        resolve!(nvnCommandBufferBindTransformFeedbackBuffer, "nvnCommandBufferBindTransformFeedbackBuffer");
        resolve!(nvnCommandBufferBindTransformFeedbackBuffers, "nvnCommandBufferBindTransformFeedbackBuffers");
        resolve!(nvnCommandBufferBindStorageBuffer, "nvnCommandBufferBindStorageBuffer");
        resolve!(nvnCommandBufferBindStorageBuffers, "nvnCommandBufferBindStorageBuffers");
        resolve!(nvnCommandBufferBindTexture, "nvnCommandBufferBindTexture");
        resolve!(nvnCommandBufferBindTextures, "nvnCommandBufferBindTextures");
        resolve!(nvnCommandBufferBindImage, "nvnCommandBufferBindImage");
        resolve!(nvnCommandBufferBindImages, "nvnCommandBufferBindImages");
        resolve!(nvnCommandBufferSetPatchSize, "nvnCommandBufferSetPatchSize");
        resolve!(nvnCommandBufferSetInnerTessellationLevels, "nvnCommandBufferSetInnerTessellationLevels");
        resolve!(nvnCommandBufferSetOuterTessellationLevels, "nvnCommandBufferSetOuterTessellationLevels");
        resolve!(nvnCommandBufferSetPrimitiveRestart, "nvnCommandBufferSetPrimitiveRestart");
        resolve!(nvnCommandBufferBeginTransformFeedback, "nvnCommandBufferBeginTransformFeedback");
        resolve!(nvnCommandBufferEndTransformFeedback, "nvnCommandBufferEndTransformFeedback");
        resolve!(nvnCommandBufferPauseTransformFeedback, "nvnCommandBufferPauseTransformFeedback");
        resolve!(nvnCommandBufferResumeTransformFeedback, "nvnCommandBufferResumeTransformFeedback");
        resolve!(nvnCommandBufferDrawTransformFeedback, "nvnCommandBufferDrawTransformFeedback");
        resolve!(nvnCommandBufferDrawArrays, "nvnCommandBufferDrawArrays");
        resolve!(nvnCommandBufferDrawElements, "nvnCommandBufferDrawElements");
        resolve!(nvnCommandBufferDrawElementsBaseVertex, "nvnCommandBufferDrawElementsBaseVertex");
        resolve!(nvnCommandBufferDrawArraysInstanced, "nvnCommandBufferDrawArraysInstanced");
        resolve!(nvnCommandBufferDrawElementsInstanced, "nvnCommandBufferDrawElementsInstanced");
        resolve!(nvnCommandBufferDrawArraysIndirect, "nvnCommandBufferDrawArraysIndirect");
        resolve!(nvnCommandBufferDrawElementsIndirect, "nvnCommandBufferDrawElementsIndirect");
        resolve!(nvnCommandBufferMultiDrawArraysIndirectCount, "nvnCommandBufferMultiDrawArraysIndirectCount");
        resolve!(nvnCommandBufferMultiDrawElementsIndirectCount, "nvnCommandBufferMultiDrawElementsIndirectCount");
        resolve!(nvnCommandBufferClearColor, "nvnCommandBufferClearColor");
        resolve!(nvnCommandBufferClearColori, "nvnCommandBufferClearColori");
        resolve!(nvnCommandBufferClearColorui, "nvnCommandBufferClearColorui");
        resolve!(nvnCommandBufferClearDepthStencil, "nvnCommandBufferClearDepthStencil");
        resolve!(nvnCommandBufferDispatchCompute, "nvnCommandBufferDispatchCompute");
        resolve!(nvnCommandBufferDispatchComputeIndirect, "nvnCommandBufferDispatchComputeIndirect");
        resolve!(nvnCommandBufferSetViewport, "nvnCommandBufferSetViewport");
        resolve!(nvnCommandBufferSetViewports, "nvnCommandBufferSetViewports");
        resolve!(nvnCommandBufferSetViewportSwizzles, "nvnCommandBufferSetViewportSwizzles");
        resolve!(nvnCommandBufferSetScissor, "nvnCommandBufferSetScissor");
        resolve!(nvnCommandBufferSetScissors, "nvnCommandBufferSetScissors");
        resolve!(nvnCommandBufferSetDepthRange, "nvnCommandBufferSetDepthRange");
        resolve!(nvnCommandBufferSetDepthBounds, "nvnCommandBufferSetDepthBounds");
        resolve!(nvnCommandBufferSetDepthRanges, "nvnCommandBufferSetDepthRanges");
        resolve!(nvnCommandBufferSetTiledCacheAction, "nvnCommandBufferSetTiledCacheAction");
        resolve!(nvnCommandBufferSetTiledCacheTileSize, "nvnCommandBufferSetTiledCacheTileSize");
        resolve!(nvnCommandBufferBindSeparateTexture, "nvnCommandBufferBindSeparateTexture");
        resolve!(nvnCommandBufferBindSeparateSampler, "nvnCommandBufferBindSeparateSampler");
        resolve!(nvnCommandBufferBindSeparateTextures, "nvnCommandBufferBindSeparateTextures");
        resolve!(nvnCommandBufferBindSeparateSamplers, "nvnCommandBufferBindSeparateSamplers");
        resolve!(nvnCommandBufferSetStencilValueMask, "nvnCommandBufferSetStencilValueMask");
        resolve!(nvnCommandBufferSetStencilMask, "nvnCommandBufferSetStencilMask");
        resolve!(nvnCommandBufferSetStencilRef, "nvnCommandBufferSetStencilRef");
        resolve!(nvnCommandBufferSetBlendColor, "nvnCommandBufferSetBlendColor");
        resolve!(nvnCommandBufferSetPointSize, "nvnCommandBufferSetPointSize");
        resolve!(nvnCommandBufferSetLineWidth, "nvnCommandBufferSetLineWidth");
        resolve!(nvnCommandBufferSetPolygonOffsetClamp, "nvnCommandBufferSetPolygonOffsetClamp");
        resolve!(nvnCommandBufferSetAlphaRef, "nvnCommandBufferSetAlphaRef");
        resolve!(nvnCommandBufferSetSampleMask, "nvnCommandBufferSetSampleMask");
        resolve!(nvnCommandBufferSetRasterizerDiscard, "nvnCommandBufferSetRasterizerDiscard");
        resolve!(nvnCommandBufferSetDepthClamp, "nvnCommandBufferSetDepthClamp");
        resolve!(nvnCommandBufferSetConservativeRasterEnable, "nvnCommandBufferSetConservativeRasterEnable");
        resolve!(nvnCommandBufferSetConservativeRasterDilate, "nvnCommandBufferSetConservativeRasterDilate");
        resolve!(nvnCommandBufferSetSubpixelPrecisionBias, "nvnCommandBufferSetSubpixelPrecisionBias");
        resolve!(nvnCommandBufferCopyBufferToTexture, "nvnCommandBufferCopyBufferToTexture");
        resolve!(nvnCommandBufferCopyTextureToBuffer, "nvnCommandBufferCopyTextureToBuffer");
        resolve!(nvnCommandBufferCopyTextureToTexture, "nvnCommandBufferCopyTextureToTexture");
        resolve!(nvnCommandBufferCopyBufferToBuffer, "nvnCommandBufferCopyBufferToBuffer");
        resolve!(nvnCommandBufferClearBuffer, "nvnCommandBufferClearBuffer");
        resolve!(nvnCommandBufferClearTexture, "nvnCommandBufferClearTexture");
        resolve!(nvnCommandBufferClearTexturei, "nvnCommandBufferClearTexturei");
        resolve!(nvnCommandBufferClearTextureui, "nvnCommandBufferClearTextureui");
        resolve!(nvnCommandBufferUpdateUniformBuffer, "nvnCommandBufferUpdateUniformBuffer");
        resolve!(nvnCommandBufferReportCounter, "nvnCommandBufferReportCounter");
        resolve!(nvnCommandBufferResetCounter, "nvnCommandBufferResetCounter");
        resolve!(nvnCommandBufferReportValue, "nvnCommandBufferReportValue");
        resolve!(nvnCommandBufferSetRenderEnable, "nvnCommandBufferSetRenderEnable");
        resolve!(nvnCommandBufferSetRenderEnableConditional, "nvnCommandBufferSetRenderEnableConditional");
        resolve!(nvnCommandBufferSetRenderTargets, "nvnCommandBufferSetRenderTargets");
        resolve!(nvnCommandBufferDiscardColor, "nvnCommandBufferDiscardColor");
        resolve!(nvnCommandBufferDiscardDepthStencil, "nvnCommandBufferDiscardDepthStencil");
        resolve!(nvnCommandBufferDownsample, "nvnCommandBufferDownsample");
        resolve!(nvnCommandBufferTiledDownsample, "nvnCommandBufferTiledDownsample");
        resolve!(nvnCommandBufferDownsampleTextureView, "nvnCommandBufferDownsampleTextureView");
        resolve!(nvnCommandBufferTiledDownsampleTextureView, "nvnCommandBufferTiledDownsampleTextureView");
        resolve!(nvnCommandBufferBarrier, "nvnCommandBufferBarrier");
        resolve!(nvnCommandBufferWaitSync, "nvnCommandBufferWaitSync");
        resolve!(nvnCommandBufferFenceSync, "nvnCommandBufferFenceSync");
        resolve!(nvnCommandBufferSetTexturePool, "nvnCommandBufferSetTexturePool");
        resolve!(nvnCommandBufferSetSamplerPool, "nvnCommandBufferSetSamplerPool");
        resolve!(nvnCommandBufferSetShaderScratchMemory, "nvnCommandBufferSetShaderScratchMemory");
        resolve!(nvnCommandBufferSaveZCullData, "nvnCommandBufferSaveZCullData");
        resolve!(nvnCommandBufferRestoreZCullData, "nvnCommandBufferRestoreZCullData");
        resolve!(nvnCommandBufferSetCopyRowStride, "nvnCommandBufferSetCopyRowStride");
        resolve!(nvnCommandBufferSetCopyImageStride, "nvnCommandBufferSetCopyImageStride");
        resolve!(nvnCommandBufferGetCopyRowStride, "nvnCommandBufferGetCopyRowStride");
        resolve!(nvnCommandBufferGetCopyImageStride, "nvnCommandBufferGetCopyImageStride");
        resolve!(nvnCommandBufferDrawTexture, "nvnCommandBufferDrawTexture");
        resolve!(nvnCommandBufferSetProgramSubroutines, "nvnCommandBufferSetProgramSubroutines");
        resolve!(nvnCommandBufferBindCoverageModulationTable, "nvnCommandBufferBindCoverageModulationTable");
        resolve!(nvnCommandBufferResolveDepthBuffer, "nvnCommandBufferResolveDepthBuffer");
        resolve!(nvnCommandBufferSetColorReductionEnable, "nvnCommandBufferSetColorReductionEnable");
        resolve!(nvnCommandBufferSetColorReductionThresholds, "nvnCommandBufferSetColorReductionThresholds");
        resolve!(nvnCommandBufferPushDebugGroupStatic, "nvnCommandBufferPushDebugGroupStatic");
        resolve!(nvnCommandBufferPushDebugGroupDynamic, "nvnCommandBufferPushDebugGroupDynamic");
        resolve!(nvnCommandBufferPushDebugGroup, "nvnCommandBufferPushDebugGroup");
        resolve!(nvnCommandBufferPopDebugGroup, "nvnCommandBufferPopDebugGroup");
        resolve!(nvnCommandBufferPopDebugGroupId, "nvnCommandBufferPopDebugGroupId");
        resolve!(nvnCommandBufferInsertDebugMarkerStatic, "nvnCommandBufferInsertDebugMarkerStatic");
        resolve!(nvnCommandBufferInsertDebugMarkerDynamic, "nvnCommandBufferInsertDebugMarkerDynamic");
        resolve!(nvnCommandBufferInsertDebugMarker, "nvnCommandBufferInsertDebugMarker");
        resolve!(nvnCommandBufferGetMemoryCallback, "nvnCommandBufferGetMemoryCallback");
        resolve!(nvnCommandBufferGetMemoryCallbackData, "nvnCommandBufferGetMemoryCallbackData");
        resolve!(nvnCommandBufferIsRecording, "nvnCommandBufferIsRecording");
        resolve!(nvnCommandBufferWaitEvent, "nvnCommandBufferWaitEvent");
        resolve!(nvnCommandBufferSignalEvent, "nvnCommandBufferSignalEvent");
        resolve!(nvnCommandBufferSetStencilCullCriteria, "nvnCommandBufferSetStencilCullCriteria");
    }

    // Sync
    {
        use sync::*;
        resolve!(nvnSyncInitialize, "nvnSyncInitialize");
        resolve!(nvnSyncFinalize, "nvnSyncFinalize");
        resolve!(nvnSyncSetDebugLabel, "nvnSyncSetDebugLabel");
        resolve!(nvnSyncWait, "nvnSyncWait");
        resolve!(nvnSyncInitializeFromFencedGLSync, "nvnSyncInitializeFromFencedGLSync");
        resolve!(nvnSyncCreateGLSync, "nvnSyncCreateGLSync");

        resolve!(nvnEventBuilderSetDefaults, "nvnEventBuilderSetDefaults");
        resolve!(nvnEventBuilderSetStorage, "nvnEventBuilderSetStorage");
        resolve!(nvnEventBuilderGetStorage, "nvnEventBuilderGetStorage");
        resolve!(nvnEventBuilderGetMemoryPool, "nvnEventBuilderGetMemoryPool");
        resolve!(nvnEventBuilderGetMemoryOffset, "nvnEventBuilderGetMemoryOffset");
        resolve!(nvnEventInitialize, "nvnEventInitialize");
        resolve!(nvnEventFinalize, "nvnEventFinalize");
        resolve!(nvnEventGetValue, "nvnEventGetValue");
        resolve!(nvnEventSignal, "nvnEventSignal");
        resolve!(nvnEventGetMemoryPool, "nvnEventGetMemoryPool");
        resolve!(nvnEventGetMemoryOffset, "nvnEventGetMemoryOffset");
    }

    // Queue sync functions (defined in queue but resolved here)
    {
        use queue::*;
        resolve!(nvnQueueFenceSync, "nvnQueueFenceSync");
        resolve!(nvnQueueWaitSync, "nvnQueueWaitSync");
    }
}
