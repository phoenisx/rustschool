#[cfg(feature = "dx12")]
use gfx_backend_dx12 as back;
#[cfg(feature = "metal")]
use gfx_backend_metal as back;
#[cfg(feature = "vulkan")]
use gfx_backend_vulkan as back;
use gfx_hal::{
    adapter, command,
    format::{self as hal_format},
    image::{self as hal_image},
    pass, pool,
    prelude::*,
    pso,
    queue::Submission,
    window::{self as hal_window},
    Backend, Features, Instance,
};

use log::{debug, error};
use std::iter;
use winit::{
    dpi::{LogicalSize, PhysicalPosition, PhysicalSize, Size},
    event, event_loop, window,
};

pub type ColorFormat = hal_format::Rgba8Srgb;

// #region Constants
const DIMS: hal_window::Extent2D = hal_window::Extent2D {
    width: 1024,
    height: 768,
};
// #endregion Constants

#[cfg(any(
    feature = "vulkan",
    feature = "dx11",
    feature = "dx12",
    feature = "metal"
))]
fn draw(
    current_frame: &mut usize,
    recreate_swapchain: &mut bool,
    frames_in_flight: usize,
    dims: &hal_window::Extent2D,
    color: [f32; 4],
    viewport: pso::Rect,
    surface: &mut <back::Backend as Backend>::Surface,
    caps: &hal_window::SurfaceCapabilities,
    format: &hal_format::Format,
    image_available_semaphores: &Vec<<back::Backend as Backend>::Semaphore>,
    render_finished_semaphores: &Vec<<back::Backend as Backend>::Semaphore>,
    in_flight_fences: &Vec<<back::Backend as Backend>::Fence>,
    swapchain: &mut <back::Backend as Backend>::Swapchain,
    gpu: &mut adapter::Gpu<back::Backend>,
    command_buffers: &mut [<back::Backend as Backend>::CommandBuffer],
    render_pass: &mut Option<<back::Backend as Backend>::RenderPass>,
    frame_buffers: &Vec<<back::Backend as Backend>::Framebuffer>,
) -> Result<(), &'static str> {
    if *recreate_swapchain {
        let swap_config =
            hal_window::SwapchainConfig::from_caps(&caps, format.clone(), dims.clone());
        let (mut _swapchain, _backbuffer) = unsafe {
            gpu.device
                .create_swapchain(surface, swap_config, None)
                .expect("Can't create swapchain")
        };

        debug!("Recreating Swapchain: {:?}", _swapchain);
        *swapchain = _swapchain;

        let attachment = pass::Attachment {
            format: Some(format.clone()),
            samples: 1,
            ops: pass::AttachmentOps::new(
                pass::AttachmentLoadOp::Clear,
                pass::AttachmentStoreOp::Store,
            ),
            stencil_ops: pass::AttachmentOps::DONT_CARE,
            layouts: hal_image::Layout::Undefined..hal_image::Layout::Present,
        };
        let subpass = pass::SubpassDesc {
            colors: &[(0, hal_image::Layout::ColorAttachmentOptimal)],
            depth_stencil: None,
            inputs: &[],
            resolves: &[],
            preserves: &[],
        };
        *render_pass = unsafe {
            gpu.device
                .create_render_pass(&[attachment], &[subpass], &[])
                .ok()
        };

        *recreate_swapchain = false;
    }
    let image_available = &image_available_semaphores[*current_frame];
    let render_finished = &render_finished_semaphores[*current_frame];
    *current_frame = (*current_frame + 1) % frames_in_flight;

    let image_index = unsafe {
        match swapchain.acquire_image(!0, Some(image_available), None) {
            Ok((i, _)) => i as usize,
            Err(err) => {
                error!("Acquiring Swapchain Image Failed: {}", err);
                *recreate_swapchain = true;
                return Err("Acquiring Swapchain Image Failed");
            }
        }
    };

    // Wait for Fence to signal
    let current_fence = &in_flight_fences[image_index];
    unsafe {
        gpu.device.wait_for_fence(current_fence, !0).unwrap();
        gpu.device.reset_fence(current_fence).unwrap();
    };

    // RECORD COMMANDS
    let buffer = &mut command_buffers[image_index];
    debug!("Color: {:?}, image_index: {}", color, image_index);
    unsafe {
        let clear_values = [command::ClearValue {
            color: command::ClearColor { float32: color },
        }];
        buffer.begin_primary(command::CommandBufferFlags::ONE_TIME_SUBMIT);
        buffer.begin_render_pass(
            render_pass.as_ref().unwrap(),
            &frame_buffers[image_index],
            viewport,
            &clear_values,
            command::SubpassContents::Inline,
        );
        // buffer.end_render_pass();
        buffer.finish();
    }

    // SUBMISSION AND PRESENT
    let submission = Submission {
        command_buffers: iter::once(&buffer),
        wait_semaphores: iter::once((image_available, pso::PipelineStage::COLOR_ATTACHMENT_OUTPUT)),
        signal_semaphores: iter::once(render_finished),
    };
    let the_command_queue = &mut gpu.queue_groups[0].queues[0]; // Since We are using only the first Queue Family
    if let Err(_) = unsafe {
        the_command_queue.submit(submission, Some(current_fence));
        swapchain
            .present(the_command_queue, image_index as u32, Some(render_finished))
            .map_err(|_| "Failed to present into the swapchain!")
    } {
        return Err("Error Presenting the Image to Screen");
    }

    return Ok(());
}

struct LocalState {
    mouse_x: f64,
    mouse_y: f64,
}

fn main() {
    simple_logger::init().unwrap();
    let event_loop = event_loop::EventLoop::new();
    let mut recreate_swapchain = false;
    let mut dims = DIMS.clone();
    let mut local_state = LocalState {
        mouse_x: 0_f64,
        mouse_y: 0_f64,
    };

    let window_builder = window::WindowBuilder::new()
        .with_min_inner_size(Size::Logical(LogicalSize::new(64.0, 64.0)))
        .with_inner_size(Size::Physical(PhysicalSize::new(DIMS.width, DIMS.height)))
        .with_title("Add Background".to_string());
    let window = window_builder.build(&event_loop).unwrap();

    let instance = back::Instance::create("Create Instance with Window", 1)
        .expect("Failed to create an instance!");
    let mut surface = unsafe {
        instance
            .create_surface(&window)
            .expect("Failed to create a surface!")
    };

    // Get Logical Device:
    let adapters = instance.enumerate_adapters();
    let mut gpu = unsafe {
        adapters[0]
            .physical_device
            .open(
                &[(&adapters[0].queue_families[0], &[1.0])],
                Features::empty(),
            )
            .unwrap()
    };

    // Command Buffers
    let mut command_pool = unsafe {
        gpu.device
            .create_command_pool(
                gpu.queue_groups[0].family, // Since I am using only one QueueFamily from PhysicalDevice
                pool::CommandPoolCreateFlags::empty(),
            )
            .expect("Can't create command pool")
    };

    // We will get N number of Command Buffers, respective to Framebuffers Length
    // As Creating and Destroying Command Buffers is expensive, so we need a proper
    // size of Command Buffers to use, and re-use.

    // Swapchain
    let caps = surface.capabilities(&adapters[0].physical_device);
    let formats = surface.supported_formats(&adapters[0].physical_device);
    let format = formats.map_or(hal_format::Format::Rgba8Srgb, |formats| {
        formats
            .iter()
            .find(|format| format.base_format().1 == hal_format::ChannelType::Srgb)
            .map(|format| *format)
            .unwrap_or(formats[0])
    });

    let swap_config = hal_window::SwapchainConfig::from_caps(&caps, format.clone(), DIMS.clone());
    let extent = swap_config.extent.to_extent();
    let (mut swapchain, backbuffer) = unsafe {
        gpu.device
            .create_swapchain(&mut surface, swap_config, None)
            .expect("Can't create swapchain")
    };
    // End Swapchain

    // Create Image Views
    let image_view_pairs: Vec<_> = unsafe {
        backbuffer
            .into_iter()
            .map(|image| {
                let rtv = gpu
                    .device
                    .create_image_view(
                        &image,
                        hal_image::ViewKind::D2,
                        format.clone(),
                        hal_format::Swizzle::NO,
                        hal_image::SubresourceRange {
                            aspects: hal_format::Aspects::COLOR,
                            levels: 0..1,
                            layers: 0..1,
                        },
                    )
                    .unwrap();
                (image, rtv)
            })
            .collect::<Vec<_>>()
    };
    // End Image Views

    // Create RenderPass
    let attachment = pass::Attachment {
        format: Some(format.clone()),
        samples: 1,
        ops: pass::AttachmentOps::new(
            pass::AttachmentLoadOp::Clear,
            pass::AttachmentStoreOp::Store,
        ),
        stencil_ops: pass::AttachmentOps::DONT_CARE,
        layouts: hal_image::Layout::Undefined..hal_image::Layout::Present,
    };
    let subpass = pass::SubpassDesc {
        colors: &[(0, hal_image::Layout::ColorAttachmentOptimal)],
        depth_stencil: None,
        inputs: &[],
        resolves: &[],
        preserves: &[],
    };
    let mut render_pass = unsafe {
        gpu.device
            .create_render_pass(&[attachment], &[subpass], &[])
            .ok()
    };
    // End RenderPass

    // Create Frame Buffers
    let frame_buffers: Vec<_> = unsafe {
        image_view_pairs
            .iter()
            .map(|&(_, ref rtv)| {
                gpu.device
                    .create_framebuffer(render_pass.as_ref().unwrap(), Some(rtv), extent)
                    .unwrap()
            })
            .collect()
    };
    // End Framebuffers

    // Allocate Command Buffers, in a vec whos capacity is decided by Framebuffers size.
    let mut command_buffers =
        unsafe { command_pool.allocate_vec(frame_buffers.len(), command::Level::Primary) };

    // Create FEnces and Semaphores for every Image Buffer
    let mut current_frame = 0;
    let (image_available_semaphores, render_finished_semaphores, in_flight_fences) = {
        let mut image_available_semaphores: Vec<<back::Backend as Backend>::Semaphore> = vec![];
        let mut render_finished_semaphores: Vec<<back::Backend as Backend>::Semaphore> = vec![];
        let mut in_flight_fences: Vec<<back::Backend as Backend>::Fence> = vec![];
        for _ in 0..2 {
            in_flight_fences.push(gpu.device.create_fence(true).unwrap());
            image_available_semaphores.push(gpu.device.create_semaphore().unwrap());
            render_finished_semaphores.push(gpu.device.create_semaphore().unwrap());
        }
        (
            image_available_semaphores,
            render_finished_semaphores,
            in_flight_fences,
        )
    };
    // ENd

    draw(
        &mut current_frame,
        &mut recreate_swapchain,
        2,
        &dims,
        [1.0, 1.0, 1.0, 1.0],
        extent.rect(),
        &mut surface,
        &caps,
        &format,
        &image_available_semaphores,
        &render_finished_semaphores,
        &in_flight_fences,
        &mut swapchain,
        &mut gpu,
        &mut command_buffers,
        &mut render_pass,
        &frame_buffers,
    );

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
                    event::WindowEvent::Resized(physical_size) => {
                        debug!("RESIZE EVENT");
                        dims = hal_window::Extent2D {
                            width: physical_size.width,
                            height: physical_size.height,
                        };
                        recreate_swapchain = true;
                    }
                    event::WindowEvent::CursorMoved { position, .. } => {
                        local_state.mouse_x = position.x;
                        local_state.mouse_y = position.y;
                    }
                    _ => (),
                }
            }
            event::Event::RedrawRequested(_) => {
                let r = (local_state.mouse_x / dims.width as f64) as f32;
                let g = (local_state.mouse_y / dims.height as f64) as f32;
                let b = (r + g) * 0.3;
                let a = 1.0;
                let color: [f32; 4] = [r, g, b, a];
                draw(
                    &mut current_frame,
                    &mut recreate_swapchain,
                    2,
                    &dims,
                    color,
                    extent.rect(),
                    &mut surface,
                    &caps,
                    &format,
                    &image_available_semaphores,
                    &render_finished_semaphores,
                    &in_flight_fences,
                    &mut swapchain,
                    &mut gpu,
                    &mut command_buffers,
                    &mut render_pass,
                    &frame_buffers,
                );
            }
            event::Event::RedrawEventsCleared => {
                window.request_redraw();
            }
            _ => (),
        }
    });
}
