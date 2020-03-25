#![allow(unused_doc_comments)]

use std::mem::{ManuallyDrop};
use std::{ptr};
use winit::{
    event, event_loop, window,
    dpi::{ Size, LogicalSize, PhysicalSize }
};
use gfx_hal::{
    window as hal_window,
    prelude::*,
    Features,
    Instance
};
#[cfg(feature = "dx12")]
use gfx_backend_dx12 as back;
#[cfg(feature = "metal")]
use gfx_backend_metal as back;
#[cfg(feature = "vulkan")]
use gfx_backend_vulkan as back;
use log::{debug, Level};

// #region Constants
const DIMS: hal_window::Extent2D = hal_window::Extent2D {
    width: 1024,
    height: 768
};
// #endregion Constants

fn main() {
    simple_logger::init_with_level(Level::Debug).unwrap();
    let event_loop = event_loop::EventLoop::new();

    let window_builder = window::WindowBuilder::new()
        .with_min_inner_size(Size::Logical(LogicalSize::new(
            64.0, 64.0,
        )))
        .with_inner_size(Size::Physical(PhysicalSize::new(
            DIMS.width,
            DIMS.height,
        )))
        .with_title("colour-uniform".to_string());

    let window = window_builder.build(&event_loop).unwrap();

    let instance =
        back::Instance::create("Create Instance with Window", 1).expect("Failed to create an instance!");
    let surface = unsafe {
        instance
            .create_surface(&window)
            .expect("Failed to create a surface!")
    };

    // Get Logical Device:
    let adapters = instance.enumerate_adapters();
    let mut gpu = unsafe {
        adapters[0]
            .physical_device
            .open(&[(&adapters[0].queue_families[0], &[1.0])], Features::empty())
            .unwrap()
    };

    /**
     * Main Swap Chain Code to understand.
     * Above and Below Code has already been covered.
     * Get Capability of ur Window Surface.
     */
    let caps = surface.capabilities(&adapters[0].physical_device);
    debug!("Capabilities: {:#?}", caps);
    // End

    event_loop.run(move |event, _, control_flow| {
        *control_flow = event_loop::ControlFlow::Wait;
        match event {
            event::Event::WindowEvent { event, .. } =>
            {
                #[allow(unused_variables)]
                match event {
                    event::WindowEvent::CloseRequested => {
                        *control_flow = event_loop::ControlFlow::Exit
                    }
                    event::WindowEvent::Resized(dims) => {
                        debug!("RESIZE EVENT");
                    }
                    event::WindowEvent::KeyboardInput {
                        input:
                            event::KeyboardInput {
                                virtual_keycode,
                                state: event::ElementState::Pressed,
                                ..
                            },
                        ..
                    } => {
                        debug!("Keyboard input, {:?}", virtual_keycode);
                    }
                    _ => (),
                }
            }
            event::Event::RedrawRequested(_) => {
                // debug!("RedrawRequested");
            }
            event::Event::RedrawEventsCleared => {
                // debug!("RedrawEventsCleared");
            }
            _ => (),
        }
    });
}
