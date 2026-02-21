#![allow(unused)]
use crate::*;

gpu_api! {
    // ── Init / Finalize ──
    pub static nvnCommandBufferInitialize: fn(*mut NvnCommandBuffer, *mut NvnDevice) -> NvnBoolean;
    pub static nvnCommandBufferFinalize: fn(*mut NvnCommandBuffer);
    pub static nvnCommandBufferSetDebugLabel: fn(*mut NvnCommandBuffer, *const u8);

    // ── Memory management ──
    pub static nvnCommandBufferSetMemoryCallback: fn(*mut NvnCommandBuffer, PfnNvnCommandBufferMemoryCallback);
    pub static nvnCommandBufferSetMemoryCallbackData: fn(*mut NvnCommandBuffer, *mut core::ffi::c_void);
    // vtable-only
    pub static nvnCommandBufferSetCommandMemoryCallbackEnabled: fn(*mut NvnCommandBuffer, NvnBoolean);
    pub static nvnCommandBufferAddCommandMemory: fn(*mut NvnCommandBuffer, *const NvnMemoryPool, isize, usize);
    pub static nvnCommandBufferAddControlMemory: fn(*mut NvnCommandBuffer, *mut core::ffi::c_void, usize);
    pub static nvnCommandBufferGetCommandMemorySize: fn(*const NvnCommandBuffer) -> usize;
    pub static nvnCommandBufferGetCommandMemoryUsed: fn(*const NvnCommandBuffer) -> usize;
    pub static nvnCommandBufferGetCommandMemoryFree: fn(*const NvnCommandBuffer) -> usize;
    pub static nvnCommandBufferGetControlMemorySize: fn(*const NvnCommandBuffer) -> usize;
    pub static nvnCommandBufferGetControlMemoryUsed: fn(*const NvnCommandBuffer) -> usize;
    pub static nvnCommandBufferGetControlMemoryFree: fn(*const NvnCommandBuffer) -> usize;

    // ── Recording ──
    pub static nvnCommandBufferBeginRecording: fn(*mut NvnCommandBuffer);
    pub static nvnCommandBufferEndRecording: fn(*mut NvnCommandBuffer) -> NvnCommandHandle;
    pub static nvnCommandBufferCallCommands: fn(*mut NvnCommandBuffer, i32, *const NvnCommandHandle);
    pub static nvnCommandBufferCopyCommands: fn(*mut NvnCommandBuffer, i32, *const NvnCommandHandle);

    // ── State binding ──
    pub static nvnCommandBufferBindBlendState: fn(*mut NvnCommandBuffer, *const NvnBlendState);
    pub static nvnCommandBufferBindChannelMaskState: fn(*mut NvnCommandBuffer, *const NvnChannelMaskState);
    pub static nvnCommandBufferBindColorState: fn(*mut NvnCommandBuffer, *const NvnColorState);
    pub static nvnCommandBufferBindMultisampleState: fn(*mut NvnCommandBuffer, *const NvnMultisampleState);
    pub static nvnCommandBufferBindPolygonState: fn(*mut NvnCommandBuffer, *const NvnPolygonState);
    pub static nvnCommandBufferBindDepthStencilState: fn(*mut NvnCommandBuffer, *const NvnDepthStencilState);
    pub static nvnCommandBufferBindVertexAttribState: fn(*mut NvnCommandBuffer, i32, *const NvnVertexAttribState);
    pub static nvnCommandBufferBindVertexStreamState: fn(*mut NvnCommandBuffer, i32, *const NvnVertexStreamState);
    pub static nvnCommandBufferBindProgram: fn(*mut NvnCommandBuffer, *const NvnProgram, i32);

    // ── Buffer binding ──
    pub static nvnCommandBufferBindVertexBuffer: fn(*mut NvnCommandBuffer, i32, NvnBufferAddress, usize);
    pub static nvnCommandBufferBindVertexBuffers: fn(*mut NvnCommandBuffer, i32, i32, *const NvnBufferRange);
    pub static nvnCommandBufferBindUniformBuffer: fn(*mut NvnCommandBuffer, NvnShaderStage, i32, NvnBufferAddress, usize);
    pub static nvnCommandBufferBindUniformBuffers: fn(*mut NvnCommandBuffer, NvnShaderStage, i32, i32, *const NvnBufferRange);
    pub static nvnCommandBufferBindTransformFeedbackBuffer: fn(*mut NvnCommandBuffer, i32, NvnBufferAddress, usize);
    pub static nvnCommandBufferBindTransformFeedbackBuffers: fn(*mut NvnCommandBuffer, i32, i32, *const NvnBufferRange);
    pub static nvnCommandBufferBindStorageBuffer: fn(*mut NvnCommandBuffer, NvnShaderStage, i32, NvnBufferAddress, usize);
    pub static nvnCommandBufferBindStorageBuffers: fn(*mut NvnCommandBuffer, NvnShaderStage, i32, i32, *const NvnBufferRange);

    // ── Texture/image binding ──
    pub static nvnCommandBufferBindTexture: fn(*mut NvnCommandBuffer, NvnShaderStage, i32, NvnTextureHandle);
    pub static nvnCommandBufferBindTextures: fn(*mut NvnCommandBuffer, NvnShaderStage, i32, i32, *const NvnTextureHandle);
    pub static nvnCommandBufferBindImage: fn(*mut NvnCommandBuffer, NvnShaderStage, i32, NvnImageHandle);
    pub static nvnCommandBufferBindImages: fn(*mut NvnCommandBuffer, NvnShaderStage, i32, i32, *const NvnImageHandle);

    // ── Tessellation ──
    pub static nvnCommandBufferSetPatchSize: fn(*mut NvnCommandBuffer, i32);
    pub static nvnCommandBufferSetInnerTessellationLevels: fn(*mut NvnCommandBuffer, *const f32);
    pub static nvnCommandBufferSetOuterTessellationLevels: fn(*mut NvnCommandBuffer, *const f32);

    // ── Primitive restart ──
    pub static nvnCommandBufferSetPrimitiveRestart: fn(*mut NvnCommandBuffer, NvnBoolean, i32);

    // ── Transform feedback ──
    pub static nvnCommandBufferBeginTransformFeedback: fn(*mut NvnCommandBuffer, NvnBufferAddress);
    pub static nvnCommandBufferEndTransformFeedback: fn(*mut NvnCommandBuffer, NvnBufferAddress);
    pub static nvnCommandBufferPauseTransformFeedback: fn(*mut NvnCommandBuffer, NvnBufferAddress);
    pub static nvnCommandBufferResumeTransformFeedback: fn(*mut NvnCommandBuffer, NvnBufferAddress);
    pub static nvnCommandBufferDrawTransformFeedback: fn(*mut NvnCommandBuffer, NvnDrawPrimitive, NvnBufferAddress);

    // ── Drawing ──
    pub static nvnCommandBufferDrawArrays: fn(*mut NvnCommandBuffer, NvnDrawPrimitive, i32, i32);
    pub static nvnCommandBufferDrawElements: fn(*mut NvnCommandBuffer, NvnDrawPrimitive, NvnIndexType, i32, NvnBufferAddress);
    pub static nvnCommandBufferDrawElementsBaseVertex: fn(*mut NvnCommandBuffer, NvnDrawPrimitive, NvnIndexType, i32, NvnBufferAddress, i32);
    pub static nvnCommandBufferDrawArraysInstanced: fn(*mut NvnCommandBuffer, NvnDrawPrimitive, i32, i32, i32, i32);
    pub static nvnCommandBufferDrawElementsInstanced: fn(*mut NvnCommandBuffer, NvnDrawPrimitive, NvnIndexType, i32, NvnBufferAddress, i32, i32, i32);
    pub static nvnCommandBufferDrawArraysIndirect: fn(*mut NvnCommandBuffer, NvnDrawPrimitive, NvnBufferAddress);
    pub static nvnCommandBufferDrawElementsIndirect: fn(*mut NvnCommandBuffer, NvnDrawPrimitive, NvnIndexType, NvnBufferAddress, NvnBufferAddress);
    pub static nvnCommandBufferMultiDrawArraysIndirectCount: fn(*mut NvnCommandBuffer, NvnDrawPrimitive, NvnBufferAddress, NvnBufferAddress, i32, isize);
    pub static nvnCommandBufferMultiDrawElementsIndirectCount: fn(*mut NvnCommandBuffer, NvnDrawPrimitive, NvnIndexType, NvnBufferAddress, NvnBufferAddress, NvnBufferAddress, i32, isize);

    // ── Clear ──
    pub static nvnCommandBufferClearColor: fn(*mut NvnCommandBuffer, i32, *const f32, i32);
    pub static nvnCommandBufferClearColori: fn(*mut NvnCommandBuffer, i32, *const i32, i32);
    pub static nvnCommandBufferClearColorui: fn(*mut NvnCommandBuffer, i32, *const u32, i32);
    pub static nvnCommandBufferClearDepthStencil: fn(*mut NvnCommandBuffer, f32, NvnBoolean, i32, i32);

    // ── Compute ──
    pub static nvnCommandBufferDispatchCompute: fn(*mut NvnCommandBuffer, i32, i32, i32);
    pub static nvnCommandBufferDispatchComputeIndirect: fn(*mut NvnCommandBuffer, NvnBufferAddress);

    // ── Viewport / Scissor ──
    pub static nvnCommandBufferSetViewport: fn(*mut NvnCommandBuffer, i32, i32, i32, i32);
    pub static nvnCommandBufferSetViewports: fn(*mut NvnCommandBuffer, i32, i32, *const f32);
    pub static nvnCommandBufferSetViewportSwizzles: fn(*mut NvnCommandBuffer, i32, i32, *const NvnViewportSwizzle);
    pub static nvnCommandBufferSetScissor: fn(*mut NvnCommandBuffer, i32, i32, i32, i32);
    pub static nvnCommandBufferSetScissors: fn(*mut NvnCommandBuffer, i32, i32, *const i32);

    // ── Depth ──
    pub static nvnCommandBufferSetDepthRange: fn(*mut NvnCommandBuffer, f32, f32);
    pub static nvnCommandBufferSetDepthBounds: fn(*mut NvnCommandBuffer, NvnBoolean, f32, f32);
    pub static nvnCommandBufferSetDepthRanges: fn(*mut NvnCommandBuffer, i32, i32, *const f32);

    // ── Tiled cache ──
    pub static nvnCommandBufferSetTiledCacheAction: fn(*mut NvnCommandBuffer, NvnTiledCacheAction);
    pub static nvnCommandBufferSetTiledCacheTileSize: fn(*mut NvnCommandBuffer, i32, i32);

    // ── Separate texture/sampler binding ──
    pub static nvnCommandBufferBindSeparateTexture: fn(*mut NvnCommandBuffer, NvnShaderStage, i32, NvnSeparateTextureHandle);
    pub static nvnCommandBufferBindSeparateSampler: fn(*mut NvnCommandBuffer, NvnShaderStage, i32, NvnSeparateSamplerHandle);
    pub static nvnCommandBufferBindSeparateTextures: fn(*mut NvnCommandBuffer, NvnShaderStage, i32, i32, *const NvnSeparateTextureHandle);
    pub static nvnCommandBufferBindSeparateSamplers: fn(*mut NvnCommandBuffer, NvnShaderStage, i32, i32, *const NvnSeparateSamplerHandle);

    // ── Stencil ──
    pub static nvnCommandBufferSetStencilValueMask: fn(*mut NvnCommandBuffer, NvnFace, i32);
    pub static nvnCommandBufferSetStencilMask: fn(*mut NvnCommandBuffer, NvnFace, i32);
    pub static nvnCommandBufferSetStencilRef: fn(*mut NvnCommandBuffer, NvnFace, i32);

    // ── Misc state ──
    pub static nvnCommandBufferSetBlendColor: fn(*mut NvnCommandBuffer, *const f32);
    pub static nvnCommandBufferSetPointSize: fn(*mut NvnCommandBuffer, f32);
    pub static nvnCommandBufferSetLineWidth: fn(*mut NvnCommandBuffer, f32);
    pub static nvnCommandBufferSetPolygonOffsetClamp: fn(*mut NvnCommandBuffer, f32, f32, f32);
    pub static nvnCommandBufferSetAlphaRef: fn(*mut NvnCommandBuffer, f32);
    pub static nvnCommandBufferSetSampleMask: fn(*mut NvnCommandBuffer, i32);
    pub static nvnCommandBufferSetRasterizerDiscard: fn(*mut NvnCommandBuffer, NvnBoolean);
    pub static nvnCommandBufferSetDepthClamp: fn(*mut NvnCommandBuffer, NvnBoolean);
    pub static nvnCommandBufferSetConservativeRasterEnable: fn(*mut NvnCommandBuffer, NvnBoolean);
    pub static nvnCommandBufferSetConservativeRasterDilate: fn(*mut NvnCommandBuffer, f32);
    pub static nvnCommandBufferSetSubpixelPrecisionBias: fn(*mut NvnCommandBuffer, i32, i32);

    // ── Copy operations ──
    pub static nvnCommandBufferCopyBufferToTexture: fn(*mut NvnCommandBuffer, NvnBufferAddress, *const NvnTexture, *const NvnTextureView, *const NvnCopyRegion, i32);
    pub static nvnCommandBufferCopyTextureToBuffer: fn(*mut NvnCommandBuffer, *const NvnTexture, *const NvnTextureView, *const NvnCopyRegion, NvnBufferAddress, i32);
    pub static nvnCommandBufferCopyTextureToTexture: fn(*mut NvnCommandBuffer, *const NvnTexture, *const NvnTextureView, *const NvnCopyRegion, *const NvnTexture, *const NvnTextureView, *const NvnCopyRegion, i32);
    pub static nvnCommandBufferCopyBufferToBuffer: fn(*mut NvnCommandBuffer, NvnBufferAddress, NvnBufferAddress, usize, i32);
    pub static nvnCommandBufferClearBuffer: fn(*mut NvnCommandBuffer, NvnBufferAddress, usize, u32);
    pub static nvnCommandBufferClearTexture: fn(*mut NvnCommandBuffer, *const NvnTexture, *const NvnTextureView, *const NvnCopyRegion, *const f32, i32);
    pub static nvnCommandBufferClearTexturei: fn(*mut NvnCommandBuffer, *const NvnTexture, *const NvnTextureView, *const NvnCopyRegion, *const i32, i32);
    pub static nvnCommandBufferClearTextureui: fn(*mut NvnCommandBuffer, *const NvnTexture, *const NvnTextureView, *const NvnCopyRegion, *const u32, i32);
    pub static nvnCommandBufferUpdateUniformBuffer: fn(*mut NvnCommandBuffer, NvnBufferAddress, usize, isize, usize, *const core::ffi::c_void);

    // ── Counters ──
    pub static nvnCommandBufferReportCounter: fn(*mut NvnCommandBuffer, NvnCounterType, NvnBufferAddress);
    pub static nvnCommandBufferResetCounter: fn(*mut NvnCommandBuffer, NvnCounterType);
    pub static nvnCommandBufferReportValue: fn(*mut NvnCommandBuffer, u32, NvnBufferAddress);

    // ── Render enable ──
    pub static nvnCommandBufferSetRenderEnable: fn(*mut NvnCommandBuffer, NvnBoolean);
    pub static nvnCommandBufferSetRenderEnableConditional: fn(*mut NvnCommandBuffer, NvnConditionalRenderMode, NvnBufferAddress);

    // ── Render targets ──
    pub static nvnCommandBufferSetRenderTargets: fn(*mut NvnCommandBuffer, i32, *const *const NvnTexture, *const *const NvnTextureView, *const NvnTexture, *const NvnTextureView);
    pub static nvnCommandBufferDiscardColor: fn(*mut NvnCommandBuffer, i32);
    pub static nvnCommandBufferDiscardDepthStencil: fn(*mut NvnCommandBuffer);

    // ── Downsample ──
    pub static nvnCommandBufferDownsample: fn(*mut NvnCommandBuffer, *const NvnTexture, *const NvnTexture);
    pub static nvnCommandBufferTiledDownsample: fn(*mut NvnCommandBuffer, *const NvnTexture, *const NvnTexture);
    pub static nvnCommandBufferDownsampleTextureView: fn(*mut NvnCommandBuffer, *const NvnTexture, *const NvnTextureView, *const NvnTexture, *const NvnTextureView);
    pub static nvnCommandBufferTiledDownsampleTextureView: fn(*mut NvnCommandBuffer, *const NvnTexture, *const NvnTextureView, *const NvnTexture, *const NvnTextureView);

    // ── Synchronization ──
    pub static nvnCommandBufferBarrier: fn(*mut NvnCommandBuffer, i32);
    pub static nvnCommandBufferWaitSync: fn(*mut NvnCommandBuffer, *const NvnSync);
    pub static nvnCommandBufferFenceSync: fn(*mut NvnCommandBuffer, *mut NvnSync, NvnSyncCondition, i32);

    // ── Pools ──
    pub static nvnCommandBufferSetTexturePool: fn(*mut NvnCommandBuffer, *const NvnTexturePool);
    pub static nvnCommandBufferSetSamplerPool: fn(*mut NvnCommandBuffer, *const NvnSamplerPool);

    // ── Shader scratch ──
    pub static nvnCommandBufferSetShaderScratchMemory: fn(*mut NvnCommandBuffer, *const NvnMemoryPool, isize, usize);

    // ── ZCull ──
    pub static nvnCommandBufferSaveZCullData: fn(*mut NvnCommandBuffer, NvnBufferAddress, usize);
    pub static nvnCommandBufferRestoreZCullData: fn(*mut NvnCommandBuffer, NvnBufferAddress, usize);

    // ── Copy stride ──
    pub static nvnCommandBufferSetCopyRowStride: fn(*mut NvnCommandBuffer, isize);
    pub static nvnCommandBufferSetCopyImageStride: fn(*mut NvnCommandBuffer, isize);
    pub static nvnCommandBufferGetCopyRowStride: fn(*const NvnCommandBuffer) -> isize;
    pub static nvnCommandBufferGetCopyImageStride: fn(*const NvnCommandBuffer) -> isize;

    // ── Draw texture ──
    pub static nvnCommandBufferDrawTexture: fn(*mut NvnCommandBuffer, NvnTextureHandle, *const NvnDrawTextureRegion, *const NvnDrawTextureRegion);

    // ── Subroutines ──
    pub static nvnCommandBufferSetProgramSubroutines: fn(*mut NvnCommandBuffer, *mut NvnProgram, NvnShaderStage, i32, i32, *const i32);

    // ── Coverage modulation ──
    pub static nvnCommandBufferBindCoverageModulationTable: fn(*mut NvnCommandBuffer, *const f32);

    // ── Depth resolve ──
    pub static nvnCommandBufferResolveDepthBuffer: fn(*mut NvnCommandBuffer);

    // ── Color reduction (vtable-only) ──
    pub static nvnCommandBufferSetColorReductionEnable: fn(*mut NvnCommandBuffer, NvnBoolean);
    pub static nvnCommandBufferSetColorReductionThresholds: fn(*mut NvnCommandBuffer, f32, f32);

    // ── Debug groups ──
    pub static nvnCommandBufferPushDebugGroupStatic: fn(*mut NvnCommandBuffer, u32, *const u8);
    pub static nvnCommandBufferPushDebugGroupDynamic: fn(*mut NvnCommandBuffer, u32, *const u8);
    pub static nvnCommandBufferPushDebugGroup: fn(*mut NvnCommandBuffer, u32, *const u8);
    pub static nvnCommandBufferPopDebugGroup: fn(*mut NvnCommandBuffer);
    pub static nvnCommandBufferPopDebugGroupId: fn(*mut NvnCommandBuffer, u32);
    pub static nvnCommandBufferInsertDebugMarkerStatic: fn(*mut NvnCommandBuffer, u32, *const u8);
    pub static nvnCommandBufferInsertDebugMarkerDynamic: fn(*mut NvnCommandBuffer, u32, *const u8);
    pub static nvnCommandBufferInsertDebugMarker: fn(*mut NvnCommandBuffer, *const u8);

    // ── Memory callback queries ──
    pub static nvnCommandBufferGetMemoryCallback: fn(*const NvnCommandBuffer) -> PfnNvnCommandBufferMemoryCallback;
    pub static nvnCommandBufferGetMemoryCallbackData: fn(*const NvnCommandBuffer) -> *mut core::ffi::c_void;
    pub static nvnCommandBufferIsRecording: fn(*const NvnCommandBuffer) -> NvnBoolean;

    // ── Event commands ──
    pub static nvnCommandBufferWaitEvent: fn(*mut NvnCommandBuffer, *const NvnEvent, NvnEventWaitMode, u32);
    pub static nvnCommandBufferSignalEvent: fn(*mut NvnCommandBuffer, *const NvnEvent, NvnEventSignalMode, NvnEventSignalLocation, i32, u32);

    // ── Stencil cull (vtable-only) ──
    pub static nvnCommandBufferSetStencilCullCriteria: fn(*mut NvnCommandBuffer, i32, i32, i32);
}

// Ergonomic wrappers around resolved GPU entry points.
// These keep per-domain APIs in their own modules while using one central resolver.
crate::nvn_wrap_ret!(command_buffer_initialize(arg0: *mut NvnCommandBuffer, arg1: *mut NvnDevice) -> NvnBoolean => SLOT_NVN_COMMAND_BUFFER_INITIALIZE);

crate::nvn_wrap_void!(command_buffer_finalize(arg0: *mut NvnCommandBuffer) => SLOT_NVN_COMMAND_BUFFER_FINALIZE);

crate::nvn_wrap_void!(command_buffer_set_debug_label(arg0: *mut NvnCommandBuffer, arg1: *const u8) => SLOT_NVN_COMMAND_BUFFER_SET_DEBUG_LABEL);

crate::nvn_wrap_void!(command_buffer_set_memory_callback(arg0: *mut NvnCommandBuffer, arg1: PfnNvnCommandBufferMemoryCallback) => SLOT_NVN_COMMAND_BUFFER_SET_MEMORY_CALLBACK);

crate::nvn_wrap_void!(command_buffer_set_memory_callback_data(arg0: *mut NvnCommandBuffer, arg1: *mut core::ffi::c_void) => SLOT_NVN_COMMAND_BUFFER_SET_MEMORY_CALLBACK_DATA);

crate::nvn_wrap_void!(command_buffer_set_command_memory_callback_enabled(arg0: *mut NvnCommandBuffer, arg1: NvnBoolean) => SLOT_NVN_COMMAND_BUFFER_SET_COMMAND_MEMORY_CALLBACK_ENABLED);

crate::nvn_wrap_void!(command_buffer_add_command_memory(arg0: *mut NvnCommandBuffer, arg1: *const NvnMemoryPool, arg2: isize, arg3: usize) => SLOT_NVN_COMMAND_BUFFER_ADD_COMMAND_MEMORY);

crate::nvn_wrap_void!(command_buffer_add_control_memory(arg0: *mut NvnCommandBuffer, arg1: *mut core::ffi::c_void, arg2: usize) => SLOT_NVN_COMMAND_BUFFER_ADD_CONTROL_MEMORY);

crate::nvn_wrap_ret!(command_buffer_get_command_memory_size(arg0: *const NvnCommandBuffer) -> usize => SLOT_NVN_COMMAND_BUFFER_GET_COMMAND_MEMORY_SIZE);

crate::nvn_wrap_ret!(command_buffer_get_command_memory_used(arg0: *const NvnCommandBuffer) -> usize => SLOT_NVN_COMMAND_BUFFER_GET_COMMAND_MEMORY_USED);

crate::nvn_wrap_ret!(command_buffer_get_command_memory_free(arg0: *const NvnCommandBuffer) -> usize => SLOT_NVN_COMMAND_BUFFER_GET_COMMAND_MEMORY_FREE);

crate::nvn_wrap_ret!(command_buffer_get_control_memory_size(arg0: *const NvnCommandBuffer) -> usize => SLOT_NVN_COMMAND_BUFFER_GET_CONTROL_MEMORY_SIZE);

crate::nvn_wrap_ret!(command_buffer_get_control_memory_used(arg0: *const NvnCommandBuffer) -> usize => SLOT_NVN_COMMAND_BUFFER_GET_CONTROL_MEMORY_USED);

crate::nvn_wrap_ret!(command_buffer_get_control_memory_free(arg0: *const NvnCommandBuffer) -> usize => SLOT_NVN_COMMAND_BUFFER_GET_CONTROL_MEMORY_FREE);

crate::nvn_wrap_void!(command_buffer_begin_recording(arg0: *mut NvnCommandBuffer) => SLOT_NVN_COMMAND_BUFFER_BEGIN_RECORDING);

crate::nvn_wrap_ret!(command_buffer_end_recording(arg0: *mut NvnCommandBuffer) -> NvnCommandHandle => SLOT_NVN_COMMAND_BUFFER_END_RECORDING);

crate::nvn_wrap_void!(command_buffer_call_commands(arg0: *mut NvnCommandBuffer, arg1: i32, arg2: *const NvnCommandHandle) => SLOT_NVN_COMMAND_BUFFER_CALL_COMMANDS);

crate::nvn_wrap_void!(command_buffer_copy_commands(arg0: *mut NvnCommandBuffer, arg1: i32, arg2: *const NvnCommandHandle) => SLOT_NVN_COMMAND_BUFFER_COPY_COMMANDS);

crate::nvn_wrap_void!(command_buffer_bind_blend_state(arg0: *mut NvnCommandBuffer, arg1: *const NvnBlendState) => SLOT_NVN_COMMAND_BUFFER_BIND_BLEND_STATE);

crate::nvn_wrap_void!(command_buffer_bind_channel_mask_state(arg0: *mut NvnCommandBuffer, arg1: *const NvnChannelMaskState) => SLOT_NVN_COMMAND_BUFFER_BIND_CHANNEL_MASK_STATE);

crate::nvn_wrap_void!(command_buffer_bind_color_state(arg0: *mut NvnCommandBuffer, arg1: *const NvnColorState) => SLOT_NVN_COMMAND_BUFFER_BIND_COLOR_STATE);

crate::nvn_wrap_void!(command_buffer_bind_multisample_state(arg0: *mut NvnCommandBuffer, arg1: *const NvnMultisampleState) => SLOT_NVN_COMMAND_BUFFER_BIND_MULTISAMPLE_STATE);

crate::nvn_wrap_void!(command_buffer_bind_polygon_state(arg0: *mut NvnCommandBuffer, arg1: *const NvnPolygonState) => SLOT_NVN_COMMAND_BUFFER_BIND_POLYGON_STATE);

crate::nvn_wrap_void!(command_buffer_bind_depth_stencil_state(arg0: *mut NvnCommandBuffer, arg1: *const NvnDepthStencilState) => SLOT_NVN_COMMAND_BUFFER_BIND_DEPTH_STENCIL_STATE);

crate::nvn_wrap_void!(command_buffer_bind_vertex_attrib_state(arg0: *mut NvnCommandBuffer, arg1: i32, arg2: *const NvnVertexAttribState) => SLOT_NVN_COMMAND_BUFFER_BIND_VERTEX_ATTRIB_STATE);

crate::nvn_wrap_void!(command_buffer_bind_vertex_stream_state(arg0: *mut NvnCommandBuffer, arg1: i32, arg2: *const NvnVertexStreamState) => SLOT_NVN_COMMAND_BUFFER_BIND_VERTEX_STREAM_STATE);

crate::nvn_wrap_void!(command_buffer_bind_program(arg0: *mut NvnCommandBuffer, arg1: *const NvnProgram, arg2: i32) => SLOT_NVN_COMMAND_BUFFER_BIND_PROGRAM);

crate::nvn_wrap_void!(command_buffer_bind_vertex_buffer(arg0: *mut NvnCommandBuffer, arg1: i32, arg2: NvnBufferAddress, arg3: usize) => SLOT_NVN_COMMAND_BUFFER_BIND_VERTEX_BUFFER);

crate::nvn_wrap_void!(command_buffer_bind_vertex_buffers(arg0: *mut NvnCommandBuffer, arg1: i32, arg2: i32, arg3: *const NvnBufferRange) => SLOT_NVN_COMMAND_BUFFER_BIND_VERTEX_BUFFERS);

crate::nvn_wrap_void!(command_buffer_bind_uniform_buffer(arg0: *mut NvnCommandBuffer, arg1: NvnShaderStage, arg2: i32, arg3: NvnBufferAddress, arg4: usize) => SLOT_NVN_COMMAND_BUFFER_BIND_UNIFORM_BUFFER);

crate::nvn_wrap_void!(command_buffer_bind_uniform_buffers(arg0: *mut NvnCommandBuffer, arg1: NvnShaderStage, arg2: i32, arg3: i32, arg4: *const NvnBufferRange) => SLOT_NVN_COMMAND_BUFFER_BIND_UNIFORM_BUFFERS);

crate::nvn_wrap_void!(command_buffer_bind_transform_feedback_buffer(arg0: *mut NvnCommandBuffer, arg1: i32, arg2: NvnBufferAddress, arg3: usize) => SLOT_NVN_COMMAND_BUFFER_BIND_TRANSFORM_FEEDBACK_BUFFER);

crate::nvn_wrap_void!(command_buffer_bind_transform_feedback_buffers(arg0: *mut NvnCommandBuffer, arg1: i32, arg2: i32, arg3: *const NvnBufferRange) => SLOT_NVN_COMMAND_BUFFER_BIND_TRANSFORM_FEEDBACK_BUFFERS);

crate::nvn_wrap_void!(command_buffer_bind_storage_buffer(arg0: *mut NvnCommandBuffer, arg1: NvnShaderStage, arg2: i32, arg3: NvnBufferAddress, arg4: usize) => SLOT_NVN_COMMAND_BUFFER_BIND_STORAGE_BUFFER);

crate::nvn_wrap_void!(command_buffer_bind_storage_buffers(arg0: *mut NvnCommandBuffer, arg1: NvnShaderStage, arg2: i32, arg3: i32, arg4: *const NvnBufferRange) => SLOT_NVN_COMMAND_BUFFER_BIND_STORAGE_BUFFERS);

crate::nvn_wrap_void!(command_buffer_bind_texture(arg0: *mut NvnCommandBuffer, arg1: NvnShaderStage, arg2: i32, arg3: NvnTextureHandle) => SLOT_NVN_COMMAND_BUFFER_BIND_TEXTURE);

crate::nvn_wrap_void!(command_buffer_bind_textures(arg0: *mut NvnCommandBuffer, arg1: NvnShaderStage, arg2: i32, arg3: i32, arg4: *const NvnTextureHandle) => SLOT_NVN_COMMAND_BUFFER_BIND_TEXTURES);

crate::nvn_wrap_void!(command_buffer_bind_image(arg0: *mut NvnCommandBuffer, arg1: NvnShaderStage, arg2: i32, arg3: NvnImageHandle) => SLOT_NVN_COMMAND_BUFFER_BIND_IMAGE);

crate::nvn_wrap_void!(command_buffer_bind_images(arg0: *mut NvnCommandBuffer, arg1: NvnShaderStage, arg2: i32, arg3: i32, arg4: *const NvnImageHandle) => SLOT_NVN_COMMAND_BUFFER_BIND_IMAGES);

crate::nvn_wrap_void!(command_buffer_set_patch_size(arg0: *mut NvnCommandBuffer, arg1: i32) => SLOT_NVN_COMMAND_BUFFER_SET_PATCH_SIZE);

crate::nvn_wrap_void!(command_buffer_set_inner_tessellation_levels(arg0: *mut NvnCommandBuffer, arg1: *const f32) => SLOT_NVN_COMMAND_BUFFER_SET_INNER_TESSELLATION_LEVELS);

crate::nvn_wrap_void!(command_buffer_set_outer_tessellation_levels(arg0: *mut NvnCommandBuffer, arg1: *const f32) => SLOT_NVN_COMMAND_BUFFER_SET_OUTER_TESSELLATION_LEVELS);

crate::nvn_wrap_void!(command_buffer_set_primitive_restart(arg0: *mut NvnCommandBuffer, arg1: NvnBoolean, arg2: i32) => SLOT_NVN_COMMAND_BUFFER_SET_PRIMITIVE_RESTART);

crate::nvn_wrap_void!(command_buffer_begin_transform_feedback(arg0: *mut NvnCommandBuffer, arg1: NvnBufferAddress) => SLOT_NVN_COMMAND_BUFFER_BEGIN_TRANSFORM_FEEDBACK);

crate::nvn_wrap_void!(command_buffer_end_transform_feedback(arg0: *mut NvnCommandBuffer, arg1: NvnBufferAddress) => SLOT_NVN_COMMAND_BUFFER_END_TRANSFORM_FEEDBACK);

crate::nvn_wrap_void!(command_buffer_pause_transform_feedback(arg0: *mut NvnCommandBuffer, arg1: NvnBufferAddress) => SLOT_NVN_COMMAND_BUFFER_PAUSE_TRANSFORM_FEEDBACK);

crate::nvn_wrap_void!(command_buffer_resume_transform_feedback(arg0: *mut NvnCommandBuffer, arg1: NvnBufferAddress) => SLOT_NVN_COMMAND_BUFFER_RESUME_TRANSFORM_FEEDBACK);

crate::nvn_wrap_void!(command_buffer_draw_transform_feedback(arg0: *mut NvnCommandBuffer, arg1: NvnDrawPrimitive, arg2: NvnBufferAddress) => SLOT_NVN_COMMAND_BUFFER_DRAW_TRANSFORM_FEEDBACK);

crate::nvn_wrap_void!(command_buffer_draw_arrays(arg0: *mut NvnCommandBuffer, arg1: NvnDrawPrimitive, arg2: i32, arg3: i32) => SLOT_NVN_COMMAND_BUFFER_DRAW_ARRAYS);

crate::nvn_wrap_void!(command_buffer_draw_elements(arg0: *mut NvnCommandBuffer, arg1: NvnDrawPrimitive, arg2: NvnIndexType, arg3: i32, arg4: NvnBufferAddress) => SLOT_NVN_COMMAND_BUFFER_DRAW_ELEMENTS);

crate::nvn_wrap_void!(command_buffer_draw_elements_base_vertex(arg0: *mut NvnCommandBuffer, arg1: NvnDrawPrimitive, arg2: NvnIndexType, arg3: i32, arg4: NvnBufferAddress, arg5: i32) => SLOT_NVN_COMMAND_BUFFER_DRAW_ELEMENTS_BASE_VERTEX);

crate::nvn_wrap_void!(command_buffer_draw_arrays_instanced(arg0: *mut NvnCommandBuffer, arg1: NvnDrawPrimitive, arg2: i32, arg3: i32, arg4: i32, arg5: i32) => SLOT_NVN_COMMAND_BUFFER_DRAW_ARRAYS_INSTANCED);

crate::nvn_wrap_void!(command_buffer_draw_elements_instanced(arg0: *mut NvnCommandBuffer, arg1: NvnDrawPrimitive, arg2: NvnIndexType, arg3: i32, arg4: NvnBufferAddress, arg5: i32, arg6: i32, arg7: i32) => SLOT_NVN_COMMAND_BUFFER_DRAW_ELEMENTS_INSTANCED);

crate::nvn_wrap_void!(command_buffer_draw_arrays_indirect(arg0: *mut NvnCommandBuffer, arg1: NvnDrawPrimitive, arg2: NvnBufferAddress) => SLOT_NVN_COMMAND_BUFFER_DRAW_ARRAYS_INDIRECT);

crate::nvn_wrap_void!(command_buffer_draw_elements_indirect(arg0: *mut NvnCommandBuffer, arg1: NvnDrawPrimitive, arg2: NvnIndexType, arg3: NvnBufferAddress, arg4: NvnBufferAddress) => SLOT_NVN_COMMAND_BUFFER_DRAW_ELEMENTS_INDIRECT);

crate::nvn_wrap_void!(command_buffer_multi_draw_arrays_indirect_count(arg0: *mut NvnCommandBuffer, arg1: NvnDrawPrimitive, arg2: NvnBufferAddress, arg3: NvnBufferAddress, arg4: i32, arg5: isize) => SLOT_NVN_COMMAND_BUFFER_MULTI_DRAW_ARRAYS_INDIRECT_COUNT);

crate::nvn_wrap_void!(command_buffer_multi_draw_elements_indirect_count(arg0: *mut NvnCommandBuffer, arg1: NvnDrawPrimitive, arg2: NvnIndexType, arg3: NvnBufferAddress, arg4: NvnBufferAddress, arg5: NvnBufferAddress, arg6: i32, arg7: isize) => SLOT_NVN_COMMAND_BUFFER_MULTI_DRAW_ELEMENTS_INDIRECT_COUNT);

crate::nvn_wrap_void!(command_buffer_clear_color(arg0: *mut NvnCommandBuffer, arg1: i32, arg2: *const f32, arg3: i32) => SLOT_NVN_COMMAND_BUFFER_CLEAR_COLOR);

crate::nvn_wrap_void!(command_buffer_clear_colori(arg0: *mut NvnCommandBuffer, arg1: i32, arg2: *const i32, arg3: i32) => SLOT_NVN_COMMAND_BUFFER_CLEAR_COLORI);

crate::nvn_wrap_void!(command_buffer_clear_colorui(arg0: *mut NvnCommandBuffer, arg1: i32, arg2: *const u32, arg3: i32) => SLOT_NVN_COMMAND_BUFFER_CLEAR_COLORUI);

crate::nvn_wrap_void!(command_buffer_clear_depth_stencil(arg0: *mut NvnCommandBuffer, arg1: f32, arg2: NvnBoolean, arg3: i32, arg4: i32) => SLOT_NVN_COMMAND_BUFFER_CLEAR_DEPTH_STENCIL);

crate::nvn_wrap_void!(command_buffer_dispatch_compute(arg0: *mut NvnCommandBuffer, arg1: i32, arg2: i32, arg3: i32) => SLOT_NVN_COMMAND_BUFFER_DISPATCH_COMPUTE);

crate::nvn_wrap_void!(command_buffer_dispatch_compute_indirect(arg0: *mut NvnCommandBuffer, arg1: NvnBufferAddress) => SLOT_NVN_COMMAND_BUFFER_DISPATCH_COMPUTE_INDIRECT);

crate::nvn_wrap_void!(command_buffer_set_viewport(arg0: *mut NvnCommandBuffer, arg1: i32, arg2: i32, arg3: i32, arg4: i32) => SLOT_NVN_COMMAND_BUFFER_SET_VIEWPORT);

crate::nvn_wrap_void!(command_buffer_set_viewports(arg0: *mut NvnCommandBuffer, arg1: i32, arg2: i32, arg3: *const f32) => SLOT_NVN_COMMAND_BUFFER_SET_VIEWPORTS);

crate::nvn_wrap_void!(command_buffer_set_viewport_swizzles(arg0: *mut NvnCommandBuffer, arg1: i32, arg2: i32, arg3: *const NvnViewportSwizzle) => SLOT_NVN_COMMAND_BUFFER_SET_VIEWPORT_SWIZZLES);

crate::nvn_wrap_void!(command_buffer_set_scissor(arg0: *mut NvnCommandBuffer, arg1: i32, arg2: i32, arg3: i32, arg4: i32) => SLOT_NVN_COMMAND_BUFFER_SET_SCISSOR);

crate::nvn_wrap_void!(command_buffer_set_scissors(arg0: *mut NvnCommandBuffer, arg1: i32, arg2: i32, arg3: *const i32) => SLOT_NVN_COMMAND_BUFFER_SET_SCISSORS);

crate::nvn_wrap_void!(command_buffer_set_depth_range(arg0: *mut NvnCommandBuffer, arg1: f32, arg2: f32) => SLOT_NVN_COMMAND_BUFFER_SET_DEPTH_RANGE);

crate::nvn_wrap_void!(command_buffer_set_depth_bounds(arg0: *mut NvnCommandBuffer, arg1: NvnBoolean, arg2: f32, arg3: f32) => SLOT_NVN_COMMAND_BUFFER_SET_DEPTH_BOUNDS);

crate::nvn_wrap_void!(command_buffer_set_depth_ranges(arg0: *mut NvnCommandBuffer, arg1: i32, arg2: i32, arg3: *const f32) => SLOT_NVN_COMMAND_BUFFER_SET_DEPTH_RANGES);

crate::nvn_wrap_void!(command_buffer_set_tiled_cache_action(arg0: *mut NvnCommandBuffer, arg1: NvnTiledCacheAction) => SLOT_NVN_COMMAND_BUFFER_SET_TILED_CACHE_ACTION);

crate::nvn_wrap_void!(command_buffer_set_tiled_cache_tile_size(arg0: *mut NvnCommandBuffer, arg1: i32, arg2: i32) => SLOT_NVN_COMMAND_BUFFER_SET_TILED_CACHE_TILE_SIZE);

crate::nvn_wrap_void!(command_buffer_bind_separate_texture(arg0: *mut NvnCommandBuffer, arg1: NvnShaderStage, arg2: i32, arg3: NvnSeparateTextureHandle) => SLOT_NVN_COMMAND_BUFFER_BIND_SEPARATE_TEXTURE);

crate::nvn_wrap_void!(command_buffer_bind_separate_sampler(arg0: *mut NvnCommandBuffer, arg1: NvnShaderStage, arg2: i32, arg3: NvnSeparateSamplerHandle) => SLOT_NVN_COMMAND_BUFFER_BIND_SEPARATE_SAMPLER);

crate::nvn_wrap_void!(command_buffer_bind_separate_textures(arg0: *mut NvnCommandBuffer, arg1: NvnShaderStage, arg2: i32, arg3: i32, arg4: *const NvnSeparateTextureHandle) => SLOT_NVN_COMMAND_BUFFER_BIND_SEPARATE_TEXTURES);

crate::nvn_wrap_void!(command_buffer_bind_separate_samplers(arg0: *mut NvnCommandBuffer, arg1: NvnShaderStage, arg2: i32, arg3: i32, arg4: *const NvnSeparateSamplerHandle) => SLOT_NVN_COMMAND_BUFFER_BIND_SEPARATE_SAMPLERS);

crate::nvn_wrap_void!(command_buffer_set_stencil_value_mask(arg0: *mut NvnCommandBuffer, arg1: NvnFace, arg2: i32) => SLOT_NVN_COMMAND_BUFFER_SET_STENCIL_VALUE_MASK);

crate::nvn_wrap_void!(command_buffer_set_stencil_mask(arg0: *mut NvnCommandBuffer, arg1: NvnFace, arg2: i32) => SLOT_NVN_COMMAND_BUFFER_SET_STENCIL_MASK);

crate::nvn_wrap_void!(command_buffer_set_stencil_ref(arg0: *mut NvnCommandBuffer, arg1: NvnFace, arg2: i32) => SLOT_NVN_COMMAND_BUFFER_SET_STENCIL_REF);

crate::nvn_wrap_void!(command_buffer_set_blend_color(arg0: *mut NvnCommandBuffer, arg1: *const f32) => SLOT_NVN_COMMAND_BUFFER_SET_BLEND_COLOR);

crate::nvn_wrap_void!(command_buffer_set_point_size(arg0: *mut NvnCommandBuffer, arg1: f32) => SLOT_NVN_COMMAND_BUFFER_SET_POINT_SIZE);

crate::nvn_wrap_void!(command_buffer_set_line_width(arg0: *mut NvnCommandBuffer, arg1: f32) => SLOT_NVN_COMMAND_BUFFER_SET_LINE_WIDTH);

crate::nvn_wrap_void!(command_buffer_set_polygon_offset_clamp(arg0: *mut NvnCommandBuffer, arg1: f32, arg2: f32, arg3: f32) => SLOT_NVN_COMMAND_BUFFER_SET_POLYGON_OFFSET_CLAMP);

crate::nvn_wrap_void!(command_buffer_set_alpha_ref(arg0: *mut NvnCommandBuffer, arg1: f32) => SLOT_NVN_COMMAND_BUFFER_SET_ALPHA_REF);

crate::nvn_wrap_void!(command_buffer_set_sample_mask(arg0: *mut NvnCommandBuffer, arg1: i32) => SLOT_NVN_COMMAND_BUFFER_SET_SAMPLE_MASK);

crate::nvn_wrap_void!(command_buffer_set_rasterizer_discard(arg0: *mut NvnCommandBuffer, arg1: NvnBoolean) => SLOT_NVN_COMMAND_BUFFER_SET_RASTERIZER_DISCARD);

crate::nvn_wrap_void!(command_buffer_set_depth_clamp(arg0: *mut NvnCommandBuffer, arg1: NvnBoolean) => SLOT_NVN_COMMAND_BUFFER_SET_DEPTH_CLAMP);

crate::nvn_wrap_void!(command_buffer_set_conservative_raster_enable(arg0: *mut NvnCommandBuffer, arg1: NvnBoolean) => SLOT_NVN_COMMAND_BUFFER_SET_CONSERVATIVE_RASTER_ENABLE);

crate::nvn_wrap_void!(command_buffer_set_conservative_raster_dilate(arg0: *mut NvnCommandBuffer, arg1: f32) => SLOT_NVN_COMMAND_BUFFER_SET_CONSERVATIVE_RASTER_DILATE);

crate::nvn_wrap_void!(command_buffer_set_subpixel_precision_bias(arg0: *mut NvnCommandBuffer, arg1: i32, arg2: i32) => SLOT_NVN_COMMAND_BUFFER_SET_SUBPIXEL_PRECISION_BIAS);

crate::nvn_wrap_void!(command_buffer_copy_buffer_to_texture(arg0: *mut NvnCommandBuffer, arg1: NvnBufferAddress, arg2: *const NvnTexture, arg3: *const NvnTextureView, arg4: *const NvnCopyRegion, arg5: i32) => SLOT_NVN_COMMAND_BUFFER_COPY_BUFFER_TO_TEXTURE);

crate::nvn_wrap_void!(command_buffer_copy_texture_to_buffer(arg0: *mut NvnCommandBuffer, arg1: *const NvnTexture, arg2: *const NvnTextureView, arg3: *const NvnCopyRegion, arg4: NvnBufferAddress, arg5: i32) => SLOT_NVN_COMMAND_BUFFER_COPY_TEXTURE_TO_BUFFER);

crate::nvn_wrap_void!(command_buffer_copy_texture_to_texture(arg0: *mut NvnCommandBuffer, arg1: *const NvnTexture, arg2: *const NvnTextureView, arg3: *const NvnCopyRegion, arg4: *const NvnTexture, arg5: *const NvnTextureView, arg6: *const NvnCopyRegion, arg7: i32) => SLOT_NVN_COMMAND_BUFFER_COPY_TEXTURE_TO_TEXTURE);

crate::nvn_wrap_void!(command_buffer_copy_buffer_to_buffer(arg0: *mut NvnCommandBuffer, arg1: NvnBufferAddress, arg2: NvnBufferAddress, arg3: usize, arg4: i32) => SLOT_NVN_COMMAND_BUFFER_COPY_BUFFER_TO_BUFFER);

crate::nvn_wrap_void!(command_buffer_clear_buffer(arg0: *mut NvnCommandBuffer, arg1: NvnBufferAddress, arg2: usize, arg3: u32) => SLOT_NVN_COMMAND_BUFFER_CLEAR_BUFFER);

crate::nvn_wrap_void!(command_buffer_clear_texture(arg0: *mut NvnCommandBuffer, arg1: *const NvnTexture, arg2: *const NvnTextureView, arg3: *const NvnCopyRegion, arg4: *const f32, arg5: i32) => SLOT_NVN_COMMAND_BUFFER_CLEAR_TEXTURE);

crate::nvn_wrap_void!(command_buffer_clear_texturei(arg0: *mut NvnCommandBuffer, arg1: *const NvnTexture, arg2: *const NvnTextureView, arg3: *const NvnCopyRegion, arg4: *const i32, arg5: i32) => SLOT_NVN_COMMAND_BUFFER_CLEAR_TEXTUREI);

crate::nvn_wrap_void!(command_buffer_clear_textureui(arg0: *mut NvnCommandBuffer, arg1: *const NvnTexture, arg2: *const NvnTextureView, arg3: *const NvnCopyRegion, arg4: *const u32, arg5: i32) => SLOT_NVN_COMMAND_BUFFER_CLEAR_TEXTUREUI);

crate::nvn_wrap_void!(command_buffer_update_uniform_buffer(arg0: *mut NvnCommandBuffer, arg1: NvnBufferAddress, arg2: usize, arg3: isize, arg4: usize, arg5: *const core::ffi::c_void) => SLOT_NVN_COMMAND_BUFFER_UPDATE_UNIFORM_BUFFER);

crate::nvn_wrap_void!(command_buffer_report_counter(arg0: *mut NvnCommandBuffer, arg1: NvnCounterType, arg2: NvnBufferAddress) => SLOT_NVN_COMMAND_BUFFER_REPORT_COUNTER);

crate::nvn_wrap_void!(command_buffer_reset_counter(arg0: *mut NvnCommandBuffer, arg1: NvnCounterType) => SLOT_NVN_COMMAND_BUFFER_RESET_COUNTER);

crate::nvn_wrap_void!(command_buffer_report_value(arg0: *mut NvnCommandBuffer, arg1: u32, arg2: NvnBufferAddress) => SLOT_NVN_COMMAND_BUFFER_REPORT_VALUE);

crate::nvn_wrap_void!(command_buffer_set_render_enable(arg0: *mut NvnCommandBuffer, arg1: NvnBoolean) => SLOT_NVN_COMMAND_BUFFER_SET_RENDER_ENABLE);

crate::nvn_wrap_void!(command_buffer_set_render_enable_conditional(arg0: *mut NvnCommandBuffer, arg1: NvnConditionalRenderMode, arg2: NvnBufferAddress) => SLOT_NVN_COMMAND_BUFFER_SET_RENDER_ENABLE_CONDITIONAL);

crate::nvn_wrap_void!(command_buffer_set_render_targets(arg0: *mut NvnCommandBuffer, arg1: i32, arg2: *const *const NvnTexture, arg3: *const *const NvnTextureView, arg4: *const NvnTexture, arg5: *const NvnTextureView) => SLOT_NVN_COMMAND_BUFFER_SET_RENDER_TARGETS);

crate::nvn_wrap_void!(command_buffer_discard_color(arg0: *mut NvnCommandBuffer, arg1: i32) => SLOT_NVN_COMMAND_BUFFER_DISCARD_COLOR);

crate::nvn_wrap_void!(command_buffer_discard_depth_stencil(arg0: *mut NvnCommandBuffer) => SLOT_NVN_COMMAND_BUFFER_DISCARD_DEPTH_STENCIL);

crate::nvn_wrap_void!(command_buffer_downsample(arg0: *mut NvnCommandBuffer, arg1: *const NvnTexture, arg2: *const NvnTexture) => SLOT_NVN_COMMAND_BUFFER_DOWNSAMPLE);

crate::nvn_wrap_void!(command_buffer_tiled_downsample(arg0: *mut NvnCommandBuffer, arg1: *const NvnTexture, arg2: *const NvnTexture) => SLOT_NVN_COMMAND_BUFFER_TILED_DOWNSAMPLE);

crate::nvn_wrap_void!(command_buffer_downsample_texture_view(arg0: *mut NvnCommandBuffer, arg1: *const NvnTexture, arg2: *const NvnTextureView, arg3: *const NvnTexture, arg4: *const NvnTextureView) => SLOT_NVN_COMMAND_BUFFER_DOWNSAMPLE_TEXTURE_VIEW);

crate::nvn_wrap_void!(command_buffer_tiled_downsample_texture_view(arg0: *mut NvnCommandBuffer, arg1: *const NvnTexture, arg2: *const NvnTextureView, arg3: *const NvnTexture, arg4: *const NvnTextureView) => SLOT_NVN_COMMAND_BUFFER_TILED_DOWNSAMPLE_TEXTURE_VIEW);

crate::nvn_wrap_void!(command_buffer_barrier(arg0: *mut NvnCommandBuffer, arg1: i32) => SLOT_NVN_COMMAND_BUFFER_BARRIER);

crate::nvn_wrap_void!(command_buffer_wait_sync(arg0: *mut NvnCommandBuffer, arg1: *const NvnSync) => SLOT_NVN_COMMAND_BUFFER_WAIT_SYNC);

crate::nvn_wrap_void!(command_buffer_fence_sync(arg0: *mut NvnCommandBuffer, arg1: *mut NvnSync, arg2: NvnSyncCondition, arg3: i32) => SLOT_NVN_COMMAND_BUFFER_FENCE_SYNC);

crate::nvn_wrap_void!(command_buffer_set_texture_pool(arg0: *mut NvnCommandBuffer, arg1: *const NvnTexturePool) => SLOT_NVN_COMMAND_BUFFER_SET_TEXTURE_POOL);

crate::nvn_wrap_void!(command_buffer_set_sampler_pool(arg0: *mut NvnCommandBuffer, arg1: *const NvnSamplerPool) => SLOT_NVN_COMMAND_BUFFER_SET_SAMPLER_POOL);

crate::nvn_wrap_void!(command_buffer_set_shader_scratch_memory(arg0: *mut NvnCommandBuffer, arg1: *const NvnMemoryPool, arg2: isize, arg3: usize) => SLOT_NVN_COMMAND_BUFFER_SET_SHADER_SCRATCH_MEMORY);

crate::nvn_wrap_void!(command_buffer_save_z_cull_data(arg0: *mut NvnCommandBuffer, arg1: NvnBufferAddress, arg2: usize) => SLOT_NVN_COMMAND_BUFFER_SAVE_Z_CULL_DATA);

crate::nvn_wrap_void!(command_buffer_restore_z_cull_data(arg0: *mut NvnCommandBuffer, arg1: NvnBufferAddress, arg2: usize) => SLOT_NVN_COMMAND_BUFFER_RESTORE_Z_CULL_DATA);

crate::nvn_wrap_void!(command_buffer_set_copy_row_stride(arg0: *mut NvnCommandBuffer, arg1: isize) => SLOT_NVN_COMMAND_BUFFER_SET_COPY_ROW_STRIDE);

crate::nvn_wrap_void!(command_buffer_set_copy_image_stride(arg0: *mut NvnCommandBuffer, arg1: isize) => SLOT_NVN_COMMAND_BUFFER_SET_COPY_IMAGE_STRIDE);

crate::nvn_wrap_ret!(command_buffer_get_copy_row_stride(arg0: *const NvnCommandBuffer) -> isize => SLOT_NVN_COMMAND_BUFFER_GET_COPY_ROW_STRIDE);

crate::nvn_wrap_ret!(command_buffer_get_copy_image_stride(arg0: *const NvnCommandBuffer) -> isize => SLOT_NVN_COMMAND_BUFFER_GET_COPY_IMAGE_STRIDE);

crate::nvn_wrap_void!(command_buffer_draw_texture(arg0: *mut NvnCommandBuffer, arg1: NvnTextureHandle, arg2: *const NvnDrawTextureRegion, arg3: *const NvnDrawTextureRegion) => SLOT_NVN_COMMAND_BUFFER_DRAW_TEXTURE);

crate::nvn_wrap_void!(command_buffer_set_program_subroutines(arg0: *mut NvnCommandBuffer, arg1: *mut NvnProgram, arg2: NvnShaderStage, arg3: i32, arg4: i32, arg5: *const i32) => SLOT_NVN_COMMAND_BUFFER_SET_PROGRAM_SUBROUTINES);

crate::nvn_wrap_void!(command_buffer_bind_coverage_modulation_table(arg0: *mut NvnCommandBuffer, arg1: *const f32) => SLOT_NVN_COMMAND_BUFFER_BIND_COVERAGE_MODULATION_TABLE);

crate::nvn_wrap_void!(command_buffer_resolve_depth_buffer(arg0: *mut NvnCommandBuffer) => SLOT_NVN_COMMAND_BUFFER_RESOLVE_DEPTH_BUFFER);

crate::nvn_wrap_void!(command_buffer_set_color_reduction_enable(arg0: *mut NvnCommandBuffer, arg1: NvnBoolean) => SLOT_NVN_COMMAND_BUFFER_SET_COLOR_REDUCTION_ENABLE);

crate::nvn_wrap_void!(command_buffer_set_color_reduction_thresholds(arg0: *mut NvnCommandBuffer, arg1: f32, arg2: f32) => SLOT_NVN_COMMAND_BUFFER_SET_COLOR_REDUCTION_THRESHOLDS);

crate::nvn_wrap_void!(command_buffer_push_debug_group_static(arg0: *mut NvnCommandBuffer, arg1: u32, arg2: *const u8) => SLOT_NVN_COMMAND_BUFFER_PUSH_DEBUG_GROUP_STATIC);

crate::nvn_wrap_void!(command_buffer_push_debug_group_dynamic(arg0: *mut NvnCommandBuffer, arg1: u32, arg2: *const u8) => SLOT_NVN_COMMAND_BUFFER_PUSH_DEBUG_GROUP_DYNAMIC);

crate::nvn_wrap_void!(command_buffer_push_debug_group(arg0: *mut NvnCommandBuffer, arg1: u32, arg2: *const u8) => SLOT_NVN_COMMAND_BUFFER_PUSH_DEBUG_GROUP);

crate::nvn_wrap_void!(command_buffer_pop_debug_group(arg0: *mut NvnCommandBuffer) => SLOT_NVN_COMMAND_BUFFER_POP_DEBUG_GROUP);

crate::nvn_wrap_void!(command_buffer_pop_debug_group_id(arg0: *mut NvnCommandBuffer, arg1: u32) => SLOT_NVN_COMMAND_BUFFER_POP_DEBUG_GROUP_ID);

crate::nvn_wrap_void!(command_buffer_insert_debug_marker_static(arg0: *mut NvnCommandBuffer, arg1: u32, arg2: *const u8) => SLOT_NVN_COMMAND_BUFFER_INSERT_DEBUG_MARKER_STATIC);

crate::nvn_wrap_void!(command_buffer_insert_debug_marker_dynamic(arg0: *mut NvnCommandBuffer, arg1: u32, arg2: *const u8) => SLOT_NVN_COMMAND_BUFFER_INSERT_DEBUG_MARKER_DYNAMIC);

crate::nvn_wrap_void!(command_buffer_insert_debug_marker(arg0: *mut NvnCommandBuffer, arg1: *const u8) => SLOT_NVN_COMMAND_BUFFER_INSERT_DEBUG_MARKER);

crate::nvn_wrap_ret!(command_buffer_get_memory_callback(arg0: *const NvnCommandBuffer) -> PfnNvnCommandBufferMemoryCallback => SLOT_NVN_COMMAND_BUFFER_GET_MEMORY_CALLBACK);

crate::nvn_wrap_ret!(command_buffer_get_memory_callback_data(arg0: *const NvnCommandBuffer) -> *mut core::ffi::c_void => SLOT_NVN_COMMAND_BUFFER_GET_MEMORY_CALLBACK_DATA);

crate::nvn_wrap_ret!(command_buffer_is_recording(arg0: *const NvnCommandBuffer) -> NvnBoolean => SLOT_NVN_COMMAND_BUFFER_IS_RECORDING);

crate::nvn_wrap_void!(command_buffer_wait_event(arg0: *mut NvnCommandBuffer, arg1: *const NvnEvent, arg2: NvnEventWaitMode, arg3: u32) => SLOT_NVN_COMMAND_BUFFER_WAIT_EVENT);

crate::nvn_wrap_void!(command_buffer_signal_event(arg0: *mut NvnCommandBuffer, arg1: *const NvnEvent, arg2: NvnEventSignalMode, arg3: NvnEventSignalLocation, arg4: i32, arg5: u32) => SLOT_NVN_COMMAND_BUFFER_SIGNAL_EVENT);

crate::nvn_wrap_void!(command_buffer_set_stencil_cull_criteria(arg0: *mut NvnCommandBuffer, arg1: i32, arg2: i32, arg3: i32) => SLOT_NVN_COMMAND_BUFFER_SET_STENCIL_CULL_CRITERIA);

