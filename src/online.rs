
#[cfg(feature = "latency-slider")]
pub mod EmuNetplay  {
    use crate::SyncEnv;
    use crate::LatencySlider;
    use crate::swapchain::*;
    
    pub fn check_online_fix_emu() {
        if (LatencySlider::Is_Online() && !SyncEnv::online_fix_enabled()) {
            patch_enable_online_fix();
        } else {
            patch_disable_online_fix();
        }
    }
    
    fn patch_enable_online_fix() {
        println!("[ssbusync] EnabledOnline Emulator Fix \n");
        SyncEnv::set_online_fix_enabled(true);
        patch_zero_ahead_index();
    }
    
    fn patch_disable_online_fix() {
        println!("[ssbusync] Disabled Online Emulator Fix \n");
        SyncEnv::set_online_fix_enabled(false);
        patch_one_ahead_index();
    }
    
}

// pub mod SwitchNetplay  {
//     use crate::SyncEnv;
//     use crate::LatencySlider;
//     use crate::swapchain::*;
    
    
//     }

    


    

