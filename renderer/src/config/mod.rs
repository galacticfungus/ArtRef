mod device;
mod indirect_count;
mod extensions;
mod queue;

pub use indirect_count::DrawIndirectCount;
pub use extensions::DeviceExtensions;
use super::{QueueFamily, QueueToCreate, PciVendor, Features, Feature, ExtensionManager};
use crate::Version;

use ash::vk;

use std::collections::HashMap;


// Responsible for configuring the underlying device, creating queues, enabling features, loading device extensions and specifying surface parameters
pub struct ConfigureDevice<'a> {
    instance: &'a ash::Instance,
    device_handle: vk::PhysicalDevice,
    queue_families: Vec<QueueFamily>,
    api_version: Version,
    driver_version: u32,
    vendor_id: PciVendor,
    device_id: u32,
    device_name: [i8; ash::vk::MAX_PHYSICAL_DEVICE_NAME_SIZE],
    device_type: vk::PhysicalDeviceType,
    // Available device extensions
    available_extensions: Vec<vk::ExtensionProperties>,
    // Device Extensions to load
    extensions_to_load: HashMap<DeviceExtensions, bool>,
    // Available Features
    device_features: vk::PhysicalDeviceFeatures,
    // Enabled Features
    enabled_features: vk::PhysicalDeviceFeatures,
    // We only store the results from the queries here, we dont select surface options until we create a swapchain
    surface_capabilities: vk::SurfaceCapabilitiesKHR,
    surface_formats: Vec<vk::SurfaceFormatKHR>,
    present_modes: Vec<vk::PresentModeKHR>,
}

#[derive(Debug)]
// This class does not create any actual queues it merely gathers all the queues that the user wants to
// create in order to hopefully optimize queue creation, in addition it performs no validation of the results
// Meaning that if a queue could no
pub struct QueueManager<'a> {
    queues_to_create: Vec<QueueToCreate>,
    family_data: &'a [QueueFamily],
    index: usize, // Index of the next queue that is create
}