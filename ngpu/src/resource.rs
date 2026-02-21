#![allow(unused)]
use crate::*;

gpu_api! {
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


crate::nvn_wrap_void!(texture_builder_set_device(arg0: *mut NvnTextureBuilder, arg1: *mut NvnDevice) => SLOT_NVN_TEXTURE_BUILDER_SET_DEVICE);

crate::nvn_wrap_void!(texture_builder_set_defaults(arg0: *mut NvnTextureBuilder) => SLOT_NVN_TEXTURE_BUILDER_SET_DEFAULTS);

crate::nvn_wrap_void!(texture_builder_set_flags(arg0: *mut NvnTextureBuilder, arg1: i32) => SLOT_NVN_TEXTURE_BUILDER_SET_FLAGS);

crate::nvn_wrap_void!(texture_builder_set_target(arg0: *mut NvnTextureBuilder, arg1: NvnTextureTarget) => SLOT_NVN_TEXTURE_BUILDER_SET_TARGET);

crate::nvn_wrap_void!(texture_builder_set_width(arg0: *mut NvnTextureBuilder, arg1: i32) => SLOT_NVN_TEXTURE_BUILDER_SET_WIDTH);

crate::nvn_wrap_void!(texture_builder_set_height(arg0: *mut NvnTextureBuilder, arg1: i32) => SLOT_NVN_TEXTURE_BUILDER_SET_HEIGHT);

crate::nvn_wrap_void!(texture_builder_set_depth(arg0: *mut NvnTextureBuilder, arg1: i32) => SLOT_NVN_TEXTURE_BUILDER_SET_DEPTH);

crate::nvn_wrap_void!(texture_builder_set_size1_d(arg0: *mut NvnTextureBuilder, arg1: i32) => SLOT_NVN_TEXTURE_BUILDER_SET_SIZE1_D);

crate::nvn_wrap_void!(texture_builder_set_size2_d(arg0: *mut NvnTextureBuilder, arg1: i32, arg2: i32) => SLOT_NVN_TEXTURE_BUILDER_SET_SIZE2_D);

crate::nvn_wrap_void!(texture_builder_set_size3_d(arg0: *mut NvnTextureBuilder, arg1: i32, arg2: i32, arg3: i32) => SLOT_NVN_TEXTURE_BUILDER_SET_SIZE3_D);

crate::nvn_wrap_void!(texture_builder_set_levels(arg0: *mut NvnTextureBuilder, arg1: i32) => SLOT_NVN_TEXTURE_BUILDER_SET_LEVELS);

crate::nvn_wrap_void!(texture_builder_set_format(arg0: *mut NvnTextureBuilder, arg1: NvnFormat) => SLOT_NVN_TEXTURE_BUILDER_SET_FORMAT);

crate::nvn_wrap_void!(texture_builder_set_samples(arg0: *mut NvnTextureBuilder, arg1: i32) => SLOT_NVN_TEXTURE_BUILDER_SET_SAMPLES);

crate::nvn_wrap_void!(texture_builder_set_swizzle(arg0: *mut NvnTextureBuilder, arg1: NvnTextureSwizzle, arg2: NvnTextureSwizzle, arg3: NvnTextureSwizzle, arg4: NvnTextureSwizzle) => SLOT_NVN_TEXTURE_BUILDER_SET_SWIZZLE);

crate::nvn_wrap_void!(texture_builder_set_depth_stencil_mode(arg0: *mut NvnTextureBuilder, arg1: NvnTextureDepthStencilMode) => SLOT_NVN_TEXTURE_BUILDER_SET_DEPTH_STENCIL_MODE);

crate::nvn_wrap_ret!(texture_builder_get_storage_size(arg0: *const NvnTextureBuilder) -> usize => SLOT_NVN_TEXTURE_BUILDER_GET_STORAGE_SIZE);

crate::nvn_wrap_ret!(texture_builder_get_storage_alignment(arg0: *const NvnTextureBuilder) -> usize => SLOT_NVN_TEXTURE_BUILDER_GET_STORAGE_ALIGNMENT);

crate::nvn_wrap_void!(texture_builder_set_storage(arg0: *mut NvnTextureBuilder, arg1: *mut NvnMemoryPool, arg2: isize) => SLOT_NVN_TEXTURE_BUILDER_SET_STORAGE);

crate::nvn_wrap_void!(texture_builder_set_packaged_texture_data(arg0: *mut NvnTextureBuilder, arg1: *const core::ffi::c_void) => SLOT_NVN_TEXTURE_BUILDER_SET_PACKAGED_TEXTURE_DATA);

crate::nvn_wrap_void!(texture_builder_set_packaged_texture_layout(arg0: *mut NvnTextureBuilder, arg1: *const NvnPackagedTextureLayout) => SLOT_NVN_TEXTURE_BUILDER_SET_PACKAGED_TEXTURE_LAYOUT);

crate::nvn_wrap_void!(texture_builder_set_stride(arg0: *mut NvnTextureBuilder, arg1: isize) => SLOT_NVN_TEXTURE_BUILDER_SET_STRIDE);

crate::nvn_wrap_void!(texture_builder_set_gl_texture_name(arg0: *mut NvnTextureBuilder, arg1: u32) => SLOT_NVN_TEXTURE_BUILDER_SET_GL_TEXTURE_NAME);

crate::nvn_wrap_ret!(texture_builder_get_storage_class(arg0: *const NvnTextureBuilder) -> NvnStorageClass => SLOT_NVN_TEXTURE_BUILDER_GET_STORAGE_CLASS);

crate::nvn_wrap_ret!(texture_builder_get_device(arg0: *const NvnTextureBuilder) -> *const NvnDevice => SLOT_NVN_TEXTURE_BUILDER_GET_DEVICE);

crate::nvn_wrap_ret!(texture_builder_get_flags(arg0: *const NvnTextureBuilder) -> NvnTextureFlags => SLOT_NVN_TEXTURE_BUILDER_GET_FLAGS);

crate::nvn_wrap_ret!(texture_builder_get_target(arg0: *const NvnTextureBuilder) -> NvnTextureTarget => SLOT_NVN_TEXTURE_BUILDER_GET_TARGET);

crate::nvn_wrap_ret!(texture_builder_get_width(arg0: *const NvnTextureBuilder) -> i32 => SLOT_NVN_TEXTURE_BUILDER_GET_WIDTH);

crate::nvn_wrap_ret!(texture_builder_get_height(arg0: *const NvnTextureBuilder) -> i32 => SLOT_NVN_TEXTURE_BUILDER_GET_HEIGHT);

crate::nvn_wrap_ret!(texture_builder_get_depth(arg0: *const NvnTextureBuilder) -> i32 => SLOT_NVN_TEXTURE_BUILDER_GET_DEPTH);

crate::nvn_wrap_ret!(texture_builder_get_levels(arg0: *const NvnTextureBuilder) -> i32 => SLOT_NVN_TEXTURE_BUILDER_GET_LEVELS);

crate::nvn_wrap_ret!(texture_builder_get_format(arg0: *const NvnTextureBuilder) -> NvnFormat => SLOT_NVN_TEXTURE_BUILDER_GET_FORMAT);

crate::nvn_wrap_ret!(texture_builder_get_samples(arg0: *const NvnTextureBuilder) -> i32 => SLOT_NVN_TEXTURE_BUILDER_GET_SAMPLES);

crate::nvn_wrap_void!(texture_builder_get_swizzle(arg0: *const NvnTextureBuilder, arg1: *mut NvnTextureSwizzle, arg2: *mut NvnTextureSwizzle, arg3: *mut NvnTextureSwizzle, arg4: *mut NvnTextureSwizzle) => SLOT_NVN_TEXTURE_BUILDER_GET_SWIZZLE);

crate::nvn_wrap_ret!(texture_builder_get_depth_stencil_mode(arg0: *const NvnTextureBuilder) -> NvnTextureDepthStencilMode => SLOT_NVN_TEXTURE_BUILDER_GET_DEPTH_STENCIL_MODE);

crate::nvn_wrap_ret!(texture_builder_get_packaged_texture_data(arg0: *const NvnTextureBuilder) -> *const core::ffi::c_void => SLOT_NVN_TEXTURE_BUILDER_GET_PACKAGED_TEXTURE_DATA);

crate::nvn_wrap_ret!(texture_builder_get_packaged_texture_layout(arg0: *const NvnTextureBuilder) -> *const NvnPackagedTextureLayout => SLOT_NVN_TEXTURE_BUILDER_GET_PACKAGED_TEXTURE_LAYOUT);

crate::nvn_wrap_ret!(texture_builder_get_stride(arg0: *const NvnTextureBuilder) -> isize => SLOT_NVN_TEXTURE_BUILDER_GET_STRIDE);

crate::nvn_wrap_void!(texture_builder_get_sparse_tile_layout(arg0: *const NvnTextureBuilder, arg1: *mut NvnTextureSparseTileLayout) => SLOT_NVN_TEXTURE_BUILDER_GET_SPARSE_TILE_LAYOUT);

crate::nvn_wrap_ret!(texture_builder_get_gl_texture_name(arg0: *const NvnTextureBuilder) -> u32 => SLOT_NVN_TEXTURE_BUILDER_GET_GL_TEXTURE_NAME);

crate::nvn_wrap_ret!(texture_builder_get_z_cull_storage_size(arg0: *const NvnTextureBuilder) -> usize => SLOT_NVN_TEXTURE_BUILDER_GET_Z_CULL_STORAGE_SIZE);

crate::nvn_wrap_ret!(texture_builder_get_memory_pool(arg0: *const NvnTextureBuilder) -> *const NvnMemoryPool => SLOT_NVN_TEXTURE_BUILDER_GET_MEMORY_POOL);

crate::nvn_wrap_ret!(texture_builder_get_memory_offset(arg0: *const NvnTextureBuilder) -> isize => SLOT_NVN_TEXTURE_BUILDER_GET_MEMORY_OFFSET);

crate::nvn_wrap_ret!(texture_builder_get_raw_storage_class(arg0: *const NvnTextureBuilder) -> NvnStorageClass => SLOT_NVN_TEXTURE_BUILDER_GET_RAW_STORAGE_CLASS);

crate::nvn_wrap_void!(texture_view_set_defaults(arg0: *mut NvnTextureView) => SLOT_NVN_TEXTURE_VIEW_SET_DEFAULTS);

crate::nvn_wrap_void!(texture_view_set_levels(arg0: *mut NvnTextureView, arg1: i32, arg2: i32) => SLOT_NVN_TEXTURE_VIEW_SET_LEVELS);

crate::nvn_wrap_void!(texture_view_set_layers(arg0: *mut NvnTextureView, arg1: i32, arg2: i32) => SLOT_NVN_TEXTURE_VIEW_SET_LAYERS);

crate::nvn_wrap_void!(texture_view_set_format(arg0: *mut NvnTextureView, arg1: NvnFormat) => SLOT_NVN_TEXTURE_VIEW_SET_FORMAT);

crate::nvn_wrap_void!(texture_view_set_swizzle(arg0: *mut NvnTextureView, arg1: NvnTextureSwizzle, arg2: NvnTextureSwizzle, arg3: NvnTextureSwizzle, arg4: NvnTextureSwizzle) => SLOT_NVN_TEXTURE_VIEW_SET_SWIZZLE);

crate::nvn_wrap_void!(texture_view_set_depth_stencil_mode(arg0: *mut NvnTextureView, arg1: NvnTextureDepthStencilMode) => SLOT_NVN_TEXTURE_VIEW_SET_DEPTH_STENCIL_MODE);

crate::nvn_wrap_void!(texture_view_set_target(arg0: *mut NvnTextureView, arg1: NvnTextureTarget) => SLOT_NVN_TEXTURE_VIEW_SET_TARGET);

crate::nvn_wrap_ret!(texture_view_get_levels(arg0: *const NvnTextureView, arg1: *mut i32, arg2: *mut i32) -> NvnBoolean => SLOT_NVN_TEXTURE_VIEW_GET_LEVELS);

crate::nvn_wrap_ret!(texture_view_get_layers(arg0: *const NvnTextureView, arg1: *mut i32, arg2: *mut i32) -> NvnBoolean => SLOT_NVN_TEXTURE_VIEW_GET_LAYERS);

crate::nvn_wrap_ret!(texture_view_get_format(arg0: *const NvnTextureView, arg1: *mut NvnFormat) -> NvnBoolean => SLOT_NVN_TEXTURE_VIEW_GET_FORMAT);

crate::nvn_wrap_ret!(texture_view_get_swizzle(arg0: *const NvnTextureView, arg1: *mut NvnTextureSwizzle, arg2: *mut NvnTextureSwizzle, arg3: *mut NvnTextureSwizzle, arg4: *mut NvnTextureSwizzle) -> NvnBoolean => SLOT_NVN_TEXTURE_VIEW_GET_SWIZZLE);

crate::nvn_wrap_ret!(texture_view_get_depth_stencil_mode(arg0: *const NvnTextureView, arg1: *mut NvnTextureDepthStencilMode) -> NvnBoolean => SLOT_NVN_TEXTURE_VIEW_GET_DEPTH_STENCIL_MODE);

crate::nvn_wrap_ret!(texture_view_get_target(arg0: *const NvnTextureView, arg1: *mut NvnTextureTarget) -> NvnBoolean => SLOT_NVN_TEXTURE_VIEW_GET_TARGET);

crate::nvn_wrap_ret!(texture_view_compare(arg0: *const NvnTextureView, arg1: *const NvnTextureView) -> NvnBoolean => SLOT_NVN_TEXTURE_VIEW_COMPARE);

crate::nvn_wrap_ret!(texture_initialize(arg0: *mut NvnTexture, arg1: *const NvnTextureBuilder) -> NvnBoolean => SLOT_NVN_TEXTURE_INITIALIZE);

crate::nvn_wrap_ret!(texture_get_z_cull_storage_size(arg0: *const NvnTexture) -> usize => SLOT_NVN_TEXTURE_GET_Z_CULL_STORAGE_SIZE);

crate::nvn_wrap_void!(texture_finalize(arg0: *mut NvnTexture) => SLOT_NVN_TEXTURE_FINALIZE);

crate::nvn_wrap_void!(texture_set_debug_label(arg0: *mut NvnTexture, arg1: *const u8) => SLOT_NVN_TEXTURE_SET_DEBUG_LABEL);

crate::nvn_wrap_ret!(texture_get_storage_class(arg0: *const NvnTexture) -> NvnStorageClass => SLOT_NVN_TEXTURE_GET_STORAGE_CLASS);

crate::nvn_wrap_ret!(texture_get_view_offset(arg0: *const NvnTexture, arg1: *const NvnTextureView) -> isize => SLOT_NVN_TEXTURE_GET_VIEW_OFFSET);

crate::nvn_wrap_ret!(texture_get_flags(arg0: *const NvnTexture) -> NvnTextureFlags => SLOT_NVN_TEXTURE_GET_FLAGS);

crate::nvn_wrap_ret!(texture_get_target(arg0: *const NvnTexture) -> NvnTextureTarget => SLOT_NVN_TEXTURE_GET_TARGET);

crate::nvn_wrap_ret!(texture_get_width(arg0: *const NvnTexture) -> i32 => SLOT_NVN_TEXTURE_GET_WIDTH);

crate::nvn_wrap_ret!(texture_get_height(arg0: *const NvnTexture) -> i32 => SLOT_NVN_TEXTURE_GET_HEIGHT);

crate::nvn_wrap_ret!(texture_get_depth(arg0: *const NvnTexture) -> i32 => SLOT_NVN_TEXTURE_GET_DEPTH);

crate::nvn_wrap_ret!(texture_get_levels(arg0: *const NvnTexture) -> i32 => SLOT_NVN_TEXTURE_GET_LEVELS);

crate::nvn_wrap_ret!(texture_get_format(arg0: *const NvnTexture) -> NvnFormat => SLOT_NVN_TEXTURE_GET_FORMAT);

crate::nvn_wrap_ret!(texture_get_samples(arg0: *const NvnTexture) -> i32 => SLOT_NVN_TEXTURE_GET_SAMPLES);

crate::nvn_wrap_void!(texture_get_swizzle(arg0: *const NvnTexture, arg1: *mut NvnTextureSwizzle, arg2: *mut NvnTextureSwizzle, arg3: *mut NvnTextureSwizzle, arg4: *mut NvnTextureSwizzle) => SLOT_NVN_TEXTURE_GET_SWIZZLE);

crate::nvn_wrap_ret!(texture_get_depth_stencil_mode(arg0: *const NvnTexture) -> NvnTextureDepthStencilMode => SLOT_NVN_TEXTURE_GET_DEPTH_STENCIL_MODE);

crate::nvn_wrap_ret!(texture_get_stride(arg0: *const NvnTexture) -> isize => SLOT_NVN_TEXTURE_GET_STRIDE);

crate::nvn_wrap_ret!(texture_get_texture_address(arg0: *const NvnTexture) -> NvnTextureAddress => SLOT_NVN_TEXTURE_GET_TEXTURE_ADDRESS);

crate::nvn_wrap_void!(texture_get_sparse_tile_layout(arg0: *const NvnTexture, arg1: *mut NvnTextureSparseTileLayout) => SLOT_NVN_TEXTURE_GET_SPARSE_TILE_LAYOUT);

crate::nvn_wrap_void!(texture_write_texels(arg0: *const NvnTexture, arg1: *const NvnTextureView, arg2: *const NvnCopyRegion, arg3: *const core::ffi::c_void) => SLOT_NVN_TEXTURE_WRITE_TEXELS);

crate::nvn_wrap_void!(texture_write_texels_strided(arg0: *const NvnTexture, arg1: *const NvnTextureView, arg2: *const NvnCopyRegion, arg3: *const core::ffi::c_void, arg4: isize, arg5: isize) => SLOT_NVN_TEXTURE_WRITE_TEXELS_STRIDED);

crate::nvn_wrap_void!(texture_read_texels(arg0: *const NvnTexture, arg1: *const NvnTextureView, arg2: *const NvnCopyRegion, arg3: *mut core::ffi::c_void) => SLOT_NVN_TEXTURE_READ_TEXELS);

crate::nvn_wrap_void!(texture_read_texels_strided(arg0: *const NvnTexture, arg1: *const NvnTextureView, arg2: *const NvnCopyRegion, arg3: *mut core::ffi::c_void, arg4: isize, arg5: isize) => SLOT_NVN_TEXTURE_READ_TEXELS_STRIDED);

crate::nvn_wrap_void!(texture_flush_texels(arg0: *const NvnTexture, arg1: *const NvnTextureView, arg2: *const NvnCopyRegion) => SLOT_NVN_TEXTURE_FLUSH_TEXELS);

crate::nvn_wrap_void!(texture_invalidate_texels(arg0: *const NvnTexture, arg1: *const NvnTextureView, arg2: *const NvnCopyRegion) => SLOT_NVN_TEXTURE_INVALIDATE_TEXELS);

crate::nvn_wrap_ret!(texture_get_memory_pool(arg0: *const NvnTexture) -> *const NvnMemoryPool => SLOT_NVN_TEXTURE_GET_MEMORY_POOL);

crate::nvn_wrap_ret!(texture_get_memory_offset(arg0: *const NvnTexture) -> isize => SLOT_NVN_TEXTURE_GET_MEMORY_OFFSET);

crate::nvn_wrap_ret!(texture_get_storage_size(arg0: *const NvnTexture) -> i32 => SLOT_NVN_TEXTURE_GET_STORAGE_SIZE);

crate::nvn_wrap_ret!(texture_compare(arg0: *const NvnTexture, arg1: *const NvnTexture) -> NvnBoolean => SLOT_NVN_TEXTURE_COMPARE);

crate::nvn_wrap_ret!(texture_get_debug_id(arg0: *const NvnTexture) -> u64 => SLOT_NVN_TEXTURE_GET_DEBUG_ID);

crate::nvn_wrap_ret!(texture_get_raw_storage_class(arg0: *const NvnTexture) -> NvnStorageClass => SLOT_NVN_TEXTURE_GET_RAW_STORAGE_CLASS);

crate::nvn_wrap_void!(sampler_builder_set_device(arg0: *mut NvnSamplerBuilder, arg1: *mut NvnDevice) => SLOT_NVN_SAMPLER_BUILDER_SET_DEVICE);

crate::nvn_wrap_void!(sampler_builder_set_defaults(arg0: *mut NvnSamplerBuilder) => SLOT_NVN_SAMPLER_BUILDER_SET_DEFAULTS);

crate::nvn_wrap_void!(sampler_builder_set_min_mag_filter(arg0: *mut NvnSamplerBuilder, arg1: NvnMinFilter, arg2: NvnMagFilter) => SLOT_NVN_SAMPLER_BUILDER_SET_MIN_MAG_FILTER);

crate::nvn_wrap_void!(sampler_builder_set_wrap_mode(arg0: *mut NvnSamplerBuilder, arg1: NvnWrapMode, arg2: NvnWrapMode, arg3: NvnWrapMode) => SLOT_NVN_SAMPLER_BUILDER_SET_WRAP_MODE);

crate::nvn_wrap_void!(sampler_builder_set_lod_clamp(arg0: *mut NvnSamplerBuilder, arg1: f32, arg2: f32) => SLOT_NVN_SAMPLER_BUILDER_SET_LOD_CLAMP);

crate::nvn_wrap_void!(sampler_builder_set_lod_bias(arg0: *mut NvnSamplerBuilder, arg1: f32) => SLOT_NVN_SAMPLER_BUILDER_SET_LOD_BIAS);

crate::nvn_wrap_void!(sampler_builder_set_compare(arg0: *mut NvnSamplerBuilder, arg1: NvnCompareMode, arg2: NvnCompareFunc) => SLOT_NVN_SAMPLER_BUILDER_SET_COMPARE);

crate::nvn_wrap_void!(sampler_builder_set_border_color(arg0: *mut NvnSamplerBuilder, arg1: *const f32) => SLOT_NVN_SAMPLER_BUILDER_SET_BORDER_COLOR);

crate::nvn_wrap_void!(sampler_builder_set_border_colori(arg0: *mut NvnSamplerBuilder, arg1: *const i32) => SLOT_NVN_SAMPLER_BUILDER_SET_BORDER_COLORI);

crate::nvn_wrap_void!(sampler_builder_set_border_colorui(arg0: *mut NvnSamplerBuilder, arg1: *const u32) => SLOT_NVN_SAMPLER_BUILDER_SET_BORDER_COLORUI);

crate::nvn_wrap_void!(sampler_builder_set_max_anisotropy(arg0: *mut NvnSamplerBuilder, arg1: f32) => SLOT_NVN_SAMPLER_BUILDER_SET_MAX_ANISOTROPY);

crate::nvn_wrap_void!(sampler_builder_set_reduction_filter(arg0: *mut NvnSamplerBuilder, arg1: NvnSamplerReduction) => SLOT_NVN_SAMPLER_BUILDER_SET_REDUCTION_FILTER);

crate::nvn_wrap_void!(sampler_builder_set_lod_snap(arg0: *mut NvnSamplerBuilder, arg1: f32) => SLOT_NVN_SAMPLER_BUILDER_SET_LOD_SNAP);

crate::nvn_wrap_ret!(sampler_builder_get_device(arg0: *const NvnSamplerBuilder) -> *const NvnDevice => SLOT_NVN_SAMPLER_BUILDER_GET_DEVICE);

crate::nvn_wrap_void!(sampler_builder_get_min_mag_filter(arg0: *const NvnSamplerBuilder, arg1: *mut NvnMinFilter, arg2: *mut NvnMagFilter) => SLOT_NVN_SAMPLER_BUILDER_GET_MIN_MAG_FILTER);

crate::nvn_wrap_void!(sampler_builder_get_wrap_mode(arg0: *const NvnSamplerBuilder, arg1: *mut NvnWrapMode, arg2: *mut NvnWrapMode, arg3: *mut NvnWrapMode) => SLOT_NVN_SAMPLER_BUILDER_GET_WRAP_MODE);

crate::nvn_wrap_void!(sampler_builder_get_lod_clamp(arg0: *const NvnSamplerBuilder, arg1: *mut f32, arg2: *mut f32) => SLOT_NVN_SAMPLER_BUILDER_GET_LOD_CLAMP);

crate::nvn_wrap_ret!(sampler_builder_get_lod_bias(arg0: *const NvnSamplerBuilder) -> f32 => SLOT_NVN_SAMPLER_BUILDER_GET_LOD_BIAS);

crate::nvn_wrap_void!(sampler_builder_get_compare(arg0: *const NvnSamplerBuilder, arg1: *mut NvnCompareMode, arg2: *mut NvnCompareFunc) => SLOT_NVN_SAMPLER_BUILDER_GET_COMPARE);

crate::nvn_wrap_void!(sampler_builder_get_border_color(arg0: *const NvnSamplerBuilder, arg1: *mut f32) => SLOT_NVN_SAMPLER_BUILDER_GET_BORDER_COLOR);

crate::nvn_wrap_void!(sampler_builder_get_border_colori(arg0: *const NvnSamplerBuilder, arg1: *mut i32) => SLOT_NVN_SAMPLER_BUILDER_GET_BORDER_COLORI);

crate::nvn_wrap_void!(sampler_builder_get_border_colorui(arg0: *const NvnSamplerBuilder, arg1: *mut u32) => SLOT_NVN_SAMPLER_BUILDER_GET_BORDER_COLORUI);

crate::nvn_wrap_ret!(sampler_builder_get_max_anisotropy(arg0: *const NvnSamplerBuilder) -> f32 => SLOT_NVN_SAMPLER_BUILDER_GET_MAX_ANISOTROPY);

crate::nvn_wrap_ret!(sampler_builder_get_reduction_filter(arg0: *const NvnSamplerBuilder) -> NvnSamplerReduction => SLOT_NVN_SAMPLER_BUILDER_GET_REDUCTION_FILTER);

crate::nvn_wrap_ret!(sampler_builder_get_lod_snap(arg0: *const NvnSamplerBuilder) -> f32 => SLOT_NVN_SAMPLER_BUILDER_GET_LOD_SNAP);

crate::nvn_wrap_ret!(sampler_initialize(arg0: *mut NvnSampler, arg1: *const NvnSamplerBuilder) -> NvnBoolean => SLOT_NVN_SAMPLER_INITIALIZE);

crate::nvn_wrap_void!(sampler_finalize(arg0: *mut NvnSampler) => SLOT_NVN_SAMPLER_FINALIZE);

crate::nvn_wrap_void!(sampler_set_debug_label(arg0: *mut NvnSampler, arg1: *const u8) => SLOT_NVN_SAMPLER_SET_DEBUG_LABEL);

crate::nvn_wrap_void!(sampler_get_min_mag_filter(arg0: *const NvnSampler, arg1: *mut NvnMinFilter, arg2: *mut NvnMagFilter) => SLOT_NVN_SAMPLER_GET_MIN_MAG_FILTER);

crate::nvn_wrap_void!(sampler_get_wrap_mode(arg0: *const NvnSampler, arg1: *mut NvnWrapMode, arg2: *mut NvnWrapMode, arg3: *mut NvnWrapMode) => SLOT_NVN_SAMPLER_GET_WRAP_MODE);

crate::nvn_wrap_void!(sampler_get_lod_clamp(arg0: *const NvnSampler, arg1: *mut f32, arg2: *mut f32) => SLOT_NVN_SAMPLER_GET_LOD_CLAMP);

crate::nvn_wrap_ret!(sampler_get_lod_bias(arg0: *const NvnSampler) -> f32 => SLOT_NVN_SAMPLER_GET_LOD_BIAS);

crate::nvn_wrap_void!(sampler_get_compare(arg0: *const NvnSampler, arg1: *mut NvnCompareMode, arg2: *mut NvnCompareFunc) => SLOT_NVN_SAMPLER_GET_COMPARE);

crate::nvn_wrap_void!(sampler_get_border_color(arg0: *const NvnSampler, arg1: *mut f32) => SLOT_NVN_SAMPLER_GET_BORDER_COLOR);

crate::nvn_wrap_void!(sampler_get_border_colori(arg0: *const NvnSampler, arg1: *mut i32) => SLOT_NVN_SAMPLER_GET_BORDER_COLORI);

crate::nvn_wrap_void!(sampler_get_border_colorui(arg0: *const NvnSampler, arg1: *mut u32) => SLOT_NVN_SAMPLER_GET_BORDER_COLORUI);

crate::nvn_wrap_ret!(sampler_get_max_anisotropy(arg0: *const NvnSampler) -> f32 => SLOT_NVN_SAMPLER_GET_MAX_ANISOTROPY);

crate::nvn_wrap_ret!(sampler_get_reduction_filter(arg0: *const NvnSampler) -> NvnSamplerReduction => SLOT_NVN_SAMPLER_GET_REDUCTION_FILTER);

crate::nvn_wrap_ret!(sampler_compare(arg0: *const NvnSampler, arg1: *const NvnSampler) -> NvnBoolean => SLOT_NVN_SAMPLER_COMPARE);

crate::nvn_wrap_ret!(sampler_get_debug_id(arg0: *const NvnSampler) -> u64 => SLOT_NVN_SAMPLER_GET_DEBUG_ID);

crate::nvn_wrap_ret!(program_initialize(arg0: *mut NvnProgram, arg1: *mut NvnDevice) -> NvnBoolean => SLOT_NVN_PROGRAM_INITIALIZE);

crate::nvn_wrap_void!(program_finalize(arg0: *mut NvnProgram) => SLOT_NVN_PROGRAM_FINALIZE);

crate::nvn_wrap_void!(program_set_debug_label(arg0: *mut NvnProgram, arg1: *const u8) => SLOT_NVN_PROGRAM_SET_DEBUG_LABEL);

crate::nvn_wrap_ret!(program_set_shaders(arg0: *mut NvnProgram, arg1: i32, arg2: *const NvnShaderData) -> NvnBoolean => SLOT_NVN_PROGRAM_SET_SHADERS);

crate::nvn_wrap_ret!(program_set_shaders_ext(arg0: *mut NvnProgram, arg1: i32, arg2: *const NvnShaderData) -> NvnBoolean => SLOT_NVN_PROGRAM_SET_SHADERS_EXT);

crate::nvn_wrap_void!(program_set_sample_shading(arg0: *mut NvnProgram, arg1: NvnBoolean) => SLOT_NVN_PROGRAM_SET_SAMPLE_SHADING);

crate::nvn_wrap_ret!(program_set_subroutine_linkage(arg0: *mut NvnProgram, arg1: i32, arg2: *const NvnSubroutineLinkageMapPtr) -> NvnBoolean => SLOT_NVN_PROGRAM_SET_SUBROUTINE_LINKAGE);

crate::nvn_wrap_void!(blend_state_set_defaults(arg0: *mut NvnBlendState) => SLOT_NVN_BLEND_STATE_SET_DEFAULTS);

crate::nvn_wrap_void!(blend_state_set_blend_target(arg0: *mut NvnBlendState, arg1: i32) => SLOT_NVN_BLEND_STATE_SET_BLEND_TARGET);

crate::nvn_wrap_void!(blend_state_set_blend_func(arg0: *mut NvnBlendState, arg1: NvnBlendFunc, arg2: NvnBlendFunc, arg3: NvnBlendFunc, arg4: NvnBlendFunc) => SLOT_NVN_BLEND_STATE_SET_BLEND_FUNC);

crate::nvn_wrap_void!(blend_state_set_blend_equation(arg0: *mut NvnBlendState, arg1: NvnBlendEquation, arg2: NvnBlendEquation) => SLOT_NVN_BLEND_STATE_SET_BLEND_EQUATION);

crate::nvn_wrap_void!(blend_state_set_advanced_mode(arg0: *mut NvnBlendState, arg1: NvnBlendAdvancedMode) => SLOT_NVN_BLEND_STATE_SET_ADVANCED_MODE);

crate::nvn_wrap_void!(blend_state_set_advanced_overlap(arg0: *mut NvnBlendState, arg1: NvnBlendAdvancedOverlap) => SLOT_NVN_BLEND_STATE_SET_ADVANCED_OVERLAP);

crate::nvn_wrap_void!(blend_state_set_advanced_premultiplied_src(arg0: *mut NvnBlendState, arg1: NvnBoolean) => SLOT_NVN_BLEND_STATE_SET_ADVANCED_PREMULTIPLIED_SRC);

crate::nvn_wrap_void!(blend_state_set_advanced_normalized_dst(arg0: *mut NvnBlendState, arg1: NvnBoolean) => SLOT_NVN_BLEND_STATE_SET_ADVANCED_NORMALIZED_DST);

crate::nvn_wrap_ret!(blend_state_get_blend_target(arg0: *const NvnBlendState) -> i32 => SLOT_NVN_BLEND_STATE_GET_BLEND_TARGET);

crate::nvn_wrap_void!(blend_state_get_blend_func(arg0: *const NvnBlendState, arg1: *mut NvnBlendFunc, arg2: *mut NvnBlendFunc, arg3: *mut NvnBlendFunc, arg4: *mut NvnBlendFunc) => SLOT_NVN_BLEND_STATE_GET_BLEND_FUNC);

crate::nvn_wrap_void!(blend_state_get_blend_equation(arg0: *const NvnBlendState, arg1: *mut NvnBlendEquation, arg2: *mut NvnBlendEquation) => SLOT_NVN_BLEND_STATE_GET_BLEND_EQUATION);

crate::nvn_wrap_ret!(blend_state_get_advanced_mode(arg0: *const NvnBlendState) -> NvnBlendAdvancedMode => SLOT_NVN_BLEND_STATE_GET_ADVANCED_MODE);

crate::nvn_wrap_ret!(blend_state_get_advanced_overlap(arg0: *const NvnBlendState) -> NvnBlendAdvancedOverlap => SLOT_NVN_BLEND_STATE_GET_ADVANCED_OVERLAP);

crate::nvn_wrap_ret!(blend_state_get_advanced_premultiplied_src(arg0: *const NvnBlendState) -> NvnBoolean => SLOT_NVN_BLEND_STATE_GET_ADVANCED_PREMULTIPLIED_SRC);

crate::nvn_wrap_ret!(blend_state_get_advanced_normalized_dst(arg0: *const NvnBlendState) -> NvnBoolean => SLOT_NVN_BLEND_STATE_GET_ADVANCED_NORMALIZED_DST);

crate::nvn_wrap_void!(color_state_set_defaults(arg0: *mut NvnColorState) => SLOT_NVN_COLOR_STATE_SET_DEFAULTS);

crate::nvn_wrap_void!(color_state_set_blend_enable(arg0: *mut NvnColorState, arg1: i32, arg2: NvnBoolean) => SLOT_NVN_COLOR_STATE_SET_BLEND_ENABLE);

crate::nvn_wrap_void!(color_state_set_logic_op(arg0: *mut NvnColorState, arg1: NvnLogicOp) => SLOT_NVN_COLOR_STATE_SET_LOGIC_OP);

crate::nvn_wrap_void!(color_state_set_alpha_test(arg0: *mut NvnColorState, arg1: NvnAlphaFunc) => SLOT_NVN_COLOR_STATE_SET_ALPHA_TEST);

crate::nvn_wrap_ret!(color_state_get_blend_enable(arg0: *const NvnColorState, arg1: i32) -> NvnBoolean => SLOT_NVN_COLOR_STATE_GET_BLEND_ENABLE);

crate::nvn_wrap_ret!(color_state_get_logic_op(arg0: *const NvnColorState) -> NvnLogicOp => SLOT_NVN_COLOR_STATE_GET_LOGIC_OP);

crate::nvn_wrap_ret!(color_state_get_alpha_test(arg0: *const NvnColorState) -> NvnAlphaFunc => SLOT_NVN_COLOR_STATE_GET_ALPHA_TEST);

crate::nvn_wrap_void!(channel_mask_state_set_defaults(arg0: *mut NvnChannelMaskState) => SLOT_NVN_CHANNEL_MASK_STATE_SET_DEFAULTS);

crate::nvn_wrap_void!(channel_mask_state_set_channel_mask(arg0: *mut NvnChannelMaskState, arg1: i32, arg2: NvnBoolean, arg3: NvnBoolean, arg4: NvnBoolean, arg5: NvnBoolean) => SLOT_NVN_CHANNEL_MASK_STATE_SET_CHANNEL_MASK);

crate::nvn_wrap_void!(channel_mask_state_get_channel_mask(arg0: *const NvnChannelMaskState, arg1: i32, arg2: *mut NvnBoolean, arg3: *mut NvnBoolean, arg4: *mut NvnBoolean, arg5: *mut NvnBoolean) => SLOT_NVN_CHANNEL_MASK_STATE_GET_CHANNEL_MASK);

crate::nvn_wrap_void!(multisample_state_set_defaults(arg0: *mut NvnMultisampleState) => SLOT_NVN_MULTISAMPLE_STATE_SET_DEFAULTS);

crate::nvn_wrap_void!(multisample_state_set_multisample_enable(arg0: *mut NvnMultisampleState, arg1: NvnBoolean) => SLOT_NVN_MULTISAMPLE_STATE_SET_MULTISAMPLE_ENABLE);

crate::nvn_wrap_void!(multisample_state_set_samples(arg0: *mut NvnMultisampleState, arg1: i32) => SLOT_NVN_MULTISAMPLE_STATE_SET_SAMPLES);

crate::nvn_wrap_void!(multisample_state_set_alpha_to_coverage_enable(arg0: *mut NvnMultisampleState, arg1: NvnBoolean) => SLOT_NVN_MULTISAMPLE_STATE_SET_ALPHA_TO_COVERAGE_ENABLE);

crate::nvn_wrap_void!(multisample_state_set_alpha_to_coverage_dither(arg0: *mut NvnMultisampleState, arg1: NvnBoolean) => SLOT_NVN_MULTISAMPLE_STATE_SET_ALPHA_TO_COVERAGE_DITHER);

crate::nvn_wrap_ret!(multisample_state_get_multisample_enable(arg0: *const NvnMultisampleState) -> NvnBoolean => SLOT_NVN_MULTISAMPLE_STATE_GET_MULTISAMPLE_ENABLE);

crate::nvn_wrap_ret!(multisample_state_get_samples(arg0: *const NvnMultisampleState) -> i32 => SLOT_NVN_MULTISAMPLE_STATE_GET_SAMPLES);

crate::nvn_wrap_ret!(multisample_state_get_alpha_to_coverage_enable(arg0: *const NvnMultisampleState) -> NvnBoolean => SLOT_NVN_MULTISAMPLE_STATE_GET_ALPHA_TO_COVERAGE_ENABLE);

crate::nvn_wrap_ret!(multisample_state_get_alpha_to_coverage_dither(arg0: *const NvnMultisampleState) -> NvnBoolean => SLOT_NVN_MULTISAMPLE_STATE_GET_ALPHA_TO_COVERAGE_DITHER);

crate::nvn_wrap_void!(multisample_state_set_raster_samples(arg0: *mut NvnMultisampleState, arg1: i32) => SLOT_NVN_MULTISAMPLE_STATE_SET_RASTER_SAMPLES);

crate::nvn_wrap_ret!(multisample_state_get_raster_samples(arg0: *mut NvnMultisampleState) -> i32 => SLOT_NVN_MULTISAMPLE_STATE_GET_RASTER_SAMPLES);

crate::nvn_wrap_void!(multisample_state_set_coverage_modulation_mode(arg0: *mut NvnMultisampleState, arg1: NvnCoverageModulationMode) => SLOT_NVN_MULTISAMPLE_STATE_SET_COVERAGE_MODULATION_MODE);

crate::nvn_wrap_ret!(multisample_state_get_coverage_modulation_mode(arg0: *const NvnMultisampleState) -> NvnCoverageModulationMode => SLOT_NVN_MULTISAMPLE_STATE_GET_COVERAGE_MODULATION_MODE);

crate::nvn_wrap_void!(multisample_state_set_coverage_to_color_enable(arg0: *mut NvnMultisampleState, arg1: NvnBoolean) => SLOT_NVN_MULTISAMPLE_STATE_SET_COVERAGE_TO_COLOR_ENABLE);

crate::nvn_wrap_ret!(multisample_state_get_coverage_to_color_enable(arg0: *const NvnMultisampleState) -> NvnBoolean => SLOT_NVN_MULTISAMPLE_STATE_GET_COVERAGE_TO_COLOR_ENABLE);

crate::nvn_wrap_void!(multisample_state_set_coverage_to_color_output(arg0: *mut NvnMultisampleState, arg1: i32) => SLOT_NVN_MULTISAMPLE_STATE_SET_COVERAGE_TO_COLOR_OUTPUT);

crate::nvn_wrap_ret!(multisample_state_get_coverage_to_color_output(arg0: *const NvnMultisampleState) -> i32 => SLOT_NVN_MULTISAMPLE_STATE_GET_COVERAGE_TO_COLOR_OUTPUT);

crate::nvn_wrap_void!(multisample_state_set_sample_locations_enable(arg0: *mut NvnMultisampleState, arg1: NvnBoolean) => SLOT_NVN_MULTISAMPLE_STATE_SET_SAMPLE_LOCATIONS_ENABLE);

crate::nvn_wrap_ret!(multisample_state_get_sample_locations_enable(arg0: *const NvnMultisampleState) -> NvnBoolean => SLOT_NVN_MULTISAMPLE_STATE_GET_SAMPLE_LOCATIONS_ENABLE);

crate::nvn_wrap_void!(multisample_state_get_sample_locations_grid(arg0: *mut NvnMultisampleState, arg1: *mut i32, arg2: *mut i32) => SLOT_NVN_MULTISAMPLE_STATE_GET_SAMPLE_LOCATIONS_GRID);

crate::nvn_wrap_void!(multisample_state_set_sample_locations_grid_enable(arg0: *mut NvnMultisampleState, arg1: NvnBoolean) => SLOT_NVN_MULTISAMPLE_STATE_SET_SAMPLE_LOCATIONS_GRID_ENABLE);

crate::nvn_wrap_ret!(multisample_state_get_sample_locations_grid_enable(arg0: *const NvnMultisampleState) -> NvnBoolean => SLOT_NVN_MULTISAMPLE_STATE_GET_SAMPLE_LOCATIONS_GRID_ENABLE);

crate::nvn_wrap_void!(multisample_state_set_sample_locations(arg0: *mut NvnMultisampleState, arg1: i32, arg2: i32, arg3: *const f32) => SLOT_NVN_MULTISAMPLE_STATE_SET_SAMPLE_LOCATIONS);

crate::nvn_wrap_void!(polygon_state_set_defaults(arg0: *mut NvnPolygonState) => SLOT_NVN_POLYGON_STATE_SET_DEFAULTS);

crate::nvn_wrap_void!(polygon_state_set_cull_face(arg0: *mut NvnPolygonState, arg1: NvnFace) => SLOT_NVN_POLYGON_STATE_SET_CULL_FACE);

crate::nvn_wrap_void!(polygon_state_set_front_face(arg0: *mut NvnPolygonState, arg1: NvnFrontFace) => SLOT_NVN_POLYGON_STATE_SET_FRONT_FACE);

crate::nvn_wrap_void!(polygon_state_set_polygon_mode(arg0: *mut NvnPolygonState, arg1: NvnPolygonMode) => SLOT_NVN_POLYGON_STATE_SET_POLYGON_MODE);

crate::nvn_wrap_void!(polygon_state_set_polygon_offset_enables(arg0: *mut NvnPolygonState, arg1: i32) => SLOT_NVN_POLYGON_STATE_SET_POLYGON_OFFSET_ENABLES);

crate::nvn_wrap_ret!(polygon_state_get_cull_face(arg0: *const NvnPolygonState) -> NvnFace => SLOT_NVN_POLYGON_STATE_GET_CULL_FACE);

crate::nvn_wrap_ret!(polygon_state_get_front_face(arg0: *const NvnPolygonState) -> NvnFrontFace => SLOT_NVN_POLYGON_STATE_GET_FRONT_FACE);

crate::nvn_wrap_ret!(polygon_state_get_polygon_mode(arg0: *const NvnPolygonState) -> NvnPolygonMode => SLOT_NVN_POLYGON_STATE_GET_POLYGON_MODE);

crate::nvn_wrap_ret!(polygon_state_get_polygon_offset_enables(arg0: *const NvnPolygonState) -> NvnPolygonOffsetEnable => SLOT_NVN_POLYGON_STATE_GET_POLYGON_OFFSET_ENABLES);

crate::nvn_wrap_void!(depth_stencil_state_set_defaults(arg0: *mut NvnDepthStencilState) => SLOT_NVN_DEPTH_STENCIL_STATE_SET_DEFAULTS);

crate::nvn_wrap_void!(depth_stencil_state_set_depth_test_enable(arg0: *mut NvnDepthStencilState, arg1: NvnBoolean) => SLOT_NVN_DEPTH_STENCIL_STATE_SET_DEPTH_TEST_ENABLE);

crate::nvn_wrap_void!(depth_stencil_state_set_depth_write_enable(arg0: *mut NvnDepthStencilState, arg1: NvnBoolean) => SLOT_NVN_DEPTH_STENCIL_STATE_SET_DEPTH_WRITE_ENABLE);

crate::nvn_wrap_void!(depth_stencil_state_set_depth_func(arg0: *mut NvnDepthStencilState, arg1: NvnDepthFunc) => SLOT_NVN_DEPTH_STENCIL_STATE_SET_DEPTH_FUNC);

crate::nvn_wrap_void!(depth_stencil_state_set_stencil_test_enable(arg0: *mut NvnDepthStencilState, arg1: NvnBoolean) => SLOT_NVN_DEPTH_STENCIL_STATE_SET_STENCIL_TEST_ENABLE);

crate::nvn_wrap_void!(depth_stencil_state_set_stencil_func(arg0: *mut NvnDepthStencilState, arg1: NvnFace, arg2: NvnStencilFunc) => SLOT_NVN_DEPTH_STENCIL_STATE_SET_STENCIL_FUNC);

crate::nvn_wrap_void!(depth_stencil_state_set_stencil_op(arg0: *mut NvnDepthStencilState, arg1: NvnFace, arg2: NvnStencilOp, arg3: NvnStencilOp, arg4: NvnStencilOp) => SLOT_NVN_DEPTH_STENCIL_STATE_SET_STENCIL_OP);

crate::nvn_wrap_ret!(depth_stencil_state_get_depth_test_enable(arg0: *const NvnDepthStencilState) -> NvnBoolean => SLOT_NVN_DEPTH_STENCIL_STATE_GET_DEPTH_TEST_ENABLE);

crate::nvn_wrap_ret!(depth_stencil_state_get_depth_write_enable(arg0: *const NvnDepthStencilState) -> NvnBoolean => SLOT_NVN_DEPTH_STENCIL_STATE_GET_DEPTH_WRITE_ENABLE);

crate::nvn_wrap_ret!(depth_stencil_state_get_depth_func(arg0: *const NvnDepthStencilState) -> NvnDepthFunc => SLOT_NVN_DEPTH_STENCIL_STATE_GET_DEPTH_FUNC);

crate::nvn_wrap_ret!(depth_stencil_state_get_stencil_test_enable(arg0: *const NvnDepthStencilState) -> NvnBoolean => SLOT_NVN_DEPTH_STENCIL_STATE_GET_STENCIL_TEST_ENABLE);

crate::nvn_wrap_ret!(depth_stencil_state_get_stencil_func(arg0: *const NvnDepthStencilState, arg1: NvnFace) -> NvnStencilFunc => SLOT_NVN_DEPTH_STENCIL_STATE_GET_STENCIL_FUNC);

crate::nvn_wrap_void!(depth_stencil_state_get_stencil_op(arg0: *const NvnDepthStencilState, arg1: NvnFace, arg2: *mut NvnStencilOp, arg3: *mut NvnStencilOp, arg4: *mut NvnStencilOp) => SLOT_NVN_DEPTH_STENCIL_STATE_GET_STENCIL_OP);

crate::nvn_wrap_void!(vertex_attrib_state_set_defaults(arg0: *mut NvnVertexAttribState) => SLOT_NVN_VERTEX_ATTRIB_STATE_SET_DEFAULTS);

crate::nvn_wrap_void!(vertex_attrib_state_set_format(arg0: *mut NvnVertexAttribState, arg1: NvnFormat, arg2: isize) => SLOT_NVN_VERTEX_ATTRIB_STATE_SET_FORMAT);

crate::nvn_wrap_void!(vertex_attrib_state_set_stream_index(arg0: *mut NvnVertexAttribState, arg1: i32) => SLOT_NVN_VERTEX_ATTRIB_STATE_SET_STREAM_INDEX);

crate::nvn_wrap_void!(vertex_attrib_state_get_format(arg0: *const NvnVertexAttribState, arg1: *mut NvnFormat, arg2: *mut isize) => SLOT_NVN_VERTEX_ATTRIB_STATE_GET_FORMAT);

crate::nvn_wrap_ret!(vertex_attrib_state_get_stream_index(arg0: *const NvnVertexAttribState) -> i32 => SLOT_NVN_VERTEX_ATTRIB_STATE_GET_STREAM_INDEX);

crate::nvn_wrap_void!(vertex_stream_state_set_defaults(arg0: *mut NvnVertexStreamState) => SLOT_NVN_VERTEX_STREAM_STATE_SET_DEFAULTS);

crate::nvn_wrap_void!(vertex_stream_state_set_stride(arg0: *mut NvnVertexStreamState, arg1: isize) => SLOT_NVN_VERTEX_STREAM_STATE_SET_STRIDE);

crate::nvn_wrap_void!(vertex_stream_state_set_divisor(arg0: *mut NvnVertexStreamState, arg1: i32) => SLOT_NVN_VERTEX_STREAM_STATE_SET_DIVISOR);

crate::nvn_wrap_ret!(vertex_stream_state_get_stride(arg0: *const NvnVertexStreamState) -> isize => SLOT_NVN_VERTEX_STREAM_STATE_GET_STRIDE);

crate::nvn_wrap_ret!(vertex_stream_state_get_divisor(arg0: *const NvnVertexStreamState) -> i32 => SLOT_NVN_VERTEX_STREAM_STATE_GET_DIVISOR);

