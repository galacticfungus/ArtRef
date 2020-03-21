use ash::version::DeviceV1_0;
use ash::version::InstanceV1_0;
use ash::vk;

use std::ffi::CStr;
use std::collections::HashMap;

// use super::Gpu;
use super::{QueueFamily, PciVendor, Features, Feature, ExtensionManager, DeviceExtensions, QueueManager, VulkanDevice, PresentMode};
use crate::error;
use crate::{Version, PickManager};

// Notes from Nvidia: Don’t overlap compute work on the graphics queue with compute work on a
// dedicated asynchronous compute queue. This may lead to gaps in execution of the
// asynchronous compute queue

// Responsible for configuring the underlying device, creating queues, enabling features, loading device extensions and specifying surface parameters
pub struct ConfigureDevice<'a> {
    instance: &'a ash::Instance,
    device_handle: vk::PhysicalDevice,
    queue_families: Vec<QueueFamily>,
    api_version: Version,
    driver_version: u32,
    vendor_id: PciVendor,
    device_id: u32,
    device_name: [i8; ash::vk::MAX_PHYSICAL_DEVICE_NAME_SIZE],
    device_type: vk::PhysicalDeviceType,
    available_extensions: Vec<vk::ExtensionProperties>,
    extensions_to_load: HashMap<DeviceExtensions, bool>,
    device_features: vk::PhysicalDeviceFeatures,
    enabled_features: vk::PhysicalDeviceFeatures,
    surface_capabilities: vk::SurfaceCapabilitiesKHR,
    surface_formats: Vec<vk::SurfaceFormatKHR>,
    present_modes: Vec<vk::PresentModeKHR>,
    present_mode: vk::PresentModeKHR,
    surface_format: vk::SurfaceFormatKHR,
}

impl<'a> ConfigureDevice<'a> {
    pub fn new(instance: &'a ash::Instance, 
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
        present_modes: Vec<vk::PresentModeKHR>) -> ConfigureDevice<'a> {
        ConfigureDevice {
            instance,
            device_handle,
            queue_families,
            api_version: Version::from(api_version),
            driver_version,
            vendor_id,
            device_id,
            device_name,
            device_type,
            available_extensions,
            extensions_to_load: HashMap::new(),
            device_features,
            enabled_features: vk::PhysicalDeviceFeatures::default(),
            surface_capabilities,
            surface_formats,
            present_modes,
            // This mode is guarenteed to be available so we use it as the default
            present_mode: vk::PresentModeKHR::FIFO,
            surface_format: vk::SurfaceFormatKHR::default(),
        }
    }

    // Retrieve a handle to a feature to either check its availability or enable that feature
    pub fn feature(&mut self, feature: &Features) -> Feature {
        match feature {
            Features::GeometryShader => Feature::new(
                self.device_features.geometry_shader > 0,
                &mut self.enabled_features.geometry_shader,
            ),
            Features::TesselationShader => Feature::new(
                self.device_features.tessellation_shader > 0,
                &mut self.enabled_features.tessellation_shader,
            ),
        }
    }

    // Will enable a device feature or return an error
    pub fn enable_feature(&mut self, requested_feature: Features) -> Result<&mut Self, error::Error> {
        let gpu_feature = self.feature(&requested_feature);
        if gpu_feature.is_available() {
            gpu_feature.enable();
            Ok(self)
        } else {
            Err(error::Error::MissingFeature(requested_feature))
        }
    }

    // Will see if a feature can be enabled and enable it if it is supported
    pub fn try_enable_feature(&mut self, feature: Features) -> &mut Self {
        let feature = self.feature(&feature);
        feature.enable_if_able();
        self
    }

    /// The mode picked is the first one available that is added to the list, The default mode is Fifo as that mode is guarenteed to be available
    pub fn select_present_mode<F>(&mut self, select_mode :F) -> Result<&mut Self, error::Error> 
    where F: Fn(&mut PickManager<PresentMode>) -> (),
    {
        let mut modes_picked = Vec::new();
        let mut picker = PickManager::new(&mut modes_picked);
        select_mode(&mut picker);
        // The picker must return a valid option
        // iterate over the choices selecting the first one that is available
        // TODO: This can be moved into PickManager
        for mode in modes_picked {
            let actual_mode: vk::PresentModeKHR = mode.into();
            if self.present_modes.contains(&actual_mode) {
                self.present_mode = actual_mode;
                break;
            }
        }
        // The default mode is Fifo so no need to reset it
        Ok(self)
    }

    pub fn select_surface_format<F>(&mut self, select_format: F) -> &mut Self 
    where F: Fn(&mut PickManager<vk::SurfaceFormatKHR>) -> (),
    {
        let gu = self.surface_formats[0];
        let ob = vk::SurfaceFormatKHR::default();
        let ass = ob.format;
        self.surface_format.format = vk::Format::R8G8B8A8_SRGB;
        //ash::vk::Format::R8G8B8A8_SRGB
        //ash::vk::ColorSpaceKHR::SRGB_NONLINEAR
        self
    }

    pub fn select_surface_colour_space<F>(&mut self, select_colour_space: F) -> &mut Self
    where F: Fn() -> () {
        self.surface_format.color_space = vk::ColorSpaceKHR::SRGB_NONLINEAR;
        self
    }

    pub fn select_extent<F>(&mut self, extent: F) -> &mut Self
    where F: Fn(&vk::Extent2D, &vk::Extent2D, &vk::Extent2D) -> () {
        // TODO: Define a helper extent struct to make this easier
        // TODO: Something like maxHeight MaxWidth minHeight maxHeight currentHeight currentWidth - as well as helper methods
        // if extent is 0,0 then window is minimized or hidden, basically it's surface is currently unavailable
        extent(&self.surface_capabilities.min_image_extent, &self.surface_capabilities.max_image_extent, &self.surface_capabilities.current_extent);
        let g = vk::Extent2D::default();
        self
    }

    pub fn extensions_to_load<F>(
        &mut self,
        select_extensions: F,
    ) -> &mut Self
    where
        F: Fn(&mut ExtensionManager<DeviceExtensions>) -> (),
    {
        let mut mng = ExtensionManager::new();
        select_extensions(&mut mng);
        let extensions_to_load = mng.get_extensions();
        // Add the extensions selected to the list of extensions to load marking whether that extension is available
        for extension in extensions_to_load {
            if self.is_extension_available(&extension) {
                self.extensions_to_load.insert(extension, true);
            } else {
                self.extensions_to_load.insert(extension, false);
            }
        }
        self
    }

    fn is_extension_available(&self, extension: &DeviceExtensions) -> bool {
        // self.available_extensions.iter().map(|ext| unsafe { CStr::from_ptr(ext.extension_name.as_ptr()) } ).any(|ext_name| ext_name == extension.get_name())
        for available_extension in self.available_extensions.iter() {
            let available_name = unsafe { CStr::from_ptr(available_extension.extension_name.as_ptr()) };
            println!("Comparing names");
            if available_name == extension.get_name() {
                println!("Comparision valid");
                return true;
            } else {
                println!("Comparision worked but failed");
            }
        }
        false
    }

    // This function will return an error when a queue is requested that is not available
    pub fn define_queues<F>(&mut self, get_queues_to_create: F) -> Result<&mut Self, error::Error>
    where
        F: Fn(&mut QueueManager) -> (),
    {
        let mut qm = QueueManager::new(self.queue_families.as_slice());
        get_queues_to_create(&mut qm);

        let QueueManager {
            queues_to_create, ..
        } = qm;

        for queue_to_create in queues_to_create {
            // Best is subjective but we prefer using specialized queues over general ones
            let index = match self.find_best_family(queue_to_create.supported_operations(), queue_to_create.must_present()) {
                Ok(index) => index,
                // TODO: Here we need to build a data structure that knows what queues were requested and what queues were available as well as the successfully allocated queues
                Err(error) => unimplemented!(
                    "Error handling not finished when a queue can not be constructed"
                ),
            };
            let families = self.queue_families.as_mut_slice();
            let family_to_use = &mut families[index];
            // let family_to_use = &mut self.queue_data[index];
            // The family_to_use must have space remaining or no best family would have been found
            family_to_use.add_queue_to_create(queue_to_create);
        }
        Ok(self)
    }

    pub fn create_device(self) -> VulkanDevice {
        // Create Vulkan structs from self.queues_to_create
        // Each family queue becomes a struct sent to
        // let Self { instance, gpu } = self;

        let mut queue_map = HashMap::new();
        let mut queues_to_submit = Vec::new();
        println!("Creating Queues");
        for queue_family in self.queue_families.as_slice().iter() {
            println!("Processing {:?}", queue_family);
            let mut priorities = Vec::new();
            let queues_to_create = queue_family.queues_to_create();
            let queue_count = queues_to_create.len();
            println!("Processing Queues to create: {:?}", queues_to_create);
            for (queue_index, queue_to_create) in queues_to_create.iter().enumerate() {
                println!("Creating priority");
                priorities.push(queue_to_create.priority());
                // We access queues using the family index and a queue index, we need to map this to a creation index
                // This means that creation_index points to (family_index, queue_index)
                println!("Adding map index");
                queue_map.insert(
                    queue_to_create.index(),
                    (queue_family.index(), queue_index),
                );
            }
            // TODO: Ensure that family_index can fit in a u32
            // TODO: Ensure that queue_count can fit in a u32
            let queue_to_submit = vk::DeviceQueueCreateInfo {
                p_queue_priorities: priorities.as_ptr(),
                queue_family_index: queue_family.index() as u32,
                queue_count: queue_count as u32,
                ..Default::default()
            };
            queues_to_submit.push(queue_to_submit);
        }

        println!("Submitting Queues");
        // The queue map lets us map from creation index to queue index
        let device_extensions: Vec<*const std::os::raw::c_char> = self.extensions_to_load
                                .iter()
                                .filter(|(_, &present)| present == true)
                                .map(|(ext, _)| ext.get_name().as_ptr())
                                .collect();
        let create_info = vk::DeviceCreateInfo {
            enabled_extension_count: device_extensions.len() as u32,
            pp_enabled_extension_names: device_extensions.as_ptr(),
            p_enabled_features: &self.enabled_features,
            queue_create_info_count: queues_to_submit.len() as u32,
            p_queue_create_infos: queues_to_submit.as_ptr(),
            ..Default::default()
        };
        println!("Creating Device");
        // This should be safe as all data structures are in scope and there are no user parameters
        let device = unsafe { self.instance.create_device(self.device_handle, &create_info, None) }
            .expect("Failed to create device");
        // Get a handle to each of the queues in the order they were created using queue_map as the map
        let mut queues = Vec::with_capacity(queue_map.keys().len());
        for create_index in queue_map.keys() {
            let (queue_index, family_index) = queue_map[create_index];
            let queue = unsafe { device.get_device_queue(family_index as u32, queue_index as u32) };
            queues.push(queue);
        }
        
        VulkanDevice::new(self.device_handle, 
            queues, 
            self.enabled_features,
            self.extensions_to_load,
            self.surface_capabilities,
            self.surface_formats,
            self.present_modes,
            device,
            self.vendor_id,
            self.device_id,
            self.api_version,
            self.driver_version,
            self.device_name)
    }

    // We prioritize queue families that provide the least functionality when allocating queues
    fn find_best_family(&self, operations_needed: vk::QueueFlags, must_present: bool) -> Result<usize, error::Error> {
        let mut best_result = 100;
        let mut best_index = None;
        // self.gpu.get_supported_queues()
        for (index, family) in self.queue_families.as_slice().iter().enumerate() {
            // does current queue support wanted type
            if family.has_support_for(operations_needed, must_present) {
                if family.is_full() == false {
                    // what is the total number of queue types that it supports
                    let total_queue_types = family.total_queue_types();
                    // We favour the smallest one we can find
                    if total_queue_types < best_result {
                        best_result = total_queue_types;
                        best_index = Some(index);
                    }
                }
            }
        }
        if let Some(index_to_use) = best_index {
            Ok(index_to_use)
        } else {
            Err(error::Error::NoValidQueueFamily)
        }
    }
}

impl<'a> std::fmt::Debug for ConfigureDevice<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Create a VulkanVersion struct
        fmt.debug_struct("DeviceConfigure")
            .field("available_extensions", &self.available_extensions)
            .field("api_version", &self.api_version)
            // Safe since device_name must be a CStr and the string itself will always be valid for the lifetime of the pointer
            .field("device_name", unsafe { &CStr::from_ptr(self.device_name.as_ptr()) } )
            .field("device_features", &self.device_features)
            .field("device_type", &self.device_type)
            .field("driver_version", &self.driver_version)
            .field("enabled_features", &self.enabled_features)
            .finish()
        // f.write_fmt(format_args!("Available Extensions: {:?}", self.available_extensions))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PresentMode;
    pub struct TestConfigureDeviceBuilder<'a> {
        queue_families: Vec<QueueFamily>,
        api_version: Option<Version>,
        driver_version: Option<u32>,
        vendor_id: Option<PciVendor>,
        instance: &'a ash::Instance,
        device_name: Option<[i8; ash::vk::MAX_PHYSICAL_DEVICE_NAME_SIZE]>,
        device_type: Option<vk::PhysicalDeviceType>,
        available_extensions: Option<Vec<vk::ExtensionProperties>>,
        // extensions_to_load: Option<Vec<&'static CStr>>,
        // device_features: Option<vk::PhysicalDeviceFeatures>,
        // enabled_features: Option<vk::PhysicalDeviceFeatures>,
        // surface_capabilities: Option<vk::SurfaceCapabilitiesKHR>,
        // surface_formats: Option<Vec<vk::SurfaceFormatKHR>>,
        present_modes: Option<Vec<vk::PresentModeKHR>>,
    }

    impl<'a> TestConfigureDeviceBuilder<'a> {
        pub fn new(instance: &ash::Instance) -> TestConfigureDeviceBuilder {
            TestConfigureDeviceBuilder {
                instance,
                api_version: None,
                driver_version: None,
                vendor_id: None,
                device_name: None,
                device_type: None,
                queue_families: Vec::default(),
                available_extensions: None,
                present_modes: None,
            }
        }

        pub fn pick_vendor(mut self, vendor: PciVendor) -> Self {
            self.vendor_id = Some(vendor);
            self
        }

        pub fn pick_device_type(mut self, device_type: vk::PhysicalDeviceType) -> Self {
            self.device_type = Some(device_type);
            self
        }

        pub fn pick_api_version(mut self, major: u32, minor: u32) {
            self.api_version = Some(Version::from(ash::vk_make_version!(major, minor, 0)));
        }

        pub fn pick_driver_version(mut self, version: u32) {
            self.driver_version = Some(version);
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
            let test_family = QueueFamily::create_test_family(self.queue_families.len(), operations_supported, slots_available, presentable);
            self.queue_families.push(test_family);
            self
        }

        pub fn add_supported_extension(mut self, extension_to_support: DeviceExtensions) -> Self {
            let mut extension_name: [std::os::raw::c_char; vk::MAX_EXTENSION_NAME_SIZE] = [0; vk::MAX_EXTENSION_NAME_SIZE];
                
            // No need to include NUL byte since extension name is initilized with NUL bytes
            let source_bytes = extension_to_support.get_name().to_bytes();
            // This ensures that our destination array slice has the same length as the source
            // Since they have the same max length this is safe
            let mutable_slice_of_array = &mut extension_name.as_mut()[..source_bytes.len()];
            // This is done to ensure that the byte sign is the same regardless of platform
            let character_byte_slice = unsafe{ std::slice::from_raw_parts(source_bytes.as_ptr() as *const std::os::raw::c_char, source_bytes.len()) };
            // This copies the contents of extension_to_support.extension_name to the extension_name array
            mutable_slice_of_array.copy_from_slice(character_byte_slice);
            let raw_extension = vk::ExtensionProperties {
                extension_name,
                spec_version: 1,
            };
            // Add the newly crafted extension to the list of available extensions
            if let Some(available_extensions) = self.available_extensions.as_mut() {
                available_extensions.push(raw_extension);
            } else {
                let extensions = vec![raw_extension];
                self.available_extensions = Some(extensions);
            }
            self
        }

        pub fn add_present_mode(mut self, present_mode: vk::PresentModeKHR) -> Self {
            if let Some(present_modes) = &mut self.present_modes {
                present_modes.push(present_mode);
            } else {
                let mut present_modes = Vec::new();
                present_modes.push(present_mode);
                self.present_modes = Some(present_modes);
            }
            self
        }

        pub fn build(self) -> ConfigureDevice<'a> {
            ConfigureDevice {
                instance: self.instance,
                api_version: self.api_version.unwrap_or(Default::default()),
                device_id: 0,
                vendor_id: self.vendor_id.unwrap_or(Default::default()),
                device_type: self.device_type.unwrap_or(Default::default()),
                driver_version: self.driver_version.unwrap_or(Default::default()),
                device_name: self.device_name.unwrap_or_else(|| {
                    let mut default_device_name: [std::os::raw::c_char; vk::MAX_PHYSICAL_DEVICE_NAME_SIZE] = [0; vk::MAX_PHYSICAL_DEVICE_NAME_SIZE];
                    for (i, letter) in b"Default Test Device\0".into_iter().enumerate() {
                        default_device_name[i] = *letter as std::os::raw::c_char;
                    }
                    default_device_name
                }),
                queue_families: self.queue_families,
                device_handle: vk::PhysicalDevice::default(),
                available_extensions: self.available_extensions.unwrap_or(Default::default()),
                extensions_to_load: HashMap::new(),
                device_features: vk::PhysicalDeviceFeatures::default(),
                enabled_features: Default::default(),
                surface_formats: Default::default(),
                surface_capabilities: vk::SurfaceCapabilitiesKHR::default(),
                present_modes: self.present_modes.unwrap_or(Default::default()),
                // TODO: Present mode and surface format should default to None
                present_mode: vk::PresentModeKHR::default(),
                surface_format: vk::SurfaceFormatKHR::default(),
            }
        }
    }
    use ash::version::EntryV1_0;

    impl<'a> ConfigureDevice<'a> {
        pub fn create_test_instance() -> ash::Instance {
            let entry = ash::Entry::new().expect("Failed to load Vulkan");
            let create_info = vk::InstanceCreateInfo::default();
            unsafe { entry.create_instance(&create_info, None) }
                .expect("Failed to create instance")
        }
    }

    #[test]
    fn test_find_best_family() {
        let instance = ConfigureDevice::create_test_instance();
        let config = TestConfigureDeviceBuilder::new(&instance)
            .add_queue(vk::QueueFlags::GRAPHICS | vk::QueueFlags::COMPUTE | vk::QueueFlags::TRANSFER, 6, false)
            .add_queue(vk::QueueFlags::GRAPHICS | vk::QueueFlags::COMPUTE, 6, true)
            .add_queue(vk::QueueFlags::TRANSFER | vk::QueueFlags::SPARSE_BINDING, 4, false)
            .build();
        let res = config.find_best_family(vk::QueueFlags::GRAPHICS, false);
        assert_eq!(res.unwrap(), 1);
    }

    #[test]
    fn find_best_family_with_present_test() {
        let instance = ConfigureDevice::create_test_instance();
        let config = TestConfigureDeviceBuilder::new(&instance)
            .add_queue(vk::QueueFlags::GRAPHICS | vk::QueueFlags::COMPUTE | vk::QueueFlags::TRANSFER, 6, true)
            .add_queue(vk::QueueFlags::GRAPHICS | vk::QueueFlags::COMPUTE, 6, false)
            .add_queue(vk::QueueFlags::TRANSFER | vk::QueueFlags::SPARSE_BINDING, 4, false)
            .build();
        // Since the "best" queue doesn't support presentation we fallback to the best queue that does
        let res = config.find_best_family(vk::QueueFlags::GRAPHICS, true);
        assert_eq!(res.unwrap(), 0);
    }

    #[test]
    fn test_queue_creation() {
        let instance = ConfigureDevice::create_test_instance();
        let mut config = TestConfigureDeviceBuilder::new(&instance)
            .add_queue(vk::QueueFlags::GRAPHICS | vk::QueueFlags::COMPUTE, 6, true)
            .add_queue(vk::QueueFlags::TRANSFER | vk::QueueFlags::SPARSE_BINDING, 4, false)
            .build();
        config.define_queues(|mng| {
                // User needs to be able to check what is available
                mng.create_queue_that_supports(vk::QueueFlags::GRAPHICS, 1.0, true);
                mng.create_graphics_queue(1.0, true);
                mng.create_graphics_queue(0.75, false);
                mng.create_transfer_queue(1.0);
                mng.create_compute_queue(1.0); // Family Queue 1 should have 3 queues, Family Queue 2 should have 1
            })
            .expect("Failed to create the queues");
        // TODO: Test something
        let queue_families = config.queue_families;
        println!("{:?}", queue_families);
        assert_eq!(queue_families[0].queues_to_create().len(), 4);
        assert_eq!(queue_families[1].queues_to_create().len(), 1);
    }

    #[test]
    fn test_queue_creation_conditional() {
        //test_device(vendor: PciVendor, device_type: vk::PhysicalDeviceType, queue_families: Vec<QueueFamily>) -> Self {
        //create_test_family(index: usize, queue_types: vk::QueueFlags, queue_count: u32) -> QueueFamily {
        let instance = ConfigureDevice::create_test_instance();
        let mut configure = TestConfigureDeviceBuilder::new(&instance)
            .add_queue(vk::QueueFlags::GRAPHICS | vk::QueueFlags::COMPUTE, 1, true)
            .add_queue(vk::QueueFlags::TRANSFER | vk::QueueFlags::SPARSE_BINDING, 1, false)
            .build();
        configure.define_queues(|mng| {
                // User needs to be able to check what queues are available
                if mng.queues_that_support(vk::QueueFlags::GRAPHICS) > 1 {
                    panic!("More than one graphics queue");
                }
                match mng.total_queues() {
                    1 => mng.create_compute_queue(1.0),
                    2 => {
                        mng.create_compute_queue(1.0);
                        mng.create_transfer_queue(1.0);
                    },
                    n if n > 2 => mng.create_compute_queue(1.0),
                    _ => panic!(""),
                }
            })
            .expect("Failed to create the queues");
        // TODO: Test something
        let queue_families = configure.queue_families;
        assert_eq!(queue_families[0].queues_to_create().len(), 1);
        assert_eq!(queue_families[1].queues_to_create().len(), 1);
    }

    #[test]
    fn test_is_extension_available() {
        let instance = ConfigureDevice::create_test_instance();
        let configure = TestConfigureDeviceBuilder::new(&instance)
            .add_supported_extension(DeviceExtensions::Swapchain)
            .add_supported_extension(DeviceExtensions::Swapchain)
            .build();
        // TODO: Fix test
        println!("{:?}", configure);
        assert!(configure.is_extension_available(&DeviceExtensions::Swapchain) == false);
        assert!(configure.is_extension_available(&DeviceExtensions::Swapchain));
    }

    #[test]
    fn test_define_extension_available() {
        let instance = ConfigureDevice::create_test_instance();
        let mut configure = TestConfigureDeviceBuilder::new(&instance)
            .add_supported_extension(DeviceExtensions::Swapchain)
            .add_supported_extension(DeviceExtensions::Swapchain)
            .build();
        // TODO: Some of these extensions are not device extensions but instance extensions
        println!("{:?}", configure);
        configure.extensions_to_load(|mng| {
            mng.add_extension(DeviceExtensions::Swapchain);
            mng.add_extension(DeviceExtensions::Swapchain);
        });
        assert!(configure.extensions_to_load[&DeviceExtensions::Swapchain]);
        assert!(configure.extensions_to_load[&DeviceExtensions::Swapchain] == false);
    }

    #[test]
    fn test_adding_present_modes() {
        let instance = ConfigureDevice::create_test_instance();
        let mut configure = TestConfigureDeviceBuilder::new(&instance)
            .add_present_mode(vk::PresentModeKHR::FIFO)
            .add_present_mode(vk::PresentModeKHR::MAILBOX)
            .build();
        // TODO: Some of these extensions are not device extensions but instance extensions
        println!("{:?}", configure);
        let result = configure.select_present_mode(|mng| {
            mng.pick_mode(PresentMode::Immediate);
            mng.pick_mode(PresentMode::Mailbox);
            mng.pick_mode(PresentMode::Fifo);
        }).expect("Failed to select a present mode");
        // First mode picked is the first one that is available
        assert_eq!(result.present_mode, vk::PresentModeKHR::MAILBOX);
    }
}
