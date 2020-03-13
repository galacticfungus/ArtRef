use ash::vk;
use super::{PciVendor, ConfigureDevice};

/// A fully configured device ready for use
pub struct VulkanDevice {
    // TODO: Extract hte relevant daa for a configured device from gpu
    physical_device: vk::PhysicalDevice,
    queues: Vec<vk::Queue>,
    device: ash::Device,
    enabled_features: vk::PhysicalDeviceFeatures,
    surface_capabilities: vk::SurfaceCapabilitiesKHR,
    surface_formats: Vec<vk::SurfaceFormatKHR>,
    present_modes: Vec<vk::PresentModeKHR>,
    api_version: u32,
    driver_version: u32,
    vendor_id: PciVendor,
    device_id: u32,
    device_name: [i8; ash::vk::MAX_PHYSICAL_DEVICE_NAME_SIZE],
}

impl VulkanDevice {
    // pub fn new() -> VulkanDevice {
    //     VulkanDevice {

    //     }
    // }
}