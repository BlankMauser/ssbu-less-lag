// mod frametracer;
mod off_by_one;
mod pacer;
mod swapchain;
mod vsync;
// mod vsync_history;

const DISABLE_VSYNC: bool = true;

#[skyline::main(name = "testing")]
pub fn main() {
    swapchain::install(DISABLE_VSYNC);
    off_by_one::install();
    pacer::install();

    if DISABLE_VSYNC {
        vsync::install(false);
    }
}
