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

pub fn install() {
    disable_frame_pacer();
}
