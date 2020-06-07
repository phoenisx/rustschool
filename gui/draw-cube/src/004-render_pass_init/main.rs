use std::mem::ManuallyDrop;
use std::ptr;

use gfx_hal::{
    adapter::Adapter,
    command,
    format::{self as hal_format},
    image::{Layout},
    pool::CommandPoolCreateFlags,
    prelude::*,
    pass::{Attachment, AttachmentOps, SubpassDesc},
    pso::{Rect, Viewport},
    queue::{family},
    window as hal_window, Backend, Features, Instance,
};
use winit::{
    dpi::{LogicalSize, PhysicalSize},
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
    window_dims: hal_window::Extent2D,
    viewport: Viewport,
    // Vulkan backend instance object
    instance: B::Instance,
    // Vulkan backend surface object
    surface: ManuallyDrop<B::Surface>,
    // Device Adpter, containing Physical and Queue details
    adapter: Adapter<B>,
    // Logical Device object
    device: B::Device,
    // Queue Group for rendering reference
    queue_group: family::QueueGroup<B>,
    // Collection Swapchain Image, Empty buffer initially
    frame_count: usize,
    // Desired Format / Selected Format
    format: hal_format::Format,
    // Render Pass instance
    render_pass: ManuallyDrop<B::RenderPass>,
}

impl<B: Backend> Renderer<B> {
    fn new(
        instance: B::Instance,
        mut surface: B::Surface,
        init_extent: hal_window::Extent2D,
    ) -> Self {
        let mut adapters = instance.enumerate_adapters();
        let (memory_types, limits, adapter) = {
            let adapter = adapters.remove(0);
            (
                adapter.physical_device.memory_properties().memory_types,
                adapter.physical_device.limits(),
                adapter,
            )
        };

        let (device, queue_group, supported_family) = {
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

            (
                gpu.device,
                gpu.queue_groups.pop().unwrap(),
                supported_family,
            )
        };

        // Configure Swapchain
        let (frame_count, format) = {
            let caps = surface.capabilities(&adapter.physical_device);

            debug!("Capabilities: {:#?}", caps);

            let supported_formats = surface.supported_formats(&adapter.physical_device);
            // We need a supported format for the OS Window, so that Images drawn on
            // Swapchain are of that same format.
            let format = supported_formats.map_or(hal_format::Format::Rgba8Srgb, |formats| {
                formats
                    .iter()
                    .find(|format| format.base_format().1 == hal_format::ChannelType::Srgb)
                    .map(|format| *format)
                    .unwrap_or(formats[0])
            });

            let swap_config = hal_window::SwapchainConfig::from_caps(&caps, format, init_extent);
            let image_extent = swap_config.extent.to_extent();

            unsafe {
                surface
                    .configure_swapchain(&device, swap_config)
                    .expect("Can't configure swapchain");
            };

            (3, format)
        };

        let render_pass = {
            let color_attachment = Attachment {
                format: Some(format),
                samples: 1,
                ops: AttachmentOps::INIT,
                stencil_ops: AttachmentOps::DONT_CARE,
                layouts: Layout::Undefined..Layout::Present,
            };

            let subpass = SubpassDesc {
                colors: &[(0, Layout::ColorAttachmentOptimal)],
                depth_stencil: None,
                inputs: &[],
                resolves: &[],
                preserves: &[],
            };

            unsafe {
                device
                    .create_render_pass(&[color_attachment], &[subpass], &[])
                    .expect("Out of memory")
            }
        };

        let viewport = Viewport {
            rect: Rect {
                x: 0,
                y: 0,
                w: init_extent.width as _,
                h: init_extent.height as _,
            },
            depth: 0.0..1.0,
        };

        Renderer {
            window_dims: init_extent,
            viewport,
            instance,
            surface: ManuallyDrop::new(surface),
            adapter,
            device,
            queue_group,
            frame_count,
            format,
            render_pass: ManuallyDrop::new(render_pass),
        }
    }

    fn set_dims(&mut self, dims: PhysicalSize<u32>) {
        self.window_dims = hal_window::Extent2D {
            width: dims.width,
            height: dims.height,
        };
    }

    fn recreate_swapchain(&mut self) {
        let caps = self.surface.capabilities(&self.adapter.physical_device);
        let swap_config =
            hal_window::SwapchainConfig::from_caps(&caps, self.format, self.window_dims);
        println!("SwapConfig Changed: {:?}", swap_config);
        let image_extent = swap_config.extent.to_extent();

        unsafe {
            self.surface
                .configure_swapchain(&self.device, swap_config)
                .expect("Can't create swapchain");
        }

        self.viewport.rect.w = image_extent.width as _;
        self.viewport.rect.h = image_extent.height as _;
    }
}

impl<B: Backend> Drop for Renderer<B> {
    fn drop(&mut self) {
        unsafe {
            self.device
                .destroy_render_pass(ManuallyDrop::into_inner(ptr::read(&self.render_pass)));
            self.surface.unconfigure_swapchain(&self.device);
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
    let (instance, surface, window) = create_backend(window_builder, &ev_loop);

    let mut renderer = Renderer::<back::Backend>::new(instance, surface, extent);

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
                        renderer.set_dims(dims);
                        renderer.recreate_swapchain();
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
                // window.request_redraw();
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
