use super::{
    DeviceExtensions, DeviceFilter, DeviceSelector, ExtensionManager, Features, FiltersDevices,
    Gpu, SelectedDevice, SuitableDevices,
};
use crate::{error::{Error, ErrorKind}, ConfigureDevice, ConfigurePresenter, PciVendor, QueueFamily};
use erupt::extensions::khr_surface as surface;
use erupt::vk1_0 as vk;
use std::collections::HashSet;

impl<'a> DeviceSelector<'a> {
    fn get_device_queues(
        instance: &erupt::InstanceLoader,
        physical_device: vk::PhysicalDevice,
        surface: surface::SurfaceKHR,
    ) -> (bool, Vec<QueueFamily>) {
        let queue_families =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device, None) };
        let device_queues: Vec<_> = queue_families
            .into_iter()
            .enumerate()
            .map(|(index, fam)| {
                let presentable =
                    Self::get_surface_support(instance, physical_device, index as u32, surface);
                QueueFamily::new(
                    index,
                    fam.queue_flags,
                    fam.queue_count,
                    fam.timestamp_valid_bits,
                    fam.min_image_transfer_granularity,
                    presentable,
                )
            })
            .collect();
        let gpu_presentable = device_queues.iter().any(|queue| queue.presentable());
        (gpu_presentable, device_queues)
    }

    pub fn get_device_properties(
        instance: &erupt::InstanceLoader,
        physical_device: vk::PhysicalDevice,
        surface: surface::SurfaceKHR,
    ) -> Result<Gpu, Error> {
        // TODO: This needs to return a result as it can fail with out of memories as well as surface lost
        let (presentable, device_queues) =
            Self::get_device_queues(instance, physical_device, surface);
        let device_properties =
            unsafe { instance.get_physical_device_properties(physical_device, None) };
        // TODO: If we have enabled some layers here we can pass them in to obtain additional extensions
        // This is a Vulkan CString meaning that it is null terminated as well as UTF8 encoded
        
        let available_extensions =
            match unsafe { instance.enumerate_device_extension_properties(physical_device, None, None) }.result() {
                Ok(extensions) => extensions,
                Err(error) => {
                        use std::ffi::CStr;
                        let c_device_name: &CStr = unsafe { CStr::from_ptr(device_properties.device_name.as_ptr()) };
                        let device_name = match c_device_name.to_str() { // An error here should be impossible since Vulkan Specification states that the c strings must be UTF8
                            Ok(device_name) => Some(device_name.to_string()),
                            Err(_) => None, // Not being able to parse this probably means that the underlying data structure is corrupted since it required to be UTF8
                                            // TODO: Can this be added as context
                        };
                        return Err(Error::new(ErrorKind::FailedToGetDeviceExtensions(device_name), Some(Error::from(error)))
                            .with_context(&"Occured while getting a devices properties during device selection"));
                    },
            };
        // VK_ERROR_OUT_OF_HOST_MEMORY
        // VK_ERROR_OUT_OF_DEVICE_MEMORY
        // VK_ERROR_LAYER_NOT_PRESENT
        
        let device_features =
            unsafe { instance.get_physical_device_features(physical_device, None) };

        // TODO: We only get surface details if a surface was provided and only then if the device can present
        let surface_formats = Self::get_surface_formats(instance, physical_device, surface)
            .expect("Failed to retrieve Surface Formats");
        let present_modes =
            Self::get_surface_presentation_modes(instance, physical_device, surface)
                .expect("Failed to retrieve Surface present modes");
        let surface_capabilities =
            Self::get_surface_capabilities(instance, physical_device, surface)
                .expect("Failed to retrieve surface capabilities");
        let gpu = Gpu::new(
            physical_device,
            device_properties,
            device_queues,
            available_extensions,
            device_features,
            surface_capabilities,
            surface_formats,
            present_modes,
            presentable,
        );
        Ok(gpu)
    }

    // Gets the surface capabilities that are selected when creating a swapchain
    pub fn get_surface_capabilities(
        instance: &erupt::InstanceLoader,
        physical_device: vk::PhysicalDevice,
        surface: surface::SurfaceKHR,
    ) -> Result<surface::SurfaceCapabilitiesKHR, Error> {
        let surface_capabilities = unsafe {
            instance.get_physical_device_surface_capabilities_khr(physical_device, surface, None)
        }
        .result()
        .map_err(|error| Error::new(ErrorKind::SurfaceLost, None))?;
        
        // Errors that can be returned
        // VK_ERROR_OUT_OF_HOST_MEMORY
        // VK_ERROR_OUT_OF_DEVICE_MEMORY
        // VK_ERROR_SURFACE_LOST_KHR
        Ok(surface_capabilities)
    }

    pub fn get_surface_support(
        instance: &erupt::InstanceLoader,
        physical_device: vk::PhysicalDevice,
        family_index: u32,
        surface: surface::SurfaceKHR,
    ) -> bool {
        unsafe {
            instance.get_physical_device_surface_support_khr(
                physical_device,
                family_index,
                surface,
                None,
            )
        // VK_ERROR_OUT_OF_HOST_MEMORY
        // VK_ERROR_OUT_OF_DEVICE_MEMORY
        // VK_ERROR_SURFACE_LOST_KHR
        }
        .expect(format!(
            "Failed to retrieve surface support for family index {}",
            family_index
        ))
    }

    // Get the surface formats supported, used in swap chain creation
    pub fn get_surface_formats(
        instance: &erupt::InstanceLoader,
        physical_device: vk::PhysicalDevice,
        surface: surface::SurfaceKHR,
    ) -> Result<Vec<surface::SurfaceFormatKHR>, Error> {
        let surface_formats = unsafe {
            instance.get_physical_device_surface_formats_khr(physical_device, surface, None)
        }
        .expect("Failed to get surface formats for device");
        // VK_ERROR_OUT_OF_HOST_MEMORY
        // VK_ERROR_OUT_OF_DEVICE_MEMORY
        // VK_ERROR_SURFACE_LOST_KHR
        Ok(surface_formats)
    }

    //
    pub fn get_surface_presentation_modes(
        instance: &erupt::InstanceLoader,
        physical_device: erupt::vk1_0::PhysicalDevice,
        surface: surface::SurfaceKHR,
    ) -> Result<Vec<surface::PresentModeKHR>, Error> {
        let present_modes = unsafe {
            instance.get_physical_device_surface_present_modes_khr(physical_device, surface, None)
        }
        .expect("Failed to get Surface present modes");
        // VK_ERROR_OUT_OF_HOST_MEMORY
        // VK_ERROR_OUT_OF_DEVICE_MEMORY
        // VK_ERROR_SURFACE_LOST_KHR
        Ok(present_modes)
    }

    // TODO: This may need to return a result since a selector can fail to be created
    pub fn new(
        instance: &'a erupt::InstanceLoader,
        surface: surface::SurfaceKHR,
    ) -> Result<Self, Error> {
        let devices = unsafe { instance.enumerate_physical_devices(None) }.result()?;
        // Can fail with
        // VK_ERROR_INITIALIZATION_FAILED
        // VK_ERROR_OUT_OF_HOST_MEMORY
        // VK_ERROR_OUT_OF_DEVICE_MEMORY


        if devices.is_empty() {
            return Err(Error::new(ErrorKind::NoDevicesFound, None));
        }

        let mut available_devices = Vec::with_capacity(devices.len());
        // TODO: This no longer filters devices that can't present from the list which means it needs to be done at device selection time
        for physical_device in devices {
            let new_device = Self::get_device_properties(&instance, physical_device, surface);
            match new_device {
                Ok(device) => available_devices.push(device),
                Err(error) => return Err(error),
            }
        }

        let devices = SuitableDevices {
            suitable_devices: available_devices,
        };
        let selector = DeviceSelector {
            instance,
            suitable_devices: devices,
            surface,
        };
        Ok(selector)
    }

    pub fn if_vendor<F>(mut self, vendor: PciVendor, device_filter: F) -> Self
    where
        F: Fn(&mut DeviceFilter),
    {
        let mut i = 0;
        // TODO: Replace this with drain_filter when available
        let mut vendor_devices = Vec::new();
        while i != self.suitable_devices.suitable_devices.len() {
            if self.suitable_devices.suitable_devices[i].is_vendor(&vendor) {
                // TODO: This can be optimized by obtaining a raw slice to the vec and tracking removed items but drain_filter already does this but is currently nightly
                let filtered_gpu = self.suitable_devices.suitable_devices.remove(i);
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
        self.suitable_devices
            .suitable_devices
            .append(&mut filtered_devices);
        // other_devices.append(&mut filtered_nvidia);
        self
    }

    // TODO: If no surface was created then configurepresenter is None
    /// Select one of the devices that has not been filtered yet
    pub fn select_device(mut self) -> (SelectedDevice, ConfigurePresenter) {
        // TODO: Is this really optimal, we can use some inherit qualities of the remaining devices to pick one, ie available memory
        let device_picked = self.suitable_devices.suitable_devices.swap_remove(0);
        let Gpu {
            api_version,
            available_extensions,
            device_features,
            device_handle,
            device_id,
            device_name,
            device_type,
            driver_version,
            present_modes,
            queue_families,
            surface_capabilities,
            surface_formats,
            vendor_id,
            presentable, // TODO: We currently don't make use of this, we can use it for filtering, more useful when we support creating non-graphics pipelines from devices with no support for presenting
        } = device_picked;
        let configure_presenter = ConfigurePresenter::new(
            self.surface,
            surface_capabilities,
            surface_formats,
            present_modes,
        );
        let device_selected = SelectedDevice::new(
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
        );
        (device_selected, configure_presenter)
    }

    pub fn required_device_extensions<F>(
        mut self,
        select_extensions: F,
    ) -> Result<Self, Error>
    where
        F: Fn(&mut ExtensionManager<DeviceExtensions>) -> (),
    {
        self.suitable_devices
            .required_device_extensions(select_extensions)?;
        Ok(self)
    }

    pub fn is_discrete(mut self) -> Self {
        self.suitable_devices.is_discrete();
        self
    }

    pub fn has_queue(mut self, operations_supported: vk::QueueFlags, must_present: bool) -> Self {
        self.suitable_devices
            .has_queue(operations_supported, must_present);
        self
    }

    pub fn requires_queue(
        mut self,
        operations_required: vk::QueueFlags,
        must_present: bool,
    ) -> Result<Self, Error> {
        self.suitable_devices.requires_queue(operations_required)?;
        Ok(self)
    }

    pub fn has_graphics_queue(mut self) -> Self {
        self.suitable_devices.has_graphics_queue();
        self
    }

    pub fn is_integrated(mut self) -> Self {
        self.suitable_devices.is_integrated();
        self
    }

    pub fn has_feature(mut self, feature: &Features) -> Self {
        self.suitable_devices.has_feature(feature);
        self
    }
}

impl<'a> std::fmt::Debug for DeviceSelector<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.write_fmt(format_args!(
            "Devices in Selector: {:?}",
            self.suitable_devices.suitable_devices
        ))
    }
}
