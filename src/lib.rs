#![allow(warnings)]
mod off_by_one;
mod pacer;
mod profiling;
mod sequencing;
mod swapchain;
//mod vsync;
mod vsync_history;
mod compatibility;

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct SsbuSyncConfig {
    pub disable_vsync: bool,
    pub disable_pacer: bool,
    pub enable_triple_buffer: bool,
    pub emulator_check: Option<bool>,
}

impl Default for SsbuSyncConfig {
    fn default() -> Self {
        Self {
            disable_vsync: true,
            disable_pacer: false,
            enable_triple_buffer: false,
            emulator_check: Some(is_emulator()),
        }
    }
}

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

pub fn Install_SSBU_Sync(config: SsbuSyncConfig) {
    let emulator = config.emulator_check.unwrap_or_else(is_emulator);
    println!("[ssbusync] Installing Hooks. \n");
    if emulator {println!("[ssbusync] Emulator Detected. \n"); }
    
    vsync_history::install();
    swapchain::install(config);
    off_by_one::install();

    // Emulator always forces pacer-disable
    if config.disable_pacer || emulator {
        pacer::install();
    }
}

pub fn Enable_Double_Buffer() {
    if !is_emulator() {
        swapchain::enable_double_buffer();
    }
}

pub fn Enable_Triple_Buffer() {
    if !is_emulator() {
        swapchain::enable_triple_buffer();
    }
}

#[cfg(feature = "nro-entry")]
#[skyline::main(name = "ssbusync")]
pub fn main() {
    unsafe {
        if compatibility::disablers() {
            println!("[ssbusync] Disabler detected -> not installing hooks. \n");
            return;
        }
    }
    println!("[ssbusync] No disablers detected. \n");
    Install_SSBU_Sync(SsbuSyncConfig::default());
    // profiling::setup();
    // sequencing::install();
}
