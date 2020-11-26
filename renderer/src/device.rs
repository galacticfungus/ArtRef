use erupt::vk1_0 as vk;
use crate::{Version, PciVendor, DeviceExtensions, RendererQueues, ConfigurePipeline, Renderpass};

use std::collections::HashMap;
use std::os::raw::c_char;

/// A fully configured device ready for use

pub struct VulkanDevice {
    pub(crate) physical_device: vk::PhysicalDevice,
    // A structure that contains information regarding all the Vulkan queues we created
    pub(crate) render_queues: RendererQueues,
    pub(crate) device: erupt::DeviceLoader,
    pub(crate) enabled_features: vk::PhysicalDeviceFeatures,
    pub(crate) extensions_loaded: HashMap<DeviceExtensions, bool>,
    pub(crate) api_version: Version,
    pub(crate) driver_version: u32,
    pub(crate) vendor_id: PciVendor,
    pub(crate) device_id: u32,
    pub(crate) device_name: [i8; erupt::vk1_0::MAX_PHYSICAL_DEVICE_NAME_SIZE as usize],
}

impl VulkanDevice {
    pub fn new(physical_device: vk::PhysicalDevice, 
        render_queues: RendererQueues,
        enabled_features: vk::PhysicalDeviceFeatures, 
        extensions_loaded: HashMap<DeviceExtensions, bool>,
        device: erupt::DeviceLoader,
        vendor_id: PciVendor,
        device_id: u32,
        api_version: Version,
        driver_version: u32,
        device_name: [c_char; erupt::vk1_0::MAX_PHYSICAL_DEVICE_NAME_SIZE as usize]) -> VulkanDevice {
        VulkanDevice {
            render_queues,
            enabled_features,
            extensions_loaded,
            device,
            api_version,
            physical_device,
            vendor_id,
            driver_version,
            device_id,
            device_name,
        }
    }

    pub fn handle(&self) -> &erupt::DeviceLoader {
        &self.device
    }

    pub fn queues(&self) -> &RendererQueues {
        &self.render_queues
    }
    // TODO: Do we need seperate config for graphics or compute pipelines
    pub fn create_pipeline(&self) -> ConfigurePipeline {
        ConfigurePipeline::new(&self.device)
    }

    pub fn create_renderpass(&self) -> Renderpass {
        Renderpass::new(&self.device)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn test_() {
        
    }
}