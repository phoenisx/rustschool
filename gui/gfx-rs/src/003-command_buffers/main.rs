#[cfg(feature = "dx12")]
use gfx_backend_dx12 as back;
#[cfg(feature = "metal")]
use gfx_backend_metal as back;
#[cfg(feature = "vulkan")]
use gfx_backend_vulkan as back;

/**
 * `prelude::*` Brings in PhysicalDevice/Device Traits in scope
 *
 */
use gfx_hal::{command, pool, prelude::*, Features, Instance};
use log::debug;

fn main() {
    simple_logger::init().unwrap();
    let instance = back::Instance::create("Create Instance with Window", 1)
        .expect("Failed to create an instance!");
    let adapters = instance.enumerate_adapters();

    let gpu = unsafe {
        adapters[0]
            .physical_device
            .open(
                &[(&adapters[0].queue_families[0], &[1.0])],
                Features::empty(),
            )
            .unwrap()
    };
    let mut command_pool = unsafe {
        gpu.device
            .create_command_pool(
                gpu.queue_groups[0].family, // Since I am using only one QueueFamily from PhysicalDevice
                pool::CommandPoolCreateFlags::empty(),
            )
            .expect("Can't create command pool")
    };

    // Get Command Buffer from the Pool
    let mut command_buffer = unsafe {
        command_pool.allocate_one(command::Level::Primary)
    };

    debug!("Command Pool: {:#?}", command_pool);
    debug!("Command Buffer: {:#?}", command_buffer);
}
