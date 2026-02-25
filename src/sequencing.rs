use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::Instant,
};
use symbaker::symbaker;
use skyline::{hooks::InlineCtx, patching::Patch};

use crate::{profiling::OsTick, vsync_history::get_system_tick};

fn patch_scene_manager_calls() {
    Patch::in_text(0x374c624).nop().unwrap();
    Patch::in_text(0x374c2dc).nop().unwrap();
    Patch::in_text(0x374c410).nop().unwrap();
}

#[symbaker]
#[skyline::from_offset(0x3724a80)]
unsafe fn run_scene_manager_impl(ptr: u64);

static SHOULD_RUN: AtomicBool = AtomicBool::new(true);

fn runner_thread(ptr: u64) {
    loop {
        while !SHOULD_RUN.load(std::sync::atomic::Ordering::Acquire) {
            std::thread::yield_now();
        }

        unsafe { run_scene_manager_impl(ptr) };

        SHOULD_RUN.store(false, std::sync::atomic::Ordering::Release);
    }
}

static mut TOP: Option<Instant> = None;
// 374b4d4
// Deprecated scene manager hook 
#[symbaker]
#[skyline::hook(offset = 0x374b290, inline)]
pub unsafe fn run_scene_manager(ctx: &InlineCtx) {
    let ptr = *skyline::hooks::getRegionAddress(skyline::hooks::Region::Text)
        .cast::<u8>()
        .add(0x593a4c0)
        .cast::<u64>();

    TOP = Some(Instant::now());

    crate::profiling::start_frame(OsTick::new(unsafe { get_system_tick() }));
    run_scene_manager_impl(ptr);

    // super::off_by_one::post_scene_update_submit_render(ctx);

    // if SHOULD_RUN.swap(true, Ordering::AcqRel) {
    //     let _ = std::thread::spawn(move || runner_thread(ptr));
    // }

    // TOP = Some(Instant::now());

    // while SHOULD_RUN.load(Ordering::Acquire) {
    //     std::thread::yield_now();
    // }
}

#[symbaker]
#[skyline::hook(offset = 0x374c050, inline)]
unsafe fn log_frame_duration(ctx: &InlineCtx) {
    // println!(
    //     "CPU Frame took {:.3}ms",
    //     (Instant::now() - TOP.unwrap_unchecked()).as_micros() as f32 / 1000.0
    // );
}

#[symbaker]
#[skyline::hook(offset = 0x374b130, inline)]
unsafe fn run_scene_wait(ctx: &InlineCtx) {
    // println!(
    //     "in paralle: {:.3}ms",
    //     dbg!((Instant::now() - TOP.unwrap_unchecked()).as_micros()) as f32 / 1000.0
    // );
    // while SHOULD_RUN.load(Ordering::Acquire) {
    //     std::thread::yield_now();
    // }
}

// #[skyline::hook(offset = 0x374b290, inline)]
// unsafe fn log_thing(_: &InlineCtx) {
//     TOP = Some(Instant::now());
// }

// #[skyline::hook(offset = 0x374b4f4, inline)]
// unsafe fn log_thing_2(_: &InlineCtx) {
//     println!(
//         "Duration: {:.3}ms",
//         (Instant::now() - TOP.unwrap_unchecked()).as_micros() as f32 / 1000.0
//     );
// }

#[symbaker]
#[skyline::hook(offset = 0x386fc80, inline)]
unsafe fn wait_on_present_sync(ctx: &InlineCtx) {
    static mut NVN_SYNC_WAIT: Option<extern "C" fn(u64, u64)> = None;
    if NVN_SYNC_WAIT.is_none() {
        let base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text).cast::<u8>();
        NVN_SYNC_WAIT = Some(*base.add(0x5940878).cast::<extern "C" fn(u64, u64)>())
    }
    let now = Instant::now();
    let nvn_sync_wait = NVN_SYNC_WAIT.unwrap_unchecked();
    let sync = *((ctx.registers[23].x() + 0x28) as *const u64);
    nvn_sync_wait(sync, u64::MAX);
    println!("Stalled {:.3}ms", now.elapsed().as_micros() as f32 / 1000.0);
}

#[symbaker]
#[skyline::hook(offset = 0x386fcdc, inline)]
unsafe fn queue_wait_sync(ctx: &InlineCtx) {
    static mut NVN_QUEUE_WAIT_SYNC: Option<extern "C" fn(u64, u64) -> bool> = None;

    if NVN_QUEUE_WAIT_SYNC.is_none() {
        let base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text).cast::<u8>();
        NVN_QUEUE_WAIT_SYNC = Some(
            *base
                .add(0x5940880)
                .cast::<extern "C" fn(u64, u64) -> bool>(),
        )
    }

    let nvn_queue_wait_sync = NVN_QUEUE_WAIT_SYNC.unwrap_unchecked();
    let queue = *((ctx.registers[23].x() + 0x18) as *const u64);
    nvn_queue_wait_sync(queue, ctx.registers[0].x());
    println!("here");
}

fn patch_sync_wait() {
    Patch::in_text(0x386fcdc).nop().unwrap();
}

pub fn install() {
    patch_scene_manager_calls();
    patch_sync_wait();
    skyline::install_hooks!(run_scene_manager);
}
