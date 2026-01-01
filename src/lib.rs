mod off_by_one;
mod pacer;
mod profiling;
mod sequencing;
mod swapchain;
mod vsync;
mod vsync_history;

const DISABLE_VSYNC: bool = true;

unsafe extern "C" {
    #[link_name = "_ZN2nn2os16GetCurrentThreadEv"]
    pub fn get_current_thread() -> u64;

    #[link_name = "_ZN2nn2os20ChangeThreadPriorityEPNS0_10ThreadTypeEi"]
    pub fn change_thread_priority(thread: u64, prio: i32);
}

#[skyline::main(name = "testing")]
pub fn main() {
    vsync_history::install();
    swapchain::install(DISABLE_VSYNC);
    off_by_one::install();
    pacer::install();
    // profiling::setup();
    // sequencing::install();

    // if DISABLE_VSYNC {
    //     vsync::install(true);
    // }
}
