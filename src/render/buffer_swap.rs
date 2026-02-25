use std::sync::atomic::{AtomicU8, Ordering};
use crate::pacer::*;
use crate::swapchain::*;
use crate::SyncEnv;

/// Current swapchain buffer mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BufferMode {
    /// lower latency
    Double = 2,
    /// default SSBU behaviour, higher latency.
    Triple = 3,
}

impl BufferMode {
    fn from_u8(v: u8) -> Option<Self> {
        match v {
            2 => Some(Self::Double),
            3 => Some(Self::Triple),
            _ => None,
        }
    }

    /// Number of active textures for this mode.
    pub fn texture_count(self) -> i32 {
        self as i32
    }
}

/// Guard against immediate mode thrashing after a completed swap.
static SWAP_COOLDOWN_FRAMES: AtomicU8 = AtomicU8::new(0);

// Simple callback table — fixed-size, no alloc.
const MAX_CALLBACKS: usize = 8;
static mut CALLBACKS: [Option<fn(BufferMode)>; MAX_CALLBACKS] = [None; MAX_CALLBACKS];
static CALLBACK_COUNT: AtomicU8 = AtomicU8::new(0);

// ── Query ────────────────────────────────────────────────────────────

/// Returns the current buffer mode. Defaults to Triple before init.
pub fn current_buffer_mode() -> BufferMode {
    if SyncEnv::triple_enabled() {
        BufferMode::Triple
    } else {
        BufferMode::Double
    }
}

/// Returns the mode that should currently be enforced on the NVN window.
pub fn desired_buffer_mode() -> BufferMode {
    current_buffer_mode()
}

pub fn frame_index_mode() -> BufferMode {
    current_buffer_mode()
}

/// Returns whether we are in the process of swapping.
pub fn is_buffer_swapping() -> bool {
    crate::SyncEnv::swapping_buffer()
}

/// Returns the number of active window textures by querying the cached NVN
/// window pointer directly.  Returns `None` if the window has not been seen
/// yet (pointer not cached).
pub fn get_active_texture_count() -> Option<i32> {
    if !window_target_is_valid() {
        return None;
    }
    let window = window_target();
    if window == 0 {
        return None;
    }
    unsafe {
        let func_ptr = get_window_num_active_textures_fn();
        Some(func_ptr(window))
    }
}

pub fn get_window_texture_capacity() -> Option<i32> {
    if !window_target_is_valid() {
        return None;
    }
    let window = window_target();
    if window == 0 {
        return None;
    }
    unsafe {
        let func_ptr = get_window_num_textures_fn();
        Some(func_ptr(window))
    }
}

// ── Mode transitions ─────────────────────────────────────────────────

/// Begin changing buffer
pub fn start_swap_buffer(mode: BufferMode) -> bool {
    if crate::is_emulator() {
        println!("[ssbu-sync] buffer swap not allowed on emulator!");
        return false;
    }
    if SWAP_COOLDOWN_FRAMES.load(Ordering::Acquire) != 0 {
        return false;
    }
    let prev_triple = crate::SyncEnv::triple_enabled();
    if prev_triple == (mode == BufferMode::Triple) && !is_buffer_swapping() {
        return false; // already in this mode
    }

    let texture_count = get_active_texture_count();
    let capacity = get_window_texture_capacity();
    if let Some(active) = texture_count {
        println!("[ssbu-sync] window texture count: {}\n", active);
    }
    if let Some(total) = capacity {
        println!("[ssbu-sync] window texture capacity: {}\n", total);
    }

    let desired = mode.texture_count();
    if texture_count == Some(desired) {
        SyncEnv::set_swapping_buffer(false);
        SyncEnv::set_triple_enabled(mode == BufferMode::Triple);
        println!("[ssbu-sync] window already correct texture count\n");
        return false;
    }

    println!(
        "[ssbu-sync] texture count before swap: {}\n",
        texture_count
            .map(|v| v.to_string())
            .unwrap_or_else(|| "unknown".to_string())
    );

    if !try_set_window_textures(desired) {
        println!("[ssbu-sync] unable to start swap for {:?}\n", mode);
        return false;
    }

    crate::SyncEnv::set_swapping_buffer(true);
    crate::SyncEnv::set_triple_enabled(mode == BufferMode::Triple);
    println!("[ssbu-sync] Swapping buffer mode to {:?} ...\n", mode);
    true
}

pub fn check_swap_finished() {
    let cooldown = SWAP_COOLDOWN_FRAMES.load(Ordering::Acquire);
    if cooldown != 0 {
        SWAP_COOLDOWN_FRAMES.store(cooldown.saturating_sub(1), Ordering::Release);
    }

    if !is_buffer_swapping() {
        return;
    }

    let desired = if crate::SyncEnv::triple_enabled() { 3 } else { 2 };
    if (get_active_texture_count().unwrap_or(0) as u8) == desired {
        let mode = if desired == 3 { BufferMode::Triple } else { BufferMode::Double };
        finish_install_buffer(mode);
    }
}

fn finish_install_buffer(mode: BufferMode) {
    let triple = mode == BufferMode::Triple;
    set_runtime_frame_index_mode(triple);
    patch_pacer_bias(triple);
    SyncEnv::set_swapping_buffer(false);
    SyncEnv::set_triple_enabled(mode == BufferMode::Triple);
    SWAP_COOLDOWN_FRAMES.store(6, Ordering::Release);
    println!("[ssbu-sync] patched new buffer mode {:?} ...\n", mode);
    fire_callbacks(mode);
}

// ── Callbacks ────────────────────────────────────────────────────────

/// Register a callback that fires whenever the buffer mode changes.
pub fn subscribe_buffer_mode_change(cb: fn(BufferMode)) -> bool {
    let idx = CALLBACK_COUNT.fetch_add(1, Ordering::AcqRel) as usize;
    if idx >= MAX_CALLBACKS {
        CALLBACK_COUNT.fetch_sub(1, Ordering::AcqRel);
        return false;
    }
    unsafe {
        CALLBACKS[idx] = Some(cb);
    }
    true
}

pub fn init_buffer_mode(mode: BufferMode) {
    SyncEnv::set_swapping_buffer(false);
    SyncEnv::set_triple_enabled(mode == BufferMode::Triple);
    SWAP_COOLDOWN_FRAMES.store(0, Ordering::Release);
    
    if crate::is_emulator() {
        println!("[ssbu-sync] cant set buffer mode on emulator");
    }
    
    println!("[ssbu-sync] Initializing Buffer Mode {:?}", mode);
}

fn fire_callbacks(mode: BufferMode) {
    let count = CALLBACK_COUNT.load(Ordering::Acquire) as usize;
    for i in 0..count.min(MAX_CALLBACKS) {
        unsafe {
            if let Some(cb) = CALLBACKS[i] {
                cb(mode);
            }
        }
    }
}
