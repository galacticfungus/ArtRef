use erupt::vk1_0 as vk;
use erupt::extensions::khr_surface as surface;
use crate::{Version, PciVendor, DeviceExtensions, Features, DeviceQueue, RendererQueues};

use std::collections::HashMap;
use std::os::raw::c_char;

/// A fully configured device ready for use

pub struct VulkanDevice {
    // TODO: Extract the relevant data for a configured device from gpu
    physical_device: vk::PhysicalDevice,
    // A structure that contains information regarding all the Vulkan queues we created
    render_queues: RendererQueues,
    // A vector of the vulkan queues we created
    // queues: Vec<DeviceQueue>,
    device: erupt::DeviceLoader,
    enabled_features: vk::PhysicalDeviceFeatures,
    extensions_loaded: HashMap<DeviceExtensions, bool>,
    surface_capabilities: surface::SurfaceCapabilitiesKHR,
    surface_formats: Vec<surface::SurfaceFormatKHR>,
    present_modes: Vec<surface::PresentModeKHR>,
    api_version: Version,
    driver_version: u32,
    vendor_id: PciVendor,
    device_id: u32,
    device_name: [i8; erupt::vk1_0::MAX_PHYSICAL_DEVICE_NAME_SIZE as usize],
}

impl VulkanDevice {
    pub fn new(physical_device: vk::PhysicalDevice, 
        // queues: Vec<DeviceQueue>,
        render_queues: RendererQueues,
        enabled_features: vk::PhysicalDeviceFeatures, 
        extensions_loaded: HashMap<DeviceExtensions, bool>,
        surface_capabilities: surface::SurfaceCapabilitiesKHR,
        surface_formats: Vec<surface::SurfaceFormatKHR>,
        present_modes: Vec<surface::PresentModeKHR>,
        device: erupt::DeviceLoader,
        vendor_id: PciVendor,
        device_id: u32,
        api_version: Version,
        driver_version: u32,
        device_name: [c_char; erupt::vk1_0::MAX_PHYSICAL_DEVICE_NAME_SIZE as usize]) -> VulkanDevice {
        VulkanDevice {
            // queues,
            render_queues,
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

    pub fn get_surface_formats(&self) -> &[surface::SurfaceFormatKHR] {
        self.surface_formats.as_slice()
    }

    pub fn get_present_modes(&self) -> &[surface::PresentModeKHR] {
        self.present_modes.as_slice()
    }

    pub fn get_surface_capabilities(&self) -> &surface::SurfaceCapabilitiesKHR {
        &self.surface_capabilities
    }

    pub fn handle(&self) -> &erupt::DeviceLoader {
        &self.device
    }

    pub fn queues(&self) -> &RendererQueues {
        &self.render_queues
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;

    mock!{
        RenderDevice {}
    }
    

    #[test]
    fn test_() {
        
    }
}