use ash::vk;
use super::{PciVendor, ConfigureDevice, Extensions};
use std::ffi::CStr;
use std::os::raw::c_char;
use std::collections::HashMap;

/// A fully configured device ready for use
pub struct VulkanDevice {
    // TODO: Extract hte relevant daa for a configured device from gpu
    physical_device: vk::PhysicalDevice,
    // A vector of the vulkan queues we created
    queues: Vec<vk::Queue>,
    device: ash::Device,
    enabled_features: vk::PhysicalDeviceFeatures,
    extensions_loaded: HashMap<Extensions, bool>,
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
    pub fn new(physical_device: vk::PhysicalDevice, 
        queues: Vec<vk::Queue>, 
        enabled_features: vk::PhysicalDeviceFeatures, 
        extensions_loaded: HashMap<Extensions, bool>,
        surface_capabilities: vk::SurfaceCapabilitiesKHR,
        surface_formats: Vec<vk::SurfaceFormatKHR>,
        present_modes: Vec<vk::PresentModeKHR>,
        device: ash::Device,
        vendor_id: PciVendor,
        device_id: u32,
        api_version: u32,
        driver_version: u32,
        device_name: [c_char; vk::MAX_PHYSICAL_DEVICE_NAME_SIZE]) -> VulkanDevice {
        VulkanDevice {
            queues,
            enabled_features,
            extensions_loaded,
            surface_capabilities,
            surface_formats,
            present_modes,
            device,
            api_version,
            physical_device,
            vendor_id,
            driver_version,
            device_id,
            device_name,
        }
    }
}