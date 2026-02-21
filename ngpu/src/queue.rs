#![allow(unused)]
use crate::*;

gpu_api! {
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
    pub static nvnQueueFenceSync: fn(*mut NvnQueue, *mut NvnSync, NvnSyncCondition, i32);
    pub static nvnQueueWaitSync: fn(*mut NvnQueue, *const NvnSync) -> NvnBoolean;
}


crate::nvn_wrap_ret!(queue_get_error(arg0: *mut NvnQueue, arg1: *mut NvnQueueErrorInfo) -> NvnQueueGetErrorResult => SLOT_NVN_QUEUE_GET_ERROR);

crate::nvn_wrap_ret!(queue_get_total_command_memory_used(arg0: *mut NvnQueue) -> usize => SLOT_NVN_QUEUE_GET_TOTAL_COMMAND_MEMORY_USED);

crate::nvn_wrap_ret!(queue_get_total_control_memory_used(arg0: *mut NvnQueue) -> usize => SLOT_NVN_QUEUE_GET_TOTAL_CONTROL_MEMORY_USED);

crate::nvn_wrap_ret!(queue_get_total_compute_memory_used(arg0: *mut NvnQueue) -> usize => SLOT_NVN_QUEUE_GET_TOTAL_COMPUTE_MEMORY_USED);

crate::nvn_wrap_void!(queue_reset_memory_usage_counts(arg0: *mut NvnQueue) => SLOT_NVN_QUEUE_RESET_MEMORY_USAGE_COUNTS);

crate::nvn_wrap_void!(queue_builder_set_device(arg0: *mut NvnQueueBuilder, arg1: *mut NvnDevice) => SLOT_NVN_QUEUE_BUILDER_SET_DEVICE);

crate::nvn_wrap_void!(queue_builder_set_defaults(arg0: *mut NvnQueueBuilder) => SLOT_NVN_QUEUE_BUILDER_SET_DEFAULTS);

crate::nvn_wrap_void!(queue_builder_set_flags(arg0: *mut NvnQueueBuilder, arg1: i32) => SLOT_NVN_QUEUE_BUILDER_SET_FLAGS);

crate::nvn_wrap_void!(queue_builder_set_command_memory_size(arg0: *mut NvnQueueBuilder, arg1: usize) => SLOT_NVN_QUEUE_BUILDER_SET_COMMAND_MEMORY_SIZE);

crate::nvn_wrap_void!(queue_builder_set_compute_memory_size(arg0: *mut NvnQueueBuilder, arg1: usize) => SLOT_NVN_QUEUE_BUILDER_SET_COMPUTE_MEMORY_SIZE);

crate::nvn_wrap_void!(queue_builder_set_control_memory_size(arg0: *mut NvnQueueBuilder, arg1: usize) => SLOT_NVN_QUEUE_BUILDER_SET_CONTROL_MEMORY_SIZE);

crate::nvn_wrap_ret!(queue_builder_get_queue_memory_size(arg0: *const NvnQueueBuilder) -> usize => SLOT_NVN_QUEUE_BUILDER_GET_QUEUE_MEMORY_SIZE);

crate::nvn_wrap_void!(queue_builder_set_queue_memory(arg0: *mut NvnQueueBuilder, arg1: *mut core::ffi::c_void, arg2: usize) => SLOT_NVN_QUEUE_BUILDER_SET_QUEUE_MEMORY);

crate::nvn_wrap_void!(queue_builder_set_command_flush_threshold(arg0: *mut NvnQueueBuilder, arg1: usize) => SLOT_NVN_QUEUE_BUILDER_SET_COMMAND_FLUSH_THRESHOLD);

crate::nvn_wrap_void!(queue_builder_set_queue_priority(arg0: *mut NvnQueueBuilder, arg1: i32) => SLOT_NVN_QUEUE_BUILDER_SET_QUEUE_PRIORITY);

crate::nvn_wrap_ret!(queue_builder_get_queue_priority(arg0: *const NvnQueueBuilder) -> i32 => SLOT_NVN_QUEUE_BUILDER_GET_QUEUE_PRIORITY);

crate::nvn_wrap_ret!(queue_builder_get_device(arg0: *const NvnQueueBuilder) -> *const NvnDevice => SLOT_NVN_QUEUE_BUILDER_GET_DEVICE);

crate::nvn_wrap_ret!(queue_builder_get_flags(arg0: *const NvnQueueBuilder) -> i32 => SLOT_NVN_QUEUE_BUILDER_GET_FLAGS);

crate::nvn_wrap_ret!(queue_builder_get_command_memory_size(arg0: *const NvnQueueBuilder) -> usize => SLOT_NVN_QUEUE_BUILDER_GET_COMMAND_MEMORY_SIZE);

crate::nvn_wrap_ret!(queue_builder_get_compute_memory_size(arg0: *const NvnQueueBuilder) -> usize => SLOT_NVN_QUEUE_BUILDER_GET_COMPUTE_MEMORY_SIZE);

crate::nvn_wrap_ret!(queue_builder_get_control_memory_size(arg0: *const NvnQueueBuilder) -> usize => SLOT_NVN_QUEUE_BUILDER_GET_CONTROL_MEMORY_SIZE);

crate::nvn_wrap_ret!(queue_builder_get_command_flush_threshold(arg0: *const NvnQueueBuilder) -> usize => SLOT_NVN_QUEUE_BUILDER_GET_COMMAND_FLUSH_THRESHOLD);

crate::nvn_wrap_ret!(queue_builder_get_memory_size(arg0: *const NvnQueueBuilder) -> usize => SLOT_NVN_QUEUE_BUILDER_GET_MEMORY_SIZE);

crate::nvn_wrap_ret!(queue_builder_get_memory(arg0: *const NvnQueueBuilder) -> *mut core::ffi::c_void => SLOT_NVN_QUEUE_BUILDER_GET_MEMORY);

crate::nvn_wrap_ret!(queue_initialize(arg0: *mut NvnQueue, arg1: *const NvnQueueBuilder) -> NvnBoolean => SLOT_NVN_QUEUE_INITIALIZE);

crate::nvn_wrap_void!(queue_finalize(arg0: *mut NvnQueue) => SLOT_NVN_QUEUE_FINALIZE);

crate::nvn_wrap_void!(queue_set_debug_label(arg0: *mut NvnQueue, arg1: *const u8) => SLOT_NVN_QUEUE_SET_DEBUG_LABEL);

crate::nvn_wrap_void!(queue_submit_commands(arg0: *mut NvnQueue, arg1: i32, arg2: *const NvnCommandHandle) => SLOT_NVN_QUEUE_SUBMIT_COMMANDS);

crate::nvn_wrap_void!(queue_flush(arg0: *mut NvnQueue) => SLOT_NVN_QUEUE_FLUSH);

crate::nvn_wrap_void!(queue_finish(arg0: *mut NvnQueue) => SLOT_NVN_QUEUE_FINISH);

crate::nvn_wrap_void!(queue_present_texture(arg0: *mut NvnQueue, arg1: *mut NvnWindow, arg2: i32) => SLOT_NVN_QUEUE_PRESENT_TEXTURE);

crate::nvn_wrap_ret!(queue_acquire_texture(queue: *mut NvnQueue, window: *mut NvnWindow, texture_index: *mut i32) -> NvnQueueAcquireTextureResult => SLOT_NVN_QUEUE_ACQUIRE_TEXTURE);

crate::nvn_wrap_void!(queue_fence_sync(arg0: *mut NvnQueue, arg1: *mut NvnSync, arg2: NvnSyncCondition, arg3: i32) => SLOT_NVN_QUEUE_FENCE_SYNC);

crate::nvn_wrap_ret!(queue_wait_sync(arg0: *mut NvnQueue, arg1: *const NvnSync) -> NvnBoolean => SLOT_NVN_QUEUE_WAIT_SYNC);

#[inline(always)]
pub unsafe fn acquire_texture(
    queue: *mut NvnQueue,
    window: *mut NvnWindow,
    texture_index: *mut i32,
) -> NvnQueueAcquireTextureResult {
    queue_acquire_texture(queue, window, texture_index)
}

#[inline(always)]
pub unsafe fn nvn_queue_acquire_texture(
    queue: *mut NvnQueue,
    window: *mut NvnWindow,
    texture_index: *mut i32,
) -> NvnQueueAcquireTextureResult {
    queue_acquire_texture(queue, window, texture_index)
}
