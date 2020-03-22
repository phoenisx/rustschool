use std::mem::{ManuallyDrop};
use std::{ptr};
use winit::{
    event, event_loop, window,
    dpi::{ Size, LogicalSize, PhysicalSize }
};
use gfx_hal::{
    window as hal_window,
    Backend,
    Instance
};
#[cfg(feature = "dx12")]
use gfx_backend_dx12 as back;
#[cfg(feature = "metal")]
use gfx_backend_metal as back;
#[cfg(feature = "vulkan")]
use gfx_backend_vulkan as back;
use log::{debug};

// #region Constants
const DIMS: hal_window::Extent2D = hal_window::Extent2D {
    width: 1024,
    height: 768
};
// #endregion Constants


// #region BackendState Structs/Impls
struct BackendState<B: Backend> {
    instance: Option<B::Instance>,
    surface: ManuallyDrop<B::Surface>,
    // adapter: AdapterState<B>,
    /// Needs to be kept alive even if its not used directly
    #[allow(dead_code)]
    window: window::Window,
}

impl<B: Backend> Drop for BackendState<B> {
    fn drop(&mut self) {
        if let Some(instance) = &self.instance {
            unsafe {
                let surface = ManuallyDrop::into_inner(ptr::read(&self.surface));
                instance.destroy_surface(surface);
            }
        }
    }
}
// #endregion BackendState Structs/Impls

fn create_backend(
    wb: window::WindowBuilder,
    event_loop: &event_loop::EventLoop<()>,
) -> BackendState<back::Backend> {
    let window = wb.build(event_loop).unwrap();
    let instance =
        back::Instance::create("Create Instance with Window", 1).expect("Failed to create an instance!");
    let surface = unsafe {
        instance
            .create_surface(&window)
            .expect("Failed to create a surface!")
    };
    // let mut adapters = instance.enumerate_adapters();
    BackendState {
        instance: Some(instance),
        // adapter: AdapterState::new(&mut adapters),
        surface: ManuallyDrop::new(surface),
        window,
    }
}

fn main() {
    simple_logger::init().unwrap();
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

    #[allow(unused_variables)]
    let backend = create_backend(window_builder, &event_loop);

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
                debug!("RedrawRequested");
            }
            event::Event::RedrawEventsCleared => {
                debug!("RedrawEventsCleared");
            }
            _ => (),
        }
    });
}
