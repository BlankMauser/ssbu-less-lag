use super::*;
use crate::render::buffer_swap::*;
use std::sync::atomic::{AtomicU64, AtomicU8, Ordering};
use symbaker::{symbaker};

static WINDOW_TARGET: AtomicU64 = AtomicU64::new(0);
static PENDING_WINDOW_TEXTURES: AtomicU8 = AtomicU8::new(0);
static mut FRAME_INDEX_MODULO: u64 = 3;
static mut SET_WINDOW_NUM_ACTIVE_TEXTURES_FN: Option<extern "C" fn(u64, i32)> = None;
static mut GET_WINDOW_NUM_ACTIVE_TEXTURES_FN: Option<extern "C" fn(u64) -> i32> = None;
static mut GET_WINDOW_NUM_TEXTURES_FN: Option<extern "C" fn(u64) -> i32> = None;

const WINDOW_TARGET_VALID_BIT: u64 = 1;

#[inline]
fn encode_window_target(ptr: u64) -> u64 {
    (ptr & !WINDOW_TARGET_VALID_BIT) | WINDOW_TARGET_VALID_BIT
}

#[inline]
fn decode_window_target(raw: u64) -> u64 {
    raw & !WINDOW_TARGET_VALID_BIT
}

#[inline]
pub(crate) fn window_target_is_valid() -> bool {
    (WINDOW_TARGET.load(Ordering::Acquire) & WINDOW_TARGET_VALID_BIT) != 0
}

pub(crate) fn observe_window_target(window_target: u64, source: &str) {
    if window_target == 0 {
        return;
    }

    let raw = WINDOW_TARGET.load(Ordering::Acquire);
    let current = decode_window_target(raw);
    let valid = (raw & WINDOW_TARGET_VALID_BIT) != 0;

    if !valid {
        WINDOW_TARGET.store(encode_window_target(window_target), Ordering::Release);
        println!("[ssbu-sync] cached window target from {source}: 0x{window_target:x}");
        return;
    }

    if current != window_target {
        WINDOW_TARGET.store(encode_window_target(window_target), Ordering::Release);
        println!(
            "[ssbu-sync] window target updated from {source}: 0x{current:x} -> 0x{window_target:x}"
        );
    }
}
//static SET_WINDOW_HOOK_HITS: AtomicU64 = AtomicU64::new(0);

/// Returns the cached NVN window pointer (0 if not yet seen).
pub(crate) fn window_target() -> u64 {
    decode_window_target(WINDOW_TARGET.load(Ordering::Acquire))
}

#[inline]
fn normalize_texture_count(num: i32) -> Option<u8> {
    match num {
        2 => Some(2),
        3 => Some(3),
        _ => None,
    }
}

pub fn set_runtime_frame_index_mode(triple: bool) {
    unsafe {
        FRAME_INDEX_MODULO = if triple { 3 } else { 2 };
    }
}

unsafe fn resolve_set_window_num_active_textures_fn() -> extern "C" fn(u64, i32) {
    if let Some(func_ptr) = SET_WINDOW_NUM_ACTIVE_TEXTURES_FN {
        return func_ptr;
    }

    let func_ptr = *skyline::hooks::getRegionAddress(skyline::hooks::Region::Text)
        .cast::<u8>()
        .add(0x593fb80)
        .cast::<extern "C" fn(u64, i32)>();
    SET_WINDOW_NUM_ACTIVE_TEXTURES_FN = Some(func_ptr);
    func_ptr
}

unsafe fn resolve_get_window_num_active_textures_fn() -> extern "C" fn(u64) -> i32 {
    if let Some(func_ptr) = GET_WINDOW_NUM_ACTIVE_TEXTURES_FN {
        return func_ptr;
    }

    let func_ptr = *skyline::hooks::getRegionAddress(skyline::hooks::Region::Text)
        .cast::<u8>()
        .add(0x593fb88)
        .cast::<extern "C" fn(u64) -> i32>();
    GET_WINDOW_NUM_ACTIVE_TEXTURES_FN = Some(func_ptr);
    func_ptr
}

unsafe fn resolve_get_window_num_textures_fn() -> extern "C" fn(u64) -> i32 {
    if let Some(func_ptr) = GET_WINDOW_NUM_TEXTURES_FN {
        return func_ptr;
    }

    let func_ptr = *skyline::hooks::getRegionAddress(skyline::hooks::Region::Text)
        .cast::<u8>()
        .add(0x593fb90)
        .cast::<extern "C" fn(u64) -> i32>();
    GET_WINDOW_NUM_TEXTURES_FN = Some(func_ptr);
    func_ptr
}

pub(crate) fn get_window_num_active_textures_fn() -> extern "C" fn(u64) -> i32 {
    unsafe { resolve_get_window_num_active_textures_fn() }
}

pub(crate) fn get_window_num_textures_fn() -> extern "C" fn(u64) -> i32 {
    unsafe { resolve_get_window_num_textures_fn() }
}

unsafe fn set_window_textures_impl(window_target: u64, count: i32) {
    let func_ptr = resolve_set_window_num_active_textures_fn();
    func_ptr(window_target, count);
}

pub fn apply_pending_window_texture_request(window_target: u64, source: &str) -> bool {
    let Some(requested) = normalize_texture_count(PENDING_WINDOW_TEXTURES.load(Ordering::Acquire) as i32) else {
        return false;
    };
    if window_target == 0 {
        return false;
    }

    unsafe {
        set_window_textures_impl(window_target, requested as i32);
        let current = resolve_get_window_num_active_textures_fn()(window_target);
        if current == requested as i32 {
            PENDING_WINDOW_TEXTURES.store(0, Ordering::Release);
            println!(
                "[ssbu-sync] applied queued window texture request from {source}: {requested}"
            );
            return true;
        }
    }
    false
}

unsafe fn cache_window_target_from_ctx(ctx: &skyline::hooks::InlineCtx) -> u64 {
    let window_target = *((ctx.registers[23].x() + 0x10) as *const u64);
    // Avoid poisoning the cache with null/invalid bootstrap values.
    observe_window_target(window_target, "cache_window_target_from_ctx");
    window_target
}

/** Ultimate Render Pipeline Docs
 * Ultimate makes use of multi-threaded rendering, and does so very poorly.
 *
 * Well, poorly is probably a harsh term for what they are doing. They
 * are attempting to optimize for the switch hardware in such a way that
 * there will never be frame drops, at the cost of more input delay.
 *
 * They do this in multiple ways, but basically you can imagine a swapchain
 * class such that
 * ```
 * class SwapchainDispatches {
 *      RenderDispatchThread*   pThreads[2];
 *      nvn::Queue*             pQueue;
 *      RenderDispatches*       pDispatchNow;
 *      RenderDispatches*       pDispatchNext;
 *
 *      void SubmitDispatch(RenderDispatches* next) {
 *          pDispatchNext = next;
 *      }
 *
 *      void AwaitAndSubmitDispatches() {
 *          if (pDispatchNow != nullptr) {
 *              RenderDispatchThread* pThread = this->pThreads[pDispatchNow->ThreadId];
 *              while (pThread->CurrentBatch != pThread->BatchEnd) { std::this_thread::yield(); } // Awaits render dispatch thread to finish
 *              pDispatchNow->SubmitToQueue(pQueue);
 *          } else {
 *              pDispatchNow = pDispatchNext;
 *              pDispatchNext = nullptr;
 *          }
 *      }
 * };
 * ```
 *
 * This isn't exactly the class is implemented, however it's a close enough approximation.
 *
 * There are threads called "TaskWorkerX" where X is either 0, 1, or 2, that perform various asynchronous
 * running tasks throughout the game's lifecycle, but they also include render tasks! They build onto the dispatches
 * that are inevitably sent to the "render dispatch" threads. The main loop might look something a little like this
 * (this is a very simplified view, and many things I do not understand are happening that I need to investigate further)
 *
 * ```
 * void MainLoop() {
 *      while (true) {
 *          PresentAndAcquireNextTexture();
 *          pSwapchain->AwaitAndSubmitDispatches();
 *          UpdateTaskWorker(s_EffectManager);          // Submits effect subsystem render commands to dispatcher
 *          UpdateTaskWorker(s_UiManager);              // Submits ui subsystem render commands to dispatcher
 *          UpdatetaskWorker(s_BattleObjectManager);    // Submits battle object render commands to dispatcher
 *          SignalRenderDispatchBegin();                // Signals that render dispatch threads can begin processing their submitted tasks
 *          PollInputs();                               // Polls user inputs
 *          RunScene();                                 // Runs core game state update (updates UI, battles, everything)
 *          if (s_FramePacer->ShouldRunAgain()) {       // Checks if we are running behind, then runs the scene again
 *              PollInputs();
 *              RunScene();
 *          }
 *      }
 * }
 * ```
 * In this grossly oversimplified representation of the main loop, there are two concerning choices:
 * 1. We are polling inputs and running the scene update *after* the current state is rendered. That means that
 *      when the frame is presented that is being rendered on any given invocation of the loop, it's going to represent the
 *      state that the last frame finished with.
 * 2. Our frame pacer is just running a second frame without special casing input handling (such as by using timestamps)
 *      which means that sometimes, randomly, we will get a frame that has 1 frame less input lag. In practice, it's worse than that
 *      because you might think you have a 3 frame window for an input but you actually have a 2 frame window for an input.
 *
 * Another, more subtle, problem is the implementation of `PresentAndAcquireNextTexture`. Because we have to call it twice before it actually
 * takes the path of processing a render dispatch, we have 2 frames of input lag. You might also realize that the texture index we acquire
 * at the start of the loop is presented at the beginning of the next loop. So if we acquire texture index `2`, then we present texture index `2`.
 *
 * In order to accommodate the extra 2 frames, Ultimate's render pipeline is configured to render to the `index - 1`th texture since the commands
 * do not get processed until 2 frames later.
 *
 * Yeah, it's a mess.
 */

/** Flushes the swapchain buffers before the call to nvnQueuePresentTexture
 *
 * This hook is placed on the callsite of `PresentAndAcquireNextTexture` as described above. The first operation of
 * `PresentAndAcquireNextTexture` is to call `nvnQueuePresentTexture` on an acquired texture.
 *
 * This, in combination with the `full_swapchain_flush` hook will take the render commands generated during **this**
 * iteration of the main loop and submit them. This means that we are now presenting state we rendered on the same frame.
 *
 * This has the possibility of taking a while if the render dispatch buffers are not done yet. Should be profiled and investigated
 * in situations where frame drops happen.
 */
#[symbaker]
#[skyline::hook(offset = 0x3747b78, inline)]
unsafe fn flush_swap_buffers_before_present(ctx: &skyline::hooks::InlineCtx) {
    // SAFETY: This method is only ever called in one spot, this is effectively a local variable that we are using
    //      to cache the pointer.
    //
    //      We could fetch it from the stack but instead I would just like to cache it locally
    static mut POINTER: Option<*const ()> = None;

    #[allow(static_mut_refs)]
    if POINTER.is_none() {
        POINTER = Some(
            *skyline::hooks::getRegionAddress(skyline::hooks::Region::Text)
                .cast::<u8>()
                .add(0x5334e90)
                .cast::<*const ()>(),
        );
    }

    let ptr = POINTER.unwrap_unchecked().cast::<*const ()>();
    let ptr = **ptr.cast::<*const *const ()>().add(1);
    let sub_ptr = *ptr.cast::<*const ()>().add(0x88 / 8);
    let flag = *ptr.cast::<u8>().add(0xec);
    let function = *(*sub_ptr.cast::<*const *const ()>())
        .add(0x3)
        .cast::<extern "C" fn(*const (), u8)>();

    function(sub_ptr, flag);
    *ptr.cast::<u8>().cast_mut().add(0xec) = 0;
}

/** Calls Swapchain::AwaitAndSubmitDispatches twice
 *
 * The main problem with the call of AwaitAndSubmitDispatches initially is that it sits after the first texture present/acquire and is initially called
 * when there are no render dispatches to process. In combination with `flush_swap_buffers_before_present`, this hook will immediately move the pDispatchNext
 * batch into the pDispatchNow batch (first invocation) then await the pDispatchNow batch
 */
#[symbaker]
#[skyline::hook(offset = 0x384f460)]
unsafe fn full_swapchain_flush(arg1: u64, arg2: u32) {
    if *(arg1 as *const u8).add(0x1d18) != 0 {
        *(arg1 as *mut u8).add(0x1d30).cast::<u64>() =
            (!*(arg1 as *const u8).add(0x1d20).cast::<u32>() & 1) as u64;
        *(arg1 as *mut u8).add(0x1d28) = 1;
    }
    call_original!(arg1, arg2)
    // static mut RUN_COUNT: usize = 0;
    // if RUN_COUNT == 1 {
    //     call_original!(arg1, arg2);
    //     call_original!(arg1, arg2);
    // } else {
    //     call_original!(arg1, arg2);
    // }
    // RUN_COUNT += 1;
    // // call_original!(arg1, arg2);
}

#[symbaker]
#[skyline::hook(offset = 0x384f460)]
unsafe fn emu_full_swapchain_flush(arg1: u64, arg2: u32) {
    call_original!(arg1, arg2);
    call_original!(arg1, arg2);
    call_original!(arg1, arg2);
}

/** Prevents call to Swapchain::AwaitAndSubmitDispatches in the main loop
 *
 * Because we provide a new call in the `flush_swap_buffers_before_present` hook, we need to prevent the other one from being run.
 * Technically it might do nothing? But it might also have unintendended side-effects so it's more reliable for us to prevent it altogether
 */
fn patch_swap_flush_call() {
    skyline::patching::Patch::in_text(0x37495c8).nop().unwrap();
    skyline::patching::Patch::in_text(0x37495cc).nop().unwrap();
}

/** This changes the behavior of the nu::FrameBufferRenderTarget render command to use the current frame index
 *
 * The default implementation is to get the `(currentFrameIndex + 2) % 3`th texture to render to. This is because
 * the game intends for the render pipeline to be two frames behind. Because we are fully flushing out the swapchain
 * buffers and rendering to the active texture for each frame, we want to use the current texture and not a future one.
 *
 * This patches an instruction `add w9, w9, #0x2` to be `nop`
 */
fn use_current_frame_index() {
    skyline::patching::Patch::in_text(0x386ab4c).nop().unwrap();
}

// FRAMES IN FLIGHT MANAGEMENT:
// SSBU default path is effectively (+2) over a triple-buffered ring.
// Console double-buffer mode uses (+1) % 2.
#[symbaker]
#[skyline::hook(offset = 0x386ab4c, inline)]
fn use_next_frame_index_double(ctx: &mut skyline::hooks::InlineCtx) {
    ctx.registers[9].set_x((ctx.registers[9].x() + 1) % 2);
}

#[symbaker]
#[skyline::hook(offset = 0x386ab4c, inline)]
fn use_next_frame_index_triple(ctx: &mut skyline::hooks::InlineCtx) {
    ctx.registers[9].set_x((ctx.registers[9].x() + 1) % 3);
}

#[symbaker]
#[skyline::hook(offset = 0x386ab4c, inline)]
fn use_next_frame_index_runtime(ctx: &mut skyline::hooks::InlineCtx) {
    let modulo = unsafe { FRAME_INDEX_MODULO };
    ctx.registers[9].set_x((ctx.registers[9].x() + 1) % modulo);
}

#[symbaker]
#[skyline::hook(offset = 0x386ab4c, inline)]
fn emu_use_next_frame_index(ctx: &mut skyline::hooks::InlineCtx) {
    ctx.registers[9].set_x((ctx.registers[9].x()) % 2);
}

/** This disables a sync that is signaled by rendering wrapping up
 *
 * When vsync is disabled, this sync can prevent the start of the frame on the CPU from progressing,
 * which can cause frame drops because we are already forcing the game to "run behind". Disabling it doesn't
 * appear to have any known side effects (at least not when the swapchain is being patched, hence why it is in this file)
 *
 */
fn patch_render_sync_wait() {
    skyline::patching::Patch::in_text(0x386fcec)
        .data(0xD2800000u32)
        .unwrap();
}

// fn restore_render_sync_wait() {
//     // Original instruction at 0x386fcec:
//     //   00 01 3f d6  =>  blr x8
//     skyline::patching::Patch::in_text(0x386fcec)
//         .data(0xD63F0100u32)
//         .unwrap();
// }

#[symbaker]
#[skyline::hook(offset = 0x38601f8, inline)]
unsafe fn set_double_window_textures(ctx: &skyline::hooks::InlineCtx) {
    let window_target = *((ctx.registers[23].x() + 0x10) as *const u64);
    observe_window_target(window_target, "set_double_window_textures");
    set_window_textures_impl(window_target, 2);
}

pub fn try_set_window_textures(num: i32) -> bool {
    let Some(requested) = normalize_texture_count(num) else {
        return false;
    };
    if !window_target_is_valid() {
        return false;
    }

    let window = window_target();
    if window == 0 {
        return false;
    }

    unsafe {
        let total = resolve_get_window_num_textures_fn()(window);
        if total < requested as i32 {
            println!(
                "[ssbu-sync] cannot set active textures to {} because capacity is {}",
                requested, total
            );
            return false;
        }

        set_window_textures_impl(window, requested as i32);
        let active = resolve_get_window_num_active_textures_fn()(window);
        if active == requested as i32 {
            PENDING_WINDOW_TEXTURES.store(0, Ordering::Release);
            return true;
        }

        PENDING_WINDOW_TEXTURES.store(requested, Ordering::Release);
        println!(
            "[ssbu-sync] deferred window texture request: requested={} current={}",
            requested, active
        );
    }

    true
}

pub fn install(config: SsbuSyncConfig) {
    let emulator = config.emulator_check.unwrap();
    
    if emulator {
        patch_swap_flush_call();
    }
    use_current_frame_index();

    if config.disable_vsync {
        patch_render_sync_wait();
    }

    if emulator {
        skyline::install_hooks!(
            flush_swap_buffers_before_present,
            emu_full_swapchain_flush,
            emu_use_next_frame_index
        );
    } else {
        // Console path: keep emulator-only hooks disabled.
        skyline::install_hook!(full_swapchain_flush);
        if config.doubles_fix {
            set_runtime_frame_index_mode(config.enable_triple_buffer);
            skyline::install_hook!(use_next_frame_index_runtime);
        } else if config.enable_triple_buffer {
            skyline::install_hook!(use_next_frame_index_triple);
        } else {
            skyline::install_hook!(use_next_frame_index_double);
            skyline::install_hook!(set_double_window_textures);
        }
    }

    // Seed logical runtime mode.
    let initial = if config.enable_triple_buffer {
        BufferMode::Triple
    } else {
        BufferMode::Double
    };
    init_buffer_mode(initial);

}
