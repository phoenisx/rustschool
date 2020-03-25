use std::mem::ManuallyDrop;

#[cfg(feature = "dx12")]
use gfx_backend_dx12 as back;
#[cfg(feature = "metal")]
use gfx_backend_metal as back;
#[cfg(feature = "vulkan")]
use gfx_backend_vulkan as back;

use gfx_hal::{
    adapter::{Adapter, MemoryType},
    prelude::*,
    Backend, Limits,
};

use winit::{event_loop, window};

// Check 002-enumerate_devices to understand what adapters and devices are.
pub struct AdapterState<B: Backend> {
    pub adapter: Option<Adapter<B>>,
    pub memory_types: Vec<MemoryType>,
    pub limits: Limits,
}

// Check 001-show_window to understand what instance and surface are.
pub struct BackendState<B: Backend> {
    pub instance: Option<B::Instance>,
    pub surface: ManuallyDrop<B::Surface>,
    pub adapter_state: AdapterState<B>,
    pub window: winit::window::Window,
}

// Implementations
impl<B: Backend> AdapterState<B> {
    pub fn new(mut adapters: Vec<Adapter<B>>) -> Self {
        // We just want to work with first device, anyways I have a single GPU, so doesn't matter.
        let adapter = adapters.remove(0);
        let memory_types = adapter.physical_device.memory_properties().memory_types;
        let limits = adapter.physical_device.limits();
        AdapterState {
            adapter: Some(adapter),
            memory_types,
            limits,
        }
    }
}

pub fn create_backend(
    win_builder: window::WindowBuilder,
    event_loop: &event_loop::EventLoop<()>,
) -> BackendState<back::Backend> {
    let window = win_builder.build(event_loop).unwrap();
    let instance = back::Instance::create("Awesome Backend", 0).expect("Initializing Failed");
    let surface = unsafe {
        instance
            .create_surface(&window)
            .expect("Creating Surface Failed")
    };
    let adapters = instance.enumerate_adapters();
    BackendState {
        instance: Some(instance),
        surface: ManuallyDrop::new(surface),
        adapter_state: AdapterState::new(adapters),
        window,
    }
}

impl<B: Backend> BackendState<B> {}
