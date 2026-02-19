mod off_by_one;
mod pacer;
mod profiling;
mod sequencing;
mod swapchain;
//mod vsync;
mod vsync_history;

const DISABLE_VSYNC: bool = true;

pub fn is_emulator() -> bool {
    unsafe {
        let text_addr = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
        if text_addr == 0x8504000 || text_addr == 0x80004000 {
            // println!("we are on Emulator");
            return true;
        } else {
            // println!("we are not on Emulator");
            return false;
        }
    }
}

unsafe extern "C" {
    #[link_name = "_ZN2nn2os16GetCurrentThreadEv"]
    pub fn get_current_thread() -> u64;

    #[link_name = "_ZN2nn2os20ChangeThreadPriorityEPNS0_10ThreadTypeEi"]
    pub fn change_thread_priority(thread: u64, prio: i32);
}

#[skyline::main(name = "immediate")]
pub fn main() {
    vsync_history::install();
    swapchain::install(DISABLE_VSYNC, is_emulator());
    off_by_one::install();
    
    if is_emulator(){
        pacer::install();
    }
    // pacer::install();
    // profiling::setup();
    // sequencing::install();

    // if DISABLE_VSYNC {
    //     vsync::install(true);
    // }
}
