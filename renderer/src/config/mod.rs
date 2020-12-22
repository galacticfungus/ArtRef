mod device;
mod extensions;
mod queue;

use super::{
    DeviceQueue, ExtensionManager, Feature, Features, OperationQueue, PciVendor, QueueFamily,
    RendererQueues, RendererQueuesBuilder,
};
use crate::Version;
use erupt::extensions::khr_surface as surface;
use erupt::vk1_0 as vk;
pub use extensions::DeviceExtensions;
use std::collections::HashMap;

// Responsible for configuring the underlying device, creating queues, enabling features, loading device extensions and specifying surface parameters
pub struct ConfigureDevice<'a> {
    instance: &'a erupt::InstanceLoader,
    device_handle: vk::PhysicalDevice,
    available_queues: Vec<QueueFamily>,
    queues_to_create: Vec<DeviceQueue>,
    render_queues: Option<RendererQueues>,
    api_version: Version,
    driver_version: u32,
    vendor_id: PciVendor,
    device_id: u32,
    device_name: [i8; erupt::vk1_0::MAX_PHYSICAL_DEVICE_NAME_SIZE as usize],
    device_type: vk::PhysicalDeviceType,
    // Available device extensions
    available_device_extensions: Vec<vk::ExtensionProperties>,
    // Device Extensions to load
    extensions_to_load: HashMap<DeviceExtensions, bool>,
    // Available Features
    device_features: vk::PhysicalDeviceFeatures,
    // Enabled Features
    enabled_features: vk::PhysicalDeviceFeatures,
}

#[derive(Debug)]
// This class does not create any actual queues it merely gathers all the queues that the user wants to
// create in order to hopefully optimize queue creation, in addition it performs no validation of the results
// Meaning that if a queue could no
pub struct QueueManager<'a> {
    render_queues: RendererQueuesBuilder,
    queues_to_create: HashMap<usize, DeviceQueue>,
    family_data: &'a [QueueFamily],
    index: usize, // Index of the next queue that is create
}
