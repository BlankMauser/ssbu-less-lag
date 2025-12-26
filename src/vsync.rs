/** This is a vsync hack to shave a frame of input delay off of the game
 *
 * This can come with frame drops, and it is not stable. It also doesn't work on emulator. Better solutions are desired but for now
 * this can get the job done. It's also not very well understood (or rather, not well documented) as to why it works the way it does.
 */
use std::sync::atomic::{AtomicBool, Ordering};

use skyline::hooks::InlineCtx;

static mut OFFSET1: u64 = 0;
static mut OFFSET2: u64 = 0;

/** Helper method to calculate the base of nnSdk
 *
 * Since we are not using exlaunch (yet) this must be done manually with an assumed offset. If the version
 * of nnSdk that SSBU uses ever updates, we will need to update this method/offset
 */
unsafe fn calc_nnsdk_offset() -> u64 {
    let mut symbol = 0usize;
    skyline::nn::ro::LookupSymbol(&mut symbol, b"_ZN7android7IBinderD1Ev\0".as_ptr());
    (symbol - 0x240) as u64
}

#[skyline::hook(offset = 0x374c118, inline)]
unsafe fn run_scene_update(_: &skyline::hooks::InlineCtx) {
    // SAFETY: This is basically a local variable (again)
    static mut CONTROLLER: ninput::Controller = ninput::Controller::new(0);

    while !RUN.swap(false, Ordering::SeqCst) {
        #[allow(static_mut_refs)]
        CONTROLLER.update();
    }
}

/** Patches nvnWindowBuilderSetPresentInterval/nvnWindowSetPresentInterval (I don't remember which one)
 *
 * This forces it to always pass in 0, which disables vsync
 */
#[skyline::hook(replace = OFFSET1)]
unsafe fn set_present_interval_nvn(window: u64, _: i32) {
    call_original!(window, 0);
}

/** Patches an internal presentation/surface device implementation for setting the present interval
 *
 * This method has some checks on the maximum and minimum supported values, but if we set it to zero it
 * still works properly, so we forcibly set it to zero.
 */
#[skyline::hook(replace = OFFSET2, inline)]
unsafe fn set_present_interval_android(ctx: &mut InlineCtx) {
    ctx.registers[8].set_x(0);
}

// This is public because it might need to be set from the profiling implementation
// if we enable profiling
pub static RUN: AtomicBool = AtomicBool::new(false);

/** Hooks into smash's VsyncUpdate method (which is basically just a physical frame counter)
 *
 * We use this to manually synchronize on a vsync. We try to synchronize one vsync later than the game would normally expect to be at
 * to constantly be in a "running behind" state, which shaves a frame of input delay off.
 *
 * Note that it is not because we are forcing the frame pacer to work overtime, but rather because we are effectively only ever using 2 frame buffers
 * (imagine the game was double buffered)
 */
#[skyline::hook(offset = 0x3810a64, inline)]
unsafe fn vsync_count_thread(_: &skyline::hooks::InlineCtx) {
    RUN.store(true, Ordering::SeqCst);
}

pub fn install(profiling_enabled: bool) {
    if !profiling_enabled {
        skyline::install_hook!(vsync_count_thread);
    }

    unsafe {
        OFFSET1 = calc_nnsdk_offset() + 0x429d60;
        OFFSET2 = calc_nnsdk_offset() + 0x26e94;
    }

    skyline::install_hooks!(
        run_scene_update,
        set_present_interval_nvn,
        set_present_interval_android
    );
}
