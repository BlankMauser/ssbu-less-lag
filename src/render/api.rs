use core::sync::atomic::{AtomicU8, Ordering};
use crate::swapchain;

/// Represents the current swapchain buffer mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BufferMode {
    /// Two active textures — lower latency, may drop frames on heavy scenes.
    Double = 2,
    /// Three active textures — default SSBU behaviour, higher latency.
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

/// 0 = uninitialised, 2 = double, 3 = triple.
static CURRENT_MODE: AtomicU8 = AtomicU8::new(0);

// Simple callback table — fixed-size, no alloc.
const MAX_CALLBACKS: usize = 8;
static mut CALLBACKS: [Option<fn(BufferMode)>; MAX_CALLBACKS] = [None; MAX_CALLBACKS];
static CALLBACK_COUNT: AtomicU8 = AtomicU8::new(0);

// ── Query ────────────────────────────────────────────────────────────

/// Returns the current buffer mode, or `None` if not yet initialised.
pub fn current_buffer_mode() -> Option<BufferMode> {
    BufferMode::from_u8(CURRENT_MODE.load(Ordering::Acquire))
}

/// Returns the number of active window textures by querying the cached NVN
/// window pointer directly.  Returns `None` if the window has not been seen
/// yet (pointer not cached).
pub fn get_active_texture_count() -> Option<i32> {
    let window = swapchain::window_target();
    if window == 0 {
        return None;
    }
    unsafe {
        let func_ptr = swapchain::get_window_num_active_textures_fn();
        Some(func_ptr(window as *const _))
    }
}

// ── Mode transitions ─────────────────────────────────────────────────

/// Set the buffer mode.  If the mode is already active this is a no-op.
///
/// Returns `true` if the mode was changed, `false` if it was already set or
/// the system is running on an emulator.
pub fn set_buffer_mode(mode: BufferMode) -> bool {
    if crate::is_emulator() {
        return false;
    }

    let prev = CURRENT_MODE.load(Ordering::Acquire);
    if prev == mode as u8 {
        return false; // already in this mode
    }

    // Apply the swapchain patches.
    unsafe {
        match mode {
            BufferMode::Double => swapchain::enable_double_buffer(),
            BufferMode::Triple => swapchain::enable_triple_buffer(),
        }
    }

    CURRENT_MODE.store(mode as u8, Ordering::Release);
    fire_callbacks(mode);
    true
}

// ── Callbacks ────────────────────────────────────────────────────────

/// Register a callback that fires whenever the buffer mode changes.
///
/// Returns `true` if registered, `false` if the callback table is full.
pub fn on_buffer_mode_change(cb: fn(BufferMode)) -> bool {
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

// ── Internal: called by swapchain on first install ───────────────────

/// Called by the swapchain module during `install()` to seed the initial mode
/// without triggering callbacks (since nothing has "changed" yet).
pub(crate) fn set_initial_mode(mode: BufferMode) {
    CURRENT_MODE.store(mode as u8, Ordering::Release);
}
