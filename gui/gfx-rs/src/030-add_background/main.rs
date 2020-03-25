use gfx_hal::window as hal_window;
use log::debug;
use log4rs;
use winit::{dpi, event_loop, window};

#[cfg(feature = "dx12")]
use gfx_backend_dx12 as back;
#[cfg(feature = "metal")]
use gfx_backend_metal as back;
#[cfg(feature = "vulkan")]
use gfx_backend_vulkan as back;

mod setup;

const DIMS: hal_window::Extent2D = hal_window::Extent2D {
    width: 1024,
    height: 768,
};

fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    // Setup Configs for Winit Window
    let event_loop = event_loop::EventLoop::new();
    let window_builder = window::WindowBuilder::new()
        .with_min_inner_size(winit::dpi::Size::Logical(dpi::LogicalSize::new(64.0, 64.0)))
        .with_inner_size(winit::dpi::Size::Physical(dpi::PhysicalSize::new(
            DIMS.width,
            DIMS.height,
        )))
        .with_title("colour-uniform".to_string());
    let backend_state = setup::create_backend(window_builder, &event_loop);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Wait;

        match event {
            winit::event::Event::WindowEvent { event, .. } =>
            {
                #[allow(unused_variables)]
                match event {
                    winit::event::WindowEvent::KeyboardInput {
                        input:
                            winit::event::KeyboardInput {
                                virtual_keycode: Some(winit::event::VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    }
                    | winit::event::WindowEvent::CloseRequested => {
                        *control_flow = winit::event_loop::ControlFlow::Exit
                    }
                    winit::event::WindowEvent::Resized(dims) => {
                        debug!("RESIZE EVENT");
                    }
                    winit::event::WindowEvent::KeyboardInput {
                        input:
                            winit::event::KeyboardInput {
                                virtual_keycode,
                                state: winit::event::ElementState::Pressed,
                                ..
                            },
                        ..
                    } => {
                        if let Some(virtual_keycode) = virtual_keycode {
                            debug!("Keyboard input: {:?}", virtual_keycode);
                        }
                    }
                    _ => (),
                }
            }
            winit::event::Event::RedrawRequested(_) => {
                debug!("RedrawRequested");
            }
            winit::event::Event::RedrawEventsCleared => {
                debug!("RedrawEventsCleared");
            }
            _ => (),
        }
    });
}
