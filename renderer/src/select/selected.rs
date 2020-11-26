use super::SelectedDevice;
use crate::{Gpu, PciVendor, QueueFamily, Version};
use erupt::vk1_0 as vk;

impl SelectedDevice {
    pub fn new(
        device_handle: vk::PhysicalDevice,
        queue_families: Vec<QueueFamily>,
        api_version: Version,
        driver_version: u32,
        vendor_id: PciVendor,
        device_id: u32,
        device_name: [i8; erupt::vk1_0::MAX_PHYSICAL_DEVICE_NAME_SIZE as usize],
        device_type: vk::PhysicalDeviceType,
        available_extensions: Vec<vk::ExtensionProperties>,
        device_features: vk::PhysicalDeviceFeatures,
    ) -> SelectedDevice {
        SelectedDevice {
            device_handle,
            queue_families,
            api_version,
            driver_version,
            vendor_id,
            device_id,
            device_name,
            device_type,
            available_extensions,
            device_features,
        }
    }
}

impl From<Gpu> for SelectedDevice {
    fn from(gpu: Gpu) -> Self {
        SelectedDevice::new(
            gpu.device_handle,
            gpu.queue_families,
            gpu.api_version,
            gpu.driver_version,
            gpu.vendor_id,
            gpu.device_id,
            gpu.device_name,
            gpu.device_type,
            gpu.available_extensions,
            gpu.device_features,
        )
    }
}
