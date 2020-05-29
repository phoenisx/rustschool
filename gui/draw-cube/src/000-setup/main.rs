use std::mem::ManuallyDrop;
use std::ptr;

use gfx_hal::{window as hal_window, Backend, Instance};
use winit::{
    dpi::{LogicalSize, PhysicalSize, Size},
    event, event_loop, window,
};

#[cfg(feature = "dx12")]
use gfx_backend_dx12 as back;
#[cfg(feature = "metal")]
use gfx_backend_metal as back;
#[cfg(feature = "vulkan")]
use gfx_backend_vulkan as back;

use log::debug;
use log4rs;

const APP_NAME: &'static str = "Show Window";
const WINDOW_SIZE: [u32; 2] = [1280, 768];

struct Renderer<B: Backend> {
}

impl<B: Backend> Renderer<B> {
    fn new() -> Self {
        Renderer {}
    }
}

fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    info!("Hello World")
}
