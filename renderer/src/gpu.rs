use crate::{QueueFamily, Features, PciVendor, DeviceExtensions, Gpu};
use erupt::vk1_0 as vk;
use erupt::extensions::khr_surface as surface;
use std::ffi::CStr;

// Represents a Gpu available on the local system
// We derive clone for when we are returning an error
// and must take ownership of a borrowed value

impl Gpu {
    pub fn new(
        physical_device: vk::PhysicalDevice,
        properties: vk::PhysicalDeviceProperties,
        device_queues: Vec<QueueFamily>,
        available_extensions: Vec<vk::ExtensionProperties>,
        device_features: vk::PhysicalDeviceFeatures,
        surface_capabilities: surface::SurfaceCapabilitiesKHR,
        surface_formats: Vec<surface::SurfaceFormatKHR>,
        present_modes: Vec<surface::PresentModeKHR>,
    ) -> Self {
        Gpu {
            device_handle: physical_device,
            device_name: properties.device_name,
            device_type: properties.device_type,
            queue_families: device_queues,
            api_version: properties.api_version,
            device_id: properties.device_id,
            vendor_id: PciVendor::from(properties.vendor_id),
            driver_version: properties.driver_version,
            available_extensions,
            device_features,
            surface_capabilities,
            surface_formats,
            present_modes,
        }
    }

    pub fn is_discrete(&self) -> bool {
        if self.device_type == vk::PhysicalDeviceType::DISCRETE_GPU {
            return true;
        }
        false
    }

    pub fn is_integrated(&self) -> bool {
        if self.device_type == vk::PhysicalDeviceType::INTEGRATED_GPU {
            return true;
        }
        false
    }

    pub fn has_feature(&self, feature: &Features) -> bool {
        match feature {
            Features::GeometryShader => self.device_features.geometry_shader > 0,
            Features::TesselationShader => self.device_features.tessellation_shader > 0,
        }
    }

    pub fn supports_operations(&self, operations_requried: vk::QueueFlags, must_present: bool) -> bool {
        for queue in self.queue_families.iter() {
            if queue.has_support_for(operations_requried, must_present) == true {
                return true;
            }
        }
        false
    }

    pub fn has_graphics_queue(&self) -> bool {
        for queue in self.queue_families.iter() {
            if queue.supports_graphics() == true {
                return true;
            }
        }
        false
    }

    pub fn has_compute_queue(&self) -> bool {
        for queue in self.queue_families.iter() {
            if queue.supports_compute() == true {
                return true;
            }
        }
        false
    }

    pub fn is_vendor(&self, vendor: &PciVendor) -> bool {
        if self.vendor_id == *vendor {
            return true;
        }
        false
    }

    pub fn has_extension(&self, extension: &DeviceExtensions) -> bool {
        // Convert the ExtensionProperty to a &CStr extension name
        for available_extension in self
            .available_extensions
            .iter().map(|ext| unsafe { CStr::from_ptr(ext.extension_name.as_ptr()) } )
        {
            if extension.get_name() == available_extension {
                return true;
            }
        }
        false
    }

    pub fn get_queue_families(&self) -> &[QueueFamily] {
        self.queue_families.as_slice()
    }

    pub fn get_mut_queue_families(&mut self) -> &mut [QueueFamily] {
        self.queue_families.as_mut_slice()
    }

    pub fn take_handle(self) -> vk::PhysicalDevice {
        self.device_handle
    }

    pub fn get_handle(&self) -> vk::PhysicalDevice {
        self.device_handle
    }

    pub fn feature(&mut self, feature: &Features) -> bool {
        match feature {
            Features::GeometryShader => self.device_features.geometry_shader > 0,
            Features::TesselationShader => self.device_features.tessellation_shader > 0,
        }
    }

    pub fn get_features(&self) -> &vk::PhysicalDeviceFeatures {
        &self.device_features
    }
}

impl std::fmt::Debug for Gpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        // This is safe since the function can't be called once the pointer is dangling
        let c_str = unsafe { CStr::from_ptr(self.device_name.as_ptr()) };
        // let device_name = format!("Device: {:?}", c_str);
        writeln!(
            f,
            "{:?} a {:?} from {} supports the following queues {:?}",
            c_str, self.device_type, self.vendor_id, self.queue_families
        )
    }
}

impl std::fmt::Display for Gpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.write_fmt(format_args!("{:?}", unsafe {
            CStr::from_ptr(self.device_name.as_ptr())
        }))
    }
}