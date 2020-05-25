use gfx_hal::{
    adapter::{Adapter},
    prelude::*,
    pass::{ Attachment, AttachmentOps, SubpassDesc },
    pso::{PipelineStage},
    command::{self, ClearValue, ClearColor, CommandBufferFlags, SubpassContents},
    queue::{family, Submission},
    pool::{CommandPoolCreateFlags},
    format::{self as hal_format, Swizzle, Aspects},
    image::{Extent, ViewKind, SubresourceRange, Layout},
    window as hal_window, Backend, Features, Instance,
};

use std::mem::ManuallyDrop;
use std::ptr;
use std::iter;
use std::borrow::Borrow;

use winit::{
    dpi::{LogicalSize, PhysicalSize, PhysicalPosition},
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
    window_dims: hal_window::Extent2D,
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
    // CommandPools collection. Each command pool have one primary buffer
    // for now. I still don't get the actual use-case of command pools
    command_pools: Vec<B::CommandPool>,
    // CommandBuffers collection
    command_buffers: Vec<B::CommandBuffer>,
    // Swapchain instance
    swapchain: ManuallyDrop<B::Swapchain>,
    // Collection Swapchain Image, Empty buffer initially
    backbuffer: Vec<B::Image>,
    // Desired Format / Selected Format
    format: hal_format::Format,
    // Image Extent
    image_extent: Extent,
    // Collection of ImageViews, capacity equals Swapchain image count
    image_views: Vec<B::ImageView>,
    // Render Pass instance
    render_pass: ManuallyDrop<B::RenderPass>,
    // Synchronization Primitives:
    // Semaphores and Fences
    image_available_semaphores: Vec<B::Semaphore>,
    render_complete_semaphores: Vec<B::Semaphore>,
    submission_complete_fence: Vec<B::Fence>,

    current_frame: usize,
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

        // Get Surface Capabilities
        let (swapchain, backbuffer, image_extent, format) = {
            let caps = surface
                .capabilities(&adapter.physical_device);

            let supported_formats = surface
                .supported_formats(&adapter.physical_device);
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

            (
                swapchain,
                backbuffer,
                image_extent,
                format
            )
        };

        let image_views = backbuffer.iter()
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

        let (command_pools, mut command_buffers) = unsafe {
            let mut command_pools: Vec<B::CommandPool> = Vec::with_capacity(backbuffer.len());
            let mut command_buffers: Vec<B::CommandBuffer> = Vec::with_capacity(backbuffer.len());

            for (index, _) in backbuffer.iter().enumerate() {
                command_pools.push(device
                    .create_command_pool(
                        queue_group.family,
                        CommandPoolCreateFlags::empty()
                    )
                    .expect("Out of memory")
                );
                command_buffers.push(command_pools[index].allocate_one(
                    command::Level::Primary
                ));
            }
            (command_pools, command_buffers)
        };

        let (
            image_available_semaphores,
            render_complete_semaphores,
            submission_complete_fence
        ) = {
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

        Ok(
            Renderer {
                window_dims: init_extent,
                instance,
                surface: ManuallyDrop::new(surface),
                adapter,
                device,
                queue_group,
                command_pools,
                command_buffers,
                swapchain: ManuallyDrop::new(swapchain),
                backbuffer,
                format,
                image_extent,
                image_views,
                render_pass: ManuallyDrop::new(render_pass),
                image_available_semaphores,
                render_complete_semaphores,
                submission_complete_fence,
                current_frame: 0,
            }
        )
    }

    fn set_dims(&mut self, dims: PhysicalSize<u32>) {
        self.window_dims = hal_window::Extent2D {
            width: dims.width,
            height: dims.height,
        };
    }

    fn recreate_swapchain(&mut self) {
        let caps = self.surface.capabilities(&self.adapter.physical_device);
        let swap_config = hal_window::SwapchainConfig::from_caps(
            &caps,
            self.format,
            self.window_dims
        );
        println!("SwapConfig Changed: {:?}", swap_config);
        self.image_extent = swap_config.extent.to_extent();

        unsafe {
            self.surface
                .configure_swapchain(&self.device, swap_config)
                .expect("Can't create swapchain");
        }
    }

    fn draw(&mut self, color: [f32; 4]) -> Result<(), &'static str> {
        let frame_index = self.current_frame % self.backbuffer.len();

        let surface_image = unsafe {
            let result = self.surface
                .acquire_image(core::u64::MAX);
            match result {
                Ok((image, _)) => image,
                Err(_) => {
                    self.recreate_swapchain();
                    return Ok(());
                }
            }
        };

        let framebuffer = unsafe {
            self.device
                .create_framebuffer(
                    &self.render_pass,
                    iter::once(surface_image.borrow()),
                    Extent {
                        width: self.window_dims.width,
                        height: self.window_dims.height,
                        depth: 1,
                    },
                )
                .unwrap()
        };

        let image_available = &self.image_available_semaphores[frame_index];
        let render_finished = &self.render_complete_semaphores[frame_index];
        let submit_complete = &self.submission_complete_fence[frame_index];
        unsafe {
            self.device
                .wait_for_fence(submit_complete, !0)
                .expect("Out of memory or device lost");

            self.device
                .reset_fence(submit_complete)
                .expect("Out of memory");
            self.command_pools[frame_index].reset(false);
        }

        let buffer = &mut self.command_buffers[frame_index];
        unsafe {
            let clear_values = [ClearValue {
                color: ClearColor { float32: color }
            }];
            buffer.begin_primary(CommandBufferFlags::ONE_TIME_SUBMIT);
            buffer.begin_render_pass(
                &*self.render_pass,
                &framebuffer,
                self.image_extent.rect(),
                clear_values.iter(),
                SubpassContents::Inline,
            );
            buffer.end_render_pass();
            buffer.finish();
        }

        let command_buffers = iter::once(&buffer);
        let wait_semaphores =
            vec![(image_available, PipelineStage::BOTTOM_OF_PIPE)];

        let submission = Submission {
            command_buffers,
            wait_semaphores,
            signal_semaphores: iter::once(render_finished),
        };

        unsafe {
            self.queue_group
                .queues[0]
                .submit(submission, Some(submit_complete));
            let result = self.queue_group
                .queues[0]
                .present_surface(
                    &mut self.surface,
                    surface_image,
                    Some(render_finished)
                )
                .map_err(|_| "Failed to present into the swapchain!");

            self.device.destroy_framebuffer(framebuffer);

            if result.is_err() {
                self.recreate_swapchain();
            }
        };

        self.current_frame += 1;

        Ok(())
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
            for image_view in self.image_views.drain(..) {
                self.device.destroy_image_view(image_view);
            }
            self.device.destroy_render_pass(ManuallyDrop::into_inner(
                ptr::read(&self.render_pass)
            ));
            self.device.destroy_swapchain(ManuallyDrop::into_inner(
                ptr::read(&self.swapchain)
            ));
            for command_pool in self.command_pools.drain(..) {
                self.device.destroy_command_pool(command_pool);
            }
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

    (
        instance,
        surface,
        window
    )
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

fn main() -> Result<(), &'static str> {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let ev_loop = event_loop::EventLoop::new();
    let (window_builder, extent) = build_window(&ev_loop);
    let (instance, surface, window) = create_backend(window_builder, &ev_loop);

    let mut renderer = Renderer::<back::Backend>::new(instance, surface, extent)?;
    let mut current_pos = PhysicalPosition::new(0.0, 0.0);
    let mut red = 1.0;
    let mut green = 0.5;
    let mut blue = 0.2;
    let mut alpha = 1.0; // Alpha channel if set to 1.0 makes the color opaque...

    ev_loop.run(move |event, _, control_flow| {
        *control_flow = event_loop::ControlFlow::Wait;
        match event {
            event::Event::WindowEvent { event, .. } => {
                #[allow(unused_variables)]
                match event {
                    event::WindowEvent::CloseRequested => {
                        *control_flow = event_loop::ControlFlow::Exit
                    }
                    event::WindowEvent::CursorMoved { position, .. } => {
                        current_pos = position;
                    },
                    event::WindowEvent::Resized(dims) => {
                        // debug!("RESIZE EVENT");
                        renderer.set_dims(dims);
                        renderer.recreate_swapchain();
                    }
                    event::WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        // Will get called whenever the screen scale factor (DPI) changes,
                        // like when user move the Window from one less DPI monitor
                        // to other high scaled DPI Monitor.
                        // debug!("Scale Factor Change");
                    }
                    _ => (),
                }
            }
            event::Event::MainEventsCleared => {
                // debug!("MainEventsCleared");
                red = current_pos.x as f32/ extent.width as f32;
                green = current_pos.y as f32 / extent.height as f32;
                blue = (red + green) * 0.3;
                window.request_redraw();
            }
            event::Event::RedrawRequested(_) => {
                debug!("RedrawRequested");
                renderer.draw([red, green, blue, alpha]);
            }
            event::Event::RedrawEventsCleared => {
                // debug!("RedrawEventsCleared");
            }
            _ => (),
        }
    });
}
