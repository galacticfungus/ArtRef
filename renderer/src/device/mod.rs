mod create;
mod device;
mod indirect_count;
mod extensions;

pub use indirect_count::DrawIndirectCount;
pub use extensions::DeviceExtensions;
pub use create::ConfigureDevice;
pub use create::PresentMode;
use ash::vk;
use std::collections::HashMap;

use super::{QueueFamily, QueueToCreate, PciVendor, Features, features::Feature, ExtensionManager};

/// A fully configured device ready for use
pub struct VulkanDevice {
    // TODO: Extract hte relevant daa for a configured device from gpu
    physical_device: vk::PhysicalDevice,
    // A vector of the vulkan queues we created
    queues: Vec<vk::Queue>,
    device: ash::Device,
    enabled_features: vk::PhysicalDeviceFeatures,
    extensions_loaded: HashMap<DeviceExtensions, bool>,
    surface_capabilities: vk::SurfaceCapabilitiesKHR,
    surface_formats: Vec<vk::SurfaceFormatKHR>,
    present_modes: Vec<vk::PresentModeKHR>,
    api_version: u32,
    driver_version: u32,
    vendor_id: PciVendor,
    device_id: u32,
    device_name: [i8; ash::vk::MAX_PHYSICAL_DEVICE_NAME_SIZE],
}