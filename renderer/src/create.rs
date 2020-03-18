use ash::version::DeviceV1_0;
use ash::version::InstanceV1_0;
use ash::vk;

use std::ffi::CStr;
use std::collections::HashMap;

// use super::Gpu;
use super::{QueueFamily, QueueToCreate, PciVendor, Features, features::Feature, ExtensionManager, Extensions};
use super::VulkanDevice;
use crate::error;

// Notes from Nvidia: Don’t overlap compute work on the graphics queue with compute work on a
// dedicated asynchronous compute queue. This may lead to gaps in execution of the
// asynchronous compute queue

// Responsible for creating the device, helps with queue creation as well as enabling features
pub struct ConfigureDevice<'a> {
    instance: &'a ash::Instance,
    device_handle: vk::PhysicalDevice,
    queue_families: Vec<QueueFamily>,
    api_version: u32,
    driver_version: u32,
    vendor_id: PciVendor,
    device_id: u32,
    device_name: [i8; ash::vk::MAX_PHYSICAL_DEVICE_NAME_SIZE],
    device_type: vk::PhysicalDeviceType,
    available_extensions: Vec<vk::ExtensionProperties>,
    extensions_to_load: HashMap<Extensions, bool>,
    device_features: vk::PhysicalDeviceFeatures,
    enabled_features: vk::PhysicalDeviceFeatures,
    surface_capabilities: vk::SurfaceCapabilitiesKHR,
    surface_formats: Vec<vk::SurfaceFormatKHR>,
    present_modes: Vec<vk::PresentModeKHR>,
    present_mode: vk::PresentModeKHR,
    // queue_data: Vec<QueueFamilyData>,
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
            api_version,
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
            // This mode is guarenteed to be available
            present_mode: vk::PresentModeKHR::FIFO,
        }
    }

    // queues -- add_queue | based on families

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
    where F: Fn(&mut PresentModeManager) -> (),
    {
        let mut modes_picked = Vec::new();
        let mut picker = PresentModeManager::new(&mut modes_picked);
        select_mode(&mut picker);
        // The picker must return a valid option
        // iterate over the choices selecting the first one that is available
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

    pub fn extensions_to_load<F>(
        &mut self,
        select_extensions: F,
    ) -> Result<&mut Self, error::Error>
    where
        F: Fn(&mut ExtensionManager) -> (),
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
        Ok(self)
    }

    fn is_extension_available(&self, extension: &Extensions) -> bool {
        self.available_extensions.iter().map(|ext| unsafe { CStr::from_ptr(ext.extension_name.as_ptr()) } ).any(|ext_name| ext_name == extension.get_name())
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

// TODO: Need to destructure the GPU when it arrives in Configure Device
// impl Into<VulkanDevice> for ConfigureDevice {
//     fn into(self) -> VulkanDevice {
//         VulkanDevice {
//             api_version: self.api_version,
//             driver_version: self.driver_version,
//             vendor_id: self.vendor_id,
//             device_id: self.device_id,
//             device_name: self.device_name,
//             physical_device: self.device_handle,
//             queues: self.queue_families,
//             device: self.,
//             enabled_features: self.enabled_features,
//             surface_capabilities: self.surface_capabilities,
//             surface_formats: self.surface_formats,
//             present_modes: self.present_modes,
//         }
//     }
// }

// TODO: This can be generalized to be a pick in order

pub struct PresentModeManager<'a> {
    modes_picked: &'a mut Vec<PresentMode>,
}

impl<'a> PresentModeManager<'a> {
    pub fn new(modes_picked: &'a mut Vec<PresentMode>) -> PresentModeManager {
        PresentModeManager {
            modes_picked,
        }
    }

    pub fn pick_mode(&mut self, mode: PresentMode) -> &mut Self {
        self.modes_picked.push(mode);
        self
    }
}

#[derive(Debug)]
// This class does not create any actual queues it merely gathers all the queues that the user wants to
// create in order to hopefully optimize queue creation, in addition it performs no validation of the results
// Meaning that if a queue could no
pub struct QueueManager<'a> {
    queues_to_create: Vec<QueueToCreate>,
    family_data: &'a [QueueFamily],
    index: usize, // Index of the next queue that is create
}

impl<'a> QueueManager<'a> {
    pub fn new(family_data: &'a [QueueFamily]) -> QueueManager {
        QueueManager {
            queues_to_create: Vec::new(),
            family_data,
            index: 0,
        }
    }
    /// Returns the number of queues that support the required flags
    pub fn queues_that_support(&self, operations_required: vk::QueueFlags) -> usize {
        self.family_data
            .iter()
            .filter(|family| family.flags() & operations_required == operations_required)
            .map(|family| family.total_queues() as usize)
            .sum()
    }
    /// Returns the number of queues that can present to a surface
    pub fn queues_that_present(&self) -> usize {
        // TODO: Should we display a warning when we encounter queues that have a presentable of None since that means they have not been checked
        self.family_data
            .iter()
            .filter(|family| family.presentable() == true)
            .map(|family| family.total_queues() as usize)
            .sum()
    }
    /// Returns the total number of queues across all queue families
    pub fn total_queues(&self) -> usize {
        self.family_data
            .iter()
            .map(|family| family.total_queues() as usize)
            .sum()
    }
    
    /// Creates a queue that supports the given operations from the best fitting queue family
    /// The family is decided by  picking the family that supports the leat amount of operations
    /// requested. In addition if the must preent flag is true then the queue created will be
    /// able to present to the surface.
    // TODO: What surface, at the moment its the surface that was passed as a parameter to the device selector
    pub fn create_queue_that_supports(
        &mut self,
        required_operations: vk::QueueFlags,
        priority: f32,
        must_present: bool,
    ) {
        self.queues_to_create.push(QueueToCreate::new(
            required_operations,
            priority,
            self.index,
            must_present,
        ));
        self.index += 1;
    }

    pub fn create_graphics_queue(&mut self, priority: f32, must_present: bool) {
        self.create_queue_that_supports(vk::QueueFlags::GRAPHICS, priority, must_present);
    }

    pub fn create_transfer_queue(&mut self, priority: f32) {
        self.create_queue_that_supports(vk::QueueFlags::TRANSFER, priority, false);
    }

    pub fn create_compute_queue(&mut self, priority: f32) {
        self.create_queue_that_supports(vk::QueueFlags::COMPUTE, priority, false);
    }

    pub fn create_sparse_queue(&mut self, priority: f32) {
        self.create_queue_that_supports(vk::QueueFlags::SPARSE_BINDING, priority, false);
    }
}

pub enum PresentMode {
    /// Specifies that the presentation engine does not wait for a vertical blanking period to update the current image, meaning 
    /// this mode may result in visible tearing. No internal queuing of presentation requests is needed, as the requests are applied immediately.
    Immediate,
    /// Mailbox specifies that the presentation engine waits for the next vertical blanking period to update the current image.
    /// Tearing cannot be observed. An internal single-entry queue is used to hold pending presentation requests. If the queue is full when a 
    /// new presentation request is received, the new request replaces the existing entry, and any images associated with the prior entry become 
    /// available for re-use by the application. One request is removed from the queue and processed during each vertical blanking period in which 
    /// the queue is non-empty.
    Mailbox,
    /// VK_PRESENT_MODE_FIFO_KHR specifies that the presentation engine waits for the next vertical blanking period to update the current image.
    /// Tearing cannot be observed. An internal queue is used to hold pending presentation requests. New requests are appended to the end of the queue,
    /// and one request is removed from the beginning of the queue and processed during each vertical blanking period in which the queue is non-empty.
    /// This is the only value of presentMode that is required to be supported.
    Fifo,
    /// VK_PRESENT_MODE_FIFO_RELAXED_KHR specifies that the presentation engine generally waits for the next vertical blanking period to update the
    /// current image. If a vertical blanking period has already passed since the last update of the current image then the presentation engine does
    /// not wait for another vertical blanking period for the update, meaning this mode may result in visible tearing in this case. This mode is useful
    /// for reducing visual stutter with an application that will mostly present a new image before the next vertical blanking period, but may occasionally
    /// be late, and present a new image just after the next vertical blanking period. An internal queue is used to hold pending presentation requests.
    /// New requests are appended to the end of the queue, and one request is removed from the beginning of the queue and processed during or after each
    /// vertical blanking period in which the queue is non-empty.
    FifoRelaxed,
    /// SharedDemandRefresh specifies that the presentation engine and application have concurrent access to a single image, which is
    /// referred to as a shared presentable image. The presentation engine is only required to update the current image after a new presentation request is
    /// received. Therefore the application must make a presentation request whenever an update is required. However, the presentation engine may update the
    /// current image at any point, meaning this mode may result in visible tearing.
    SharedDemandRefresh,
    /// SharedContinuousRefresh specifies that the presentation engine and application have concurrent access to a single image, which is referred to as a
    /// shared presentable image. The presentation engine periodically updates the current image on its regular refresh cycle. The application is only required
    /// to make one initial presentation request, after which the presentation engine must update the current image without any need for further presentation
    /// requests. The application can indicate the image contents have been updated by making a presentation request, but this does not guarantee the timing of
    /// when it will be updated. This mode may result in visible tearing if rendering to the image is not timed correctly.
    SharedContinuousRefresh,
}

impl std::convert::From<ash::vk::PresentModeKHR> for PresentMode {
    fn from(present_mode: vk::PresentModeKHR) -> Self {
        match present_mode {
            vk::PresentModeKHR::IMMEDIATE => PresentMode::Immediate,
            vk::PresentModeKHR::MAILBOX => PresentMode::Mailbox,
            vk::PresentModeKHR::FIFO => PresentMode::Fifo,
            vk::PresentModeKHR::FIFO_RELAXED => PresentMode::FifoRelaxed,
            vk::PresentModeKHR::SHARED_DEMAND_REFRESH => PresentMode::SharedDemandRefresh,
            vk::PresentModeKHR::SHARED_CONTINUOUS_REFRESH => PresentMode::SharedContinuousRefresh,
            _ => unreachable!("Unknown present mode found when converting a PresentModeKHR"),
        }
    }
}

impl std::convert::From<PresentMode> for ash::vk::PresentModeKHR {
    fn from(present_mode: PresentMode) -> Self {
        match present_mode {
            PresentMode::Immediate => vk::PresentModeKHR::IMMEDIATE,
            PresentMode::Mailbox => vk::PresentModeKHR::MAILBOX,
            PresentMode::Fifo => vk::PresentModeKHR::FIFO,
            PresentMode::FifoRelaxed => vk::PresentModeKHR::FIFO_RELAXED,
            PresentMode::SharedDemandRefresh => vk::PresentModeKHR::SHARED_DEMAND_REFRESH,
            PresentMode::SharedContinuousRefresh => vk::PresentModeKHR::SHARED_CONTINUOUS_REFRESH,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct TestConfigureDeviceBuilder<'a> {
        queue_families: Vec<QueueFamily>,
        api_version: Option<u32>,
        driver_version: Option<u32>,
        vendor_id: Option<PciVendor>,
        instance: &'a ash::Instance,
        device_name: Option<[i8; ash::vk::MAX_PHYSICAL_DEVICE_NAME_SIZE]>,
        device_type: Option<vk::PhysicalDeviceType>,
        // available_extensions: Option<Vec<vk::ExtensionProperties>>,
        // extensions_to_load: Option<Vec<&'static CStr>>,
        // device_features: Option<vk::PhysicalDeviceFeatures>,
        // enabled_features: Option<vk::PhysicalDeviceFeatures>,
        // surface_capabilities: Option<vk::SurfaceCapabilitiesKHR>,
        // surface_formats: Option<Vec<vk::SurfaceFormatKHR>>,
        // present_modes: Option<Vec<vk::PresentModeKHR>>,
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
            self.api_version = Some(ash::vk_make_version!(major, minor, 0));
        }

        pub fn pick_driver_version(mut self, major: u32, minor: u32, build: u32) {
            self.driver_version = Some(ash::vk_make_version!(major, minor, 0));
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

        pub fn build(self) -> ConfigureDevice<'a> {
            ConfigureDevice {
                instance: self.instance,
                api_version: self.api_version.unwrap_or(Default::default()),
                device_id: 0,
                vendor_id: self.vendor_id.unwrap_or(Default::default()),
                device_type: self.device_type.unwrap_or(Default::default()),
                driver_version: self.driver_version.unwrap_or(Default::default()),
                device_name: self.device_name.unwrap_or_else(|| {
                    let mut default_device_name: [i8; vk::MAX_PHYSICAL_DEVICE_NAME_SIZE] = [0; vk::MAX_PHYSICAL_DEVICE_NAME_SIZE];
                    for (i, letter) in b"Default Test Device\0".into_iter().enumerate() {
                        default_device_name[i] = *letter as i8;
                    }
                    default_device_name
                }),
                queue_families: self.queue_families,
                device_handle: vk::PhysicalDevice::default(),
                available_extensions: Vec::default(),
                extensions_to_load: HashMap::new(),
                device_features: vk::PhysicalDeviceFeatures::default(),
                enabled_features: Default::default(),
                surface_formats: Default::default(),
                surface_capabilities: vk::SurfaceCapabilitiesKHR::default(),
                present_modes: Vec::default(),
                present_mode: vk::PresentModeKHR::default(),
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
    fn test_features() {
        // VulkanConfig::new()
        //     .api_version(1, 0, 0)
        //     .application_name("Bob")
        // .start_device_selection()
        // .is_discrete()
        // .supports_tesselation_shader()
        // .select_device()
        // .enable_feature(Features::TesselationShader).expect("No Tesselation Support")
        // .define_queues(|qm| {
        //     // When we create a queue we favour a family with that specific queue type
        //     // First we collect a list of Queues to create then we create the queues given the available queue families
        //     // transfer
        //     // graphics
        //     // compute
        //     // weird one - sparse
        //     qm.create_graphics_queue(1.0);
        //     qm.create_transfer_queue(1.0);
        //     qm.create_compute_queue(1.0);
        //     // So this code will take the request and attempt to create a queue in specialised families
        // }).expect("Failed to create the queues")
        // .create_device();
        // TODO: Fix test
    }
}
