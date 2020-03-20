use ash::vk;
use super::{PciVendor, DeviceExtensions};
use std::ffi::CStr;
use std::os::raw::c_char;
use std::collections::HashMap;

use super::VulkanDevice;

impl VulkanDevice {
    pub fn new(physical_device: vk::PhysicalDevice, 
        queues: Vec<vk::Queue>, 
        enabled_features: vk::PhysicalDeviceFeatures, 
        extensions_loaded: HashMap<DeviceExtensions, bool>,
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