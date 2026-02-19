use skyline::{hooks::InlineCtx, patching::Patch};

use crate::profiling::OsTick;

#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, PartialOrd, Ord)]
pub enum FrameStatus {
    #[default]
    Unknown,
    Enqueued,
    Presented,
}

#[repr(C, align(16))]
pub struct SystemEvent([u8; 256]);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Display(*mut ());

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Layer(*mut ());

unsafe impl Send for Display {}
unsafe impl Sync for Display {}
unsafe impl Send for Layer {}
unsafe impl Sync for Layer {}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct FrameInfo {
    pub status: FrameStatus,
    pub frame_number: u64,
    pub enqueue_time: i64,
    pub present_time: i64,
    pub vblank_time: i64,
}

unsafe extern "C" {
    #[link_name = "_ZN2nn2vi16ListFrameHistoryEPNS0_9FrameInfoEiPKNS0_5LayerE"]
    fn list_frame_info(frame_info: *mut FrameInfo, count: i32, layer: Layer) -> i32;

    #[link_name = "_ZN2nn2vi20GetDisplayVsyncEventEPNS_2os15SystemEventTypeEPNS0_7DisplayE"]
    fn get_display_vsync_event(event: *mut SystemEvent, display: Display) -> u32;

    #[link_name = "_ZN2nn2os15WaitSystemEventEPNS0_15SystemEventTypeE"]
    fn wait_system_event(event: *mut SystemEvent);

    #[link_name = "_ZN2nn2vi20GetLatestFrameNumberEPmPKNS0_5LayerE"]
    fn get_latest_frame_number(number: &mut u64, layer: Layer) -> u32;

    #[link_name = "_ZN2nn2os13GetSystemTickEv"]
    pub fn get_system_tick() -> i64;

    #[link_name = "_ZN2nn2os17ConvertToTimeSpanENS0_4TickE"]
    fn convert_to_timespan(tick: i64) -> i64;
}

static mut SYSTEM_EVENT: SystemEvent = SystemEvent([0u8; 256]);

static mut LAYER: Layer = Layer(std::ptr::null_mut());
static mut DISPLAY: Display = Display(std::ptr::null_mut());
static mut FRAME_INFOS: Vec<FrameInfo> = Vec::new();
static mut LAST_ENQUEUED: usize = 0;
static mut LAST_PRESENTED: usize = 0;

#[skyline::hook(offset = 0x386fca0, inline)]
unsafe fn call_acquire_texture_wrapper(ctx: &mut InlineCtx) {
    static mut ACQUIRE_TEXTURE_PTR: Option<extern "C" fn(u64, u64, *mut i32) -> u32> = None;
    static mut QUEUE_WAIT_SYNC_PTR: Option<extern "C" fn(u64, u64)> = None;
    if ACQUIRE_TEXTURE_PTR.is_none() {
        let base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text);
        ACQUIRE_TEXTURE_PTR = Some(*base.cast::<u8>().add(0x593fb50).cast::<extern "C" fn(
            u64,
            u64,
            *mut i32,
        ) -> u32>());
        QUEUE_WAIT_SYNC_PTR = Some(
            *base
                .cast::<u8>()
                .add(0x5940880)
                .cast::<extern "C" fn(u64, u64)>(),
        );
    }

    let ptr = ACQUIRE_TEXTURE_PTR.unwrap_unchecked();
    let frame_ptr = ctx.registers[2].x() as *mut i32;
    let invocation_tick = get_system_tick();
    let result = ptr(ctx.registers[0].x(), ctx.registers[1].x(), frame_ptr);
    let finish_tick = get_system_tick();
    ctx.registers[0].set_w(result);

    if result == 0 {
        crate::profiling::span(
            "nvnWindowAcquireTexture",
            OsTick::new(invocation_tick),
            OsTick::new(finish_tick),
        );

        (QUEUE_WAIT_SYNC_PTR.unwrap_unchecked())(
            *((ctx.registers[23].x() + 0x18) as *const u64),
            ctx.registers[1].x(),
        );
    }
}

#[skyline::hook(offset = 0x374c118, inline)]
unsafe fn profile_sync_wait(ctx: &mut InlineCtx) {
    static mut SYNC_WAIT: Option<extern "C" fn(u64, u64) -> u64> = None;

    if SYNC_WAIT.is_none() {
        SYNC_WAIT = Some(
            *skyline::hooks::getRegionAddress(skyline::hooks::Region::Text)
                .cast::<u8>()
                .add(0x5940878)
                .cast::<extern "C" fn(u64, u64) -> u64>(),
        );
    }

    let p_singleton = *((ctx.sp.x() + 0x268) as *const u64);
    let p_swapchain = **((p_singleton + 0x8) as *const *const u64);
    let p_gfx_device = *((p_swapchain + 0xa8) as *const u64);
    let p_render_sync = *((p_gfx_device + 0x20) as *const u64);
    let p_texture_sync = *((p_gfx_device + 0x28) as *const u64);

    crate::profiling::start_span(
        "nvnWaitSync(pTextureAvailable)",
        OsTick::new(get_system_tick()),
    );

    (SYNC_WAIT.unwrap_unchecked())(p_texture_sync, u64::MAX);

    crate::profiling::end_span(OsTick::new(get_system_tick()));

    crate::profiling::start_span("WaitForVBlank", OsTick::new(get_system_tick()));

    unsafe {
        wait_system_event(&mut SYSTEM_EVENT);
    }

    crate::profiling::end_span(OsTick::new(get_system_tick()));

    let num_frames =
        unsafe { list_frame_info(FRAME_INFOS.as_mut_ptr(), FRAME_INFOS.len() as i32, LAYER) };

    for frame_info in &FRAME_INFOS[..num_frames as usize] {
        match frame_info.status {
            FrameStatus::Unknown => {}
            FrameStatus::Enqueued => {}
            FrameStatus::Presented => {
                if frame_info.frame_number > LAST_PRESENTED as u64 {
                    crate::profiling::finish_frame(
                        frame_info.frame_number as usize,
                        OsTick::new(frame_info.present_time),
                    );
                    crate::profiling::vblank(OsTick::new(frame_info.vblank_time));
                    LAST_PRESENTED = frame_info.frame_number as usize;
                }
            }
        }
    }
}

#[skyline::hook(offset = 0x386fc80, inline)]
unsafe fn present_texture_wrapper(ctx: &InlineCtx) {
    static mut PRESENT_TEXTURE_PTR: Option<extern "C" fn(u64, u64, i32)> = None;
    if PRESENT_TEXTURE_PTR.is_none() {
        let base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text);
        PRESENT_TEXTURE_PTR = Some(
            *base
                .cast::<u8>()
                .add(0x593fac8)
                .cast::<extern "C" fn(u64, u64, i32)>(),
        );
    }

    let ptr = PRESENT_TEXTURE_PTR.unwrap_unchecked();

    let frame = ctx.registers[2].w() as i32;
    let present_tick = get_system_tick();
    ptr(ctx.registers[0].x(), ctx.registers[1].x(), frame);

    let mut frame_number = 0u64;
    if get_latest_frame_number(&mut frame_number, LAYER) == 0 {
        crate::profiling::submit_frame(frame_number as usize, OsTick::new(present_tick));
        crate::profiling::start_frame(OsTick::new(present_tick));
    }
}

pub fn vsync_thread(layer: Layer, display: Display) {
    let mut system_event = SystemEvent([0u8; 256]);

    unsafe {
        assert_eq!(get_display_vsync_event(&mut system_event, display), 0);
    }

    let max_frame_info = unsafe { list_frame_info(std::ptr::null_mut(), 0, layer) };

    let mut frame_infos = vec![FrameInfo::default(); max_frame_info as usize];

    let mut last_frame_enqueued = 0;
    let mut last_frame_presented = 0;

    loop {
        unsafe {
            wait_system_event(&mut system_event);
        }
        // crate::vsync::RUN.store(true, Ordering::SeqCst);

        let num_frames =
            unsafe { list_frame_info(frame_infos.as_mut_ptr(), max_frame_info, layer) };

        for frame_info in &frame_infos[..num_frames as usize] {
            match frame_info.status {
                FrameStatus::Unknown => {}
                FrameStatus::Enqueued => {
                    if frame_info.frame_number > last_frame_enqueued {
                        last_frame_enqueued = frame_info.frame_number;
                    }
                }
                FrameStatus::Presented => {
                    if frame_info.frame_number > last_frame_presented {
                        crate::profiling::finish_frame(
                            frame_info.frame_number as usize,
                            OsTick::new(frame_info.present_time),
                        );
                        crate::profiling::vblank(OsTick::new(frame_info.vblank_time));
                        last_frame_presented = frame_info.frame_number;
                    }
                }
            }
        }
    }
}

#[skyline::hook(offset = 0x3743ca0, inline)]
unsafe fn grab_vi_layer_handle(ctx: &InlineCtx) {
    let p_display_info = ctx.registers[19].x();

    let display = *((p_display_info + 0x70) as *const Display);
    let layer = *((p_display_info + 0x78) as *const Layer);

    DISPLAY = display;
    LAYER = layer;

    unsafe {
        assert_eq!(get_display_vsync_event(&mut SYSTEM_EVENT, display), 0);
    }

    let max_frame_info = unsafe { list_frame_info(std::ptr::null_mut(), 0, layer) };

    FRAME_INFOS = vec![FrameInfo::default(); max_frame_info as usize];
}

#[skyline::from_offset(0x3864700)]
fn init_renderpasses(arg: u64);

#[skyline::hook(offset = 0x374b11c, inline)]
fn profile_init_renderpass(ctx: &InlineCtx) {
    crate::profiling::start_span(
        "InitRenderpasses",
        OsTick::new(unsafe { get_system_tick() }),
    );
    unsafe { init_renderpasses(ctx.registers[0].x()) };
    crate::profiling::end_span(OsTick::new(unsafe { get_system_tick() }));
}

#[skyline::from_offset(0x3549170)]
fn init_task_worker(arg1: u64, arg2: u64, arg3: u32, arg4: u32);

#[skyline::hook(offset = 0x374b4f4, inline)]
fn profile_init_ui(ctx: &InlineCtx) {
    crate::profiling::start_span(
        "InitUiTaskWorker",
        OsTick::new(unsafe { get_system_tick() }),
    );
    unsafe {
        init_task_worker(
            ctx.registers[0].x(),
            ctx.registers[1].x(),
            ctx.registers[2].w(),
            ctx.registers[3].w(),
        )
    };
    crate::profiling::end_span(OsTick::new(unsafe { get_system_tick() }));
}

#[skyline::hook(offset = 0x374b524, inline)]
fn profile_init_vfx(ctx: &InlineCtx) {
    crate::profiling::start_span(
        "InitVfxTaskWorker",
        OsTick::new(unsafe { get_system_tick() }),
    );
    unsafe {
        init_task_worker(
            ctx.registers[0].x(),
            ctx.registers[1].x(),
            ctx.registers[2].w(),
            ctx.registers[3].w(),
        )
    };
    crate::profiling::end_span(OsTick::new(unsafe { get_system_tick() }));
}

#[skyline::hook(offset = 0x374b554, inline)]
fn profile_init_battle(ctx: &InlineCtx) {
    crate::profiling::start_span(
        "InitBattleTaskWorker",
        OsTick::new(unsafe { get_system_tick() }),
    );
    unsafe {
        init_task_worker(
            ctx.registers[0].x(),
            ctx.registers[1].x(),
            ctx.registers[2].w(),
            ctx.registers[3].w(),
        )
    };
    crate::profiling::end_span(OsTick::new(unsafe { get_system_tick() }));
}

#[skyline::hook(offset = 0x374b2b0)]
fn profile_unk_taskworker1(ctx: &InlineCtx) {
    crate::profiling::start_span("UnkTaskWorker1", OsTick::new(unsafe { get_system_tick() }));

    crate::profiling::end_span(OsTick::new(unsafe { get_system_tick() }));
}

#[skyline::hook(offset = 0x3724a80)]
fn scene_manager_update(manager: u64) {
    crate::profiling::start_span("RunSceneManager", OsTick::new(unsafe { get_system_tick() }));
    call_original!(manager);
    crate::profiling::end_span(OsTick::new(unsafe { get_system_tick() }));
}

#[skyline::hook(offset = 0x374bd9c, inline)]
fn cmdbuf_reset_span_start(_: &InlineCtx) {
    crate::profiling::start_span(
        "CommandBufferReset",
        OsTick::new(unsafe { get_system_tick() }),
    );
}

#[skyline::hook(offset = 0x374bfe8, inline)]
fn cmdbuf_reset_span_end(_: &InlineCtx) {
    crate::profiling::end_span(OsTick::new(unsafe { get_system_tick() }));
}

#[skyline::hook(offset = 0x374b308, inline)]
fn mutex_lock_span_begin(_: &InlineCtx) {
    crate::profiling::start_span("MutexLock", OsTick::new(unsafe { get_system_tick() }));
}
#[skyline::hook(offset = 0x374b4b4, inline)]
fn mutex_lock_span_end(_: &InlineCtx) {
    crate::profiling::end_span(OsTick::new(unsafe { get_system_tick() }));
}

#[skyline::hook(offset = 0x374b130, inline)]
fn looping_span_begin(_: &InlineCtx) {
    crate::profiling::start_span("Looping", OsTick::new(unsafe { get_system_tick() }));
}

#[skyline::hook(offset = 0x374b160, inline)]
fn looping_span_end(_: &InlineCtx) {
    crate::profiling::end_span(OsTick::new(unsafe { get_system_tick() }));
}

#[skyline::from_offset(0x3619080)]
fn ui_update(arg: u64);

#[skyline::hook(offset = 0x374b124, inline)]
fn call_ui_update(ctx: &InlineCtx) {
    crate::profiling::start_span("UiUpdate", OsTick::new(unsafe { get_system_tick() }));
    unsafe {
        ui_update(ctx.registers[0].x());
    }
    crate::profiling::end_span(OsTick::new(unsafe { get_system_tick() }));
}

pub fn install() {
    Patch::in_text(0x3860674).nop().unwrap();
    Patch::in_text(0x386fca0).nop().unwrap();
    Patch::in_text(0x386fc80).nop().unwrap();
    Patch::in_text(0x374b11c).nop().unwrap();
    Patch::in_text(0x374b124).nop().unwrap();
    Patch::in_text(0x3810a40).data(0xD65F03C0u32).unwrap();
    Patch::in_text(0x386fcdc).nop().unwrap();
    Patch::in_text(0x22deb84).data(0xD2800008u32).unwrap();

    skyline::install_hooks!(
        grab_vi_layer_handle,
        present_texture_wrapper,
        call_acquire_texture_wrapper,
        scene_manager_update,
        profile_init_renderpass,
        cmdbuf_reset_span_start,
        cmdbuf_reset_span_end,
        mutex_lock_span_begin,
        mutex_lock_span_end,
        looping_span_begin,
        looping_span_end,
        call_ui_update,
        profile_sync_wait,
    );
}