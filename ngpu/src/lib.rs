#![allow(warnings)]
use std::sync::atomic::{AtomicBool, Ordering};

pub mod device;
pub mod queue;
pub mod window;
pub mod mem;
pub mod resource;
pub mod cmdbuf;
pub mod sync;
pub mod consts;

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
macro_rules! gpu_api {
    ($(
        $(#[$meta:meta])*
        pub static $name:ident: fn($($arg:ty),* $(,)?) $(-> $ret:ty)?;
    )*) => {
        $(
            $(#[$meta])*
            pub static mut $name: core::mem::MaybeUninit<unsafe extern "C" fn($($arg),*) $(-> $ret)?> =
                core::mem::MaybeUninit::uninit();
        )*
    };
}

#[macro_export]
macro_rules! nvn_wrap_void {
    ($fn_name:ident($($arg:ident : $arg_ty:ty),* $(,)?) => $slot:ident) => {
        #[inline(always)]
        pub unsafe fn $fn_name($($arg: $arg_ty),*) {
            let fp: unsafe extern "C" fn($($arg_ty),*) =
                $crate::load_slot_fn($crate::consts::$slot);
            fp($($arg),*);
        }
    };
}

#[macro_export]
macro_rules! nvn_wrap_ret {
    ($fn_name:ident($($arg:ident : $arg_ty:ty),* $(,)?) -> $ret:ty => $slot:ident) => {
        #[inline(always)]
        pub unsafe fn $fn_name($($arg: $arg_ty),*) -> $ret {
            let fp: unsafe extern "C" fn($($arg_ty),*) -> $ret =
                $crate::load_slot_fn($crate::consts::$slot);
            fp($($arg),*)
        }
    };
}

type ProcResolverFn = unsafe extern "C" fn(u64, *const u8) -> *const ();
static INITIALIZED: AtomicBool = AtomicBool::new(false);

#[inline(always)]
unsafe fn read_slot(slot_addr: usize) -> usize {
    *(slot_addr as *const usize)
}

union SlotFnCast<F: Copy> {
    raw: usize,
    typed: F,
}

#[inline(always)]
pub unsafe fn load_slot_fn<F: Copy>(slot_addr: usize) -> F {
    SlotFnCast::<F> {
        raw: read_slot(slot_addr),
    }
    .typed
}

// ── Initialization ──

/// Resolves all NVN function pointers using the resolver callback.
///
/// `resolver` and `get_proc` correspond to the two arguments passed to
/// the symbol-loading function found in the game binary
///
/// `get_proc` is called as `get_proc(resolver, b"nvnFunctionName\0".as_ptr()) -> fn ptr`
pub unsafe fn init_from_resolver(
    resolver: u64,
    get_proc: ProcResolverFn,
) {
    macro_rules! resolve {
        ($name:ident, $cname:expr) => {
            core::ptr::addr_of_mut!($name).write(core::mem::MaybeUninit::new(core::mem::transmute(get_proc(
                resolver,
                concat!($cname, "\0").as_ptr(),
            ))));
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


/// Resolves all GPU entry points directly from static slot addresses in `consts.rs`.
///
/// This is the fastest path: no name hashing/string lookups at runtime.
#[inline(always)]
pub unsafe fn init_from_slots() {
    use consts::*;
    use cmdbuf::*;
    use device::*;
    use mem::*;
    use queue::*;
    use resource::*;
    use sync::*;
    use window::*;

    #[inline(always)]
    unsafe fn read_proc(slot_addr: usize) -> *const () {
        (*(slot_addr as *const usize)) as *const ()
    }

    macro_rules! resolve_slot {
        ($name:ident, $slot:ident) => {
            core::ptr::addr_of_mut!($name)
                .write(core::mem::MaybeUninit::new(core::mem::transmute(read_proc($slot))));
        };
    }

    resolve_slot!(nvnDeviceBuilderSetDefaults, SLOT_NVN_DEVICE_BUILDER_SET_DEFAULTS);
    resolve_slot!(nvnDeviceBuilderSetFlags, SLOT_NVN_DEVICE_BUILDER_SET_FLAGS);
    resolve_slot!(nvnDeviceBuilderGetFlags, SLOT_NVN_DEVICE_BUILDER_GET_FLAGS);
    resolve_slot!(nvnDeviceInitialize, SLOT_NVN_DEVICE_INITIALIZE);
    resolve_slot!(nvnDeviceFinalize, SLOT_NVN_DEVICE_FINALIZE);
    resolve_slot!(nvnDeviceSetDebugLabel, SLOT_NVN_DEVICE_SET_DEBUG_LABEL);
    resolve_slot!(nvnDeviceGetProcAddress, SLOT_NVN_DEVICE_GET_PROC_ADDRESS);
    resolve_slot!(nvnDeviceGetInteger, SLOT_NVN_DEVICE_GET_INTEGER);
    resolve_slot!(nvnDeviceGetCurrentTimestampInNanoseconds, SLOT_NVN_DEVICE_GET_CURRENT_TIMESTAMP_IN_NANOSECONDS);
    resolve_slot!(nvnDeviceSetIntermediateShaderCache, SLOT_NVN_DEVICE_SET_INTERMEDIATE_SHADER_CACHE);
    resolve_slot!(nvnDeviceGetTextureHandle, SLOT_NVN_DEVICE_GET_TEXTURE_HANDLE);
    resolve_slot!(nvnDeviceGetTexelFetchHandle, SLOT_NVN_DEVICE_GET_TEXEL_FETCH_HANDLE);
    resolve_slot!(nvnDeviceGetImageHandle, SLOT_NVN_DEVICE_GET_IMAGE_HANDLE);
    resolve_slot!(nvnDeviceInstallDebugCallback, SLOT_NVN_DEVICE_INSTALL_DEBUG_CALLBACK);
    resolve_slot!(nvnDeviceGenerateDebugDomainId, SLOT_NVN_DEVICE_GENERATE_DEBUG_DOMAIN_ID);
    resolve_slot!(nvnDeviceSetWindowOriginMode, SLOT_NVN_DEVICE_SET_WINDOW_ORIGIN_MODE);
    resolve_slot!(nvnDeviceSetDepthMode, SLOT_NVN_DEVICE_SET_DEPTH_MODE);
    resolve_slot!(nvnDeviceRegisterFastClearColor, SLOT_NVN_DEVICE_REGISTER_FAST_CLEAR_COLOR);
    resolve_slot!(nvnDeviceRegisterFastClearColori, SLOT_NVN_DEVICE_REGISTER_FAST_CLEAR_COLORI);
    resolve_slot!(nvnDeviceRegisterFastClearColorui, SLOT_NVN_DEVICE_REGISTER_FAST_CLEAR_COLORUI);
    resolve_slot!(nvnDeviceRegisterFastClearDepth, SLOT_NVN_DEVICE_REGISTER_FAST_CLEAR_DEPTH);
    resolve_slot!(nvnDeviceGetWindowOriginMode, SLOT_NVN_DEVICE_GET_WINDOW_ORIGIN_MODE);
    resolve_slot!(nvnDeviceGetDepthMode, SLOT_NVN_DEVICE_GET_DEPTH_MODE);
    resolve_slot!(nvnDeviceGetTimestampInNanoseconds, SLOT_NVN_DEVICE_GET_TIMESTAMP_IN_NANOSECONDS);
    resolve_slot!(nvnDeviceApplyDeferredFinalizes, SLOT_NVN_DEVICE_APPLY_DEFERRED_FINALIZES);
    resolve_slot!(nvnDeviceFinalizeCommandHandle, SLOT_NVN_DEVICE_FINALIZE_COMMAND_HANDLE);
    resolve_slot!(nvnDeviceWalkDebugDatabase, SLOT_NVN_DEVICE_WALK_DEBUG_DATABASE);
    resolve_slot!(nvnDeviceGetSeparateTextureHandle, SLOT_NVN_DEVICE_GET_SEPARATE_TEXTURE_HANDLE);
    resolve_slot!(nvnDeviceGetSeparateSamplerHandle, SLOT_NVN_DEVICE_GET_SEPARATE_SAMPLER_HANDLE);
    resolve_slot!(nvnDeviceIsExternalDebuggerAttached, SLOT_NVN_DEVICE_IS_EXTERNAL_DEBUGGER_ATTACHED);
    resolve_slot!(nvnQueueGetError, SLOT_NVN_QUEUE_GET_ERROR);
    resolve_slot!(nvnQueueGetTotalCommandMemoryUsed, SLOT_NVN_QUEUE_GET_TOTAL_COMMAND_MEMORY_USED);
    resolve_slot!(nvnQueueGetTotalControlMemoryUsed, SLOT_NVN_QUEUE_GET_TOTAL_CONTROL_MEMORY_USED);
    resolve_slot!(nvnQueueGetTotalComputeMemoryUsed, SLOT_NVN_QUEUE_GET_TOTAL_COMPUTE_MEMORY_USED);
    resolve_slot!(nvnQueueResetMemoryUsageCounts, SLOT_NVN_QUEUE_RESET_MEMORY_USAGE_COUNTS);
    resolve_slot!(nvnQueueBuilderSetDevice, SLOT_NVN_QUEUE_BUILDER_SET_DEVICE);
    resolve_slot!(nvnQueueBuilderSetDefaults, SLOT_NVN_QUEUE_BUILDER_SET_DEFAULTS);
    resolve_slot!(nvnQueueBuilderSetFlags, SLOT_NVN_QUEUE_BUILDER_SET_FLAGS);
    resolve_slot!(nvnQueueBuilderSetCommandMemorySize, SLOT_NVN_QUEUE_BUILDER_SET_COMMAND_MEMORY_SIZE);
    resolve_slot!(nvnQueueBuilderSetComputeMemorySize, SLOT_NVN_QUEUE_BUILDER_SET_COMPUTE_MEMORY_SIZE);
    resolve_slot!(nvnQueueBuilderSetControlMemorySize, SLOT_NVN_QUEUE_BUILDER_SET_CONTROL_MEMORY_SIZE);
    resolve_slot!(nvnQueueBuilderGetQueueMemorySize, SLOT_NVN_QUEUE_BUILDER_GET_QUEUE_MEMORY_SIZE);
    resolve_slot!(nvnQueueBuilderSetQueueMemory, SLOT_NVN_QUEUE_BUILDER_SET_QUEUE_MEMORY);
    resolve_slot!(nvnQueueBuilderSetCommandFlushThreshold, SLOT_NVN_QUEUE_BUILDER_SET_COMMAND_FLUSH_THRESHOLD);
    resolve_slot!(nvnQueueBuilderSetQueuePriority, SLOT_NVN_QUEUE_BUILDER_SET_QUEUE_PRIORITY);
    resolve_slot!(nvnQueueBuilderGetQueuePriority, SLOT_NVN_QUEUE_BUILDER_GET_QUEUE_PRIORITY);
    resolve_slot!(nvnQueueBuilderGetDevice, SLOT_NVN_QUEUE_BUILDER_GET_DEVICE);
    resolve_slot!(nvnQueueBuilderGetFlags, SLOT_NVN_QUEUE_BUILDER_GET_FLAGS);
    resolve_slot!(nvnQueueBuilderGetCommandMemorySize, SLOT_NVN_QUEUE_BUILDER_GET_COMMAND_MEMORY_SIZE);
    resolve_slot!(nvnQueueBuilderGetComputeMemorySize, SLOT_NVN_QUEUE_BUILDER_GET_COMPUTE_MEMORY_SIZE);
    resolve_slot!(nvnQueueBuilderGetControlMemorySize, SLOT_NVN_QUEUE_BUILDER_GET_CONTROL_MEMORY_SIZE);
    resolve_slot!(nvnQueueBuilderGetCommandFlushThreshold, SLOT_NVN_QUEUE_BUILDER_GET_COMMAND_FLUSH_THRESHOLD);
    resolve_slot!(nvnQueueBuilderGetMemorySize, SLOT_NVN_QUEUE_BUILDER_GET_MEMORY_SIZE);
    resolve_slot!(nvnQueueBuilderGetMemory, SLOT_NVN_QUEUE_BUILDER_GET_MEMORY);
    resolve_slot!(nvnQueueInitialize, SLOT_NVN_QUEUE_INITIALIZE);
    resolve_slot!(nvnQueueFinalize, SLOT_NVN_QUEUE_FINALIZE);
    resolve_slot!(nvnQueueSetDebugLabel, SLOT_NVN_QUEUE_SET_DEBUG_LABEL);
    resolve_slot!(nvnQueueSubmitCommands, SLOT_NVN_QUEUE_SUBMIT_COMMANDS);
    resolve_slot!(nvnQueueFlush, SLOT_NVN_QUEUE_FLUSH);
    resolve_slot!(nvnQueueFinish, SLOT_NVN_QUEUE_FINISH);
    resolve_slot!(nvnQueuePresentTexture, SLOT_NVN_QUEUE_PRESENT_TEXTURE);
    resolve_slot!(nvnQueueAcquireTexture, SLOT_NVN_QUEUE_ACQUIRE_TEXTURE);
    resolve_slot!(nvnWindowBuilderSetDevice, SLOT_NVN_WINDOW_BUILDER_SET_DEVICE);
    resolve_slot!(nvnWindowBuilderSetDefaults, SLOT_NVN_WINDOW_BUILDER_SET_DEFAULTS);
    resolve_slot!(nvnWindowBuilderSetNativeWindow, SLOT_NVN_WINDOW_BUILDER_SET_NATIVE_WINDOW);
    resolve_slot!(nvnWindowBuilderSetTextures, SLOT_NVN_WINDOW_BUILDER_SET_TEXTURES);
    resolve_slot!(nvnWindowBuilderSetPresentInterval, SLOT_NVN_WINDOW_BUILDER_SET_PRESENT_INTERVAL);
    resolve_slot!(nvnWindowBuilderSetNumActiveTextures, SLOT_NVN_WINDOW_BUILDER_SET_NUM_ACTIVE_TEXTURES);
    resolve_slot!(nvnWindowBuilderGetDevice, SLOT_NVN_WINDOW_BUILDER_GET_DEVICE);
    resolve_slot!(nvnWindowBuilderGetNumTextures, SLOT_NVN_WINDOW_BUILDER_GET_NUM_TEXTURES);
    resolve_slot!(nvnWindowBuilderGetTexture, SLOT_NVN_WINDOW_BUILDER_GET_TEXTURE);
    resolve_slot!(nvnWindowBuilderGetNativeWindow, SLOT_NVN_WINDOW_BUILDER_GET_NATIVE_WINDOW);
    resolve_slot!(nvnWindowBuilderGetPresentInterval, SLOT_NVN_WINDOW_BUILDER_GET_PRESENT_INTERVAL);
    resolve_slot!(nvnWindowBuilderGetNumActiveTextures, SLOT_NVN_WINDOW_BUILDER_GET_NUM_ACTIVE_TEXTURES);
    resolve_slot!(nvnWindowInitialize, SLOT_NVN_WINDOW_INITIALIZE);
    resolve_slot!(nvnWindowFinalize, SLOT_NVN_WINDOW_FINALIZE);
    resolve_slot!(nvnWindowSetDebugLabel, SLOT_NVN_WINDOW_SET_DEBUG_LABEL);
    resolve_slot!(nvnWindowAcquireTexture, SLOT_NVN_WINDOW_ACQUIRE_TEXTURE);
    resolve_slot!(nvnWindowGetNativeWindow, SLOT_NVN_WINDOW_GET_NATIVE_WINDOW);
    resolve_slot!(nvnWindowGetPresentInterval, SLOT_NVN_WINDOW_GET_PRESENT_INTERVAL);
    resolve_slot!(nvnWindowSetPresentInterval, SLOT_NVN_WINDOW_SET_PRESENT_INTERVAL);
    resolve_slot!(nvnWindowSetCrop, SLOT_NVN_WINDOW_SET_CROP);
    resolve_slot!(nvnWindowGetCrop, SLOT_NVN_WINDOW_GET_CROP);
    resolve_slot!(nvnWindowSetNumActiveTextures, SLOT_NVN_WINDOW_SET_NUM_ACTIVE_TEXTURES);
    resolve_slot!(nvnWindowGetNumActiveTextures, SLOT_NVN_WINDOW_GET_NUM_ACTIVE_TEXTURES);
    resolve_slot!(nvnWindowGetNumTextures, SLOT_NVN_WINDOW_GET_NUM_TEXTURES);
    resolve_slot!(nvnMemoryPoolBuilderSetDevice, SLOT_NVN_MEMORY_POOL_BUILDER_SET_DEVICE);
    resolve_slot!(nvnMemoryPoolBuilderSetDefaults, SLOT_NVN_MEMORY_POOL_BUILDER_SET_DEFAULTS);
    resolve_slot!(nvnMemoryPoolBuilderSetStorage, SLOT_NVN_MEMORY_POOL_BUILDER_SET_STORAGE);
    resolve_slot!(nvnMemoryPoolBuilderSetFlags, SLOT_NVN_MEMORY_POOL_BUILDER_SET_FLAGS);
    resolve_slot!(nvnMemoryPoolBuilderGetDevice, SLOT_NVN_MEMORY_POOL_BUILDER_GET_DEVICE);
    resolve_slot!(nvnMemoryPoolBuilderGetMemory, SLOT_NVN_MEMORY_POOL_BUILDER_GET_MEMORY);
    resolve_slot!(nvnMemoryPoolBuilderGetSize, SLOT_NVN_MEMORY_POOL_BUILDER_GET_SIZE);
    resolve_slot!(nvnMemoryPoolBuilderGetFlags, SLOT_NVN_MEMORY_POOL_BUILDER_GET_FLAGS);
    resolve_slot!(nvnMemoryPoolInitialize, SLOT_NVN_MEMORY_POOL_INITIALIZE);
    resolve_slot!(nvnMemoryPoolSetDebugLabel, SLOT_NVN_MEMORY_POOL_SET_DEBUG_LABEL);
    resolve_slot!(nvnMemoryPoolFinalize, SLOT_NVN_MEMORY_POOL_FINALIZE);
    resolve_slot!(nvnMemoryPoolMap, SLOT_NVN_MEMORY_POOL_MAP);
    resolve_slot!(nvnMemoryPoolFlushMappedRange, SLOT_NVN_MEMORY_POOL_FLUSH_MAPPED_RANGE);
    resolve_slot!(nvnMemoryPoolInvalidateMappedRange, SLOT_NVN_MEMORY_POOL_INVALIDATE_MAPPED_RANGE);
    resolve_slot!(nvnMemoryPoolGetBufferAddress, SLOT_NVN_MEMORY_POOL_GET_BUFFER_ADDRESS);
    resolve_slot!(nvnMemoryPoolMapVirtual, SLOT_NVN_MEMORY_POOL_MAP_VIRTUAL);
    resolve_slot!(nvnMemoryPoolGetSize, SLOT_NVN_MEMORY_POOL_GET_SIZE);
    resolve_slot!(nvnMemoryPoolGetFlags, SLOT_NVN_MEMORY_POOL_GET_FLAGS);
    resolve_slot!(nvnTexturePoolInitialize, SLOT_NVN_TEXTURE_POOL_INITIALIZE);
    resolve_slot!(nvnTexturePoolSetDebugLabel, SLOT_NVN_TEXTURE_POOL_SET_DEBUG_LABEL);
    resolve_slot!(nvnTexturePoolFinalize, SLOT_NVN_TEXTURE_POOL_FINALIZE);
    resolve_slot!(nvnTexturePoolRegisterTexture, SLOT_NVN_TEXTURE_POOL_REGISTER_TEXTURE);
    resolve_slot!(nvnTexturePoolRegisterImage, SLOT_NVN_TEXTURE_POOL_REGISTER_IMAGE);
    resolve_slot!(nvnTexturePoolGetMemoryPool, SLOT_NVN_TEXTURE_POOL_GET_MEMORY_POOL);
    resolve_slot!(nvnTexturePoolGetMemoryOffset, SLOT_NVN_TEXTURE_POOL_GET_MEMORY_OFFSET);
    resolve_slot!(nvnTexturePoolGetSize, SLOT_NVN_TEXTURE_POOL_GET_SIZE);
    resolve_slot!(nvnSamplerPoolInitialize, SLOT_NVN_SAMPLER_POOL_INITIALIZE);
    resolve_slot!(nvnSamplerPoolSetDebugLabel, SLOT_NVN_SAMPLER_POOL_SET_DEBUG_LABEL);
    resolve_slot!(nvnSamplerPoolFinalize, SLOT_NVN_SAMPLER_POOL_FINALIZE);
    resolve_slot!(nvnSamplerPoolRegisterSampler, SLOT_NVN_SAMPLER_POOL_REGISTER_SAMPLER);
    resolve_slot!(nvnSamplerPoolRegisterSamplerBuilder, SLOT_NVN_SAMPLER_POOL_REGISTER_SAMPLER_BUILDER);
    resolve_slot!(nvnSamplerPoolGetMemoryPool, SLOT_NVN_SAMPLER_POOL_GET_MEMORY_POOL);
    resolve_slot!(nvnSamplerPoolGetMemoryOffset, SLOT_NVN_SAMPLER_POOL_GET_MEMORY_OFFSET);
    resolve_slot!(nvnSamplerPoolGetSize, SLOT_NVN_SAMPLER_POOL_GET_SIZE);
    resolve_slot!(nvnBufferBuilderSetDevice, SLOT_NVN_BUFFER_BUILDER_SET_DEVICE);
    resolve_slot!(nvnBufferBuilderSetDefaults, SLOT_NVN_BUFFER_BUILDER_SET_DEFAULTS);
    resolve_slot!(nvnBufferBuilderSetStorage, SLOT_NVN_BUFFER_BUILDER_SET_STORAGE);
    resolve_slot!(nvnBufferBuilderGetDevice, SLOT_NVN_BUFFER_BUILDER_GET_DEVICE);
    resolve_slot!(nvnBufferBuilderGetMemoryPool, SLOT_NVN_BUFFER_BUILDER_GET_MEMORY_POOL);
    resolve_slot!(nvnBufferBuilderGetMemoryOffset, SLOT_NVN_BUFFER_BUILDER_GET_MEMORY_OFFSET);
    resolve_slot!(nvnBufferBuilderGetSize, SLOT_NVN_BUFFER_BUILDER_GET_SIZE);
    resolve_slot!(nvnBufferInitialize, SLOT_NVN_BUFFER_INITIALIZE);
    resolve_slot!(nvnBufferSetDebugLabel, SLOT_NVN_BUFFER_SET_DEBUG_LABEL);
    resolve_slot!(nvnBufferFinalize, SLOT_NVN_BUFFER_FINALIZE);
    resolve_slot!(nvnBufferMap, SLOT_NVN_BUFFER_MAP);
    resolve_slot!(nvnBufferGetAddress, SLOT_NVN_BUFFER_GET_ADDRESS);
    resolve_slot!(nvnBufferFlushMappedRange, SLOT_NVN_BUFFER_FLUSH_MAPPED_RANGE);
    resolve_slot!(nvnBufferInvalidateMappedRange, SLOT_NVN_BUFFER_INVALIDATE_MAPPED_RANGE);
    resolve_slot!(nvnBufferGetMemoryPool, SLOT_NVN_BUFFER_GET_MEMORY_POOL);
    resolve_slot!(nvnBufferGetMemoryOffset, SLOT_NVN_BUFFER_GET_MEMORY_OFFSET);
    resolve_slot!(nvnBufferGetSize, SLOT_NVN_BUFFER_GET_SIZE);
    resolve_slot!(nvnBufferGetDebugID, SLOT_NVN_BUFFER_GET_DEBUG_ID);
    resolve_slot!(nvnTextureBuilderSetDevice, SLOT_NVN_TEXTURE_BUILDER_SET_DEVICE);
    resolve_slot!(nvnTextureBuilderSetDefaults, SLOT_NVN_TEXTURE_BUILDER_SET_DEFAULTS);
    resolve_slot!(nvnTextureBuilderSetFlags, SLOT_NVN_TEXTURE_BUILDER_SET_FLAGS);
    resolve_slot!(nvnTextureBuilderSetTarget, SLOT_NVN_TEXTURE_BUILDER_SET_TARGET);
    resolve_slot!(nvnTextureBuilderSetWidth, SLOT_NVN_TEXTURE_BUILDER_SET_WIDTH);
    resolve_slot!(nvnTextureBuilderSetHeight, SLOT_NVN_TEXTURE_BUILDER_SET_HEIGHT);
    resolve_slot!(nvnTextureBuilderSetDepth, SLOT_NVN_TEXTURE_BUILDER_SET_DEPTH);
    resolve_slot!(nvnTextureBuilderSetSize1D, SLOT_NVN_TEXTURE_BUILDER_SET_SIZE1_D);
    resolve_slot!(nvnTextureBuilderSetSize2D, SLOT_NVN_TEXTURE_BUILDER_SET_SIZE2_D);
    resolve_slot!(nvnTextureBuilderSetSize3D, SLOT_NVN_TEXTURE_BUILDER_SET_SIZE3_D);
    resolve_slot!(nvnTextureBuilderSetLevels, SLOT_NVN_TEXTURE_BUILDER_SET_LEVELS);
    resolve_slot!(nvnTextureBuilderSetFormat, SLOT_NVN_TEXTURE_BUILDER_SET_FORMAT);
    resolve_slot!(nvnTextureBuilderSetSamples, SLOT_NVN_TEXTURE_BUILDER_SET_SAMPLES);
    resolve_slot!(nvnTextureBuilderSetSwizzle, SLOT_NVN_TEXTURE_BUILDER_SET_SWIZZLE);
    resolve_slot!(nvnTextureBuilderSetDepthStencilMode, SLOT_NVN_TEXTURE_BUILDER_SET_DEPTH_STENCIL_MODE);
    resolve_slot!(nvnTextureBuilderGetStorageSize, SLOT_NVN_TEXTURE_BUILDER_GET_STORAGE_SIZE);
    resolve_slot!(nvnTextureBuilderGetStorageAlignment, SLOT_NVN_TEXTURE_BUILDER_GET_STORAGE_ALIGNMENT);
    resolve_slot!(nvnTextureBuilderSetStorage, SLOT_NVN_TEXTURE_BUILDER_SET_STORAGE);
    resolve_slot!(nvnTextureBuilderSetPackagedTextureData, SLOT_NVN_TEXTURE_BUILDER_SET_PACKAGED_TEXTURE_DATA);
    resolve_slot!(nvnTextureBuilderSetPackagedTextureLayout, SLOT_NVN_TEXTURE_BUILDER_SET_PACKAGED_TEXTURE_LAYOUT);
    resolve_slot!(nvnTextureBuilderSetStride, SLOT_NVN_TEXTURE_BUILDER_SET_STRIDE);
    resolve_slot!(nvnTextureBuilderSetGLTextureName, SLOT_NVN_TEXTURE_BUILDER_SET_GL_TEXTURE_NAME);
    resolve_slot!(nvnTextureBuilderGetStorageClass, SLOT_NVN_TEXTURE_BUILDER_GET_STORAGE_CLASS);
    resolve_slot!(nvnTextureBuilderGetDevice, SLOT_NVN_TEXTURE_BUILDER_GET_DEVICE);
    resolve_slot!(nvnTextureBuilderGetFlags, SLOT_NVN_TEXTURE_BUILDER_GET_FLAGS);
    resolve_slot!(nvnTextureBuilderGetTarget, SLOT_NVN_TEXTURE_BUILDER_GET_TARGET);
    resolve_slot!(nvnTextureBuilderGetWidth, SLOT_NVN_TEXTURE_BUILDER_GET_WIDTH);
    resolve_slot!(nvnTextureBuilderGetHeight, SLOT_NVN_TEXTURE_BUILDER_GET_HEIGHT);
    resolve_slot!(nvnTextureBuilderGetDepth, SLOT_NVN_TEXTURE_BUILDER_GET_DEPTH);
    resolve_slot!(nvnTextureBuilderGetLevels, SLOT_NVN_TEXTURE_BUILDER_GET_LEVELS);
    resolve_slot!(nvnTextureBuilderGetFormat, SLOT_NVN_TEXTURE_BUILDER_GET_FORMAT);
    resolve_slot!(nvnTextureBuilderGetSamples, SLOT_NVN_TEXTURE_BUILDER_GET_SAMPLES);
    resolve_slot!(nvnTextureBuilderGetSwizzle, SLOT_NVN_TEXTURE_BUILDER_GET_SWIZZLE);
    resolve_slot!(nvnTextureBuilderGetDepthStencilMode, SLOT_NVN_TEXTURE_BUILDER_GET_DEPTH_STENCIL_MODE);
    resolve_slot!(nvnTextureBuilderGetPackagedTextureData, SLOT_NVN_TEXTURE_BUILDER_GET_PACKAGED_TEXTURE_DATA);
    resolve_slot!(nvnTextureBuilderGetPackagedTextureLayout, SLOT_NVN_TEXTURE_BUILDER_GET_PACKAGED_TEXTURE_LAYOUT);
    resolve_slot!(nvnTextureBuilderGetStride, SLOT_NVN_TEXTURE_BUILDER_GET_STRIDE);
    resolve_slot!(nvnTextureBuilderGetSparseTileLayout, SLOT_NVN_TEXTURE_BUILDER_GET_SPARSE_TILE_LAYOUT);
    resolve_slot!(nvnTextureBuilderGetGLTextureName, SLOT_NVN_TEXTURE_BUILDER_GET_GL_TEXTURE_NAME);
    resolve_slot!(nvnTextureBuilderGetZCullStorageSize, SLOT_NVN_TEXTURE_BUILDER_GET_Z_CULL_STORAGE_SIZE);
    resolve_slot!(nvnTextureBuilderGetMemoryPool, SLOT_NVN_TEXTURE_BUILDER_GET_MEMORY_POOL);
    resolve_slot!(nvnTextureBuilderGetMemoryOffset, SLOT_NVN_TEXTURE_BUILDER_GET_MEMORY_OFFSET);
    resolve_slot!(nvnTextureBuilderGetRawStorageClass, SLOT_NVN_TEXTURE_BUILDER_GET_RAW_STORAGE_CLASS);
    resolve_slot!(nvnTextureViewSetDefaults, SLOT_NVN_TEXTURE_VIEW_SET_DEFAULTS);
    resolve_slot!(nvnTextureViewSetLevels, SLOT_NVN_TEXTURE_VIEW_SET_LEVELS);
    resolve_slot!(nvnTextureViewSetLayers, SLOT_NVN_TEXTURE_VIEW_SET_LAYERS);
    resolve_slot!(nvnTextureViewSetFormat, SLOT_NVN_TEXTURE_VIEW_SET_FORMAT);
    resolve_slot!(nvnTextureViewSetSwizzle, SLOT_NVN_TEXTURE_VIEW_SET_SWIZZLE);
    resolve_slot!(nvnTextureViewSetDepthStencilMode, SLOT_NVN_TEXTURE_VIEW_SET_DEPTH_STENCIL_MODE);
    resolve_slot!(nvnTextureViewSetTarget, SLOT_NVN_TEXTURE_VIEW_SET_TARGET);
    resolve_slot!(nvnTextureViewGetLevels, SLOT_NVN_TEXTURE_VIEW_GET_LEVELS);
    resolve_slot!(nvnTextureViewGetLayers, SLOT_NVN_TEXTURE_VIEW_GET_LAYERS);
    resolve_slot!(nvnTextureViewGetFormat, SLOT_NVN_TEXTURE_VIEW_GET_FORMAT);
    resolve_slot!(nvnTextureViewGetSwizzle, SLOT_NVN_TEXTURE_VIEW_GET_SWIZZLE);
    resolve_slot!(nvnTextureViewGetDepthStencilMode, SLOT_NVN_TEXTURE_VIEW_GET_DEPTH_STENCIL_MODE);
    resolve_slot!(nvnTextureViewGetTarget, SLOT_NVN_TEXTURE_VIEW_GET_TARGET);
    resolve_slot!(nvnTextureViewCompare, SLOT_NVN_TEXTURE_VIEW_COMPARE);
    resolve_slot!(nvnTextureInitialize, SLOT_NVN_TEXTURE_INITIALIZE);
    resolve_slot!(nvnTextureGetZCullStorageSize, SLOT_NVN_TEXTURE_GET_Z_CULL_STORAGE_SIZE);
    resolve_slot!(nvnTextureFinalize, SLOT_NVN_TEXTURE_FINALIZE);
    resolve_slot!(nvnTextureSetDebugLabel, SLOT_NVN_TEXTURE_SET_DEBUG_LABEL);
    resolve_slot!(nvnTextureGetStorageClass, SLOT_NVN_TEXTURE_GET_STORAGE_CLASS);
    resolve_slot!(nvnTextureGetViewOffset, SLOT_NVN_TEXTURE_GET_VIEW_OFFSET);
    resolve_slot!(nvnTextureGetFlags, SLOT_NVN_TEXTURE_GET_FLAGS);
    resolve_slot!(nvnTextureGetTarget, SLOT_NVN_TEXTURE_GET_TARGET);
    resolve_slot!(nvnTextureGetWidth, SLOT_NVN_TEXTURE_GET_WIDTH);
    resolve_slot!(nvnTextureGetHeight, SLOT_NVN_TEXTURE_GET_HEIGHT);
    resolve_slot!(nvnTextureGetDepth, SLOT_NVN_TEXTURE_GET_DEPTH);
    resolve_slot!(nvnTextureGetLevels, SLOT_NVN_TEXTURE_GET_LEVELS);
    resolve_slot!(nvnTextureGetFormat, SLOT_NVN_TEXTURE_GET_FORMAT);
    resolve_slot!(nvnTextureGetSamples, SLOT_NVN_TEXTURE_GET_SAMPLES);
    resolve_slot!(nvnTextureGetSwizzle, SLOT_NVN_TEXTURE_GET_SWIZZLE);
    resolve_slot!(nvnTextureGetDepthStencilMode, SLOT_NVN_TEXTURE_GET_DEPTH_STENCIL_MODE);
    resolve_slot!(nvnTextureGetStride, SLOT_NVN_TEXTURE_GET_STRIDE);
    resolve_slot!(nvnTextureGetTextureAddress, SLOT_NVN_TEXTURE_GET_TEXTURE_ADDRESS);
    resolve_slot!(nvnTextureGetSparseTileLayout, SLOT_NVN_TEXTURE_GET_SPARSE_TILE_LAYOUT);
    resolve_slot!(nvnTextureWriteTexels, SLOT_NVN_TEXTURE_WRITE_TEXELS);
    resolve_slot!(nvnTextureWriteTexelsStrided, SLOT_NVN_TEXTURE_WRITE_TEXELS_STRIDED);
    resolve_slot!(nvnTextureReadTexels, SLOT_NVN_TEXTURE_READ_TEXELS);
    resolve_slot!(nvnTextureReadTexelsStrided, SLOT_NVN_TEXTURE_READ_TEXELS_STRIDED);
    resolve_slot!(nvnTextureFlushTexels, SLOT_NVN_TEXTURE_FLUSH_TEXELS);
    resolve_slot!(nvnTextureInvalidateTexels, SLOT_NVN_TEXTURE_INVALIDATE_TEXELS);
    resolve_slot!(nvnTextureGetMemoryPool, SLOT_NVN_TEXTURE_GET_MEMORY_POOL);
    resolve_slot!(nvnTextureGetMemoryOffset, SLOT_NVN_TEXTURE_GET_MEMORY_OFFSET);
    resolve_slot!(nvnTextureGetStorageSize, SLOT_NVN_TEXTURE_GET_STORAGE_SIZE);
    resolve_slot!(nvnTextureCompare, SLOT_NVN_TEXTURE_COMPARE);
    resolve_slot!(nvnTextureGetDebugID, SLOT_NVN_TEXTURE_GET_DEBUG_ID);
    resolve_slot!(nvnTextureGetRawStorageClass, SLOT_NVN_TEXTURE_GET_RAW_STORAGE_CLASS);
    resolve_slot!(nvnSamplerBuilderSetDevice, SLOT_NVN_SAMPLER_BUILDER_SET_DEVICE);
    resolve_slot!(nvnSamplerBuilderSetDefaults, SLOT_NVN_SAMPLER_BUILDER_SET_DEFAULTS);
    resolve_slot!(nvnSamplerBuilderSetMinMagFilter, SLOT_NVN_SAMPLER_BUILDER_SET_MIN_MAG_FILTER);
    resolve_slot!(nvnSamplerBuilderSetWrapMode, SLOT_NVN_SAMPLER_BUILDER_SET_WRAP_MODE);
    resolve_slot!(nvnSamplerBuilderSetLodClamp, SLOT_NVN_SAMPLER_BUILDER_SET_LOD_CLAMP);
    resolve_slot!(nvnSamplerBuilderSetLodBias, SLOT_NVN_SAMPLER_BUILDER_SET_LOD_BIAS);
    resolve_slot!(nvnSamplerBuilderSetCompare, SLOT_NVN_SAMPLER_BUILDER_SET_COMPARE);
    resolve_slot!(nvnSamplerBuilderSetBorderColor, SLOT_NVN_SAMPLER_BUILDER_SET_BORDER_COLOR);
    resolve_slot!(nvnSamplerBuilderSetBorderColori, SLOT_NVN_SAMPLER_BUILDER_SET_BORDER_COLORI);
    resolve_slot!(nvnSamplerBuilderSetBorderColorui, SLOT_NVN_SAMPLER_BUILDER_SET_BORDER_COLORUI);
    resolve_slot!(nvnSamplerBuilderSetMaxAnisotropy, SLOT_NVN_SAMPLER_BUILDER_SET_MAX_ANISOTROPY);
    resolve_slot!(nvnSamplerBuilderSetReductionFilter, SLOT_NVN_SAMPLER_BUILDER_SET_REDUCTION_FILTER);
    resolve_slot!(nvnSamplerBuilderSetLodSnap, SLOT_NVN_SAMPLER_BUILDER_SET_LOD_SNAP);
    resolve_slot!(nvnSamplerBuilderGetDevice, SLOT_NVN_SAMPLER_BUILDER_GET_DEVICE);
    resolve_slot!(nvnSamplerBuilderGetMinMagFilter, SLOT_NVN_SAMPLER_BUILDER_GET_MIN_MAG_FILTER);
    resolve_slot!(nvnSamplerBuilderGetWrapMode, SLOT_NVN_SAMPLER_BUILDER_GET_WRAP_MODE);
    resolve_slot!(nvnSamplerBuilderGetLodClamp, SLOT_NVN_SAMPLER_BUILDER_GET_LOD_CLAMP);
    resolve_slot!(nvnSamplerBuilderGetLodBias, SLOT_NVN_SAMPLER_BUILDER_GET_LOD_BIAS);
    resolve_slot!(nvnSamplerBuilderGetCompare, SLOT_NVN_SAMPLER_BUILDER_GET_COMPARE);
    resolve_slot!(nvnSamplerBuilderGetBorderColor, SLOT_NVN_SAMPLER_BUILDER_GET_BORDER_COLOR);
    resolve_slot!(nvnSamplerBuilderGetBorderColori, SLOT_NVN_SAMPLER_BUILDER_GET_BORDER_COLORI);
    resolve_slot!(nvnSamplerBuilderGetBorderColorui, SLOT_NVN_SAMPLER_BUILDER_GET_BORDER_COLORUI);
    resolve_slot!(nvnSamplerBuilderGetMaxAnisotropy, SLOT_NVN_SAMPLER_BUILDER_GET_MAX_ANISOTROPY);
    resolve_slot!(nvnSamplerBuilderGetReductionFilter, SLOT_NVN_SAMPLER_BUILDER_GET_REDUCTION_FILTER);
    resolve_slot!(nvnSamplerBuilderGetLodSnap, SLOT_NVN_SAMPLER_BUILDER_GET_LOD_SNAP);
    resolve_slot!(nvnSamplerInitialize, SLOT_NVN_SAMPLER_INITIALIZE);
    resolve_slot!(nvnSamplerFinalize, SLOT_NVN_SAMPLER_FINALIZE);
    resolve_slot!(nvnSamplerSetDebugLabel, SLOT_NVN_SAMPLER_SET_DEBUG_LABEL);
    resolve_slot!(nvnSamplerGetMinMagFilter, SLOT_NVN_SAMPLER_GET_MIN_MAG_FILTER);
    resolve_slot!(nvnSamplerGetWrapMode, SLOT_NVN_SAMPLER_GET_WRAP_MODE);
    resolve_slot!(nvnSamplerGetLodClamp, SLOT_NVN_SAMPLER_GET_LOD_CLAMP);
    resolve_slot!(nvnSamplerGetLodBias, SLOT_NVN_SAMPLER_GET_LOD_BIAS);
    resolve_slot!(nvnSamplerGetCompare, SLOT_NVN_SAMPLER_GET_COMPARE);
    resolve_slot!(nvnSamplerGetBorderColor, SLOT_NVN_SAMPLER_GET_BORDER_COLOR);
    resolve_slot!(nvnSamplerGetBorderColori, SLOT_NVN_SAMPLER_GET_BORDER_COLORI);
    resolve_slot!(nvnSamplerGetBorderColorui, SLOT_NVN_SAMPLER_GET_BORDER_COLORUI);
    resolve_slot!(nvnSamplerGetMaxAnisotropy, SLOT_NVN_SAMPLER_GET_MAX_ANISOTROPY);
    resolve_slot!(nvnSamplerGetReductionFilter, SLOT_NVN_SAMPLER_GET_REDUCTION_FILTER);
    resolve_slot!(nvnSamplerCompare, SLOT_NVN_SAMPLER_COMPARE);
    resolve_slot!(nvnSamplerGetDebugID, SLOT_NVN_SAMPLER_GET_DEBUG_ID);
    resolve_slot!(nvnProgramInitialize, SLOT_NVN_PROGRAM_INITIALIZE);
    resolve_slot!(nvnProgramFinalize, SLOT_NVN_PROGRAM_FINALIZE);
    resolve_slot!(nvnProgramSetDebugLabel, SLOT_NVN_PROGRAM_SET_DEBUG_LABEL);
    resolve_slot!(nvnProgramSetShaders, SLOT_NVN_PROGRAM_SET_SHADERS);
    resolve_slot!(nvnProgramSetShadersExt, SLOT_NVN_PROGRAM_SET_SHADERS_EXT);
    resolve_slot!(nvnProgramSetSampleShading, SLOT_NVN_PROGRAM_SET_SAMPLE_SHADING);
    resolve_slot!(nvnProgramSetSubroutineLinkage, SLOT_NVN_PROGRAM_SET_SUBROUTINE_LINKAGE);
    resolve_slot!(nvnBlendStateSetDefaults, SLOT_NVN_BLEND_STATE_SET_DEFAULTS);
    resolve_slot!(nvnBlendStateSetBlendTarget, SLOT_NVN_BLEND_STATE_SET_BLEND_TARGET);
    resolve_slot!(nvnBlendStateSetBlendFunc, SLOT_NVN_BLEND_STATE_SET_BLEND_FUNC);
    resolve_slot!(nvnBlendStateSetBlendEquation, SLOT_NVN_BLEND_STATE_SET_BLEND_EQUATION);
    resolve_slot!(nvnBlendStateSetAdvancedMode, SLOT_NVN_BLEND_STATE_SET_ADVANCED_MODE);
    resolve_slot!(nvnBlendStateSetAdvancedOverlap, SLOT_NVN_BLEND_STATE_SET_ADVANCED_OVERLAP);
    resolve_slot!(nvnBlendStateSetAdvancedPremultipliedSrc, SLOT_NVN_BLEND_STATE_SET_ADVANCED_PREMULTIPLIED_SRC);
    resolve_slot!(nvnBlendStateSetAdvancedNormalizedDst, SLOT_NVN_BLEND_STATE_SET_ADVANCED_NORMALIZED_DST);
    resolve_slot!(nvnBlendStateGetBlendTarget, SLOT_NVN_BLEND_STATE_GET_BLEND_TARGET);
    resolve_slot!(nvnBlendStateGetBlendFunc, SLOT_NVN_BLEND_STATE_GET_BLEND_FUNC);
    resolve_slot!(nvnBlendStateGetBlendEquation, SLOT_NVN_BLEND_STATE_GET_BLEND_EQUATION);
    resolve_slot!(nvnBlendStateGetAdvancedMode, SLOT_NVN_BLEND_STATE_GET_ADVANCED_MODE);
    resolve_slot!(nvnBlendStateGetAdvancedOverlap, SLOT_NVN_BLEND_STATE_GET_ADVANCED_OVERLAP);
    resolve_slot!(nvnBlendStateGetAdvancedPremultipliedSrc, SLOT_NVN_BLEND_STATE_GET_ADVANCED_PREMULTIPLIED_SRC);
    resolve_slot!(nvnBlendStateGetAdvancedNormalizedDst, SLOT_NVN_BLEND_STATE_GET_ADVANCED_NORMALIZED_DST);
    resolve_slot!(nvnColorStateSetDefaults, SLOT_NVN_COLOR_STATE_SET_DEFAULTS);
    resolve_slot!(nvnColorStateSetBlendEnable, SLOT_NVN_COLOR_STATE_SET_BLEND_ENABLE);
    resolve_slot!(nvnColorStateSetLogicOp, SLOT_NVN_COLOR_STATE_SET_LOGIC_OP);
    resolve_slot!(nvnColorStateSetAlphaTest, SLOT_NVN_COLOR_STATE_SET_ALPHA_TEST);
    resolve_slot!(nvnColorStateGetBlendEnable, SLOT_NVN_COLOR_STATE_GET_BLEND_ENABLE);
    resolve_slot!(nvnColorStateGetLogicOp, SLOT_NVN_COLOR_STATE_GET_LOGIC_OP);
    resolve_slot!(nvnColorStateGetAlphaTest, SLOT_NVN_COLOR_STATE_GET_ALPHA_TEST);
    resolve_slot!(nvnChannelMaskStateSetDefaults, SLOT_NVN_CHANNEL_MASK_STATE_SET_DEFAULTS);
    resolve_slot!(nvnChannelMaskStateSetChannelMask, SLOT_NVN_CHANNEL_MASK_STATE_SET_CHANNEL_MASK);
    resolve_slot!(nvnChannelMaskStateGetChannelMask, SLOT_NVN_CHANNEL_MASK_STATE_GET_CHANNEL_MASK);
    resolve_slot!(nvnMultisampleStateSetDefaults, SLOT_NVN_MULTISAMPLE_STATE_SET_DEFAULTS);
    resolve_slot!(nvnMultisampleStateSetMultisampleEnable, SLOT_NVN_MULTISAMPLE_STATE_SET_MULTISAMPLE_ENABLE);
    resolve_slot!(nvnMultisampleStateSetSamples, SLOT_NVN_MULTISAMPLE_STATE_SET_SAMPLES);
    resolve_slot!(nvnMultisampleStateSetAlphaToCoverageEnable, SLOT_NVN_MULTISAMPLE_STATE_SET_ALPHA_TO_COVERAGE_ENABLE);
    resolve_slot!(nvnMultisampleStateSetAlphaToCoverageDither, SLOT_NVN_MULTISAMPLE_STATE_SET_ALPHA_TO_COVERAGE_DITHER);
    resolve_slot!(nvnMultisampleStateGetMultisampleEnable, SLOT_NVN_MULTISAMPLE_STATE_GET_MULTISAMPLE_ENABLE);
    resolve_slot!(nvnMultisampleStateGetSamples, SLOT_NVN_MULTISAMPLE_STATE_GET_SAMPLES);
    resolve_slot!(nvnMultisampleStateGetAlphaToCoverageEnable, SLOT_NVN_MULTISAMPLE_STATE_GET_ALPHA_TO_COVERAGE_ENABLE);
    resolve_slot!(nvnMultisampleStateGetAlphaToCoverageDither, SLOT_NVN_MULTISAMPLE_STATE_GET_ALPHA_TO_COVERAGE_DITHER);
    resolve_slot!(nvnMultisampleStateSetRasterSamples, SLOT_NVN_MULTISAMPLE_STATE_SET_RASTER_SAMPLES);
    resolve_slot!(nvnMultisampleStateGetRasterSamples, SLOT_NVN_MULTISAMPLE_STATE_GET_RASTER_SAMPLES);
    resolve_slot!(nvnMultisampleStateSetCoverageModulationMode, SLOT_NVN_MULTISAMPLE_STATE_SET_COVERAGE_MODULATION_MODE);
    resolve_slot!(nvnMultisampleStateGetCoverageModulationMode, SLOT_NVN_MULTISAMPLE_STATE_GET_COVERAGE_MODULATION_MODE);
    resolve_slot!(nvnMultisampleStateSetCoverageToColorEnable, SLOT_NVN_MULTISAMPLE_STATE_SET_COVERAGE_TO_COLOR_ENABLE);
    resolve_slot!(nvnMultisampleStateGetCoverageToColorEnable, SLOT_NVN_MULTISAMPLE_STATE_GET_COVERAGE_TO_COLOR_ENABLE);
    resolve_slot!(nvnMultisampleStateSetCoverageToColorOutput, SLOT_NVN_MULTISAMPLE_STATE_SET_COVERAGE_TO_COLOR_OUTPUT);
    resolve_slot!(nvnMultisampleStateGetCoverageToColorOutput, SLOT_NVN_MULTISAMPLE_STATE_GET_COVERAGE_TO_COLOR_OUTPUT);
    resolve_slot!(nvnMultisampleStateSetSampleLocationsEnable, SLOT_NVN_MULTISAMPLE_STATE_SET_SAMPLE_LOCATIONS_ENABLE);
    resolve_slot!(nvnMultisampleStateGetSampleLocationsEnable, SLOT_NVN_MULTISAMPLE_STATE_GET_SAMPLE_LOCATIONS_ENABLE);
    resolve_slot!(nvnMultisampleStateGetSampleLocationsGrid, SLOT_NVN_MULTISAMPLE_STATE_GET_SAMPLE_LOCATIONS_GRID);
    resolve_slot!(nvnMultisampleStateSetSampleLocationsGridEnable, SLOT_NVN_MULTISAMPLE_STATE_SET_SAMPLE_LOCATIONS_GRID_ENABLE);
    resolve_slot!(nvnMultisampleStateGetSampleLocationsGridEnable, SLOT_NVN_MULTISAMPLE_STATE_GET_SAMPLE_LOCATIONS_GRID_ENABLE);
    resolve_slot!(nvnMultisampleStateSetSampleLocations, SLOT_NVN_MULTISAMPLE_STATE_SET_SAMPLE_LOCATIONS);
    resolve_slot!(nvnPolygonStateSetDefaults, SLOT_NVN_POLYGON_STATE_SET_DEFAULTS);
    resolve_slot!(nvnPolygonStateSetCullFace, SLOT_NVN_POLYGON_STATE_SET_CULL_FACE);
    resolve_slot!(nvnPolygonStateSetFrontFace, SLOT_NVN_POLYGON_STATE_SET_FRONT_FACE);
    resolve_slot!(nvnPolygonStateSetPolygonMode, SLOT_NVN_POLYGON_STATE_SET_POLYGON_MODE);
    resolve_slot!(nvnPolygonStateSetPolygonOffsetEnables, SLOT_NVN_POLYGON_STATE_SET_POLYGON_OFFSET_ENABLES);
    resolve_slot!(nvnPolygonStateGetCullFace, SLOT_NVN_POLYGON_STATE_GET_CULL_FACE);
    resolve_slot!(nvnPolygonStateGetFrontFace, SLOT_NVN_POLYGON_STATE_GET_FRONT_FACE);
    resolve_slot!(nvnPolygonStateGetPolygonMode, SLOT_NVN_POLYGON_STATE_GET_POLYGON_MODE);
    resolve_slot!(nvnPolygonStateGetPolygonOffsetEnables, SLOT_NVN_POLYGON_STATE_GET_POLYGON_OFFSET_ENABLES);
    resolve_slot!(nvnDepthStencilStateSetDefaults, SLOT_NVN_DEPTH_STENCIL_STATE_SET_DEFAULTS);
    resolve_slot!(nvnDepthStencilStateSetDepthTestEnable, SLOT_NVN_DEPTH_STENCIL_STATE_SET_DEPTH_TEST_ENABLE);
    resolve_slot!(nvnDepthStencilStateSetDepthWriteEnable, SLOT_NVN_DEPTH_STENCIL_STATE_SET_DEPTH_WRITE_ENABLE);
    resolve_slot!(nvnDepthStencilStateSetDepthFunc, SLOT_NVN_DEPTH_STENCIL_STATE_SET_DEPTH_FUNC);
    resolve_slot!(nvnDepthStencilStateSetStencilTestEnable, SLOT_NVN_DEPTH_STENCIL_STATE_SET_STENCIL_TEST_ENABLE);
    resolve_slot!(nvnDepthStencilStateSetStencilFunc, SLOT_NVN_DEPTH_STENCIL_STATE_SET_STENCIL_FUNC);
    resolve_slot!(nvnDepthStencilStateSetStencilOp, SLOT_NVN_DEPTH_STENCIL_STATE_SET_STENCIL_OP);
    resolve_slot!(nvnDepthStencilStateGetDepthTestEnable, SLOT_NVN_DEPTH_STENCIL_STATE_GET_DEPTH_TEST_ENABLE);
    resolve_slot!(nvnDepthStencilStateGetDepthWriteEnable, SLOT_NVN_DEPTH_STENCIL_STATE_GET_DEPTH_WRITE_ENABLE);
    resolve_slot!(nvnDepthStencilStateGetDepthFunc, SLOT_NVN_DEPTH_STENCIL_STATE_GET_DEPTH_FUNC);
    resolve_slot!(nvnDepthStencilStateGetStencilTestEnable, SLOT_NVN_DEPTH_STENCIL_STATE_GET_STENCIL_TEST_ENABLE);
    resolve_slot!(nvnDepthStencilStateGetStencilFunc, SLOT_NVN_DEPTH_STENCIL_STATE_GET_STENCIL_FUNC);
    resolve_slot!(nvnDepthStencilStateGetStencilOp, SLOT_NVN_DEPTH_STENCIL_STATE_GET_STENCIL_OP);
    resolve_slot!(nvnVertexAttribStateSetDefaults, SLOT_NVN_VERTEX_ATTRIB_STATE_SET_DEFAULTS);
    resolve_slot!(nvnVertexAttribStateSetFormat, SLOT_NVN_VERTEX_ATTRIB_STATE_SET_FORMAT);
    resolve_slot!(nvnVertexAttribStateSetStreamIndex, SLOT_NVN_VERTEX_ATTRIB_STATE_SET_STREAM_INDEX);
    resolve_slot!(nvnVertexAttribStateGetFormat, SLOT_NVN_VERTEX_ATTRIB_STATE_GET_FORMAT);
    resolve_slot!(nvnVertexAttribStateGetStreamIndex, SLOT_NVN_VERTEX_ATTRIB_STATE_GET_STREAM_INDEX);
    resolve_slot!(nvnVertexStreamStateSetDefaults, SLOT_NVN_VERTEX_STREAM_STATE_SET_DEFAULTS);
    resolve_slot!(nvnVertexStreamStateSetStride, SLOT_NVN_VERTEX_STREAM_STATE_SET_STRIDE);
    resolve_slot!(nvnVertexStreamStateSetDivisor, SLOT_NVN_VERTEX_STREAM_STATE_SET_DIVISOR);
    resolve_slot!(nvnVertexStreamStateGetStride, SLOT_NVN_VERTEX_STREAM_STATE_GET_STRIDE);
    resolve_slot!(nvnVertexStreamStateGetDivisor, SLOT_NVN_VERTEX_STREAM_STATE_GET_DIVISOR);
    resolve_slot!(nvnCommandBufferInitialize, SLOT_NVN_COMMAND_BUFFER_INITIALIZE);
    resolve_slot!(nvnCommandBufferFinalize, SLOT_NVN_COMMAND_BUFFER_FINALIZE);
    resolve_slot!(nvnCommandBufferSetDebugLabel, SLOT_NVN_COMMAND_BUFFER_SET_DEBUG_LABEL);
    resolve_slot!(nvnCommandBufferSetMemoryCallback, SLOT_NVN_COMMAND_BUFFER_SET_MEMORY_CALLBACK);
    resolve_slot!(nvnCommandBufferSetMemoryCallbackData, SLOT_NVN_COMMAND_BUFFER_SET_MEMORY_CALLBACK_DATA);
    resolve_slot!(nvnCommandBufferSetCommandMemoryCallbackEnabled, SLOT_NVN_COMMAND_BUFFER_SET_COMMAND_MEMORY_CALLBACK_ENABLED);
    resolve_slot!(nvnCommandBufferAddCommandMemory, SLOT_NVN_COMMAND_BUFFER_ADD_COMMAND_MEMORY);
    resolve_slot!(nvnCommandBufferAddControlMemory, SLOT_NVN_COMMAND_BUFFER_ADD_CONTROL_MEMORY);
    resolve_slot!(nvnCommandBufferGetCommandMemorySize, SLOT_NVN_COMMAND_BUFFER_GET_COMMAND_MEMORY_SIZE);
    resolve_slot!(nvnCommandBufferGetCommandMemoryUsed, SLOT_NVN_COMMAND_BUFFER_GET_COMMAND_MEMORY_USED);
    resolve_slot!(nvnCommandBufferGetCommandMemoryFree, SLOT_NVN_COMMAND_BUFFER_GET_COMMAND_MEMORY_FREE);
    resolve_slot!(nvnCommandBufferGetControlMemorySize, SLOT_NVN_COMMAND_BUFFER_GET_CONTROL_MEMORY_SIZE);
    resolve_slot!(nvnCommandBufferGetControlMemoryUsed, SLOT_NVN_COMMAND_BUFFER_GET_CONTROL_MEMORY_USED);
    resolve_slot!(nvnCommandBufferGetControlMemoryFree, SLOT_NVN_COMMAND_BUFFER_GET_CONTROL_MEMORY_FREE);
    resolve_slot!(nvnCommandBufferBeginRecording, SLOT_NVN_COMMAND_BUFFER_BEGIN_RECORDING);
    resolve_slot!(nvnCommandBufferEndRecording, SLOT_NVN_COMMAND_BUFFER_END_RECORDING);
    resolve_slot!(nvnCommandBufferCallCommands, SLOT_NVN_COMMAND_BUFFER_CALL_COMMANDS);
    resolve_slot!(nvnCommandBufferCopyCommands, SLOT_NVN_COMMAND_BUFFER_COPY_COMMANDS);
    resolve_slot!(nvnCommandBufferBindBlendState, SLOT_NVN_COMMAND_BUFFER_BIND_BLEND_STATE);
    resolve_slot!(nvnCommandBufferBindChannelMaskState, SLOT_NVN_COMMAND_BUFFER_BIND_CHANNEL_MASK_STATE);
    resolve_slot!(nvnCommandBufferBindColorState, SLOT_NVN_COMMAND_BUFFER_BIND_COLOR_STATE);
    resolve_slot!(nvnCommandBufferBindMultisampleState, SLOT_NVN_COMMAND_BUFFER_BIND_MULTISAMPLE_STATE);
    resolve_slot!(nvnCommandBufferBindPolygonState, SLOT_NVN_COMMAND_BUFFER_BIND_POLYGON_STATE);
    resolve_slot!(nvnCommandBufferBindDepthStencilState, SLOT_NVN_COMMAND_BUFFER_BIND_DEPTH_STENCIL_STATE);
    resolve_slot!(nvnCommandBufferBindVertexAttribState, SLOT_NVN_COMMAND_BUFFER_BIND_VERTEX_ATTRIB_STATE);
    resolve_slot!(nvnCommandBufferBindVertexStreamState, SLOT_NVN_COMMAND_BUFFER_BIND_VERTEX_STREAM_STATE);
    resolve_slot!(nvnCommandBufferBindProgram, SLOT_NVN_COMMAND_BUFFER_BIND_PROGRAM);
    resolve_slot!(nvnCommandBufferBindVertexBuffer, SLOT_NVN_COMMAND_BUFFER_BIND_VERTEX_BUFFER);
    resolve_slot!(nvnCommandBufferBindVertexBuffers, SLOT_NVN_COMMAND_BUFFER_BIND_VERTEX_BUFFERS);
    resolve_slot!(nvnCommandBufferBindUniformBuffer, SLOT_NVN_COMMAND_BUFFER_BIND_UNIFORM_BUFFER);
    resolve_slot!(nvnCommandBufferBindUniformBuffers, SLOT_NVN_COMMAND_BUFFER_BIND_UNIFORM_BUFFERS);
    resolve_slot!(nvnCommandBufferBindTransformFeedbackBuffer, SLOT_NVN_COMMAND_BUFFER_BIND_TRANSFORM_FEEDBACK_BUFFER);
    resolve_slot!(nvnCommandBufferBindTransformFeedbackBuffers, SLOT_NVN_COMMAND_BUFFER_BIND_TRANSFORM_FEEDBACK_BUFFERS);
    resolve_slot!(nvnCommandBufferBindStorageBuffer, SLOT_NVN_COMMAND_BUFFER_BIND_STORAGE_BUFFER);
    resolve_slot!(nvnCommandBufferBindStorageBuffers, SLOT_NVN_COMMAND_BUFFER_BIND_STORAGE_BUFFERS);
    resolve_slot!(nvnCommandBufferBindTexture, SLOT_NVN_COMMAND_BUFFER_BIND_TEXTURE);
    resolve_slot!(nvnCommandBufferBindTextures, SLOT_NVN_COMMAND_BUFFER_BIND_TEXTURES);
    resolve_slot!(nvnCommandBufferBindImage, SLOT_NVN_COMMAND_BUFFER_BIND_IMAGE);
    resolve_slot!(nvnCommandBufferBindImages, SLOT_NVN_COMMAND_BUFFER_BIND_IMAGES);
    resolve_slot!(nvnCommandBufferSetPatchSize, SLOT_NVN_COMMAND_BUFFER_SET_PATCH_SIZE);
    resolve_slot!(nvnCommandBufferSetInnerTessellationLevels, SLOT_NVN_COMMAND_BUFFER_SET_INNER_TESSELLATION_LEVELS);
    resolve_slot!(nvnCommandBufferSetOuterTessellationLevels, SLOT_NVN_COMMAND_BUFFER_SET_OUTER_TESSELLATION_LEVELS);
    resolve_slot!(nvnCommandBufferSetPrimitiveRestart, SLOT_NVN_COMMAND_BUFFER_SET_PRIMITIVE_RESTART);
    resolve_slot!(nvnCommandBufferBeginTransformFeedback, SLOT_NVN_COMMAND_BUFFER_BEGIN_TRANSFORM_FEEDBACK);
    resolve_slot!(nvnCommandBufferEndTransformFeedback, SLOT_NVN_COMMAND_BUFFER_END_TRANSFORM_FEEDBACK);
    resolve_slot!(nvnCommandBufferPauseTransformFeedback, SLOT_NVN_COMMAND_BUFFER_PAUSE_TRANSFORM_FEEDBACK);
    resolve_slot!(nvnCommandBufferResumeTransformFeedback, SLOT_NVN_COMMAND_BUFFER_RESUME_TRANSFORM_FEEDBACK);
    resolve_slot!(nvnCommandBufferDrawTransformFeedback, SLOT_NVN_COMMAND_BUFFER_DRAW_TRANSFORM_FEEDBACK);
    resolve_slot!(nvnCommandBufferDrawArrays, SLOT_NVN_COMMAND_BUFFER_DRAW_ARRAYS);
    resolve_slot!(nvnCommandBufferDrawElements, SLOT_NVN_COMMAND_BUFFER_DRAW_ELEMENTS);
    resolve_slot!(nvnCommandBufferDrawElementsBaseVertex, SLOT_NVN_COMMAND_BUFFER_DRAW_ELEMENTS_BASE_VERTEX);
    resolve_slot!(nvnCommandBufferDrawArraysInstanced, SLOT_NVN_COMMAND_BUFFER_DRAW_ARRAYS_INSTANCED);
    resolve_slot!(nvnCommandBufferDrawElementsInstanced, SLOT_NVN_COMMAND_BUFFER_DRAW_ELEMENTS_INSTANCED);
    resolve_slot!(nvnCommandBufferDrawArraysIndirect, SLOT_NVN_COMMAND_BUFFER_DRAW_ARRAYS_INDIRECT);
    resolve_slot!(nvnCommandBufferDrawElementsIndirect, SLOT_NVN_COMMAND_BUFFER_DRAW_ELEMENTS_INDIRECT);
    resolve_slot!(nvnCommandBufferMultiDrawArraysIndirectCount, SLOT_NVN_COMMAND_BUFFER_MULTI_DRAW_ARRAYS_INDIRECT_COUNT);
    resolve_slot!(nvnCommandBufferMultiDrawElementsIndirectCount, SLOT_NVN_COMMAND_BUFFER_MULTI_DRAW_ELEMENTS_INDIRECT_COUNT);
    resolve_slot!(nvnCommandBufferClearColor, SLOT_NVN_COMMAND_BUFFER_CLEAR_COLOR);
    resolve_slot!(nvnCommandBufferClearColori, SLOT_NVN_COMMAND_BUFFER_CLEAR_COLORI);
    resolve_slot!(nvnCommandBufferClearColorui, SLOT_NVN_COMMAND_BUFFER_CLEAR_COLORUI);
    resolve_slot!(nvnCommandBufferClearDepthStencil, SLOT_NVN_COMMAND_BUFFER_CLEAR_DEPTH_STENCIL);
    resolve_slot!(nvnCommandBufferDispatchCompute, SLOT_NVN_COMMAND_BUFFER_DISPATCH_COMPUTE);
    resolve_slot!(nvnCommandBufferDispatchComputeIndirect, SLOT_NVN_COMMAND_BUFFER_DISPATCH_COMPUTE_INDIRECT);
    resolve_slot!(nvnCommandBufferSetViewport, SLOT_NVN_COMMAND_BUFFER_SET_VIEWPORT);
    resolve_slot!(nvnCommandBufferSetViewports, SLOT_NVN_COMMAND_BUFFER_SET_VIEWPORTS);
    resolve_slot!(nvnCommandBufferSetViewportSwizzles, SLOT_NVN_COMMAND_BUFFER_SET_VIEWPORT_SWIZZLES);
    resolve_slot!(nvnCommandBufferSetScissor, SLOT_NVN_COMMAND_BUFFER_SET_SCISSOR);
    resolve_slot!(nvnCommandBufferSetScissors, SLOT_NVN_COMMAND_BUFFER_SET_SCISSORS);
    resolve_slot!(nvnCommandBufferSetDepthRange, SLOT_NVN_COMMAND_BUFFER_SET_DEPTH_RANGE);
    resolve_slot!(nvnCommandBufferSetDepthBounds, SLOT_NVN_COMMAND_BUFFER_SET_DEPTH_BOUNDS);
    resolve_slot!(nvnCommandBufferSetDepthRanges, SLOT_NVN_COMMAND_BUFFER_SET_DEPTH_RANGES);
    resolve_slot!(nvnCommandBufferSetTiledCacheAction, SLOT_NVN_COMMAND_BUFFER_SET_TILED_CACHE_ACTION);
    resolve_slot!(nvnCommandBufferSetTiledCacheTileSize, SLOT_NVN_COMMAND_BUFFER_SET_TILED_CACHE_TILE_SIZE);
    resolve_slot!(nvnCommandBufferBindSeparateTexture, SLOT_NVN_COMMAND_BUFFER_BIND_SEPARATE_TEXTURE);
    resolve_slot!(nvnCommandBufferBindSeparateSampler, SLOT_NVN_COMMAND_BUFFER_BIND_SEPARATE_SAMPLER);
    resolve_slot!(nvnCommandBufferBindSeparateTextures, SLOT_NVN_COMMAND_BUFFER_BIND_SEPARATE_TEXTURES);
    resolve_slot!(nvnCommandBufferBindSeparateSamplers, SLOT_NVN_COMMAND_BUFFER_BIND_SEPARATE_SAMPLERS);
    resolve_slot!(nvnCommandBufferSetStencilValueMask, SLOT_NVN_COMMAND_BUFFER_SET_STENCIL_VALUE_MASK);
    resolve_slot!(nvnCommandBufferSetStencilMask, SLOT_NVN_COMMAND_BUFFER_SET_STENCIL_MASK);
    resolve_slot!(nvnCommandBufferSetStencilRef, SLOT_NVN_COMMAND_BUFFER_SET_STENCIL_REF);
    resolve_slot!(nvnCommandBufferSetBlendColor, SLOT_NVN_COMMAND_BUFFER_SET_BLEND_COLOR);
    resolve_slot!(nvnCommandBufferSetPointSize, SLOT_NVN_COMMAND_BUFFER_SET_POINT_SIZE);
    resolve_slot!(nvnCommandBufferSetLineWidth, SLOT_NVN_COMMAND_BUFFER_SET_LINE_WIDTH);
    resolve_slot!(nvnCommandBufferSetPolygonOffsetClamp, SLOT_NVN_COMMAND_BUFFER_SET_POLYGON_OFFSET_CLAMP);
    resolve_slot!(nvnCommandBufferSetAlphaRef, SLOT_NVN_COMMAND_BUFFER_SET_ALPHA_REF);
    resolve_slot!(nvnCommandBufferSetSampleMask, SLOT_NVN_COMMAND_BUFFER_SET_SAMPLE_MASK);
    resolve_slot!(nvnCommandBufferSetRasterizerDiscard, SLOT_NVN_COMMAND_BUFFER_SET_RASTERIZER_DISCARD);
    resolve_slot!(nvnCommandBufferSetDepthClamp, SLOT_NVN_COMMAND_BUFFER_SET_DEPTH_CLAMP);
    resolve_slot!(nvnCommandBufferSetConservativeRasterEnable, SLOT_NVN_COMMAND_BUFFER_SET_CONSERVATIVE_RASTER_ENABLE);
    resolve_slot!(nvnCommandBufferSetConservativeRasterDilate, SLOT_NVN_COMMAND_BUFFER_SET_CONSERVATIVE_RASTER_DILATE);
    resolve_slot!(nvnCommandBufferSetSubpixelPrecisionBias, SLOT_NVN_COMMAND_BUFFER_SET_SUBPIXEL_PRECISION_BIAS);
    resolve_slot!(nvnCommandBufferCopyBufferToTexture, SLOT_NVN_COMMAND_BUFFER_COPY_BUFFER_TO_TEXTURE);
    resolve_slot!(nvnCommandBufferCopyTextureToBuffer, SLOT_NVN_COMMAND_BUFFER_COPY_TEXTURE_TO_BUFFER);
    resolve_slot!(nvnCommandBufferCopyTextureToTexture, SLOT_NVN_COMMAND_BUFFER_COPY_TEXTURE_TO_TEXTURE);
    resolve_slot!(nvnCommandBufferCopyBufferToBuffer, SLOT_NVN_COMMAND_BUFFER_COPY_BUFFER_TO_BUFFER);
    resolve_slot!(nvnCommandBufferClearBuffer, SLOT_NVN_COMMAND_BUFFER_CLEAR_BUFFER);
    resolve_slot!(nvnCommandBufferClearTexture, SLOT_NVN_COMMAND_BUFFER_CLEAR_TEXTURE);
    resolve_slot!(nvnCommandBufferClearTexturei, SLOT_NVN_COMMAND_BUFFER_CLEAR_TEXTUREI);
    resolve_slot!(nvnCommandBufferClearTextureui, SLOT_NVN_COMMAND_BUFFER_CLEAR_TEXTUREUI);
    resolve_slot!(nvnCommandBufferUpdateUniformBuffer, SLOT_NVN_COMMAND_BUFFER_UPDATE_UNIFORM_BUFFER);
    resolve_slot!(nvnCommandBufferReportCounter, SLOT_NVN_COMMAND_BUFFER_REPORT_COUNTER);
    resolve_slot!(nvnCommandBufferResetCounter, SLOT_NVN_COMMAND_BUFFER_RESET_COUNTER);
    resolve_slot!(nvnCommandBufferReportValue, SLOT_NVN_COMMAND_BUFFER_REPORT_VALUE);
    resolve_slot!(nvnCommandBufferSetRenderEnable, SLOT_NVN_COMMAND_BUFFER_SET_RENDER_ENABLE);
    resolve_slot!(nvnCommandBufferSetRenderEnableConditional, SLOT_NVN_COMMAND_BUFFER_SET_RENDER_ENABLE_CONDITIONAL);
    resolve_slot!(nvnCommandBufferSetRenderTargets, SLOT_NVN_COMMAND_BUFFER_SET_RENDER_TARGETS);
    resolve_slot!(nvnCommandBufferDiscardColor, SLOT_NVN_COMMAND_BUFFER_DISCARD_COLOR);
    resolve_slot!(nvnCommandBufferDiscardDepthStencil, SLOT_NVN_COMMAND_BUFFER_DISCARD_DEPTH_STENCIL);
    resolve_slot!(nvnCommandBufferDownsample, SLOT_NVN_COMMAND_BUFFER_DOWNSAMPLE);
    resolve_slot!(nvnCommandBufferTiledDownsample, SLOT_NVN_COMMAND_BUFFER_TILED_DOWNSAMPLE);
    resolve_slot!(nvnCommandBufferDownsampleTextureView, SLOT_NVN_COMMAND_BUFFER_DOWNSAMPLE_TEXTURE_VIEW);
    resolve_slot!(nvnCommandBufferTiledDownsampleTextureView, SLOT_NVN_COMMAND_BUFFER_TILED_DOWNSAMPLE_TEXTURE_VIEW);
    resolve_slot!(nvnCommandBufferBarrier, SLOT_NVN_COMMAND_BUFFER_BARRIER);
    resolve_slot!(nvnCommandBufferWaitSync, SLOT_NVN_COMMAND_BUFFER_WAIT_SYNC);
    resolve_slot!(nvnCommandBufferFenceSync, SLOT_NVN_COMMAND_BUFFER_FENCE_SYNC);
    resolve_slot!(nvnCommandBufferSetTexturePool, SLOT_NVN_COMMAND_BUFFER_SET_TEXTURE_POOL);
    resolve_slot!(nvnCommandBufferSetSamplerPool, SLOT_NVN_COMMAND_BUFFER_SET_SAMPLER_POOL);
    resolve_slot!(nvnCommandBufferSetShaderScratchMemory, SLOT_NVN_COMMAND_BUFFER_SET_SHADER_SCRATCH_MEMORY);
    resolve_slot!(nvnCommandBufferSaveZCullData, SLOT_NVN_COMMAND_BUFFER_SAVE_Z_CULL_DATA);
    resolve_slot!(nvnCommandBufferRestoreZCullData, SLOT_NVN_COMMAND_BUFFER_RESTORE_Z_CULL_DATA);
    resolve_slot!(nvnCommandBufferSetCopyRowStride, SLOT_NVN_COMMAND_BUFFER_SET_COPY_ROW_STRIDE);
    resolve_slot!(nvnCommandBufferSetCopyImageStride, SLOT_NVN_COMMAND_BUFFER_SET_COPY_IMAGE_STRIDE);
    resolve_slot!(nvnCommandBufferGetCopyRowStride, SLOT_NVN_COMMAND_BUFFER_GET_COPY_ROW_STRIDE);
    resolve_slot!(nvnCommandBufferGetCopyImageStride, SLOT_NVN_COMMAND_BUFFER_GET_COPY_IMAGE_STRIDE);
    resolve_slot!(nvnCommandBufferDrawTexture, SLOT_NVN_COMMAND_BUFFER_DRAW_TEXTURE);
    resolve_slot!(nvnCommandBufferSetProgramSubroutines, SLOT_NVN_COMMAND_BUFFER_SET_PROGRAM_SUBROUTINES);
    resolve_slot!(nvnCommandBufferBindCoverageModulationTable, SLOT_NVN_COMMAND_BUFFER_BIND_COVERAGE_MODULATION_TABLE);
    resolve_slot!(nvnCommandBufferResolveDepthBuffer, SLOT_NVN_COMMAND_BUFFER_RESOLVE_DEPTH_BUFFER);
    resolve_slot!(nvnCommandBufferSetColorReductionEnable, SLOT_NVN_COMMAND_BUFFER_SET_COLOR_REDUCTION_ENABLE);
    resolve_slot!(nvnCommandBufferSetColorReductionThresholds, SLOT_NVN_COMMAND_BUFFER_SET_COLOR_REDUCTION_THRESHOLDS);
    resolve_slot!(nvnCommandBufferPushDebugGroupStatic, SLOT_NVN_COMMAND_BUFFER_PUSH_DEBUG_GROUP_STATIC);
    resolve_slot!(nvnCommandBufferPushDebugGroupDynamic, SLOT_NVN_COMMAND_BUFFER_PUSH_DEBUG_GROUP_DYNAMIC);
    resolve_slot!(nvnCommandBufferPushDebugGroup, SLOT_NVN_COMMAND_BUFFER_PUSH_DEBUG_GROUP);
    resolve_slot!(nvnCommandBufferPopDebugGroup, SLOT_NVN_COMMAND_BUFFER_POP_DEBUG_GROUP);
    resolve_slot!(nvnCommandBufferPopDebugGroupId, SLOT_NVN_COMMAND_BUFFER_POP_DEBUG_GROUP_ID);
    resolve_slot!(nvnCommandBufferInsertDebugMarkerStatic, SLOT_NVN_COMMAND_BUFFER_INSERT_DEBUG_MARKER_STATIC);
    resolve_slot!(nvnCommandBufferInsertDebugMarkerDynamic, SLOT_NVN_COMMAND_BUFFER_INSERT_DEBUG_MARKER_DYNAMIC);
    resolve_slot!(nvnCommandBufferInsertDebugMarker, SLOT_NVN_COMMAND_BUFFER_INSERT_DEBUG_MARKER);
    resolve_slot!(nvnCommandBufferGetMemoryCallback, SLOT_NVN_COMMAND_BUFFER_GET_MEMORY_CALLBACK);
    resolve_slot!(nvnCommandBufferGetMemoryCallbackData, SLOT_NVN_COMMAND_BUFFER_GET_MEMORY_CALLBACK_DATA);
    resolve_slot!(nvnCommandBufferIsRecording, SLOT_NVN_COMMAND_BUFFER_IS_RECORDING);
    resolve_slot!(nvnCommandBufferWaitEvent, SLOT_NVN_COMMAND_BUFFER_WAIT_EVENT);
    resolve_slot!(nvnCommandBufferSignalEvent, SLOT_NVN_COMMAND_BUFFER_SIGNAL_EVENT);
    resolve_slot!(nvnCommandBufferSetStencilCullCriteria, SLOT_NVN_COMMAND_BUFFER_SET_STENCIL_CULL_CRITERIA);
    resolve_slot!(nvnSyncInitialize, SLOT_NVN_SYNC_INITIALIZE);
    resolve_slot!(nvnSyncFinalize, SLOT_NVN_SYNC_FINALIZE);
    resolve_slot!(nvnSyncSetDebugLabel, SLOT_NVN_SYNC_SET_DEBUG_LABEL);
    resolve_slot!(nvnSyncWait, SLOT_NVN_SYNC_WAIT);
    resolve_slot!(nvnSyncInitializeFromFencedGLSync, SLOT_NVN_SYNC_INITIALIZE_FROM_FENCED_GL_SYNC);
    resolve_slot!(nvnSyncCreateGLSync, SLOT_NVN_SYNC_CREATE_GL_SYNC);
    resolve_slot!(nvnEventBuilderSetDefaults, SLOT_NVN_EVENT_BUILDER_SET_DEFAULTS);
    resolve_slot!(nvnEventBuilderSetStorage, SLOT_NVN_EVENT_BUILDER_SET_STORAGE);
    resolve_slot!(nvnEventBuilderGetStorage, SLOT_NVN_EVENT_BUILDER_GET_STORAGE);
    resolve_slot!(nvnEventBuilderGetMemoryPool, SLOT_NVN_EVENT_BUILDER_GET_MEMORY_POOL);
    resolve_slot!(nvnEventBuilderGetMemoryOffset, SLOT_NVN_EVENT_BUILDER_GET_MEMORY_OFFSET);
    resolve_slot!(nvnEventInitialize, SLOT_NVN_EVENT_INITIALIZE);
    resolve_slot!(nvnEventFinalize, SLOT_NVN_EVENT_FINALIZE);
    resolve_slot!(nvnEventGetValue, SLOT_NVN_EVENT_GET_VALUE);
    resolve_slot!(nvnEventSignal, SLOT_NVN_EVENT_SIGNAL);
    resolve_slot!(nvnEventGetMemoryPool, SLOT_NVN_EVENT_GET_MEMORY_POOL);
    resolve_slot!(nvnEventGetMemoryOffset, SLOT_NVN_EVENT_GET_MEMORY_OFFSET);
    resolve_slot!(nvnQueueFenceSync, SLOT_NVN_QUEUE_FENCE_SYNC);
    resolve_slot!(nvnQueueWaitSync, SLOT_NVN_QUEUE_WAIT_SYNC);
}

/// One-time fast-path initialization using `consts.rs` slot addresses.
#[inline(always)]
pub unsafe fn initialize_from_slots() {
    if INITIALIZED.swap(true, Ordering::AcqRel) {
        return;
    }
    init_from_slots();
}

/// Fast-path entrypoint: resolve function pointers once and skip repeated setup.
#[inline(always)]
pub unsafe fn initialize(resolver: u64, get_proc: ProcResolverFn) {
    if INITIALIZED.swap(true, Ordering::AcqRel) {
        return;
    }
    init_from_resolver(resolver, get_proc);
}

#[inline(always)]
pub fn is_initialized() -> bool {
    INITIALIZED.load(Ordering::Acquire)
}
