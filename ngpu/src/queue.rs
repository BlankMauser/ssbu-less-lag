#![allow(unused)]
use crate::*;

nvn_func! {
    // Queue error/memory queries
    pub static nvnQueueGetError: fn(*mut NvnQueue, *mut NvnQueueErrorInfo) -> NvnQueueGetErrorResult;
    pub static nvnQueueGetTotalCommandMemoryUsed: fn(*mut NvnQueue) -> usize;
    pub static nvnQueueGetTotalControlMemoryUsed: fn(*mut NvnQueue) -> usize;
    pub static nvnQueueGetTotalComputeMemoryUsed: fn(*mut NvnQueue) -> usize;
    pub static nvnQueueResetMemoryUsageCounts: fn(*mut NvnQueue);

    // QueueBuilder
    pub static nvnQueueBuilderSetDevice: fn(*mut NvnQueueBuilder, *mut NvnDevice);
    pub static nvnQueueBuilderSetDefaults: fn(*mut NvnQueueBuilder);
    pub static nvnQueueBuilderSetFlags: fn(*mut NvnQueueBuilder, i32);
    pub static nvnQueueBuilderSetCommandMemorySize: fn(*mut NvnQueueBuilder, usize);
    pub static nvnQueueBuilderSetComputeMemorySize: fn(*mut NvnQueueBuilder, usize);
    pub static nvnQueueBuilderSetControlMemorySize: fn(*mut NvnQueueBuilder, usize);
    pub static nvnQueueBuilderGetQueueMemorySize: fn(*const NvnQueueBuilder) -> usize;
    pub static nvnQueueBuilderSetQueueMemory: fn(*mut NvnQueueBuilder, *mut core::ffi::c_void, usize);
    pub static nvnQueueBuilderSetCommandFlushThreshold: fn(*mut NvnQueueBuilder, usize);
    // vtable-only
    pub static nvnQueueBuilderSetQueuePriority: fn(*mut NvnQueueBuilder, i32);
    pub static nvnQueueBuilderGetQueuePriority: fn(*const NvnQueueBuilder) -> i32;
    pub static nvnQueueBuilderGetDevice: fn(*const NvnQueueBuilder) -> *const NvnDevice;
    pub static nvnQueueBuilderGetFlags: fn(*const NvnQueueBuilder) -> i32;
    pub static nvnQueueBuilderGetCommandMemorySize: fn(*const NvnQueueBuilder) -> usize;
    pub static nvnQueueBuilderGetComputeMemorySize: fn(*const NvnQueueBuilder) -> usize;
    pub static nvnQueueBuilderGetControlMemorySize: fn(*const NvnQueueBuilder) -> usize;
    pub static nvnQueueBuilderGetCommandFlushThreshold: fn(*const NvnQueueBuilder) -> usize;
    pub static nvnQueueBuilderGetMemorySize: fn(*const NvnQueueBuilder) -> usize;
    pub static nvnQueueBuilderGetMemory: fn(*const NvnQueueBuilder) -> *mut core::ffi::c_void;

    // Queue
    pub static nvnQueueInitialize: fn(*mut NvnQueue, *const NvnQueueBuilder) -> NvnBoolean;
    pub static nvnQueueFinalize: fn(*mut NvnQueue);
    pub static nvnQueueSetDebugLabel: fn(*mut NvnQueue, *const u8);
    pub static nvnQueueSubmitCommands: fn(*mut NvnQueue, i32, *const NvnCommandHandle);
    pub static nvnQueueFlush: fn(*mut NvnQueue);
    pub static nvnQueueFinish: fn(*mut NvnQueue);
    pub static nvnQueuePresentTexture: fn(*mut NvnQueue, *mut NvnWindow, i32);
    pub static nvnQueueAcquireTexture: fn(*mut NvnQueue, *mut NvnWindow, *mut i32) -> NvnQueueAcquireTextureResult;

    // Queue sync (resolved in lib.rs init alongside sync module)
    pub static nvnQueueFenceSync: fn(*mut NvnQueue, *mut NvnSync, NvnSyncCondition, i32);
    pub static nvnQueueWaitSync: fn(*mut NvnQueue, *const NvnSync) -> NvnBoolean;
}
