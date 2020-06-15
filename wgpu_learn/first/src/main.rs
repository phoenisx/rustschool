use std::{thread, time};

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

async fn run(event_loop: EventLoop<()>, window: Window) {
    println!("Hello World Before!!");
    thread::sleep(time::Duration::from_millis(2000));
    println!("Hello World After!!");
}

fn main() {
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    {
        // Temporarily avoid srgb formats for the swapchain on the web
        futures::executor::block_on(
            run(event_loop, window)
        );
    }
    println!("Hello World!!");
}
