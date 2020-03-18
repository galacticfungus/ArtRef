use crate::{QueueFamily, Features, Feature, PciVendor, Extensions};
use super::Gpu;

use ash::vk;

use std::ffi::{CStr};
use std::os::raw::c_char;

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
        surface_capabilities: vk::SurfaceCapabilitiesKHR,
        surface_formats: Vec<vk::SurfaceFormatKHR>,
        present_modes: Vec<vk::PresentModeKHR>,
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

    pub fn has_extension(&self, extension: &Extensions) -> bool {
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

impl std::convert::AsRef<Gpu> for Gpu {
    fn as_ref(&self) -> &Gpu {
        &self
    }
}

#[cfg(test)]
// This needs to be outside the test module so that other test modules can import it
pub struct TestGpuBuilder {
        vendor: Option<PciVendor>,
        device_type: Option<vk::PhysicalDeviceType>,
        queues: Vec<QueueFamily>,
        // device_name: Option<&'a str>,
        device_name: Option<[i8; vk::MAX_PHYSICAL_DEVICE_NAME_SIZE]>,
    }

    #[cfg(test)]
    impl TestGpuBuilder {
        pub fn new() -> TestGpuBuilder {
            TestGpuBuilder {
                vendor: None,
                device_type: None,
                queues: Vec::default(),
                device_name: None,
            }
        }

        pub fn pick_vendor(mut self, vendor: PciVendor) -> Self {
            self.vendor = Some(vendor);
            self
        }

        pub fn pick_device_type(mut self, device_type: vk::PhysicalDeviceType) -> Self {
            self.device_type = Some(device_type);
            self
        }

        pub fn pick_device_name(mut self, device_name: &str) -> Self {
            let mut device_name_array: [i8; vk::MAX_PHYSICAL_DEVICE_NAME_SIZE] = [0; vk::MAX_PHYSICAL_DEVICE_NAME_SIZE];
            assert!(device_name_array.len() < vk::MAX_PHYSICAL_DEVICE_NAME_SIZE);
            for (i, letter) in device_name.as_bytes().iter().enumerate() {
                device_name_array[i] = *letter as i8;
            }
            self.device_name = Some(device_name_array);
            self
        }

        pub fn add_queue(mut self, operations_supported: vk::QueueFlags, slots_available: u32, presentable: bool) -> Self {
            let test_family = QueueFamily::create_test_family(self.queues.len(), operations_supported, slots_available, presentable);
            self.queues.push(test_family);
            self
        }

        pub fn create_device(self) -> Gpu {
            
            Gpu {
                api_version: 0,
                device_id: 0,
                vendor_id: self.vendor.unwrap_or(Default::default()),
                device_type: self.device_type.unwrap_or(Default::default()),
                driver_version: 0,
                device_name: self.device_name.unwrap_or_else(|| {
                    let mut default_device_name: [i8; vk::MAX_PHYSICAL_DEVICE_NAME_SIZE] = [0; vk::MAX_PHYSICAL_DEVICE_NAME_SIZE];
                    for (i, letter) in b"Default Test Device\0".into_iter().enumerate() {
                        default_device_name[i] = *letter as i8;
                    }
                    default_device_name
                }),
                queue_families: self.queues,
                device_handle: vk::PhysicalDevice::default(),
                available_extensions: Vec::default(),
                device_features: vk::PhysicalDeviceFeatures::default(),
                surface_formats: Default::default(),
                surface_capabilities: vk::SurfaceCapabilitiesKHR::default(),
                present_modes: Vec::default(),
            }
        }
    }
