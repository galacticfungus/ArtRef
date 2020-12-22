mod device;
mod filter;
mod selected;

use erupt::vk1_0 as vk;

use super::Gpu;

use crate::error;
use crate::{DeviceExtensions, Features, PciVendor, QueueFamily};
use crate::{ExtensionManager, Version};
use erupt::extensions::khr_surface as surface;
use std::collections::HashSet;
use std::ffi::CStr;

pub struct DeviceSelector<'a> {
    instance: &'a erupt::InstanceLoader,
    suitable_devices: SuitableDevices,
    surface: surface::SurfaceKHR,
}

pub struct SuitableDevices {
    suitable_devices: Vec<Gpu>,
}

pub trait SupportDeviceFiltering {
    // Get a slice of the devices to filter
    fn devices(&self) -> &[Gpu];
    // Get a mutable vector of the devices to filter
    fn devices_mut(&mut self) -> &mut Vec<Gpu>;
}

/// This trait is implemented for free when SupportDeviceFiltering is implemented
/// Note that these filters work by collecting the index of items to be removed
pub trait FiltersDevices<'a> {
    // TODO: explicitly prefer a physical device that supports drawing and presentation in the same queue
    fn has_queue(&'a mut self, operations_supported: vk::QueueFlags, must_present: bool);
    fn requires_queue(
        &'a mut self,
        operations_required: vk::QueueFlags,
    ) -> Result<(), error::Error>;
    fn has_graphics_queue(&'a mut self);
    fn is_discrete(&'a mut self);
    fn is_integrated(&'a mut self);
    fn has_feature(&'a mut self, feature: &Features);
    fn required_device_extensions<F>(
        &'a mut self,
        select_extensions: F,
    ) -> Result<(), error::Error>
    where
        F: Fn(&mut ExtensionManager<DeviceExtensions>) -> ();
}

pub struct DeviceFilter {
    devices_to_filter: Vec<Gpu>,
    extensions_to_load: Vec<&'static CStr>,
}

pub struct SelectedDevice {
    pub(super) device_handle: vk::PhysicalDevice,
    pub(super) queue_families: Vec<QueueFamily>,
    pub(super) api_version: Version,
    pub(super) driver_version: u32,
    pub(super) vendor_id: PciVendor,
    pub(super) device_id: u32,
    pub(super) device_name: [i8; erupt::vk1_0::MAX_PHYSICAL_DEVICE_NAME_SIZE as usize],
    pub(super) device_type: vk::PhysicalDeviceType,
    pub(super) available_extensions: Vec<vk::ExtensionProperties>,
    pub(super) device_features: vk::PhysicalDeviceFeatures,
}
