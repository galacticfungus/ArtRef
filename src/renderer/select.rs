use ash::version::{InstanceV1_0, EntryV1_0};
use ash::vk;


use core::fmt::Debug;
use std::ffi::CStr;

use super::{Gpu, PciVendor, QueueFamily};
use crate::renderer::ExtensionManager;
use crate::error;

use super::create::ConfigureDevice;
use super::Features;
use super::Surface;

pub struct DeviceSelector<'a> {
    instance: &'a ash::Instance,
    entry: &'a ash::Entry,
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

// impl<T> Speaks for T where T: Animal {
//     fn speak(&self) {
//         println!("The {} said {}", self.animal_type(), self.noise());
//     }
// }

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
    fn has_geo_shader(&'a mut self) -> &'a mut Self;
    fn has_graphics_queue(&'a mut self) -> &'a mut Self;
    fn is_discrete(&'a mut self) -> &'a mut Self;
    fn is_integrated(&'a mut self) -> &'a mut Self;
    fn has_feature(&'a mut self, feature: &Features) -> &'a mut Self;
    fn required_device_extensions<F>(&'a mut self, select_extensions: F) -> Result<&'a mut Self, error::Error>
        where F: Fn(&mut ExtensionManager) -> ();
}

impl<'a, T> FiltersDevices<'a> for T
where
    T: SupportDeviceFiltering,
{
    // Set device extensions, the select_extensions closure
    fn required_device_extensions<F>(
        &'a mut self,
        select_extensions: F,
    ) -> Result<&'a mut T, error::Error>
    where
        F: Fn(&mut ExtensionManager) -> (),
    {
        use std::ffi::CString;
        // TODO: Error handling code needs to destroy the instance - using drop wont work
        // TODO: Since we cache device properties this code will fail in some circumstances unless the extension lists are pre loaded
        // TODO: For example this would be impossible when creating a selector from a vec as there is no instance to use
        // Currently select_extensions is called for each device rather than obtaining a list of needed extensions and filtering the devices all at once
        // Extension manager needs to be changed so that it checks extensions after all the requests have been made
        // or simplify extension manager to return a list of extensions that need to be loaded and do all checking outside of the manager
        let mut em = ExtensionManager::new();
        select_extensions(&mut em);
        let requested_extensions = em.get_extensions();
        let filtered_devices: Vec<usize> = self
            .devices()
            .iter()
            .enumerate()
            .filter(|(_, device)| {
                requested_extensions
                    .iter() // Iterate over the extensions to load and check that the device supports each one
                    .all(|ext| device.has_extension(ext)) == false // We only want the index of devices that fail to have all the required extensions
            })
            .map(|(index, _)| index)
            .collect();
        
        if filtered_devices.len() > 0 && filtered_devices.len() < self.devices().len() {
            // Can apply filter
            let devices_to_filter = self.devices_mut();
            for index in filtered_devices {
                devices_to_filter.swap_remove(index);
            }
        } else {
            // There are two ways we can get here
            // 1) - If there are no devices that support the required extensions
            // 2) - All the devices support the required extensions
            // only the first is a problem
            if filtered_devices.is_empty() {
                // Return both the device and the extensions that were missing as part of the error
                let devices: Vec<(Gpu, Vec<CString>)> = self.devices()
                    .into_iter()
                    .map(|device| {
                        let mut missing_extensions: Vec<CString> = Vec::new();
                        for extension in requested_extensions.iter() {
                            if device.has_extension(extension) == false {
                                missing_extensions.push((*extension).to_owned());
                            }
                        }
                        // devices needs to last as long as self ie the selector
                        (device.clone(), missing_extensions)
                    })
                    .collect();
                return Err(error::Error::MissingRequiredDeviceExtensions(devices));
            }
        }
        // Set the new extensions to load - if no devices supporting these extensions were found this point is never reached
        // TODO: Add an extension one at a time
        // TODO: extensions_to_load should be a Hashmap if optional device extensions are to be supported
        self.extensions(requested_extensions);
        Ok(self)
    }

    // Will remove any device that isn't a discrete GPU, if no devices are discrete then no devices are filtered
    fn is_discrete(&'a mut self) -> &'a mut T {
        let filtered_indexes: Vec<usize> = self
            .devices()
            .iter()
            .enumerate()
            .filter(|(_, x)| x.is_discrete() == false) // Get the index of any items that are not Discrete GPU's
            .map(|(index, _)| index)
            .collect();
        if filtered_indexes.len() > 0 && filtered_indexes.len() < self.devices().len() {
            // Can apply filter
            let mutable_devices = self.devices_mut();
            for index in filtered_indexes {
                mutable_devices.swap_remove(index);
            }
        }
        self
    }

    // Will remove any device that isn't an integrated GPU, if there are no integrated GPU's then no devices are filtered
    fn is_integrated(&'a mut self) -> &'a mut T {
        let filtered_indexes: Vec<usize> = self
            .devices()
            .iter()
            .enumerate()
            .filter(|(_, x)| x.is_integrated() == false) // Get the index of any items that are not Discrete GPU's
            .map(|(index, _)| index)
            .collect();
        if filtered_indexes.len() > 0 && filtered_indexes.len() < self.devices().len() {
            // Can apply filter
            let mutable_devices = self.devices_mut();
            for index in filtered_indexes {
                mutable_devices.swap_remove(index);
            }
        }
        self
    }

    fn has_graphics_queue(&'a mut self) -> &'a mut T {
        let indexes_to_keep: Vec<usize> = self
            .devices()
            .iter()
            .enumerate()
            .filter(|(_, device)| device.has_graphics_queue() == false)
            .map(|(index, _)| index)
            .collect();
        if indexes_to_keep.len() > 0 && indexes_to_keep.len() < self.devices().len() {
            // Can apply filter
            let suitable_devices = self.devices_mut();
            for index in indexes_to_keep {
                suitable_devices.swap_remove(index);
            }
        }
        self
    }

    fn has_queue(&'a mut self, operations_required: vk::QueueFlags, must_present: bool) -> &'a mut T {
        let indexes_to_keep: Vec<usize> = self
            .devices()
            .iter()
            .enumerate()
            .filter(|(_, device)| device.supports_operations(operations_required, must_present) == false) // Collect indexes that dont support the operations
            .map(|(index, _)| index)
            .collect();
        if indexes_to_keep.len() > 0 && indexes_to_keep.len() < self.devices().len() {
            // Can apply filter
            let suitable_devices = self.devices_mut();
            for index in indexes_to_keep {
                suitable_devices.swap_remove(index);
            }
        }
        self
    }
    // Filters out any device that doesn't support the required operations in a single queue family, if no devices are found supporting the required operations then an error is returned
    fn requires_queue(&'a mut self, operations_required: vk::QueueFlags) -> Result<&'a mut T, error::Error> {
        // TODO: Should this base its comparision on all families available or should it base it on an individual family
        // FIXME: This can simply iterate over all the families and OR their flags together and then compare that to the required operations
        self.devices_mut()
            .retain(|device| device.supports_operations(operations_required, false));
        if self.devices().is_empty() {
            // A device selector does not own the instance it is using so no need to destroy it on error
            return Err(error::Error::NoGraphicsQueue);
        }
        Ok(self)
    }

    fn has_geo_shader(&'a mut self) -> &mut T {
        let filtered_indexes: Vec<usize> = self
            .devices()
            .iter()
            .enumerate()
            .filter(|(_, x)| x.has_geo_shader() == false)
            .map(|(index, _)| index)
            .collect();
        if filtered_indexes.len() > 0 && filtered_indexes.len() < self.devices().len() {
            // Can apply filter
            for index in filtered_indexes {
                self.devices_mut().swap_remove(index);
            }
        }
        self
    }

    fn has_feature(&'a mut self, feature: &Features) -> &mut T {
        let filtered_indexes: Vec<usize> = self
            .devices()
            .iter()
            .enumerate()
            .filter(move |(_, x)| x.has_feature(feature) == false)
            .map(|(index, _)| index)
            .collect();
        if filtered_indexes.len() > 0 && filtered_indexes.len() < self.devices().len() {
            // Can apply filter
            for index in filtered_indexes {
                self.devices_mut().swap_remove(index);
            }
        }
        self
    }
}

// TODO: Filters that order the devices - Since the first device is chosen after filtering

impl<'a> DeviceSelector<'a> {
    fn get_device_queues(instance: &ash::Instance, physical_device: vk::PhysicalDevice, surface: &mut Surface) -> (bool, Vec<QueueFamily>) {
        let queue_families =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device) };
        let device_queues: Vec<_> = queue_families.into_iter()
            .enumerate()
            .map(|(index, fam)| {
                let presentable = surface.get_surface_support(physical_device, index as u32);
                QueueFamily::new(index, fam.queue_flags, fam.queue_count, fam.timestamp_valid_bits, fam.min_image_transfer_granularity, presentable)
            })
            .collect();

        let gpu_presentable = device_queues.iter().any(|queue| queue.presentable());
        (gpu_presentable, device_queues)
    }
    
    fn get_surface_properties(physical_device: vk::PhysicalDevice, surface: &mut Surface) -> Result<(vk::SurfaceCapabilitiesKHR, Vec<vk::SurfaceFormatKHR>, Vec<vk::PresentModeKHR>), error::Error> {    
        // Get surface characteristics
        let surface_capabilities = surface.get_surface_capabilities(physical_device)?;
        let surface_formats = surface.get_surface_formats(physical_device)?;
        let presentation_modes = surface.get_surface_presentation_modes(physical_device)?;

        // TODO: return a bunch of surface capabilities
        // Should this return a struct with all these items boxed
        return Ok((surface_capabilities, surface_formats, presentation_modes));
        
    }

    pub fn get_device_properties(instance: &'a ash::Instance, entry: &'a ash::Entry, physical_device: vk::PhysicalDevice, surface: &mut Surface) -> Result<Gpu, error::Error> {
        // TODO: Do we auto filter GPU's that can't present to our surface
        let (presentable, device_queues) = Self::get_device_queues(instance, physical_device, surface);
        if presentable == false {
            return Err(error::Error::NotPresentableDevice)
        }
        let device_properties =
            unsafe { instance.get_physical_device_properties(physical_device) };

        let device_extensions =
            unsafe { instance.enumerate_device_extension_properties(physical_device) }
                .expect("Failed to get device extensions");

        let device_features = unsafe { instance.get_physical_device_features(physical_device) };
        
        let (surface_capabilities, surface_formats, present_modes) = Self::get_surface_properties(physical_device, surface)?;
        
        
        let gpu = Gpu::new(
            physical_device,
            device_properties,
            device_queues,
            device_extensions,
            device_features,
            surface_capabilities,
            surface_formats,
            present_modes,
        );
        Ok(gpu)
    }

    // TODO: This may need to return a result since a selector can fail to be created if device extensions fails
    pub fn new(instance: &'a ash::Instance, entry: &'a ash::Entry, surface: &mut Surface) -> Result<Self, error::Error> {
        let devices = unsafe {
            instance
                .enumerate_physical_devices()?};
                // Can fail with
                // VK_ERROR_INITIALIZATION_FAILED
                // VK_ERROR_OUT_OF_HOST_MEMORY
                // VK_ERROR_OUT_OF_DEVICE_MEMORY
        if devices.is_empty() {
            return Err(error::Error::NoDevicesFound);
        }


        let mut available_devices = Vec::with_capacity(devices.len());
        for physical_device in devices {
            let new_device = Self::get_device_properties(instance, entry, physical_device, surface);
            match new_device {
                Ok(device) => available_devices.push(device),
                Err(error::Error::NotPresentableDevice) => continue, // If a device can't present we filter it from the selector results
                Err(error) => return Err(error),
            }
        }
        // If all the devices were filtered then none of the devices were able to present to the surface
        if available_devices.is_empty() {
            return Err(error::Error::NoDevicesCanPresent);
        }
        let selector = DeviceSelector {
            instance,
            entry,
            suitable_devices: available_devices,
            extensions_to_load: Vec::default(),
        };
        Ok(selector)
    }

    pub fn if_vendor<F>(&'a mut self, vendor: PciVendor, device_filter: F) -> &'a mut Self
    where
        F: Fn(&mut DeviceFilter),
    {   
        let mut i = 0;
        // TODO: Replace this with drain_filter when available
        let mut vendor_devices = Vec::new();
        while i != self.suitable_devices.len() {
            if self.suitable_devices[i].is_vendor(&vendor) {
                // TODO: This can be optimized by obtaining a raw slice to the vec and tracking removed items but drain_filter already does this but is currently nightly
                let filtered_gpu = self.suitable_devices.remove(i);
                vendor_devices.push(filtered_gpu);
            } else {
                i += 1;
            }
        }
        let mut vendor_filter = DeviceFilter::new(vendor_devices);
        // Execute the filter
        device_filter(&mut vendor_filter);
        // Get the devices from the filter
        let mut filtered_devices = vendor_filter.get_filtered_devices();
        self.suitable_devices.append(&mut filtered_devices);
        // other_devices.append(&mut filtered_nvidia);
        self
    }

    pub fn select_device(&mut self) -> ConfigureDevice<'a> {
        use std::mem::take;
        let extensions_to_load = take(&mut self.extensions_to_load);
        // TODO: Is this really optimal, we can use some inherit qualities of the remaining devices to pick one, ie available memory
        let mut device_picked = self.suitable_devices.swap_remove(0);
        device_picked.add_device_extensions(extensions_to_load);
        ConfigureDevice::new(self.instance, device_picked)
    }
}

pub struct DeviceFilter {
    devices_to_filter: Vec<Gpu>,
    extensions_to_load: Vec<&'static CStr>,
}

impl DeviceFilter {
    pub fn new(devices_to_filter: Vec<Gpu>) -> DeviceFilter {
        DeviceFilter { devices_to_filter, extensions_to_load: Vec::default() }
    }

    pub fn get_filtered_devices(self) -> Vec<Gpu> {
        self.devices_to_filter
    }
}

impl SupportDeviceFiltering for DeviceFilter {
    fn devices(&self) -> &[Gpu] {
        self.devices_to_filter.as_slice()
    }

    fn devices_mut(&mut self) -> &mut Vec<Gpu> {
        &mut self.devices_to_filter
    }

    fn extensions(&mut self, extensions_to_load: Vec<&'static CStr>) {
        self.extensions_to_load = extensions_to_load;
    }
}

impl<'a> SupportDeviceFiltering for DeviceSelector<'a> {
    fn devices(&self) -> &[Gpu] {
        self.suitable_devices.as_slice()
    }

    fn devices_mut(&mut self) -> &mut Vec<Gpu> {
        &mut self.suitable_devices
    }

    fn extensions(&mut self, extensions_to_load: Vec<&'static CStr>) {
        self.extensions_to_load = extensions_to_load;
    }
}

impl<'a> Debug for DeviceSelector<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.write_fmt(format_args!(
            "Devices in Selector: {:?}",
            self.suitable_devices
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::renderer::gpu::{Gpu, TestGpuBuilder};

    // TODO: How to build a customizable group of test Gpu's

    pub fn init_vulkan() -> (ash::Entry, ash::Instance) {
        let entry = ash::Entry::new().expect("Failed to init Vulkan when running test");
        let ici = vk::InstanceCreateInfo {
            ..Default::default()
        };
        let instance = unsafe { entry.create_instance(&ici, None) }.expect("Failed to create Vulkan instance");
        (entry, instance)
    }

    impl<'a> DeviceSelector<'a> {
        pub fn create_test_selector(instance: &'a ash::Instance, entry: &'a ash::Entry,
            suitable_devices: Vec<Gpu>,
        ) -> DeviceSelector<'a> {
            
            
            let selector = DeviceSelector {
                instance,
                entry,
                suitable_devices,
                extensions_to_load: Vec::default(),
            };
            selector
        }
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

    #[test]
    fn test_presentation_filter() {
        // testing the presentation filter will be challenging
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
