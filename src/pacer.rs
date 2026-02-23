use crate::SsbuSyncConfig;

/** This replaces a check for how many frames we are behind with always assuming we are 0 frames behind
 *
 * This is to keep input latency measurements consistent until we can implement a better frame pacer
 * 
 * Enabling this turns frame drops to frame skips.
 */
fn disable_frame_pacer() {
    skyline::patching::Patch::in_text(0x374c640)
        .data(0x52800008u32)
        .unwrap();
}

pub fn patch_pacer_bias(slow_pacer_bias: bool) {
    let imm = { 0u32 };
    // mov x8, #imm (imm16 in bits [20:5], rd=8)
    let instruction = 0xD2800008u32 | (imm << 5);
    skyline::patching::Patch::in_text(0x22deb84)
        .data(instruction)
        .unwrap();
}

pub fn install(config: SsbuSyncConfig) {
    let emulator = config.emulator_check.unwrap();
    // Emulator always forces pacer-disable.
    if config.disable_pacer || emulator {
        disable_frame_pacer();
    }

    if emulator {
        patch_pacer_bias(false);
    } else {
        patch_pacer_bias(config.slow_pacer_bias);
    }
}
