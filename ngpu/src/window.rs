#![allow(unused)]
use crate::*;

gpu_api! {
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

crate::nvn_wrap_void!(window_builder_set_device(builder: *mut NvnWindowBuilder, device: *mut NvnDevice) => SLOT_NVN_WINDOW_BUILDER_SET_DEVICE);
crate::nvn_wrap_void!(window_builder_set_defaults(builder: *mut NvnWindowBuilder) => SLOT_NVN_WINDOW_BUILDER_SET_DEFAULTS);
crate::nvn_wrap_void!(window_builder_set_native_window(builder: *mut NvnWindowBuilder, native_window: NvnNativeWindow) => SLOT_NVN_WINDOW_BUILDER_SET_NATIVE_WINDOW);
crate::nvn_wrap_void!(window_builder_set_textures(builder: *mut NvnWindowBuilder, texture_count: i32, textures: *const *mut NvnTexture) => SLOT_NVN_WINDOW_BUILDER_SET_TEXTURES);
crate::nvn_wrap_void!(window_builder_set_present_interval(builder: *mut NvnWindowBuilder, present_interval: i32) => SLOT_NVN_WINDOW_BUILDER_SET_PRESENT_INTERVAL);
crate::nvn_wrap_void!(window_builder_set_num_active_textures(builder: *mut NvnWindowBuilder, active_texture_count: i32) => SLOT_NVN_WINDOW_BUILDER_SET_NUM_ACTIVE_TEXTURES);
crate::nvn_wrap_ret!(window_builder_get_device(builder: *const NvnWindowBuilder) -> *const NvnDevice => SLOT_NVN_WINDOW_BUILDER_GET_DEVICE);
crate::nvn_wrap_ret!(window_builder_get_num_textures(builder: *const NvnWindowBuilder) -> i32 => SLOT_NVN_WINDOW_BUILDER_GET_NUM_TEXTURES);
crate::nvn_wrap_ret!(window_builder_get_texture(builder: *const NvnWindowBuilder, texture_index: i32) -> *const NvnTexture => SLOT_NVN_WINDOW_BUILDER_GET_TEXTURE);
crate::nvn_wrap_ret!(window_builder_get_native_window(builder: *const NvnWindowBuilder) -> NvnNativeWindow => SLOT_NVN_WINDOW_BUILDER_GET_NATIVE_WINDOW);
crate::nvn_wrap_ret!(window_builder_get_present_interval(builder: *const NvnWindowBuilder) -> i32 => SLOT_NVN_WINDOW_BUILDER_GET_PRESENT_INTERVAL);
crate::nvn_wrap_ret!(window_builder_get_num_active_textures(builder: *const NvnWindowBuilder) -> i32 => SLOT_NVN_WINDOW_BUILDER_GET_NUM_ACTIVE_TEXTURES);
crate::nvn_wrap_ret!(window_initialize(window: *mut NvnWindow, builder: *const NvnWindowBuilder) -> NvnBoolean => SLOT_NVN_WINDOW_INITIALIZE);
crate::nvn_wrap_void!(window_finalize(window: *mut NvnWindow) => SLOT_NVN_WINDOW_FINALIZE);
crate::nvn_wrap_void!(window_set_debug_label(window: *mut NvnWindow, label: *const u8) => SLOT_NVN_WINDOW_SET_DEBUG_LABEL);
crate::nvn_wrap_ret!(window_acquire_texture(window: *mut NvnWindow, texture_available_sync: *mut NvnSync, texture_index: *mut i32) -> NvnWindowAcquireTextureResult => SLOT_NVN_WINDOW_ACQUIRE_TEXTURE);
crate::nvn_wrap_ret!(window_get_native_window(window: *const NvnWindow) -> NvnNativeWindow => SLOT_NVN_WINDOW_GET_NATIVE_WINDOW);
crate::nvn_wrap_ret!(window_get_present_interval(window: *const NvnWindow) -> i32 => SLOT_NVN_WINDOW_GET_PRESENT_INTERVAL);
crate::nvn_wrap_void!(window_set_present_interval(window: *mut NvnWindow, present_interval: i32) => SLOT_NVN_WINDOW_SET_PRESENT_INTERVAL);
crate::nvn_wrap_void!(window_set_crop(window: *mut NvnWindow, x: i32, y: i32, width: i32, height: i32) => SLOT_NVN_WINDOW_SET_CROP);
crate::nvn_wrap_void!(window_get_crop(window: *const NvnWindow, crop: *mut NvnRectangle) => SLOT_NVN_WINDOW_GET_CROP);
crate::nvn_wrap_void!(window_set_num_active_textures(window: *mut NvnWindow, active_texture_count: i32) => SLOT_NVN_WINDOW_SET_NUM_ACTIVE_TEXTURES);
crate::nvn_wrap_ret!(window_get_num_active_textures(window: *const NvnWindow) -> i32 => SLOT_NVN_WINDOW_GET_NUM_ACTIVE_TEXTURES);
crate::nvn_wrap_ret!(window_get_num_textures(window: *const NvnWindow) -> i32 => SLOT_NVN_WINDOW_GET_NUM_TEXTURES);

pub unsafe fn acquire_texture(
    window: *mut NvnWindow,
    texture_available_sync: *mut NvnSync,
    texture_index: *mut i32,
) -> NvnWindowAcquireTextureResult {
    window_acquire_texture(window, texture_available_sync, texture_index)
}

pub unsafe fn get_present_interval(window: *const NvnWindow) -> i32 {
    window_get_present_interval(window)
}

pub unsafe fn set_present_interval(window: *mut NvnWindow, present_interval: i32) {
    window_set_present_interval(window, present_interval)
}

pub unsafe fn get_num_active_textures(window: *const NvnWindow) -> i32 {
    window_get_num_active_textures(window)
}

pub unsafe fn set_num_active_textures(window: *mut NvnWindow, active_texture_count: i32) {
    window_set_num_active_textures(window, active_texture_count)
}

pub unsafe fn nvn_window_acquire_texture(
    window: *mut NvnWindow,
    texture_available_sync: *mut NvnSync,
    texture_index: *mut i32,
) -> NvnWindowAcquireTextureResult {
    window_acquire_texture(window, texture_available_sync, texture_index)
}

pub unsafe fn nvn_window_get_present_interval(window: *const NvnWindow) -> i32 {
    window_get_present_interval(window)
}

pub unsafe fn nvn_window_set_present_interval(window: *mut NvnWindow, present_interval: i32) {
    window_set_present_interval(window, present_interval)
}

pub unsafe fn nvn_window_get_num_active_textures(window: *const NvnWindow) -> i32 {
    window_get_num_active_textures(window)
}

pub unsafe fn nvn_window_set_num_active_textures(window: *mut NvnWindow, active_texture_count: i32) {
    window_set_num_active_textures(window, active_texture_count)
}
