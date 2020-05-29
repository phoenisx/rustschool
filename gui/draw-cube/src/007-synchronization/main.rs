use gfx_hal::{
    command,
    format::{self as hal_format, Aspects, Swizzle},
    image::{Layout, SubresourceRange, ViewKind},
    pass::{Attachment, AttachmentOps, SubpassDesc},
    pool::CommandPoolCreateFlags,
    prelude::*,
    window as hal_window, Backend, Features, Instance,
};

use std::mem::ManuallyDrop;
use std::ptr;

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

pub struct Renderer<B: Backend> {
    // Vulkan backend instance object
    instance: B::Instance,
    // Vulkan backend surface object
    surface: ManuallyDrop<B::Surface>,
    // Logical Device object
    device: B::Device,
    // CommandPool instance
    command_pool: Option<B::CommandPool>,
    // Swapchain instance
    swapchain: Option<B::Swapchain>,
    // Collection of ImageViews, capacity equals Swapchain image count
    image_views: Vec<B::ImageView>,
    // Render Pass instance
    render_pass: Option<B::RenderPass>,
    // Framebuffers linked to ImageViews
    framebuffers: Vec<B::Framebuffer>,
    // Synchronization Primitives:
    // Semaphores and Fences
    image_available_semaphores: Vec<B::Semaphore>,
    render_complete_semaphores: Vec<B::Semaphore>,
    submission_complete_fence: Vec<B::Fence>,
}

impl<B: Backend> Renderer<B> {
    pub fn new(
        instance: B::Instance,
        mut surface: B::Surface,
        init_extent: hal_window::Extent2D,
    ) -> Result<Self, &'static str> {
        let mut adapters = instance.enumerate_adapters();
        let (memory_types, limits, adapter) = {
            let adapter = adapters.remove(0);
            (
                adapter.physical_device.memory_properties().memory_types,
                adapter.physical_device.limits(),
                adapter,
            )
        };

        let (device, queues, supported_family) = {
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

        let (command_pool, mut command_buffer) = unsafe {
            let mut command_pool = device
                .create_command_pool(queues.family, CommandPoolCreateFlags::empty())
                .expect("Out of memory");

            let command_buffer = command_pool.allocate_one(command::Level::Primary);

            (command_pool, command_buffer)
        };

        // Get Surface Capabilities
        let (swapchain, backbuffer, image_extent, format) = {
            let caps = surface.capabilities(&adapter.physical_device);

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
            let (swapchain, backbuffer) = unsafe {
                device
                    .create_swapchain(&mut surface, swap_config, None)
                    .expect("Can't create swapchain")
            };

            (swapchain, backbuffer, image_extent, format)
        };

        let image_views = backbuffer
            .into_iter()
            .map(|image| unsafe {
                device
                    .create_image_view(
                        &image,
                        ViewKind::D2,
                        format,
                        Swizzle::NO,
                        SubresourceRange {
                            aspects: Aspects::COLOR,
                            levels: 0..1,
                            layers: 0..1,
                        },
                    )
                    .map_err(|_| "Couldn't create the image_view for the image!")
            })
            .collect::<Result<Vec<B::ImageView>, &str>>()?;

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

        let framebuffers = image_views
            .iter()
            .map(|image_view| unsafe {
                device
                    .create_framebuffer(&render_pass, vec![image_view], image_extent)
                    .map_err(|_| "Couldn't create the framebuffer for the image_view!")
            })
            .collect::<Result<Vec<B::Framebuffer>, &str>>()?;

        let (image_available_semaphores, render_complete_semaphores, submission_complete_fence) = {
            let mut image_available_semaphores: Vec<B::Semaphore> = vec![];
            let mut render_finished_semaphores: Vec<B::Semaphore> = vec![];
            let mut submission_complete_fence: Vec<B::Fence> = vec![];
            for _ in 0..image_views.len() {
                image_available_semaphores.push(
                    device
                        .create_semaphore()
                        .map_err(|_| "Could not create image_available_semaphores semaphore!")?,
                );
                render_finished_semaphores.push(
                    device
                        .create_semaphore()
                        .map_err(|_| "Could not create render_finished_semaphores semaphore!")?,
                );
                submission_complete_fence.push(
                    device
                        .create_fence(true)
                        .map_err(|_| "Could not create submission_complete_fence fence!")?,
                );
            }
            (
                image_available_semaphores,
                render_finished_semaphores,
                submission_complete_fence,
            )
        };

        Ok(Renderer {
            instance,
            surface: ManuallyDrop::new(surface),
            device,
            command_pool: Some(command_pool),
            swapchain: Some(swapchain),
            image_views,
            render_pass: Some(render_pass),
            framebuffers,
            image_available_semaphores,
            render_complete_semaphores,
            submission_complete_fence,
        })
    }
}

impl<B: Backend> Drop for Renderer<B> {
    fn drop(&mut self) {
        unsafe {
            for image_available in self.image_available_semaphores.drain(..) {
                self.device.destroy_semaphore(image_available);
            }
            for render_complete in self.render_complete_semaphores.drain(..) {
                self.device.destroy_semaphore(render_complete);
            }
            for submission_complete in self.submission_complete_fence.drain(..) {
                self.device.destroy_fence(submission_complete);
            }
            for framebuffer in self.framebuffers.drain(..) {
                self.device.destroy_framebuffer(framebuffer);
            }
            for image_view in self.image_views.drain(..) {
                self.device.destroy_image_view(image_view);
            }
            self.device
                .destroy_render_pass(self.render_pass.take().unwrap());
            self.device
                .destroy_swapchain(self.swapchain.take().unwrap());
            self.device
                .destroy_command_pool(self.command_pool.take().unwrap());
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

    let renderer = Renderer::<back::Backend>::new(instance, surface, extent);

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
