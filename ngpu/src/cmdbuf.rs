#![allow(unused)]
use crate::*;

nvn_func! {
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
