#![allow(unused)]
use crate::*;

nvn_func! {
    // ── TextureBuilder ──
    pub static nvnTextureBuilderSetDevice: fn(*mut NvnTextureBuilder, *mut NvnDevice);
    pub static nvnTextureBuilderSetDefaults: fn(*mut NvnTextureBuilder);
    pub static nvnTextureBuilderSetFlags: fn(*mut NvnTextureBuilder, i32);
    pub static nvnTextureBuilderSetTarget: fn(*mut NvnTextureBuilder, NvnTextureTarget);
    pub static nvnTextureBuilderSetWidth: fn(*mut NvnTextureBuilder, i32);
    pub static nvnTextureBuilderSetHeight: fn(*mut NvnTextureBuilder, i32);
    pub static nvnTextureBuilderSetDepth: fn(*mut NvnTextureBuilder, i32);
    pub static nvnTextureBuilderSetSize1D: fn(*mut NvnTextureBuilder, i32);
    pub static nvnTextureBuilderSetSize2D: fn(*mut NvnTextureBuilder, i32, i32);
    pub static nvnTextureBuilderSetSize3D: fn(*mut NvnTextureBuilder, i32, i32, i32);
    pub static nvnTextureBuilderSetLevels: fn(*mut NvnTextureBuilder, i32);
    pub static nvnTextureBuilderSetFormat: fn(*mut NvnTextureBuilder, NvnFormat);
    pub static nvnTextureBuilderSetSamples: fn(*mut NvnTextureBuilder, i32);
    pub static nvnTextureBuilderSetSwizzle: fn(*mut NvnTextureBuilder, NvnTextureSwizzle, NvnTextureSwizzle, NvnTextureSwizzle, NvnTextureSwizzle);
    pub static nvnTextureBuilderSetDepthStencilMode: fn(*mut NvnTextureBuilder, NvnTextureDepthStencilMode);
    pub static nvnTextureBuilderGetStorageSize: fn(*const NvnTextureBuilder) -> usize;
    pub static nvnTextureBuilderGetStorageAlignment: fn(*const NvnTextureBuilder) -> usize;
    pub static nvnTextureBuilderSetStorage: fn(*mut NvnTextureBuilder, *mut NvnMemoryPool, isize);
    pub static nvnTextureBuilderSetPackagedTextureData: fn(*mut NvnTextureBuilder, *const core::ffi::c_void);
    pub static nvnTextureBuilderSetPackagedTextureLayout: fn(*mut NvnTextureBuilder, *const NvnPackagedTextureLayout);
    pub static nvnTextureBuilderSetStride: fn(*mut NvnTextureBuilder, isize);
    pub static nvnTextureBuilderSetGLTextureName: fn(*mut NvnTextureBuilder, u32);
    pub static nvnTextureBuilderGetStorageClass: fn(*const NvnTextureBuilder) -> NvnStorageClass;
    // vtable-only
    pub static nvnTextureBuilderGetDevice: fn(*const NvnTextureBuilder) -> *const NvnDevice;
    pub static nvnTextureBuilderGetFlags: fn(*const NvnTextureBuilder) -> NvnTextureFlags;
    pub static nvnTextureBuilderGetTarget: fn(*const NvnTextureBuilder) -> NvnTextureTarget;
    pub static nvnTextureBuilderGetWidth: fn(*const NvnTextureBuilder) -> i32;
    pub static nvnTextureBuilderGetHeight: fn(*const NvnTextureBuilder) -> i32;
    pub static nvnTextureBuilderGetDepth: fn(*const NvnTextureBuilder) -> i32;
    pub static nvnTextureBuilderGetLevels: fn(*const NvnTextureBuilder) -> i32;
    pub static nvnTextureBuilderGetFormat: fn(*const NvnTextureBuilder) -> NvnFormat;
    pub static nvnTextureBuilderGetSamples: fn(*const NvnTextureBuilder) -> i32;
    pub static nvnTextureBuilderGetSwizzle: fn(*const NvnTextureBuilder, *mut NvnTextureSwizzle, *mut NvnTextureSwizzle, *mut NvnTextureSwizzle, *mut NvnTextureSwizzle);
    pub static nvnTextureBuilderGetDepthStencilMode: fn(*const NvnTextureBuilder) -> NvnTextureDepthStencilMode;
    pub static nvnTextureBuilderGetPackagedTextureData: fn(*const NvnTextureBuilder) -> *const core::ffi::c_void;
    pub static nvnTextureBuilderGetPackagedTextureLayout: fn(*const NvnTextureBuilder) -> *const NvnPackagedTextureLayout;
    pub static nvnTextureBuilderGetStride: fn(*const NvnTextureBuilder) -> isize;
    pub static nvnTextureBuilderGetSparseTileLayout: fn(*const NvnTextureBuilder, *mut NvnTextureSparseTileLayout);
    pub static nvnTextureBuilderGetGLTextureName: fn(*const NvnTextureBuilder) -> u32;
    pub static nvnTextureBuilderGetZCullStorageSize: fn(*const NvnTextureBuilder) -> usize;
    pub static nvnTextureBuilderGetMemoryPool: fn(*const NvnTextureBuilder) -> *const NvnMemoryPool;
    pub static nvnTextureBuilderGetMemoryOffset: fn(*const NvnTextureBuilder) -> isize;
    // vtable-only
    pub static nvnTextureBuilderGetRawStorageClass: fn(*const NvnTextureBuilder) -> NvnStorageClass;

    // ── TextureView ──
    pub static nvnTextureViewSetDefaults: fn(*mut NvnTextureView);
    pub static nvnTextureViewSetLevels: fn(*mut NvnTextureView, i32, i32);
    pub static nvnTextureViewSetLayers: fn(*mut NvnTextureView, i32, i32);
    pub static nvnTextureViewSetFormat: fn(*mut NvnTextureView, NvnFormat);
    pub static nvnTextureViewSetSwizzle: fn(*mut NvnTextureView, NvnTextureSwizzle, NvnTextureSwizzle, NvnTextureSwizzle, NvnTextureSwizzle);
    pub static nvnTextureViewSetDepthStencilMode: fn(*mut NvnTextureView, NvnTextureDepthStencilMode);
    pub static nvnTextureViewSetTarget: fn(*mut NvnTextureView, NvnTextureTarget);
    pub static nvnTextureViewGetLevels: fn(*const NvnTextureView, *mut i32, *mut i32) -> NvnBoolean;
    pub static nvnTextureViewGetLayers: fn(*const NvnTextureView, *mut i32, *mut i32) -> NvnBoolean;
    pub static nvnTextureViewGetFormat: fn(*const NvnTextureView, *mut NvnFormat) -> NvnBoolean;
    pub static nvnTextureViewGetSwizzle: fn(*const NvnTextureView, *mut NvnTextureSwizzle, *mut NvnTextureSwizzle, *mut NvnTextureSwizzle, *mut NvnTextureSwizzle) -> NvnBoolean;
    pub static nvnTextureViewGetDepthStencilMode: fn(*const NvnTextureView, *mut NvnTextureDepthStencilMode) -> NvnBoolean;
    pub static nvnTextureViewGetTarget: fn(*const NvnTextureView, *mut NvnTextureTarget) -> NvnBoolean;
    pub static nvnTextureViewCompare: fn(*const NvnTextureView, *const NvnTextureView) -> NvnBoolean;

    // ── Texture ──
    pub static nvnTextureInitialize: fn(*mut NvnTexture, *const NvnTextureBuilder) -> NvnBoolean;
    pub static nvnTextureGetZCullStorageSize: fn(*const NvnTexture) -> usize;
    pub static nvnTextureFinalize: fn(*mut NvnTexture);
    pub static nvnTextureSetDebugLabel: fn(*mut NvnTexture, *const u8);
    pub static nvnTextureGetStorageClass: fn(*const NvnTexture) -> NvnStorageClass;
    pub static nvnTextureGetViewOffset: fn(*const NvnTexture, *const NvnTextureView) -> isize;
    pub static nvnTextureGetFlags: fn(*const NvnTexture) -> NvnTextureFlags;
    pub static nvnTextureGetTarget: fn(*const NvnTexture) -> NvnTextureTarget;
    pub static nvnTextureGetWidth: fn(*const NvnTexture) -> i32;
    pub static nvnTextureGetHeight: fn(*const NvnTexture) -> i32;
    pub static nvnTextureGetDepth: fn(*const NvnTexture) -> i32;
    pub static nvnTextureGetLevels: fn(*const NvnTexture) -> i32;
    pub static nvnTextureGetFormat: fn(*const NvnTexture) -> NvnFormat;
    pub static nvnTextureGetSamples: fn(*const NvnTexture) -> i32;
    pub static nvnTextureGetSwizzle: fn(*const NvnTexture, *mut NvnTextureSwizzle, *mut NvnTextureSwizzle, *mut NvnTextureSwizzle, *mut NvnTextureSwizzle);
    pub static nvnTextureGetDepthStencilMode: fn(*const NvnTexture) -> NvnTextureDepthStencilMode;
    pub static nvnTextureGetStride: fn(*const NvnTexture) -> isize;
    pub static nvnTextureGetTextureAddress: fn(*const NvnTexture) -> NvnTextureAddress;
    pub static nvnTextureGetSparseTileLayout: fn(*const NvnTexture, *mut NvnTextureSparseTileLayout);
    pub static nvnTextureWriteTexels: fn(*const NvnTexture, *const NvnTextureView, *const NvnCopyRegion, *const core::ffi::c_void);
    pub static nvnTextureWriteTexelsStrided: fn(*const NvnTexture, *const NvnTextureView, *const NvnCopyRegion, *const core::ffi::c_void, isize, isize);
    pub static nvnTextureReadTexels: fn(*const NvnTexture, *const NvnTextureView, *const NvnCopyRegion, *mut core::ffi::c_void);
    pub static nvnTextureReadTexelsStrided: fn(*const NvnTexture, *const NvnTextureView, *const NvnCopyRegion, *mut core::ffi::c_void, isize, isize);
    pub static nvnTextureFlushTexels: fn(*const NvnTexture, *const NvnTextureView, *const NvnCopyRegion);
    pub static nvnTextureInvalidateTexels: fn(*const NvnTexture, *const NvnTextureView, *const NvnCopyRegion);
    pub static nvnTextureGetMemoryPool: fn(*const NvnTexture) -> *const NvnMemoryPool;
    pub static nvnTextureGetMemoryOffset: fn(*const NvnTexture) -> isize;
    pub static nvnTextureGetStorageSize: fn(*const NvnTexture) -> i32;
    pub static nvnTextureCompare: fn(*const NvnTexture, *const NvnTexture) -> NvnBoolean;
    pub static nvnTextureGetDebugID: fn(*const NvnTexture) -> u64;
    // vtable-only
    pub static nvnTextureGetRawStorageClass: fn(*const NvnTexture) -> NvnStorageClass;

    // ── SamplerBuilder ──
    pub static nvnSamplerBuilderSetDevice: fn(*mut NvnSamplerBuilder, *mut NvnDevice);
    pub static nvnSamplerBuilderSetDefaults: fn(*mut NvnSamplerBuilder);
    pub static nvnSamplerBuilderSetMinMagFilter: fn(*mut NvnSamplerBuilder, NvnMinFilter, NvnMagFilter);
    pub static nvnSamplerBuilderSetWrapMode: fn(*mut NvnSamplerBuilder, NvnWrapMode, NvnWrapMode, NvnWrapMode);
    pub static nvnSamplerBuilderSetLodClamp: fn(*mut NvnSamplerBuilder, f32, f32);
    pub static nvnSamplerBuilderSetLodBias: fn(*mut NvnSamplerBuilder, f32);
    pub static nvnSamplerBuilderSetCompare: fn(*mut NvnSamplerBuilder, NvnCompareMode, NvnCompareFunc);
    pub static nvnSamplerBuilderSetBorderColor: fn(*mut NvnSamplerBuilder, *const f32);
    pub static nvnSamplerBuilderSetBorderColori: fn(*mut NvnSamplerBuilder, *const i32);
    pub static nvnSamplerBuilderSetBorderColorui: fn(*mut NvnSamplerBuilder, *const u32);
    pub static nvnSamplerBuilderSetMaxAnisotropy: fn(*mut NvnSamplerBuilder, f32);
    pub static nvnSamplerBuilderSetReductionFilter: fn(*mut NvnSamplerBuilder, NvnSamplerReduction);
    pub static nvnSamplerBuilderSetLodSnap: fn(*mut NvnSamplerBuilder, f32);
    // vtable-only
    pub static nvnSamplerBuilderGetDevice: fn(*const NvnSamplerBuilder) -> *const NvnDevice;
    pub static nvnSamplerBuilderGetMinMagFilter: fn(*const NvnSamplerBuilder, *mut NvnMinFilter, *mut NvnMagFilter);
    pub static nvnSamplerBuilderGetWrapMode: fn(*const NvnSamplerBuilder, *mut NvnWrapMode, *mut NvnWrapMode, *mut NvnWrapMode);
    pub static nvnSamplerBuilderGetLodClamp: fn(*const NvnSamplerBuilder, *mut f32, *mut f32);
    pub static nvnSamplerBuilderGetLodBias: fn(*const NvnSamplerBuilder) -> f32;
    pub static nvnSamplerBuilderGetCompare: fn(*const NvnSamplerBuilder, *mut NvnCompareMode, *mut NvnCompareFunc);
    pub static nvnSamplerBuilderGetBorderColor: fn(*const NvnSamplerBuilder, *mut f32);
    pub static nvnSamplerBuilderGetBorderColori: fn(*const NvnSamplerBuilder, *mut i32);
    pub static nvnSamplerBuilderGetBorderColorui: fn(*const NvnSamplerBuilder, *mut u32);
    pub static nvnSamplerBuilderGetMaxAnisotropy: fn(*const NvnSamplerBuilder) -> f32;
    pub static nvnSamplerBuilderGetReductionFilter: fn(*const NvnSamplerBuilder) -> NvnSamplerReduction;
    pub static nvnSamplerBuilderGetLodSnap: fn(*const NvnSamplerBuilder) -> f32;

    // ── Sampler ──
    pub static nvnSamplerInitialize: fn(*mut NvnSampler, *const NvnSamplerBuilder) -> NvnBoolean;
    pub static nvnSamplerFinalize: fn(*mut NvnSampler);
    pub static nvnSamplerSetDebugLabel: fn(*mut NvnSampler, *const u8);
    pub static nvnSamplerGetMinMagFilter: fn(*const NvnSampler, *mut NvnMinFilter, *mut NvnMagFilter);
    pub static nvnSamplerGetWrapMode: fn(*const NvnSampler, *mut NvnWrapMode, *mut NvnWrapMode, *mut NvnWrapMode);
    pub static nvnSamplerGetLodClamp: fn(*const NvnSampler, *mut f32, *mut f32);
    pub static nvnSamplerGetLodBias: fn(*const NvnSampler) -> f32;
    pub static nvnSamplerGetCompare: fn(*const NvnSampler, *mut NvnCompareMode, *mut NvnCompareFunc);
    pub static nvnSamplerGetBorderColor: fn(*const NvnSampler, *mut f32);
    pub static nvnSamplerGetBorderColori: fn(*const NvnSampler, *mut i32);
    pub static nvnSamplerGetBorderColorui: fn(*const NvnSampler, *mut u32);
    pub static nvnSamplerGetMaxAnisotropy: fn(*const NvnSampler) -> f32;
    pub static nvnSamplerGetReductionFilter: fn(*const NvnSampler) -> NvnSamplerReduction;
    pub static nvnSamplerCompare: fn(*const NvnSampler, *const NvnSampler) -> NvnBoolean;
    pub static nvnSamplerGetDebugID: fn(*const NvnSampler) -> u64;

    // ── Program ──
    pub static nvnProgramInitialize: fn(*mut NvnProgram, *mut NvnDevice) -> NvnBoolean;
    pub static nvnProgramFinalize: fn(*mut NvnProgram);
    pub static nvnProgramSetDebugLabel: fn(*mut NvnProgram, *const u8);
    pub static nvnProgramSetShaders: fn(*mut NvnProgram, i32, *const NvnShaderData) -> NvnBoolean;
    // vtable-only
    pub static nvnProgramSetShadersExt: fn(*mut NvnProgram, i32, *const NvnShaderData) -> NvnBoolean;
    pub static nvnProgramSetSampleShading: fn(*mut NvnProgram, NvnBoolean);
    pub static nvnProgramSetSubroutineLinkage: fn(*mut NvnProgram, i32, *const NvnSubroutineLinkageMapPtr) -> NvnBoolean;

    // ── BlendState ──
    pub static nvnBlendStateSetDefaults: fn(*mut NvnBlendState);
    pub static nvnBlendStateSetBlendTarget: fn(*mut NvnBlendState, i32);
    pub static nvnBlendStateSetBlendFunc: fn(*mut NvnBlendState, NvnBlendFunc, NvnBlendFunc, NvnBlendFunc, NvnBlendFunc);
    pub static nvnBlendStateSetBlendEquation: fn(*mut NvnBlendState, NvnBlendEquation, NvnBlendEquation);
    pub static nvnBlendStateSetAdvancedMode: fn(*mut NvnBlendState, NvnBlendAdvancedMode);
    pub static nvnBlendStateSetAdvancedOverlap: fn(*mut NvnBlendState, NvnBlendAdvancedOverlap);
    pub static nvnBlendStateSetAdvancedPremultipliedSrc: fn(*mut NvnBlendState, NvnBoolean);
    pub static nvnBlendStateSetAdvancedNormalizedDst: fn(*mut NvnBlendState, NvnBoolean);
    pub static nvnBlendStateGetBlendTarget: fn(*const NvnBlendState) -> i32;
    pub static nvnBlendStateGetBlendFunc: fn(*const NvnBlendState, *mut NvnBlendFunc, *mut NvnBlendFunc, *mut NvnBlendFunc, *mut NvnBlendFunc);
    pub static nvnBlendStateGetBlendEquation: fn(*const NvnBlendState, *mut NvnBlendEquation, *mut NvnBlendEquation);
    pub static nvnBlendStateGetAdvancedMode: fn(*const NvnBlendState) -> NvnBlendAdvancedMode;
    pub static nvnBlendStateGetAdvancedOverlap: fn(*const NvnBlendState) -> NvnBlendAdvancedOverlap;
    pub static nvnBlendStateGetAdvancedPremultipliedSrc: fn(*const NvnBlendState) -> NvnBoolean;
    pub static nvnBlendStateGetAdvancedNormalizedDst: fn(*const NvnBlendState) -> NvnBoolean;

    // ── ColorState ──
    pub static nvnColorStateSetDefaults: fn(*mut NvnColorState);
    pub static nvnColorStateSetBlendEnable: fn(*mut NvnColorState, i32, NvnBoolean);
    pub static nvnColorStateSetLogicOp: fn(*mut NvnColorState, NvnLogicOp);
    pub static nvnColorStateSetAlphaTest: fn(*mut NvnColorState, NvnAlphaFunc);
    pub static nvnColorStateGetBlendEnable: fn(*const NvnColorState, i32) -> NvnBoolean;
    pub static nvnColorStateGetLogicOp: fn(*const NvnColorState) -> NvnLogicOp;
    pub static nvnColorStateGetAlphaTest: fn(*const NvnColorState) -> NvnAlphaFunc;

    // ── ChannelMaskState ──
    pub static nvnChannelMaskStateSetDefaults: fn(*mut NvnChannelMaskState);
    pub static nvnChannelMaskStateSetChannelMask: fn(*mut NvnChannelMaskState, i32, NvnBoolean, NvnBoolean, NvnBoolean, NvnBoolean);
    pub static nvnChannelMaskStateGetChannelMask: fn(*const NvnChannelMaskState, i32, *mut NvnBoolean, *mut NvnBoolean, *mut NvnBoolean, *mut NvnBoolean);

    // ── MultisampleState ──
    pub static nvnMultisampleStateSetDefaults: fn(*mut NvnMultisampleState);
    pub static nvnMultisampleStateSetMultisampleEnable: fn(*mut NvnMultisampleState, NvnBoolean);
    pub static nvnMultisampleStateSetSamples: fn(*mut NvnMultisampleState, i32);
    pub static nvnMultisampleStateSetAlphaToCoverageEnable: fn(*mut NvnMultisampleState, NvnBoolean);
    pub static nvnMultisampleStateSetAlphaToCoverageDither: fn(*mut NvnMultisampleState, NvnBoolean);
    pub static nvnMultisampleStateGetMultisampleEnable: fn(*const NvnMultisampleState) -> NvnBoolean;
    pub static nvnMultisampleStateGetSamples: fn(*const NvnMultisampleState) -> i32;
    pub static nvnMultisampleStateGetAlphaToCoverageEnable: fn(*const NvnMultisampleState) -> NvnBoolean;
    pub static nvnMultisampleStateGetAlphaToCoverageDither: fn(*const NvnMultisampleState) -> NvnBoolean;
    pub static nvnMultisampleStateSetRasterSamples: fn(*mut NvnMultisampleState, i32);
    pub static nvnMultisampleStateGetRasterSamples: fn(*mut NvnMultisampleState) -> i32;
    pub static nvnMultisampleStateSetCoverageModulationMode: fn(*mut NvnMultisampleState, NvnCoverageModulationMode);
    pub static nvnMultisampleStateGetCoverageModulationMode: fn(*const NvnMultisampleState) -> NvnCoverageModulationMode;
    pub static nvnMultisampleStateSetCoverageToColorEnable: fn(*mut NvnMultisampleState, NvnBoolean);
    pub static nvnMultisampleStateGetCoverageToColorEnable: fn(*const NvnMultisampleState) -> NvnBoolean;
    pub static nvnMultisampleStateSetCoverageToColorOutput: fn(*mut NvnMultisampleState, i32);
    pub static nvnMultisampleStateGetCoverageToColorOutput: fn(*const NvnMultisampleState) -> i32;
    pub static nvnMultisampleStateSetSampleLocationsEnable: fn(*mut NvnMultisampleState, NvnBoolean);
    pub static nvnMultisampleStateGetSampleLocationsEnable: fn(*const NvnMultisampleState) -> NvnBoolean;
    pub static nvnMultisampleStateGetSampleLocationsGrid: fn(*mut NvnMultisampleState, *mut i32, *mut i32);
    pub static nvnMultisampleStateSetSampleLocationsGridEnable: fn(*mut NvnMultisampleState, NvnBoolean);
    pub static nvnMultisampleStateGetSampleLocationsGridEnable: fn(*const NvnMultisampleState) -> NvnBoolean;
    pub static nvnMultisampleStateSetSampleLocations: fn(*mut NvnMultisampleState, i32, i32, *const f32);

    // ── PolygonState ──
    pub static nvnPolygonStateSetDefaults: fn(*mut NvnPolygonState);
    pub static nvnPolygonStateSetCullFace: fn(*mut NvnPolygonState, NvnFace);
    pub static nvnPolygonStateSetFrontFace: fn(*mut NvnPolygonState, NvnFrontFace);
    pub static nvnPolygonStateSetPolygonMode: fn(*mut NvnPolygonState, NvnPolygonMode);
    pub static nvnPolygonStateSetPolygonOffsetEnables: fn(*mut NvnPolygonState, i32);
    pub static nvnPolygonStateGetCullFace: fn(*const NvnPolygonState) -> NvnFace;
    pub static nvnPolygonStateGetFrontFace: fn(*const NvnPolygonState) -> NvnFrontFace;
    pub static nvnPolygonStateGetPolygonMode: fn(*const NvnPolygonState) -> NvnPolygonMode;
    pub static nvnPolygonStateGetPolygonOffsetEnables: fn(*const NvnPolygonState) -> NvnPolygonOffsetEnable;

    // ── DepthStencilState ──
    pub static nvnDepthStencilStateSetDefaults: fn(*mut NvnDepthStencilState);
    pub static nvnDepthStencilStateSetDepthTestEnable: fn(*mut NvnDepthStencilState, NvnBoolean);
    pub static nvnDepthStencilStateSetDepthWriteEnable: fn(*mut NvnDepthStencilState, NvnBoolean);
    pub static nvnDepthStencilStateSetDepthFunc: fn(*mut NvnDepthStencilState, NvnDepthFunc);
    pub static nvnDepthStencilStateSetStencilTestEnable: fn(*mut NvnDepthStencilState, NvnBoolean);
    pub static nvnDepthStencilStateSetStencilFunc: fn(*mut NvnDepthStencilState, NvnFace, NvnStencilFunc);
    pub static nvnDepthStencilStateSetStencilOp: fn(*mut NvnDepthStencilState, NvnFace, NvnStencilOp, NvnStencilOp, NvnStencilOp);
    pub static nvnDepthStencilStateGetDepthTestEnable: fn(*const NvnDepthStencilState) -> NvnBoolean;
    pub static nvnDepthStencilStateGetDepthWriteEnable: fn(*const NvnDepthStencilState) -> NvnBoolean;
    pub static nvnDepthStencilStateGetDepthFunc: fn(*const NvnDepthStencilState) -> NvnDepthFunc;
    pub static nvnDepthStencilStateGetStencilTestEnable: fn(*const NvnDepthStencilState) -> NvnBoolean;
    pub static nvnDepthStencilStateGetStencilFunc: fn(*const NvnDepthStencilState, NvnFace) -> NvnStencilFunc;
    pub static nvnDepthStencilStateGetStencilOp: fn(*const NvnDepthStencilState, NvnFace, *mut NvnStencilOp, *mut NvnStencilOp, *mut NvnStencilOp);

    // ── VertexAttribState ──
    pub static nvnVertexAttribStateSetDefaults: fn(*mut NvnVertexAttribState);
    pub static nvnVertexAttribStateSetFormat: fn(*mut NvnVertexAttribState, NvnFormat, isize);
    pub static nvnVertexAttribStateSetStreamIndex: fn(*mut NvnVertexAttribState, i32);
    pub static nvnVertexAttribStateGetFormat: fn(*const NvnVertexAttribState, *mut NvnFormat, *mut isize);
    pub static nvnVertexAttribStateGetStreamIndex: fn(*const NvnVertexAttribState) -> i32;

    // ── VertexStreamState ──
    pub static nvnVertexStreamStateSetDefaults: fn(*mut NvnVertexStreamState);
    pub static nvnVertexStreamStateSetStride: fn(*mut NvnVertexStreamState, isize);
    pub static nvnVertexStreamStateSetDivisor: fn(*mut NvnVertexStreamState, i32);
    pub static nvnVertexStreamStateGetStride: fn(*const NvnVertexStreamState) -> isize;
    pub static nvnVertexStreamStateGetDivisor: fn(*const NvnVertexStreamState) -> i32;
}
