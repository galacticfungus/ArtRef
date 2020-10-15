use super::{DeviceSelector, Gpu, DeviceFilter};
use crate::error;
use crate::{QueueFamily, Surface, PciVendor, ConfigureDevice};

use erupt::vk1_0 as vk;
use erupt::extensions::khr_surface as surface;

impl<'a> DeviceSelector<'a> {
    fn get_device_queues(instance: &erupt::InstanceLoader, physical_device: vk::PhysicalDevice, surface: &mut Surface) -> (bool, Vec<QueueFamily>) {
        let queue_families =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device, None) };
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
    
    fn get_surface_properties(physical_device: erupt::vk1_0::PhysicalDevice, surface: &mut Surface) -> Result<(surface::SurfaceCapabilitiesKHR, Vec<surface::SurfaceFormatKHR>, Vec<surface::PresentModeKHR>), error::Error> {
        // Get surface characteristics
        let surface_capabilities = surface.get_surface_capabilities(physical_device)?;
        let surface_formats = surface.get_surface_formats(physical_device)?;
        let presentation_modes = surface.get_surface_presentation_modes(physical_device)?;

        // TODO: return a bunch of surface capabilities
        // TODO: Should this return a struct with all these items boxed
        return Ok((surface_capabilities, surface_formats, presentation_modes));
        
    }

    pub fn get_device_properties(instance: &'a erupt::InstanceLoader, physical_device: vk::PhysicalDevice, surface: &mut Surface) -> Result<Gpu, error::Error> {
        // TODO: Do we auto filter GPU's that can't present to our surface
        let (presentable, device_queues) = Self::get_device_queues(instance, physical_device, surface);
        if presentable == false {
            return Err(error::Error::NotPresentableDevice)
        }
        let device_properties =
            unsafe { instance.get_physical_device_properties(physical_device, None) };

        let available_extensions =
            unsafe { instance.enumerate_device_extension_properties(physical_device, None, None) }
                .expect("Failed to get device extensions");
        let device_features = unsafe { instance.get_physical_device_features(physical_device, None) };
        
        let (surface_capabilities, surface_formats, present_modes) = Self::get_surface_properties(physical_device, surface)?;
        
        
        let gpu = Gpu::new(
            physical_device,
            device_properties,
            device_queues,
            available_extensions,
            device_features,
            surface_capabilities,
            surface_formats,
            present_modes,
        );
        Ok(gpu)
    }

    // TODO: This may need to return a result since a selector can fail to be created if device extensions fails
    pub fn new(instance: &'a erupt::InstanceLoader, surface: &mut Surface) -> Result<Self, error::Error> {
        let devices = unsafe {
            instance
                .enumerate_physical_devices(None)}.expect("Failed to get list of possible devices");
                // Can fail with
                // VK_ERROR_INITIALIZATION_FAILED
                // VK_ERROR_OUT_OF_HOST_MEMORY
                // VK_ERROR_OUT_OF_DEVICE_MEMORY
            
        if devices.is_empty() {
            return Err(error::Error::NoDevicesFound);
        }


        let mut available_devices = Vec::with_capacity(devices.len());
        for physical_device in devices {
            let new_device = Self::get_device_properties(instance, physical_device, surface);
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
            suitable_devices: available_devices,
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

    pub fn select_device(&mut self) -> Gpu {
        // TODO: Is this really optimal, we can use some inherit qualities of the remaining devices to pick one, ie available memory
        let device_picked = self.suitable_devices.swap_remove(0);
        device_picked
    }
}

impl<'a> std::fmt::Debug for DeviceSelector<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.write_fmt(format_args!(
            "Devices in Selector: {:?}",
            self.suitable_devices
        ))
    }
}