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
#[skyline::hook(offset = 0x3747b78, inline)]
unsafe fn flush_swap_buffers_before_present(_: &skyline::hooks::InlineCtx) {
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
#[skyline::hook(offset = 0x384f460)]
unsafe fn full_swapchain_flush(arg1: u64, arg2: u32) {
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

pub fn install(is_vsync_disabled: bool) {
    patch_swap_flush_call();
    use_current_frame_index();

    if is_vsync_disabled {
        patch_render_sync_wait();
    }

    skyline::install_hooks!(flush_swap_buffers_before_present, full_swapchain_flush);
}
