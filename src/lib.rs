use std::{
    io::Write,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4, SocketAddrV6, TcpListener},
    time::Duration,
};

// mod frametracer;
mod off_by_one;
mod pacer;
mod profiling;
mod sequencing;
mod swapchain;
mod vsync;
mod vsync_history;

const DISABLE_VSYNC: bool = false;

#[skyline::main(name = "testing")]
pub fn main() {
    vsync_history::install();
    swapchain::install(DISABLE_VSYNC);
    off_by_one::install();
    pacer::install();
    profiling::setup();
    sequencing::install();

    // if DISABLE_VSYNC {
    //     vsync::install(true);
    // }
}
