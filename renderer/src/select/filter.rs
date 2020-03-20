use super::{DeviceFilter, SupportDeviceFiltering, Gpu, DeviceSelector, FiltersDevices, DeviceExtensions};
use crate::{ExtensionManager, Features};
use crate::error;

use std::ffi::CStr;

use ash::vk;

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
        F: Fn(&mut ExtensionManager<DeviceExtensions>) -> (),
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
                                missing_extensions.push(extension.get_name().to_owned());
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