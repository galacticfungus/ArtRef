mod gpu;
mod device;
mod filter;

use ash::version::{InstanceV1_0, EntryV1_0};
use ash::vk;

#[cfg(test)]
pub use gpu::TestGpuBuilder;

use core::fmt::Debug;
use std::ffi::CStr;

use crate::renderer::{PciVendor, QueueFamily, Features,};
use crate::renderer::ExtensionManager;
use crate::error;


pub struct DeviceSelector<'a> {
    instance: &'a ash::Instance,
    // entry: &'a ash::Entry,
    suitable_devices: Vec<Gpu>,
    extensions_to_load: Vec<&'static std::ffi::CStr>,
}

pub trait SupportDeviceFiltering {
    // Get a slice of the devices to filter
    fn devices(&self) -> &[Gpu];
    // Get a mutable vector of the devices to filter
    fn devices_mut(&mut self) -> &mut Vec<Gpu>;
    fn extensions(&mut self, extensions_to_load: Vec<&'static CStr>);
}

/// This trait is implemented for free when SupportDeviceFiltering is implemented
/// Note that these filters often work in reverse meaning that the indexes of elements that
/// are to be removed are collected
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
        where F: Fn(&mut ExtensionManager) -> ();
}

// Must be clonable so that errors can access a list of Gpu's
#[derive(Clone)]
pub struct Gpu {
    device_handle: vk::PhysicalDevice,
    queue_families: Vec<QueueFamily>,
    api_version: u32,
    driver_version: u32,
    vendor_id: PciVendor,
    device_id: u32,
    device_name: [i8; ash::vk::MAX_PHYSICAL_DEVICE_NAME_SIZE],
    device_type: vk::PhysicalDeviceType,
    available_extensions: Vec<vk::ExtensionProperties>,
    device_features: vk::PhysicalDeviceFeatures,
    surface_capabilities: vk::SurfaceCapabilitiesKHR,
    surface_formats: Vec<vk::SurfaceFormatKHR>,
    present_modes: Vec<vk::PresentModeKHR>,
    // pipelinecacheID,
    // limits,
    // sparse_properties,
}

pub struct DeviceFilter {
    devices_to_filter: Vec<Gpu>,
    extensions_to_load: Vec<&'static CStr>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::gpu::TestGpuBuilder;

    // Creates the necessary Vulkan objects to perform a test
    pub fn init_vulkan() -> (ash::Entry, ash::Instance) {
        let entry = ash::Entry::new().expect("Failed to init Vulkan when running test");
        let ici = vk::InstanceCreateInfo {
            ..Default::default()
        };
        let instance = unsafe { entry.create_instance(&ici, None) }.expect("Failed to create Vulkan instance");
        (entry, instance)
    }

    impl<'a> DeviceSelector<'a> {
        // Creates a test selector from a vector of Gpu's
        pub fn create_test_selector(instance: &'a ash::Instance, entry: &'a ash::Entry,
            suitable_devices: Vec<Gpu>,
        ) -> DeviceSelector<'a> {
            
            
            let selector = DeviceSelector {
                instance,
                suitable_devices,
                extensions_to_load: Vec::default(),
            };
            selector
        }
    }

    #[test]
    fn test_gpu_feature_api() {
        use crate::renderer::features::Features;
        let mut gpu = TestGpuBuilder::new()
                            .create_device();
        // TODO: Finish test
        println!("Test is {:?}", gpu);
        // b.feature(Features::GeometryShader);
        let c = gpu.feature(&Features::GeometryShader);
        
    }

    #[test]
    fn test_is_discrete() {
        let (entry, instance) = init_vulkan();
        let gpu = TestGpuBuilder::new()
            .pick_device_type(vk::PhysicalDeviceType::DISCRETE_GPU)
            .create_device();
        let gpu2 = TestGpuBuilder::new()
            .pick_device_type(vk::PhysicalDeviceType::DISCRETE_GPU)
            .create_device();
        let gpu3 = TestGpuBuilder::new()
            .pick_device_type(vk::PhysicalDeviceType::INTEGRATED_GPU)
            .create_device();
        let devices = vec![gpu, gpu2, gpu3];
        let mut selector = DeviceSelector::create_test_selector(&instance, &entry, devices);
        println!("{:?}", selector);
        let result = selector.is_discrete();
        println!("{:?}", result);
        assert_eq!(result.suitable_devices.len(), 2);
    }

    #[test]
    fn test_if_vendor() {
        // How to test this - only way is to scan each device once at init and store the results
        // But this requires being able to create fake devices easily
        let (entry, instance) = init_vulkan();
        let gpu = TestGpuBuilder::new()
            .pick_vendor(PciVendor::NVidia)
            .pick_device_type(vk::PhysicalDeviceType::INTEGRATED_GPU)
            .create_device();
        let gpu2 = TestGpuBuilder::new()
            .pick_vendor(PciVendor::NVidia)
            .pick_device_type(vk::PhysicalDeviceType::DISCRETE_GPU)
            .create_device();
        let gpu3 = TestGpuBuilder::new()
            .pick_device_type(vk::PhysicalDeviceType::INTEGRATED_GPU)
            .create_device();
        let devices = vec![gpu, gpu2, gpu3];
        // Apply the following only to Nvidia devices
        let mut selector = DeviceSelector::create_test_selector(&instance, &entry, devices);
        // Get the number of items remaining
        let result = selector.if_vendor(PciVendor::NVidia, |x| {
            x.is_discrete();
        }).suitable_devices.len();
        // There are 2 nvidia devices and one of them is discrete, that leaves 2 devices since the discrete filter is only applied to nvidia cards
        assert_eq!(result, 2);
    }
}
