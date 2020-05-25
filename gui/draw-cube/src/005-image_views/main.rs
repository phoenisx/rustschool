use std::mem::ManuallyDrop;
use std::ptr;
use std::cell::RefCell;
use std::rc::Rc;

use gfx_hal::{
    adapter::{Adapter, MemoryType},
    command,
    prelude::*,
    image,
    format as hal_format,
    queue::family::QueueGroup,
    pool::CommandPoolCreateFlags,
    window as hal_window, Backend, Features, Instance, Limits,
};
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

struct AdapterState<B: Backend> {
    adapter: Option<Adapter<B>>,
    memory_types: Vec<MemoryType>,
    limits: Limits,
}

impl<B: Backend> AdapterState<B> {
    fn new(adapters: &mut Vec<Adapter<B>>) -> Self {
        let adapter = adapters.remove(0);
        Self {
            memory_types: adapter.physical_device.memory_properties().memory_types,
            limits: adapter.physical_device.limits(),
            adapter: Some(adapter),
        }
    }
}

struct DeviceState<B: Backend> {
    physical_device: B::PhysicalDevice,
    device: B::Device,
    queues: QueueGroup<B>,
}

impl<B: Backend> DeviceState<B> {
    fn new(adapter: Adapter<B>, surface: &B::Surface) -> Self {
        let supported_family = adapter
            .queue_families
            .iter()
            .find(|family| {
                surface.supports_queue_family(family) && family.queue_type().supports_graphics()
            })
            .unwrap();

        let mut gpu = unsafe {
            adapter
                .physical_device
                .open(&[(supported_family, &[1.0])], Features::empty())
                .unwrap()
        };

        Self {
            physical_device: adapter.physical_device,
            device: gpu.device,
            queues: gpu.queue_groups.pop().unwrap(),
        }
    }
}

struct FrameBufferState<B: Backend> {
    command_pool: Option<B::CommandPool>,
    command_buffer: B::CommandBuffer,
}

impl<B: Backend> FrameBufferState<B> {
    fn new(deviceState: &DeviceState<B>) -> Self {
        let (command_pool, mut command_buffer) = unsafe {
            let mut command_pool = deviceState
                .device
                .create_command_pool(
                    deviceState.queues.family,
                    CommandPoolCreateFlags::empty()
                )
                .expect("Out of memory");

            let command_buffer = command_pool.allocate_one(
                command::Level::Primary
            );

            (command_pool, command_buffer)
        };

        Self {
            command_pool: Some(command_pool),
            command_buffer,
        }
    }
}

struct SwapchainState<B: Backend> {
    swapchain: Option<B::Swapchain>,
    backbuffer: Option<Vec<B::Image>>,
    deviceState: Rc<RefCell<DeviceState<B>>>,
    extent: image::Extent,
    format: hal_format::Format,
}

impl<B: Backend> SwapchainState<B> {
    fn new(backendState: &mut BackendState<B>, deviceState: Rc<RefCell<DeviceState<B>>>) -> Self {
        // Get Surface Capabilities
        let caps = backendState
            .surface
            .capabilities(&deviceState.borrow().physical_device);

        let supported_formats = backendState
            .surface
            .supported_formats(&deviceState.borrow().physical_device);
        // We need a supported format for the OS Window, so that Images drawn on
        // Swapchain are of that same format.
        let format = supported_formats.map_or(hal_format::Format::Rgba8Srgb, |formats| {
            formats
                .iter()
                .find(|format| format.base_format().1 == hal_format::ChannelType::Srgb)
                .map(|format| *format)
                .unwrap_or(formats[0])
        });

        debug!("Selected Format:: {:#?}", format);

        let swap_config = hal_window::SwapchainConfig::from_caps(&caps, format, backendState.extent);
        let extent = swap_config.extent.to_extent();
        let (swapchain, backbuffer) = unsafe {
            deviceState
                .borrow()
                .device
                .create_swapchain(&mut backendState.surface, swap_config, None)
                .expect("Can't create swapchain")
        };

        SwapchainState {
            swapchain: Some(swapchain),
            backbuffer: Some(backbuffer),
            deviceState,
            extent,
            format,
        }
    }
}

/// Ok I understand now, why this Drop is required.
/// We need to destroy swapchain from device first
/// then only actualy drop the actual instance of self.swapchain
impl<B: Backend> Drop for SwapchainState<B> {
    fn drop(&mut self) {
        unsafe {
            self.deviceState
                .borrow()
                .device
                .destroy_swapchain(self.swapchain.take().unwrap());
        }
    }
}

struct Renderer<B: Backend> {
    image_views: ManuallyDrop<B::ImageView>,
}

impl<B: Backend> Renderer<B> {
    fn new()
}

struct BackendState<B: Backend> {
    // Vulkan backend instance object
    instance: Option<B::Instance>,
    // Vulkan backend surface object
    surface: ManuallyDrop<B::Surface>,
    // Vulkan backend surface object
    adapterState: AdapterState<B>,
    extent: hal_window::Extent2D,
    // `winit` Window object.
    window: window::Window,
}

impl<B: Backend> BackendState<B> {
    fn update_extent(&mut self, extent: hal_window::Extent2D) {
        self.extent = extent;
    }
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

fn create_backend(
    wb: window::WindowBuilder,
    ev_loop: &event_loop::EventLoop<()>,
    extent: hal_window::Extent2D,
) -> BackendState<back::Backend> {
    let window = wb.build(ev_loop).unwrap();

    let instance = back::Instance::create(APP_NAME, 1).expect("Failed to create an instance!");
    let surface = unsafe {
        instance
            .create_surface(&window)
            .expect("Failed to create a surface!")
    };

    let mut adapters = instance.enumerate_adapters();

    BackendState {
        instance: Some(instance),
        surface: ManuallyDrop::new(surface),
        adapterState: AdapterState::new(&mut adapters),
        extent,
        window,
    }
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
    #[allow(unused_variables)]
    let mut backend = create_backend(window_builder, &ev_loop, extent);

    let deviceState = DeviceState::new(
        backend.adapterState.adapter.take().unwrap(),
        &backend.surface
    );

    let frameBufferState = FrameBufferState::new(
        &deviceState
    );

    // This is temporary and will be removed in later sections
    let rcDeviceState = Rc::new(
        RefCell::new(deviceState)
    );

    let swapchainState = SwapchainState::new(
        &mut backend,
        Rc::clone(&rcDeviceState)
    );

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
                        backend.update_extent(hal_window::Extent2D {
                            width: dims.width,
                            height: dims.height,
                        });
                        debug!("RESIZE EVENT");
                    }
                    event::WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        // Will get called whenever the screen scale factor (DPI) changes,
                        // like when user move the Window from one less DPI monitor
                        // to other high scaled DPI Monitor.
                        backend.update_extent(hal_window::Extent2D {
                            width: new_inner_size.width,
                            height: new_inner_size.height,
                        });
                        debug!("Scale Factor Change");
                    }
                    _ => (),
                }
            }
            event::Event::MainEventsCleared => {
                // debug!("MainEventsCleared");
                backend.window.request_redraw();
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
