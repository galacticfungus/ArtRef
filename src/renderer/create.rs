use ash::version::DeviceV1_0;
use ash::version::InstanceV1_0;
use ash::vk;

use std::collections::HashMap;

use super::features::Features;
use super::Gpu;
use super::{QueueFamily, QueueToCreate};
use super::VulkanDevice;
use crate::error;

// Notes from Nvidia: Don’t overlap compute work on the graphics queue with compute work on a
// dedicated asynchronous compute queue. This may lead to gaps in execution of the
// asynchronous compute queue

// Responsible for creating the device, helps with queue creation as well as enabling features
pub struct ConfigureDevice<'a> {
    instance: &'a ash::Instance,
    gpu: Gpu,
    // queue_data: Vec<QueueFamilyData>,
}

impl<'a> ConfigureDevice<'a> {
    pub fn new(instance: &'a ash::Instance, gpu: Gpu) -> ConfigureDevice<'a> {
        // let gpu_family_data = gpu.get_queue_families();
        // let mut info = Vec::with_capacity(gpu_family_data.len());
        // for (index, queue_family) in gpu_family_data.iter().enumerate() {
        //     let queue_count = queue_family.get_total_queue_types();
        //     let data = QueueFamilyData::new(queue_family.get_flags(), queue_family.get_total_queues(), index, queue_count);
        //     info.push(data);
        // }
        ConfigureDevice {
            instance,
            gpu,
            // queue_data: info,
        }
    }

    // queues -- add_queue | based on families

    // Will enable a device feature or return an error
    pub fn enable_feature(&mut self, requested_feature: Features) -> Result<&mut Self, error::Error> {
        let gpu_feature = self.gpu.feature(&requested_feature);
        if gpu_feature.is_available() {
            gpu_feature.enable();
            Ok(self)
        } else {
            Err(error::Error::MissingFeature(requested_feature))
        }
    }

    // Will see if a feature can be enabled and enable it if it is supported
    pub fn try_enable_feature(&mut self, feature: Features) -> &mut Self {
        let feature = self.gpu.feature(&feature);
        feature.enable_if_able();
        self
    }

    // This function will return an error when a queue is requested that is not available
    pub fn define_queues<F>(&mut self, get_queues_to_create: F) -> Result<&mut Self, error::Error>
    where
        F: Fn(&mut QueueManager) -> (),
    {
        let mut qm = QueueManager::new(self.gpu.get_queue_families());
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
            let families = self.gpu.get_mut_queue_families();
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
        let Self { instance, gpu } = self;

        let mut queue_map = HashMap::new();
        let mut queues_to_submit = Vec::new();
        println!("Creating Queues");
        for queue_family in gpu.get_queue_families().iter() {
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
        let device_extensions = gpu.get_extensions();
        let create_info = vk::DeviceCreateInfo {
            enabled_extension_count: device_extensions.len() as u32,
            pp_enabled_extension_names: device_extensions.as_ptr(),
            p_enabled_features: gpu.get_features(),
            queue_create_info_count: queues_to_submit.len() as u32,
            p_queue_create_infos: queues_to_submit.as_ptr(),
            ..Default::default()
        };
        println!("Creating Device");
        // This should be safe as all data structures are in scope and there are no user parameters
        let device = unsafe { instance.create_device(gpu.get_handle(), &create_info, None) }
            .expect("Failed to create device");
        // Get a handle to each of the queues in the order they were created using queue_map as the map
        let mut queues = Vec::with_capacity(queue_map.keys().len());
        for create_index in queue_map.keys() {
            let (queue_index, family_index) = queue_map[create_index];
            let queue = unsafe { device.get_device_queue(family_index as u32, queue_index as u32) };
            queues.push(queue);
        }
        println!("Returning RenderDevice");
        // TODO: FromConfigureDevice for VulkanDevice
        panic!("")
        // VulkanDevice {
        //     // gpu,
        //     // queues,
        //     // device,
        // }
    }

    // We prioritize queue families that provide the least functionality when allocating queues
    fn find_best_family(&self, operations_needed: vk::QueueFlags, must_present: bool) -> Result<usize, error::Error> {
        let mut best_result = 100;
        let mut best_index = None;
        // self.gpu.get_supported_queues()
        for (index, family) in self.gpu.get_queue_families().iter().enumerate() {
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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::renderer::gpu::TestGpuBuilder;

    use ash::version::EntryV1_0;
    use ash::vk_make_version;

    impl<'a> ConfigureDevice<'a> {
        pub fn use_test_device(
            entry: ash::Entry,
            instance: &'a ash::Instance,
            test_gpu: Gpu,
        ) -> ConfigureDevice {
            ConfigureDevice {
                instance,
                gpu: test_gpu,
            }
        }

        pub fn create_test_instance() -> ash::Instance {
            let entry = ash::Entry::new().expect("Failed to load Vulkan");
            let create_info = vk::InstanceCreateInfo::default();
            unsafe { entry.create_instance(&create_info, None) }
                .expect("Failed to create instance")
        }

        pub fn create_test_configure(
            instance: &'a ash::Instance,
            test_gpu: Gpu,
        ) -> ConfigureDevice<'a> {
            ConfigureDevice {
                instance,
                gpu: test_gpu,
            }
        }
    }

    #[test]
    fn test_find_best_family() {
        let gpu = TestGpuBuilder::new()
            .add_queue(vk::QueueFlags::GRAPHICS | vk::QueueFlags::COMPUTE | vk::QueueFlags::TRANSFER, 6, false)
            .add_queue(vk::QueueFlags::GRAPHICS | vk::QueueFlags::COMPUTE, 6, true)
            .add_queue(vk::QueueFlags::TRANSFER | vk::QueueFlags::SPARSE_BINDING, 4, false)
            .create_device();        
        let instance = ConfigureDevice::create_test_instance();
        let config = ConfigureDevice::create_test_configure(&instance, gpu);
        let res = config.find_best_family(vk::QueueFlags::GRAPHICS, false);
        assert_eq!(res.unwrap(), 1);
    }

    #[test]
    fn find_best_family_with_present_test() {
        let gpu = TestGpuBuilder::new()
            .add_queue(vk::QueueFlags::GRAPHICS | vk::QueueFlags::COMPUTE | vk::QueueFlags::TRANSFER, 6, true)
            .add_queue(vk::QueueFlags::GRAPHICS | vk::QueueFlags::COMPUTE, 6, false)
            .add_queue(vk::QueueFlags::TRANSFER | vk::QueueFlags::SPARSE_BINDING, 4, false)
            .create_device();
        // Since the "best" queue doesn't support 
        let instance = ConfigureDevice::create_test_instance();
        let config = ConfigureDevice::create_test_configure(&instance, gpu);
        let res = config.find_best_family(vk::QueueFlags::GRAPHICS, true);
        assert_eq!(res.unwrap(), 0);
    }

    #[test]
    fn test_queue_creation() {
        //test_device(vendor: PciVendor, device_type: vk::PhysicalDeviceType, queue_families: Vec<QueueFamily>) -> Self {
        //create_test_family(index: usize, queue_types: vk::QueueFlags, queue_count: u32) -> QueueFamily {
        let gpu = TestGpuBuilder::new()
            .add_queue(vk::QueueFlags::GRAPHICS | vk::QueueFlags::COMPUTE, 6, true)
            .add_queue(vk::QueueFlags::TRANSFER | vk::QueueFlags::SPARSE_BINDING, 4, false)
            .create_device();
        let instance = ConfigureDevice::create_test_instance();
        let mut configure = ConfigureDevice::create_test_configure(&instance, gpu);
        configure.define_queues(|mng| {
                // User needs to be able to check what is available
                mng.create_queue_that_supports(vk::QueueFlags::GRAPHICS, 1.0, true);
                mng.create_graphics_queue(1.0, true);
                mng.create_graphics_queue(0.75, false);
                mng.create_transfer_queue(1.0);
                mng.create_compute_queue(1.0); // Family Queue 1 should have 3 queues, Family Queue 2 should have 1
            })
            .expect("Failed to create the queues");
        // TODO: Test something
        let queue_families = configure.gpu.get_queue_families();
        println!("{:?}", queue_families);
        assert_eq!(queue_families[0].queues_to_create().len(), 3);
        assert_eq!(queue_families[1].queues_to_create().len(), 1);
    }

    #[test]
    fn test_queue_creation_conditional() {
        //test_device(vendor: PciVendor, device_type: vk::PhysicalDeviceType, queue_families: Vec<QueueFamily>) -> Self {
        //create_test_family(index: usize, queue_types: vk::QueueFlags, queue_count: u32) -> QueueFamily {
        let gpu = TestGpuBuilder::new()
            .add_queue(vk::QueueFlags::GRAPHICS | vk::QueueFlags::COMPUTE, 1, true)
            .add_queue(vk::QueueFlags::TRANSFER | vk::QueueFlags::SPARSE_BINDING, 1, false)
            .create_device();
        let instance = ConfigureDevice::create_test_instance();
        let mut configure = ConfigureDevice::create_test_configure(&instance, gpu);
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
        let queue_families = configure.gpu.get_queue_families();
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
