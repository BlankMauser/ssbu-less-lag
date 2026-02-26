use crate::SyncEnv;

pub fn ToggleOnlineFix(toggle: bool) {
        if SyncEnv::emulator_value() {
            EmuNetplay::toggle_online_fix_emu(toggle);
        } else {
            SwitchNetplay::toggle_online_fix_switch(toggle);
        }
}

pub mod EmuNetplay  
{
    use crate::SyncEnv;
    use crate::swapchain::*;
    
    pub fn toggle_online_fix_emu(toggle: bool) 
    {
        if (SyncEnv::online_fix_enabled() != toggle) 
        {
            toggle_one_ahead_index(toggle);
            SyncEnv::set_online_fix_enabled(toggle);
        }
    }
    
}

pub mod SwitchNetplay  
{
    use crate::SyncEnv;
    use crate::swapchain::*;
    
    pub fn toggle_online_fix_switch(toggle: bool) 
    {
        if (SyncEnv::online_fix_enabled() != toggle) 
        {
            toggle_one_ahead_index(toggle);
            SyncEnv::set_online_fix_enabled(toggle);
        }
    }
    
}
