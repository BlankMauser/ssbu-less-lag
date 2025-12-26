use std::{
    collections::{BTreeMap, HashMap},
    sync::Mutex,
};

use skyline::{hooks::InlineCtx, patching::Patch};

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

#[derive(Default, Copy, Clone)]
pub struct ApplicationFrameInfo {
    pub acquire_invocation: i64,
    pub acquire_finish: i64,
    pub sync_finish: i64,
    pub init_exit: i64,
    pub input: i64,
}

impl ApplicationFrameInfo {
    const fn new() -> Self {
        Self {
            acquire_invocation: 0,
            acquire_finish: 0,
            sync_finish: 0,
            init_exit: 0,
            input: 0,
        }
    }
}

pub static mut FRAMES_IN_FLIGHT: [ApplicationFrameInfo; 3] =
    [const { ApplicationFrameInfo::new() }; 3];

static mut LAYER: Layer = Layer(std::ptr::null_mut());

#[skyline::hook(offset = 0x386fca0, inline)]
unsafe fn call_acquire_texture_wrapper(ctx: &mut InlineCtx) {
    static mut ACQUIRE_TEXTURE_PTR: Option<extern "C" fn(u64, u64, *mut i32) -> u32> = None;
    if ACQUIRE_TEXTURE_PTR.is_none() {
        let base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text);
        ACQUIRE_TEXTURE_PTR = Some(*base.cast::<u8>().add(0x593fb50).cast::<extern "C" fn(
            u64,
            u64,
            *mut i32,
        ) -> u32>());
    }

    let ptr = ACQUIRE_TEXTURE_PTR.unwrap_unchecked();
    let frame_ptr = ctx.registers[2].x() as *mut i32;
    let invocation_tick = get_system_tick();
    let result = ptr(ctx.registers[0].x(), ctx.registers[1].x(), frame_ptr);
    let finish_tick = get_system_tick();
    ctx.registers[0].set_w(result);

    // println!("Acquired {}", *frame_ptr);

    if result == 0 {
        FRAMES_IN_FLIGHT[*frame_ptr as usize].acquire_invocation = invocation_tick;
        FRAMES_IN_FLIGHT[*frame_ptr as usize].acquire_finish = finish_tick;
    }
}

#[skyline::hook(offset = 0x386fce0, inline)]
unsafe fn record_sync_finish(ctx: &InlineCtx) {
    let ptr = (ctx.registers[23].x() + 0x30) as *const i32;

    FRAMES_IN_FLIGHT[*ptr as usize].sync_finish = get_system_tick();
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
        let frame_in_flight = FRAMES_IN_FLIGHT[frame as usize];
        crate::frametracer::frame(
            "window_acquire",
            frame_number as usize,
            frame_in_flight.acquire_finish,
        );
        crate::frametracer::frame(
            "wait_texture",
            frame_number as usize,
            frame_in_flight.sync_finish,
        );
        crate::frametracer::frame(
            "previous_submit_finish",
            frame_number as usize,
            frame_in_flight.init_exit,
        );
        crate::frametracer::frame("poll_inputs", frame_number as usize, frame_in_flight.input);
        crate::frametracer::frame("queue_present", frame_number as usize, present_tick);
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

        let num_frames =
            unsafe { list_frame_info(frame_infos.as_mut_ptr(), max_frame_info, layer) };

        println!("{:?}", &frame_infos[..num_frames as usize]);

        for frame_info in &frame_infos[..num_frames as usize] {
            match frame_info.status {
                FrameStatus::Unknown => {}
                FrameStatus::Enqueued => {
                    if frame_info.frame_number > last_frame_enqueued {
                        crate::frametracer::frame(
                            "vi_enqueue",
                            frame_info.frame_number as usize,
                            frame_info.enqueue_time,
                        );
                        last_frame_enqueued = frame_info.frame_number;
                    }
                }
                FrameStatus::Presented => {
                    if frame_info.frame_number > last_frame_presented {
                        println!("Finishing!");
                        crate::frametracer::frame(
                            "vi_presented",
                            frame_info.frame_number as usize,
                            frame_info.enqueue_time,
                        );
                        crate::frametracer::frame(
                            "vi_vblank",
                            frame_info.frame_number as usize,
                            frame_info.vblank_time,
                        );
                        crate::frametracer::finish_frame(frame_info.frame_number as usize);
                        crate::frametracer::marker("vblank", frame_info.vblank_time);
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

    LAYER = layer;

    let _ = std::thread::spawn(move || {
        vsync_thread(layer, display);
    });
}

pub unsafe fn flush() {
    static mut POINTER: Option<*const ()> = None;
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

pub unsafe fn current_frame_in_flight() -> i32 {
    static mut POINTER: Option<*const ()> = None;
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
    let ptr = *ptr.cast::<u8>().add(0xa8).cast::<*const ()>();
    *ptr.cast::<u8>().add(0x30).cast::<i32>()
}

#[skyline::hook(offset = 0x386ff14, inline)]
unsafe fn record_init_exit(ctx: &InlineCtx) {
    let tick = get_system_tick();
    let p_frame = (*((ctx.registers[19].x() + 0xa8) as *const u64) + 0x30) as *const i32;
    FRAMES_IN_FLIGHT[*p_frame as usize].init_exit = tick;
}

#[skyline::hook(offset = 0x374c118, inline)]
unsafe fn record_input(ctx: &InlineCtx) {
    let tick = get_system_tick();
    // println!("{}", current_frame_in_flight());
    FRAMES_IN_FLIGHT[current_frame_in_flight() as usize].input = tick;
}

#[skyline::hook(offset = 0x386ff28, inline)]
fn print_sync_wait_result(ctx: &InlineCtx) {
    println!("Sync Wait: {}", ctx.registers[8].x());
}

pub fn install() {
    Patch::in_text(0x3860674).nop().unwrap();
    Patch::in_text(0x386fca0).nop().unwrap();
    Patch::in_text(0x386fc80).nop().unwrap();
    Patch::in_text(0x3810a40).data(0xD65F03C0u32).unwrap();

    skyline::install_hooks!(
        grab_vi_layer_handle,
        present_texture_wrapper,
        call_acquire_texture_wrapper,
        record_sync_finish,
        record_init_exit,
        record_input
    );
}
