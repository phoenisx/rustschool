use gfx_hal::{
    Instance
};
#[cfg(feature = "dx12")]
use gfx_backend_dx12 as back;
#[cfg(feature = "metal")]
use gfx_backend_metal as back;
#[cfg(feature = "vulkan")]
use gfx_backend_vulkan as back;
use log::{debug};

fn main() {
    simple_logger::init().unwrap();
    let instance =
        back::Instance::create("Create Instance with Window", 1).expect("Failed to create an instance!");
    let adapters = instance.enumerate_adapters();
    for (index, adapter) in adapters.iter().enumerate() {
        debug!("Adapter[{}]: {:#?}", index, adapter.info);
        debug!("Adapter[{}]: {:#?}", index, adapter.physical_device);
        debug!("Adapter[{}]: {:#?}", index, adapter.queue_families);
    }
    // A single adapter here, consists of Vulkan specs Physical/Logical Device instance.
}
