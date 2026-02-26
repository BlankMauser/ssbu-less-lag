use symbaker::symbaker_module;

// #[symbaker_module]
// #[cfg(feature = "nro-entry")]
// mod offsets_impl {
//     // Offsets copied from https://github.com/HDR-Development/HewDraw-Remix/blob/pre-release/utils/src/offsets.rs
//     // 
//     #[export_name = "offsets_exec_command"]
//     pub const fn exec_command() -> usize {
//         0x6bac10
//     }

//     #[export_name = "offsets_get_command_flag_cat"]
//     pub const fn get_command_flag_cat() -> usize {
//         0x6ba9a0
//     }

//     #[export_name = "offsets_demon_on_link_capture_event"]
//     pub const fn demon_on_link_capture_event() -> usize {
//         0x933800
//     }

//     #[export_name = "offsets_dolly_super_special_check"]
//     pub const fn dolly_super_special_check() -> usize {
//         0x970ff0
//     }

//     #[export_name = "offsets_dolly_super_special_check_param"]
//     pub const fn dolly_super_special_check_param() -> usize {
//         0x971250
//     }

//     #[export_name = "offsets_force_linear_histun"]
//     pub const fn force_linear_histun() -> usize {
//         0x62ba74
//     }

//     #[export_name = "offsets_get_param_int_impl"]
//     pub const fn get_param_int_impl() -> usize {
//         0x4e53a0
//     }

//     #[export_name = "offsets_get_param_float_impl"]
//     pub const fn get_param_float_impl() -> usize {
//         0x4e53e0
//     }

//     #[export_name = "offsets_set_fighter_vtable"]
//     pub const fn set_fighter_vtable() -> usize {
//         0x14f4994
//     }

//     #[export_name = "offsets_set_weapon_vtable"]
//     pub const fn set_weapon_vtable() -> usize {
//         0x14f4eac
//     }

//     #[export_name = "offsets_set_item_vtable"]
//     pub const fn set_item_vtable() -> usize {
//         0x14f5144
//     }

//     #[export_name = "offsets_get_battle_object_from_id"]
//     pub const fn get_battle_object_from_id() -> usize {
//         0x3ac560
//     }

//     #[export_name = "offsets_fighter_handle_damage"]
//     pub const fn fighter_handle_damage() -> usize {
//         0x6310c0
//     }

//     #[export_name = "offsets_p_p_game_state"]
//     pub const fn p_p_game_state() -> usize {
//         0x52c2760
//     }

//     #[export_name = "offsets_map_controls"]
//     pub const fn map_controls() -> usize {
//         0x1750f70
//     }

//     #[export_name = "offsets_once_per_game_frame"]
//     pub const fn once_per_game_frame() -> usize {
//         0x135b810
//     }

//     #[export_name = "offsets_on_rule_select"]
//     pub const fn on_rule_select() -> usize {
//         0x1792c60
//     }

//     #[export_name = "offsets_global_frame_counter"]
//     pub const fn global_frame_counter() -> usize {
//         0x52e7b44
//     }

//     #[export_name = "offsets_get_match_mode"]
//     pub const fn get_match_mode() -> usize {
//         0x1743870
//     }
  
//     #[export_name = "offsets_kill_zoom_regular"]
//     pub const fn kill_zoom_regular() -> usize {
//         0x633de0
//     }

//     #[export_name = "offsets_kill_zoom_throw"]
//     pub const fn kill_zoom_throw() -> usize {
//         0x6373a4
//     }

//     #[export_name = "offsets_analog_trigger_l"]
//     pub const fn analog_trigger_l() -> usize {
//         0x3666ee0
//     }

//     #[export_name = "offsets_analog_trigger_r"]
//     pub const fn analog_trigger_r() -> usize {
//         0x3666ef4
//     }
// }