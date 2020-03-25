#[cfg(feature = "dx12")]
use gfx_backend_dx12 as back;
#[cfg(feature = "metal")]
use gfx_backend_metal as back;
#[cfg(feature = "vulkan")]
use gfx_backend_vulkan as back;
use gfx_hal::{prelude::*, Features, Instance};
use log::debug;
use log4rs;

fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    let instance = back::Instance::create("Create Instance with Window", 1)
        .expect("Failed to create an instance!");
    let adapters = instance.enumerate_adapters();
    for (index, adapter) in adapters.iter().enumerate() {
        debug!("Adapter[{}]: {:#?}", index, adapter.info);
        debug!("Adapter[{}]: {:#?}", index, adapter.physical_device);
        debug!("Adapter[{}]: {:#?}", index, adapter.queue_families);
    }
    // Get Logical Device
    let mut gpu = unsafe {
        adapters[0]
            .physical_device
            .open(&[(&adapters[0].queue_families[0], &[1.0])], Features::empty())
            .unwrap()
    };

    debug!("GPU: {:#?}", gpu);
    // gpu.device, can be considered as logical device, as it will be used to get Command Pools and stuff
    debug!("Logical device: {:#?}", gpu.device);

    // Get Supported Memory Types by adapter
    for (index, adapter) in adapters.iter().enumerate() {
        debug!("Memory Properties[{}]: {:#?}", index, adapter.physical_device.memory_properties());
    }
}
