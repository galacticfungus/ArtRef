mod device;
mod filter;

use ash::version::{InstanceV1_0, EntryV1_0};
use ash::vk;

use super::Gpu;

use std::ffi::CStr;

use crate::{PciVendor, QueueFamily, Features, DeviceExtensions};
use crate::ExtensionManager;
use crate::error;


pub struct DeviceSelector<'a> {
    instance: &'a ash::Instance,
    // entry: &'a ash::Entry,
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
    fn has_queue(&'a mut self, operations_supported: vk::QueueFlags, must_present: bool) -> &'a mut Self;
    fn requires_queue(
        &'a mut self,
        operations_required: vk::QueueFlags,
    ) -> Result<&'a mut Self, error::Error>;
    fn has_graphics_queue(&'a mut self) -> &'a mut Self;
    fn is_discrete(&'a mut self) -> &'a mut Self;
    fn is_integrated(&'a mut self) -> &'a mut Self;
    fn has_feature(&'a mut self, feature: &Features) -> &'a mut Self;
    fn required_device_extensions<F>(&'a mut self, select_extensions: F) -> Result<&'a mut Self, error::Error>
        where F: Fn(&mut ExtensionManager<DeviceExtensions>) -> ();
}

pub struct DeviceFilter {
    devices_to_filter: Vec<Gpu>,
    extensions_to_load: Vec<&'static CStr>,
}