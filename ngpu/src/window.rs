#![allow(unused)]
use crate::*;

nvn_func! {
    // WindowBuilder
    pub static nvnWindowBuilderSetDevice: fn(*mut NvnWindowBuilder, *mut NvnDevice);
    pub static nvnWindowBuilderSetDefaults: fn(*mut NvnWindowBuilder);
    pub static nvnWindowBuilderSetNativeWindow: fn(*mut NvnWindowBuilder, NvnNativeWindow);
    pub static nvnWindowBuilderSetTextures: fn(*mut NvnWindowBuilder, i32, *const *mut NvnTexture);
    pub static nvnWindowBuilderSetPresentInterval: fn(*mut NvnWindowBuilder, i32);
    // vtable-only
    pub static nvnWindowBuilderSetNumActiveTextures: fn(*mut NvnWindowBuilder, i32);
    pub static nvnWindowBuilderGetDevice: fn(*const NvnWindowBuilder) -> *const NvnDevice;
    pub static nvnWindowBuilderGetNumTextures: fn(*const NvnWindowBuilder) -> i32;
    pub static nvnWindowBuilderGetTexture: fn(*const NvnWindowBuilder, i32) -> *const NvnTexture;
    pub static nvnWindowBuilderGetNativeWindow: fn(*const NvnWindowBuilder) -> NvnNativeWindow;
    pub static nvnWindowBuilderGetPresentInterval: fn(*const NvnWindowBuilder) -> i32;
    pub static nvnWindowBuilderGetNumActiveTextures: fn(*const NvnWindowBuilder) -> i32;

    // Window
    pub static nvnWindowInitialize: fn(*mut NvnWindow, *const NvnWindowBuilder) -> NvnBoolean;
    pub static nvnWindowFinalize: fn(*mut NvnWindow);
    pub static nvnWindowSetDebugLabel: fn(*mut NvnWindow, *const u8);
    pub static nvnWindowAcquireTexture: fn(*mut NvnWindow, *mut NvnSync, *mut i32) -> NvnWindowAcquireTextureResult;
    pub static nvnWindowGetNativeWindow: fn(*const NvnWindow) -> NvnNativeWindow;
    pub static nvnWindowGetPresentInterval: fn(*const NvnWindow) -> i32;
    pub static nvnWindowSetPresentInterval: fn(*mut NvnWindow, i32);
    pub static nvnWindowSetCrop: fn(*mut NvnWindow, i32, i32, i32, i32);
    pub static nvnWindowGetCrop: fn(*const NvnWindow, *mut NvnRectangle);
    // vtable-only
    pub static nvnWindowSetNumActiveTextures: fn(*mut NvnWindow, i32);
    pub static nvnWindowGetNumActiveTextures: fn(*const NvnWindow) -> i32;
    pub static nvnWindowGetNumTextures: fn(*const NvnWindow) -> i32;
}
