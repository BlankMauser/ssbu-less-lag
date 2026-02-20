#![allow(unused)]
use crate::*;

nvn_func! {
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
