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
    // Vulkan backend instance object
    instance: B::Instance,
    // Vulkan backend surface object
    surface: ManuallyDrop<B::Surface>,
}

impl<B: Backend> Renderer<B> {
    fn new(instance: B::Instance, surface: B::Surface) -> Self {
        Renderer {
            instance,
            surface: ManuallyDrop::new(surface),
        }
    }
}

impl<B: Backend> Drop for Renderer<B> {
    fn drop(&mut self) {
        unsafe {
            // up here ManuallyDrop gives us the inner resource with ownership
            // where `ptr::read` doesn't do anything just reads the resource
            // without manipulating the actual memory
            let surface = ManuallyDrop::into_inner(ptr::read(&self.surface));
            self.instance.destroy_surface(surface);
        }
    }
}

fn create_backend(
    wb: window::WindowBuilder,
    ev_loop: &event_loop::EventLoop<()>,
    extent: hal_window::Extent2D,
) -> (back::Instance, back::Surface, window::Window) {
    let window = wb.build(ev_loop).unwrap();

    let instance = back::Instance::create(APP_NAME, 1).expect("Failed to create an instance!");
    let surface = unsafe {
        instance
            .create_surface(&window)
            .expect("Failed to create a surface!")
    };

    (instance, surface, window)
}

fn build_window(
    ev_loop: &event_loop::EventLoop<()>,
) -> (window::WindowBuilder, hal_window::Extent2D) {
    // We need to first get Logical and Physical Size of the screen
    let (logical_window_size, physical_window_size) = {
        let dpi = ev_loop.primary_monitor().scale_factor();
        let logical: LogicalSize<u32> = WINDOW_SIZE.into();

        // Phsical Size is the actual internal screen size, a factor of DPI
        let physical: PhysicalSize<u32> = logical.to_physical(dpi);

        (logical, physical)
    };

    let window_builder = window::WindowBuilder::new()
        .with_title(APP_NAME)
        .with_inner_size(logical_window_size);

    (
        window_builder,
        hal_window::Extent2D {
            width: physical_window_size.width,
            height: physical_window_size.height,
        },
    )
}

fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let ev_loop = event_loop::EventLoop::new();
    let (window_builder, extent) = build_window(&ev_loop);
    let (instance, surface, window) = create_backend(window_builder, &ev_loop, extent);

    let renderer = Renderer::<back::Backend>::new(instance, surface);

    ev_loop.run(move |event, _, control_flow| {
        *control_flow = event_loop::ControlFlow::Wait;
        match event {
            event::Event::WindowEvent { event, .. } => {
                #[allow(unused_variables)]
                match event {
                    event::WindowEvent::CloseRequested => {
                        *control_flow = event_loop::ControlFlow::Exit
                    }
                    event::WindowEvent::Resized(dims) => {
                        debug!("RESIZE EVENT");
                    }
                    event::WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        // Will get called whenever the screen scale factor (DPI) changes,
                        // like when user move the Window from one less DPI monitor
                        // to other high scaled DPI Monitor.
                        debug!("Scale Factor Change");
                    }
                    _ => (),
                }
            }
            event::Event::MainEventsCleared => {
                debug!("MainEventsCleared");
                window.request_redraw();
            }
            event::Event::RedrawRequested(_) => {
                debug!("RedrawRequested");
            }
            event::Event::RedrawEventsCleared => {
                debug!("RedrawEventsCleared");
            }
            _ => (),
        }
    });
}
