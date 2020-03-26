use super::{QueueFamily, QueueToCreate, QueueManager};
use ash::vk;

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