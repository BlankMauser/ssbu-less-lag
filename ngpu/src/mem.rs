use crate::*;

gpu_api! {
    // ── MemoryPoolBuilder ──
    pub static nvnMemoryPoolBuilderSetDevice: fn(*mut NvnMemoryPoolBuilder, *mut NvnDevice);
    pub static nvnMemoryPoolBuilderSetDefaults: fn(*mut NvnMemoryPoolBuilder);
    pub static nvnMemoryPoolBuilderSetStorage: fn(*mut NvnMemoryPoolBuilder, *mut core::ffi::c_void, usize);
    pub static nvnMemoryPoolBuilderSetFlags: fn(*mut NvnMemoryPoolBuilder, i32);
    // vtable-only
    pub static nvnMemoryPoolBuilderGetDevice: fn(*const NvnMemoryPoolBuilder) -> *const NvnDevice;
    pub static nvnMemoryPoolBuilderGetMemory: fn(*const NvnMemoryPoolBuilder) -> *mut core::ffi::c_void;
    pub static nvnMemoryPoolBuilderGetSize: fn(*const NvnMemoryPoolBuilder) -> usize;
    pub static nvnMemoryPoolBuilderGetFlags: fn(*const NvnMemoryPoolBuilder) -> NvnMemoryPoolFlags;

    // ── MemoryPool ──
    pub static nvnMemoryPoolInitialize: fn(*mut NvnMemoryPool, *const NvnMemoryPoolBuilder) -> NvnBoolean;
    pub static nvnMemoryPoolSetDebugLabel: fn(*mut NvnMemoryPool, *const u8);
    pub static nvnMemoryPoolFinalize: fn(*mut NvnMemoryPool);
    pub static nvnMemoryPoolMap: fn(*const NvnMemoryPool) -> *mut core::ffi::c_void;
    pub static nvnMemoryPoolFlushMappedRange: fn(*const NvnMemoryPool, isize, usize);
    pub static nvnMemoryPoolInvalidateMappedRange: fn(*const NvnMemoryPool, isize, usize);
    pub static nvnMemoryPoolGetBufferAddress: fn(*const NvnMemoryPool) -> NvnBufferAddress;
    pub static nvnMemoryPoolMapVirtual: fn(*mut NvnMemoryPool, i32, *const NvnMappingRequest) -> NvnBoolean;
    pub static nvnMemoryPoolGetSize: fn(*const NvnMemoryPool) -> usize;
    pub static nvnMemoryPoolGetFlags: fn(*const NvnMemoryPool) -> NvnMemoryPoolFlags;

    // ── TexturePool ──
    pub static nvnTexturePoolInitialize: fn(*mut NvnTexturePool, *const NvnMemoryPool, isize, i32) -> NvnBoolean;
    pub static nvnTexturePoolSetDebugLabel: fn(*mut NvnTexturePool, *const u8);
    pub static nvnTexturePoolFinalize: fn(*mut NvnTexturePool);
    pub static nvnTexturePoolRegisterTexture: fn(*const NvnTexturePool, i32, *const NvnTexture, *const NvnTextureView);
    pub static nvnTexturePoolRegisterImage: fn(*const NvnTexturePool, i32, *const NvnTexture, *const NvnTextureView);
    pub static nvnTexturePoolGetMemoryPool: fn(*const NvnTexturePool) -> *const NvnMemoryPool;
    pub static nvnTexturePoolGetMemoryOffset: fn(*const NvnTexturePool) -> isize;
    pub static nvnTexturePoolGetSize: fn(*const NvnTexturePool) -> i32;

    // ── SamplerPool ──
    pub static nvnSamplerPoolInitialize: fn(*mut NvnSamplerPool, *const NvnMemoryPool, isize, i32) -> NvnBoolean;
    pub static nvnSamplerPoolSetDebugLabel: fn(*mut NvnSamplerPool, *const u8);
    pub static nvnSamplerPoolFinalize: fn(*mut NvnSamplerPool);
    pub static nvnSamplerPoolRegisterSampler: fn(*const NvnSamplerPool, i32, *const NvnSampler);
    pub static nvnSamplerPoolRegisterSamplerBuilder: fn(*const NvnSamplerPool, i32, *const NvnSamplerBuilder);
    pub static nvnSamplerPoolGetMemoryPool: fn(*const NvnSamplerPool) -> *const NvnMemoryPool;
    pub static nvnSamplerPoolGetMemoryOffset: fn(*const NvnSamplerPool) -> isize;
    pub static nvnSamplerPoolGetSize: fn(*const NvnSamplerPool) -> i32;

    // ── BufferBuilder ──
    pub static nvnBufferBuilderSetDevice: fn(*mut NvnBufferBuilder, *mut NvnDevice);
    pub static nvnBufferBuilderSetDefaults: fn(*mut NvnBufferBuilder);
    pub static nvnBufferBuilderSetStorage: fn(*mut NvnBufferBuilder, *mut NvnMemoryPool, isize, usize);
    // vtable-only
    pub static nvnBufferBuilderGetDevice: fn(*const NvnBufferBuilder) -> *const NvnDevice;
    pub static nvnBufferBuilderGetMemoryPool: fn(*const NvnBufferBuilder) -> *const NvnMemoryPool;
    pub static nvnBufferBuilderGetMemoryOffset: fn(*const NvnBufferBuilder) -> isize;
    pub static nvnBufferBuilderGetSize: fn(*const NvnBufferBuilder) -> usize;

    // ── Buffer ──
    pub static nvnBufferInitialize: fn(*mut NvnBuffer, *const NvnBufferBuilder) -> NvnBoolean;
    pub static nvnBufferSetDebugLabel: fn(*mut NvnBuffer, *const u8);
    pub static nvnBufferFinalize: fn(*mut NvnBuffer);
    pub static nvnBufferMap: fn(*const NvnBuffer) -> *mut core::ffi::c_void;
    pub static nvnBufferGetAddress: fn(*const NvnBuffer) -> NvnBufferAddress;
    pub static nvnBufferFlushMappedRange: fn(*const NvnBuffer, isize, usize);
    pub static nvnBufferInvalidateMappedRange: fn(*const NvnBuffer, isize, usize);
    pub static nvnBufferGetMemoryPool: fn(*const NvnBuffer) -> *mut NvnMemoryPool;
    pub static nvnBufferGetMemoryOffset: fn(*const NvnBuffer) -> isize;
    pub static nvnBufferGetSize: fn(*const NvnBuffer) -> usize;
    pub static nvnBufferGetDebugID: fn(*const NvnBuffer) -> u64;
}

crate::nvn_wrap_void!(memory_pool_builder_set_device(arg0: *mut NvnMemoryPoolBuilder, arg1: *mut NvnDevice) => SLOT_NVN_MEMORY_POOL_BUILDER_SET_DEVICE);

crate::nvn_wrap_void!(memory_pool_builder_set_defaults(arg0: *mut NvnMemoryPoolBuilder) => SLOT_NVN_MEMORY_POOL_BUILDER_SET_DEFAULTS);

crate::nvn_wrap_void!(memory_pool_builder_set_storage(arg0: *mut NvnMemoryPoolBuilder, arg1: *mut core::ffi::c_void, arg2: usize) => SLOT_NVN_MEMORY_POOL_BUILDER_SET_STORAGE);

crate::nvn_wrap_void!(memory_pool_builder_set_flags(arg0: *mut NvnMemoryPoolBuilder, arg1: i32) => SLOT_NVN_MEMORY_POOL_BUILDER_SET_FLAGS);

crate::nvn_wrap_ret!(memory_pool_builder_get_device(arg0: *const NvnMemoryPoolBuilder) -> *const NvnDevice => SLOT_NVN_MEMORY_POOL_BUILDER_GET_DEVICE);

crate::nvn_wrap_ret!(memory_pool_builder_get_memory(arg0: *const NvnMemoryPoolBuilder) -> *mut core::ffi::c_void => SLOT_NVN_MEMORY_POOL_BUILDER_GET_MEMORY);

crate::nvn_wrap_ret!(memory_pool_builder_get_size(arg0: *const NvnMemoryPoolBuilder) -> usize => SLOT_NVN_MEMORY_POOL_BUILDER_GET_SIZE);

crate::nvn_wrap_ret!(memory_pool_builder_get_flags(arg0: *const NvnMemoryPoolBuilder) -> NvnMemoryPoolFlags => SLOT_NVN_MEMORY_POOL_BUILDER_GET_FLAGS);

crate::nvn_wrap_ret!(memory_pool_initialize(arg0: *mut NvnMemoryPool, arg1: *const NvnMemoryPoolBuilder) -> NvnBoolean => SLOT_NVN_MEMORY_POOL_INITIALIZE);

crate::nvn_wrap_void!(memory_pool_set_debug_label(arg0: *mut NvnMemoryPool, arg1: *const u8) => SLOT_NVN_MEMORY_POOL_SET_DEBUG_LABEL);

crate::nvn_wrap_void!(memory_pool_finalize(arg0: *mut NvnMemoryPool) => SLOT_NVN_MEMORY_POOL_FINALIZE);

crate::nvn_wrap_ret!(memory_pool_map(arg0: *const NvnMemoryPool) -> *mut core::ffi::c_void => SLOT_NVN_MEMORY_POOL_MAP);

crate::nvn_wrap_void!(memory_pool_flush_mapped_range(arg0: *const NvnMemoryPool, arg1: isize, arg2: usize) => SLOT_NVN_MEMORY_POOL_FLUSH_MAPPED_RANGE);

crate::nvn_wrap_void!(memory_pool_invalidate_mapped_range(arg0: *const NvnMemoryPool, arg1: isize, arg2: usize) => SLOT_NVN_MEMORY_POOL_INVALIDATE_MAPPED_RANGE);

crate::nvn_wrap_ret!(memory_pool_get_buffer_address(arg0: *const NvnMemoryPool) -> NvnBufferAddress => SLOT_NVN_MEMORY_POOL_GET_BUFFER_ADDRESS);

crate::nvn_wrap_ret!(memory_pool_map_virtual(arg0: *mut NvnMemoryPool, arg1: i32, arg2: *const NvnMappingRequest) -> NvnBoolean => SLOT_NVN_MEMORY_POOL_MAP_VIRTUAL);

crate::nvn_wrap_ret!(memory_pool_get_size(arg0: *const NvnMemoryPool) -> usize => SLOT_NVN_MEMORY_POOL_GET_SIZE);

crate::nvn_wrap_ret!(memory_pool_get_flags(arg0: *const NvnMemoryPool) -> NvnMemoryPoolFlags => SLOT_NVN_MEMORY_POOL_GET_FLAGS);

crate::nvn_wrap_ret!(texture_pool_initialize(arg0: *mut NvnTexturePool, arg1: *const NvnMemoryPool, arg2: isize, arg3: i32) -> NvnBoolean => SLOT_NVN_TEXTURE_POOL_INITIALIZE);

crate::nvn_wrap_void!(texture_pool_set_debug_label(arg0: *mut NvnTexturePool, arg1: *const u8) => SLOT_NVN_TEXTURE_POOL_SET_DEBUG_LABEL);

crate::nvn_wrap_void!(texture_pool_finalize(arg0: *mut NvnTexturePool) => SLOT_NVN_TEXTURE_POOL_FINALIZE);

crate::nvn_wrap_void!(texture_pool_register_texture(arg0: *const NvnTexturePool, arg1: i32, arg2: *const NvnTexture, arg3: *const NvnTextureView) => SLOT_NVN_TEXTURE_POOL_REGISTER_TEXTURE);

crate::nvn_wrap_void!(texture_pool_register_image(arg0: *const NvnTexturePool, arg1: i32, arg2: *const NvnTexture, arg3: *const NvnTextureView) => SLOT_NVN_TEXTURE_POOL_REGISTER_IMAGE);

crate::nvn_wrap_ret!(texture_pool_get_memory_pool(arg0: *const NvnTexturePool) -> *const NvnMemoryPool => SLOT_NVN_TEXTURE_POOL_GET_MEMORY_POOL);

crate::nvn_wrap_ret!(texture_pool_get_memory_offset(arg0: *const NvnTexturePool) -> isize => SLOT_NVN_TEXTURE_POOL_GET_MEMORY_OFFSET);

crate::nvn_wrap_ret!(texture_pool_get_size(arg0: *const NvnTexturePool) -> i32 => SLOT_NVN_TEXTURE_POOL_GET_SIZE);

crate::nvn_wrap_ret!(sampler_pool_initialize(arg0: *mut NvnSamplerPool, arg1: *const NvnMemoryPool, arg2: isize, arg3: i32) -> NvnBoolean => SLOT_NVN_SAMPLER_POOL_INITIALIZE);

crate::nvn_wrap_void!(sampler_pool_set_debug_label(arg0: *mut NvnSamplerPool, arg1: *const u8) => SLOT_NVN_SAMPLER_POOL_SET_DEBUG_LABEL);

crate::nvn_wrap_void!(sampler_pool_finalize(arg0: *mut NvnSamplerPool) => SLOT_NVN_SAMPLER_POOL_FINALIZE);

crate::nvn_wrap_void!(sampler_pool_register_sampler(arg0: *const NvnSamplerPool, arg1: i32, arg2: *const NvnSampler) => SLOT_NVN_SAMPLER_POOL_REGISTER_SAMPLER);

crate::nvn_wrap_void!(sampler_pool_register_sampler_builder(arg0: *const NvnSamplerPool, arg1: i32, arg2: *const NvnSamplerBuilder) => SLOT_NVN_SAMPLER_POOL_REGISTER_SAMPLER_BUILDER);

crate::nvn_wrap_ret!(sampler_pool_get_memory_pool(arg0: *const NvnSamplerPool) -> *const NvnMemoryPool => SLOT_NVN_SAMPLER_POOL_GET_MEMORY_POOL);

crate::nvn_wrap_ret!(sampler_pool_get_memory_offset(arg0: *const NvnSamplerPool) -> isize => SLOT_NVN_SAMPLER_POOL_GET_MEMORY_OFFSET);

crate::nvn_wrap_ret!(sampler_pool_get_size(arg0: *const NvnSamplerPool) -> i32 => SLOT_NVN_SAMPLER_POOL_GET_SIZE);

crate::nvn_wrap_void!(buffer_builder_set_device(arg0: *mut NvnBufferBuilder, arg1: *mut NvnDevice) => SLOT_NVN_BUFFER_BUILDER_SET_DEVICE);

crate::nvn_wrap_void!(buffer_builder_set_defaults(arg0: *mut NvnBufferBuilder) => SLOT_NVN_BUFFER_BUILDER_SET_DEFAULTS);

crate::nvn_wrap_void!(buffer_builder_set_storage(arg0: *mut NvnBufferBuilder, arg1: *mut NvnMemoryPool, arg2: isize, arg3: usize) => SLOT_NVN_BUFFER_BUILDER_SET_STORAGE);

crate::nvn_wrap_ret!(buffer_builder_get_device(arg0: *const NvnBufferBuilder) -> *const NvnDevice => SLOT_NVN_BUFFER_BUILDER_GET_DEVICE);

crate::nvn_wrap_ret!(buffer_builder_get_memory_pool(arg0: *const NvnBufferBuilder) -> *const NvnMemoryPool => SLOT_NVN_BUFFER_BUILDER_GET_MEMORY_POOL);

crate::nvn_wrap_ret!(buffer_builder_get_memory_offset(arg0: *const NvnBufferBuilder) -> isize => SLOT_NVN_BUFFER_BUILDER_GET_MEMORY_OFFSET);

crate::nvn_wrap_ret!(buffer_builder_get_size(arg0: *const NvnBufferBuilder) -> usize => SLOT_NVN_BUFFER_BUILDER_GET_SIZE);

crate::nvn_wrap_ret!(buffer_initialize(arg0: *mut NvnBuffer, arg1: *const NvnBufferBuilder) -> NvnBoolean => SLOT_NVN_BUFFER_INITIALIZE);

crate::nvn_wrap_void!(buffer_set_debug_label(arg0: *mut NvnBuffer, arg1: *const u8) => SLOT_NVN_BUFFER_SET_DEBUG_LABEL);

crate::nvn_wrap_void!(buffer_finalize(arg0: *mut NvnBuffer) => SLOT_NVN_BUFFER_FINALIZE);

crate::nvn_wrap_ret!(buffer_map(arg0: *const NvnBuffer) -> *mut core::ffi::c_void => SLOT_NVN_BUFFER_MAP);

crate::nvn_wrap_ret!(buffer_get_address(arg0: *const NvnBuffer) -> NvnBufferAddress => SLOT_NVN_BUFFER_GET_ADDRESS);

crate::nvn_wrap_void!(buffer_flush_mapped_range(arg0: *const NvnBuffer, arg1: isize, arg2: usize) => SLOT_NVN_BUFFER_FLUSH_MAPPED_RANGE);

crate::nvn_wrap_void!(buffer_invalidate_mapped_range(arg0: *const NvnBuffer, arg1: isize, arg2: usize) => SLOT_NVN_BUFFER_INVALIDATE_MAPPED_RANGE);

crate::nvn_wrap_ret!(buffer_get_memory_pool(arg0: *const NvnBuffer) -> *mut NvnMemoryPool => SLOT_NVN_BUFFER_GET_MEMORY_POOL);

crate::nvn_wrap_ret!(buffer_get_memory_offset(arg0: *const NvnBuffer) -> isize => SLOT_NVN_BUFFER_GET_MEMORY_OFFSET);

crate::nvn_wrap_ret!(buffer_get_size(arg0: *const NvnBuffer) -> usize => SLOT_NVN_BUFFER_GET_SIZE);

crate::nvn_wrap_ret!(buffer_get_debug_id(arg0: *const NvnBuffer) -> u64 => SLOT_NVN_BUFFER_GET_DEBUG_ID);

