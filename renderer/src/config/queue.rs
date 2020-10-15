use std::collections::HashMap;
use super::{QueueFamily, DeviceQueue, QueueManager, RendererQueues, OperationQueue, RendererQueuesBuilder};
use erupt::vk1_0 as vk;

impl<'a> QueueManager<'a> {
    pub fn new(family_data: &'a [QueueFamily]) -> QueueManager {
        QueueManager {
            // Hashmap tracks the queues by family index and is used to create the actual queues
            render_queues: RendererQueuesBuilder::new(),
            // The queues abstraction that is used to access the various queues
            // TODO: RenderQueues structure that allows us to get access to the appropriate queue from the renderer itself
            queues_to_create: HashMap::new(),
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
    /// requested. In addition if the must present flag is true then the queue created must be
    /// able to present to the surface.
    // TODO: What surface, at the moment its the surface that was passed as a parameter to the device selector
    pub fn create_queue_that_supports(
        &mut self,
        required_operations: vk::QueueFlags,
        priority: f32,
        must_present: bool,
    ) -> Option<(u32, u32)> {
        // Get best queue family for queue
        let best = self.find_best_family(required_operations, must_present);
        // TODO: We can't add RendererQueue objects here since we don't know what type of queue is being created
        if let Some(best_family) = best {
            // Does the hashmap already contain an entry for this family
            use std::convert::TryFrom;
            let best_family_index = match u32::try_from(best_family) {
                Ok(best_family) => best_family,
                Err(_) => panic!("Queue Family index exceeded u32"),
            };
            match self.queues_to_create.get_mut(&best_family) {
                Some(device_queue) => {
                    let updated_queue = device_queue.request_queue(priority);
                    // TODO: if updated_queue is not None then the queue can probably be created
                    match updated_queue {
                        Some(updated) => return Some((updated, best_family_index)),
                        None => return None, // Queue can't be created as it is full
                    }
                },
                None => {
                    let queue_family = &self.family_data[best_family];
                    let mut new_queue = DeviceQueue::new(best_family, queue_family.total_queues());
                    // Returns None if the queue family is full
                    let queue_index = new_queue.request_queue(priority);
                    match queue_index {
                        Some(queue_index) => {
                            self.queues_to_create.insert(best_family, new_queue);
                            return Some((queue_index, best_family_index));
                        },
                        None => return None,
                    }
                },
            }
        }
        // Could not find a suitable family
        None
    }

    // We prioritize queue families that provide the least functionality when allocating queues
    fn find_best_family(&self, operations_needed: vk::QueueFlags, must_present: bool) -> Option<usize> {
        let mut best_result = 100;
        let mut best_index = None;
        // self.gpu.get_supported_queues()
        for (index, family) in self.family_data.iter().enumerate() {
            // does current queue support wanted type
            if family.has_support_for(operations_needed, must_present) {
                // what is the total number of queue types that it supports
                let total_queue_types = family.total_queue_types();
                // We favour the smallest one we can find
                if total_queue_types < best_result {
                    best_result = total_queue_types;
                    best_index = Some(index);
                }
            }
        }
        best_index
    }

    pub fn create_graphics_queue(&mut self, priority: f32, must_present: bool) {
        match self.create_queue_that_supports(vk::QueueFlags::GRAPHICS, priority, must_present) {
            Some((index_to_use, family_index)) => {
                self.render_queues.create_graphics_queue(family_index, priority, index_to_use, must_present);
            },
            None => {},
        }
        // TODO: Add the family and queue to the RendererQueue object
        
        
    }

    pub fn create_transfer_queue(&mut self, priority: f32) {
        match self.create_queue_that_supports(vk::QueueFlags::TRANSFER, priority, false) {
            Some((index_to_use, family_index)) => {
                self.render_queues.create_transfer_queue(family_index, priority, index_to_use);
            },
            None => {},
        }
    }

    pub fn create_compute_queue(&mut self, priority: f32) {
        match self.create_queue_that_supports(vk::QueueFlags::COMPUTE, priority, false) {
            Some((index_to_use, family_index)) => {
                self.render_queues.create_compute_queue(family_index, priority, index_to_use);
            },
            None => {},
        }
    }

    // pub fn create_multiple_compute_queues(&mut self, priorities: &[f32]) {
    //     self.create_multiple_queues_that_support(vk::QueueFlags::COMPUTE, priorities, false)
    // }

    pub fn create_sparse_queue(&mut self, priority: f32) {
        match self.create_queue_that_supports(vk::QueueFlags::SPARSE_BINDING, priority, false) {
            Some((index_to_use, family_index)) => {
                self.render_queues.create_sparse_queue(family_index, priority, index_to_use);
            },
            None => {},
        }
    }
}