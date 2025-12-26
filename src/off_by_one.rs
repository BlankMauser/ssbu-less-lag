/** Updates the models of every battle object to match their animation
 *
 * We have to call this manually, normally it's called from TaskWorker2
 */
#[skyline::from_offset(0x3ab590)]
fn update_models(p: *const ());

/** This initializes a task worker, I'm not sure what the arguments do entirely but I believe that one of them
 * is a TlsSlot
 */
#[skyline::from_offset(0x3549170)]
unsafe fn initialize_task_worker(ptr: *mut u8, func: *const (), arg1: u32, arg2: i32);

/** I believe this waits on a task worker to finish running, but I'm not entirely sure
 */
#[skyline::from_offset(0x354c720)]
unsafe fn wait_task_worker(arg1: u32, arg2: *mut u32);

unsafe fn start_task_worker_queue(pointer: *mut (), function_ptr: *const ()) {
    // I could declare a struct for these members, but I don't really want to misrepresent them
    // For now they will stay as offsets and raw pointer math
    let flag = *pointer.add(0x20).cast::<u8>();

    if flag == 1 {
        return;
    }

    // This is supposed to be an atomic store and release, I don't do that though
    *pointer.cast::<u8>().add(0x8) = 1;
    let mut start = pointer.cast::<*mut u8>();
    let mut ptr = *start;

    while !ptr.is_null() {
        let object = *ptr.add(0x48).cast::<*mut u8>();
        let value = *ptr.add(0x70);
        if value != 1 {
            if value == 2 {
                *start = object;
                let vtable = *ptr.cast::<*mut u64>();
                let func_ptr = *vtable.add(1).cast::<extern "C" fn(*mut u8)>();
                (func_ptr)(ptr);
                ptr = object;
                if object.is_null() {
                    break;
                }
                continue;
            }

            initialize_task_worker(
                ptr.add(8),
                function_ptr,
                *ptr.add(0x68).cast::<u32>(),
                *ptr.add(0x6c).cast::<i32>(),
            );
        }

        start = (*start).add(0x48).cast();
        ptr = object;
    }

    *pointer.cast::<u8>().add(0x8) = 0;
}

/** Performs graphics updates manually, replacing the operations we patch out in the rest of this file
 *
 * This method runs immediately after the frame pacer check is done (so it encompasses all possible scene updates).
 * TODO: This method should be profiled and benchmarked for how long some of these operations take. I'm pretty sure
 * it's decently fast but I don't know
 */
#[skyline::hook(offset = 0x374c7b4, inline)]
unsafe fn post_scene_update_submit_render(_: &skyline::hooks::InlineCtx) {
    // SAFETY: These are basically local variables or global constants, and we cache them so that we don't have to fetch them every time from the skyline API
    static mut P_BATTLE_OBJECT_MANAGER: *const *const () = std::ptr::null();
    static mut P_TASK_WORKER_QUEUE: *mut () = std::ptr::null_mut();
    static mut SUBMIT_COMMANDS_FN_PTR: *const () = std::ptr::null();
    static mut FIGHTER_RENDER_COMMANDS_FN_PTR: *const () = std::ptr::null();
    static mut FIGHTER_ARRAY_START: *const () = std::ptr::null();
    static mut DID_INIT: bool = false;

    if !DID_INIT {
        DID_INIT = true;
        let base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text).cast::<u8>();
        P_BATTLE_OBJECT_MANAGER = base.add(0x5332120).cast::<*const ()>();
        P_TASK_WORKER_QUEUE = base.add(0x5332558).cast();
        SUBMIT_COMMANDS_FN_PTR = base.add(0x3548240).cast();
        FIGHTER_RENDER_COMMANDS_FN_PTR = base.add(0x374f050).cast();
        FIGHTER_ARRAY_START = base.add(0x5332f58).cast();
    }

    if !(*P_BATTLE_OBJECT_MANAGER).is_null() {
        update_models(*P_BATTLE_OBJECT_MANAGER);
    }

    // I don't know how much memory this actually takes up but I'm sure it's not *that* much
    let mut task_worker_info = [0u32; 16];

    initialize_task_worker(
        task_worker_info.as_mut_ptr().cast(),
        FIGHTER_RENDER_COMMANDS_FN_PTR,
        0, // 0, 0 taken from invocation we stub
        0,
    );

    if task_worker_info[4] != 0 {
        let tls_slot = *(*FIGHTER_ARRAY_START
            .cast::<*const u8>()
            .add(task_worker_info[5] as usize))
        .add(0x10)
        .cast::<u32>();
        wait_task_worker(tls_slot, task_worker_info.as_mut_ptr().add(4));
    }

    start_task_worker_queue(P_TASK_WORKER_QUEUE, SUBMIT_COMMANDS_FN_PTR);
}

/** This prevents the render dispatch threads from starting
 *
 * This is an inlined call to an equivalent of `start_task_worker_queue`, which we reimplement for `post_scene_update_submit_render`
 */
fn prevent_render_dispatch_signal() {
    skyline::patching::Patch::in_text(0x374c054)
        .data(0x14000029u32)
        .unwrap();
}

/** This patches a vtable function pointer that calls BattleObjectManager::UpdateObjectModels
 *
 * This is called by TaskWorker2. Other tasks executed by the same `start_task_worker` invocation are mission critical to the frame
 * so they cannot be moved. Instead of moving the task worker updates, we just stub that function and call it ourself in
 * `post_scene_update_submit_render`
 */
fn prevent_task_worker_updating_models() {
    extern "C" fn stub() {}

    skyline::patching::Patch::in_text(0x4f623d0)
        .data(stub as *const () as u64)
        .unwrap();
}

/** This prevents adding fighter render command recording tasks to the task worker
 *
 * By default this is called before updating the scene's state for the frame. We do this manually in `post_scene_update_submit_render`
 */
fn prevent_fighter_render_command_recording() {
    skyline::patching::Patch::in_text(0x374b554).nop().unwrap();
}

pub fn install() {
    prevent_render_dispatch_signal();
    prevent_task_worker_updating_models();
    prevent_fighter_render_command_recording();

    skyline::install_hook!(post_scene_update_submit_render);
}
