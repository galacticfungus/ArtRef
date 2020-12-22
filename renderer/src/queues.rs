use erupt::vk1_0 as vk;

#[derive(Debug, Clone)]
pub struct QueueFamily {
    family_index: usize,
    flags: vk::QueueFlags,
    total_queues: u32,
    time_stamp: u32,
    image_granularity: vk::Extent3D,
    total_queue_types: u32,
    can_present: bool,
}

impl QueueFamily {
    pub fn new(
        family_index: usize,
        flags: vk::QueueFlags,
        total_queues: u32,
        time_stamp: u32,
        image_granularity: vk::Extent3D,
        can_present: bool,
    ) -> QueueFamily {
        let mut supported_operations = 0;
        let mut data = flags.bits();
        while data > 0 {
            if data & 1 == 1 {
                supported_operations += 1;
            }
            data >>= 1;
        }

        QueueFamily {
            family_index,
            flags,
            total_queues,
            time_stamp,
            image_granularity,
            total_queue_types: supported_operations,
            can_present,
        }
    }

    // Returns true if this queue supports graphics
    pub fn supports_graphics(&self) -> bool {
        if self.flags & vk::QueueFlags::GRAPHICS == vk::QueueFlags::GRAPHICS {
            return true;
        }
        false
    }

    pub fn has_support_for(&self, operation_type: vk::QueueFlags, must_present: bool) -> bool {
        if operation_type & self.flags == operation_type {
            if must_present {
                return self.presentable();
            }
            return true;
        }
        false
    }

    pub fn supports_compute(&self) -> bool {
        if self.flags & vk::QueueFlags::COMPUTE == vk::QueueFlags::COMPUTE {
            return true;
        }
        false
    }

    pub fn presentable(&self) -> bool {
        self.can_present
    }

    pub fn flags(&self) -> vk::QueueFlags {
        self.flags
    }

    pub fn total_queues(&self) -> u32 {
        self.total_queues
    }

    pub fn total_queue_types(&self) -> usize {
        let mut current_count = 0;
        let mut data = self.flags.bits();
        while data > 0 {
            if data & 1 == 1 {
                current_count += 1;
            }
            data >>= 1;
        }
        current_count
    }

    pub fn family_index(&self) -> usize {
        self.family_index
    }
}

// This struct is used to create the required queues
#[derive(Debug, Clone)]
pub struct DeviceQueue {
    priorities: Vec<f32>,
    family_index: usize,
    available_queues: u32,
    reserved_queues: u32,
}

impl DeviceQueue {
    pub fn new(family_index: usize, available_queues: u32) -> DeviceQueue {
        DeviceQueue {
            priorities: Vec::new(),
            available_queues,
            family_index,
            reserved_queues: 0,
        }
    }

    pub fn request_queue(&mut self, priority: f32) -> Option<u32> {
        if self.reserved_queues < self.available_queues {
            let current_index = self.reserved_queues;
            self.reserved_queues += 1;
            self.priorities.push(priority);
            return Some(current_index);
        }
        None
    }

    pub fn family_index(&self) -> usize {
        self.family_index
    }

    pub fn priorities(&self) -> &[f32] {
        self.priorities.as_slice()
    }

    pub fn reserved_queues(&self) -> u32 {
        self.reserved_queues
    }
}
#[derive(Debug)]
pub struct RendererQueues {
    graphics_queues: Vec<OperationQueue>,
    transfer_queues: Vec<OperationQueue>,
    sparse_queues: Vec<OperationQueue>,
    compute_queues: Vec<OperationQueue>,
}

#[derive(Debug)]
pub struct RendererQueuesBuilder {
    graphics_queues: Vec<OperationQueue>,
    transfer_queues: Vec<OperationQueue>,
    sparse_queues: Vec<OperationQueue>,
    compute_queues: Vec<OperationQueue>,
}

impl RendererQueuesBuilder {
    pub fn new() -> RendererQueuesBuilder {
        RendererQueuesBuilder {
            graphics_queues: Vec::new(),
            transfer_queues: Vec::new(),
            sparse_queues: Vec::new(),
            compute_queues: Vec::new(),
        }
    }

    pub fn build(self) -> RendererQueues {
        let RendererQueuesBuilder {
            graphics_queues,
            transfer_queues,
            sparse_queues,
            compute_queues,
        } = self;
        RendererQueues {
            graphics_queues,
            transfer_queues,
            sparse_queues,
            compute_queues,
        }
    }

    pub fn create_graphics_queue(
        &mut self,
        family_index: u32,
        priority: f32,
        index_to_use: u32,
        can_present: bool,
    ) {
        let queue = OperationQueue::new(
            family_index,
            priority,
            vk::QueueFlags::GRAPHICS,
            index_to_use,
            can_present,
        );
        self.graphics_queues.push(queue);
    }

    pub fn create_transfer_queue(&mut self, family_index: u32, priority: f32, index_to_use: u32) {
        let queue = OperationQueue::new(
            family_index,
            priority,
            vk::QueueFlags::TRANSFER,
            index_to_use,
            false,
        );
        self.transfer_queues.push(queue);
    }

    pub fn create_compute_queue(&mut self, family_index: u32, priority: f32, index_to_use: u32) {
        let queue = OperationQueue::new(
            family_index,
            priority,
            vk::QueueFlags::COMPUTE,
            index_to_use,
            false,
        );
        self.compute_queues.push(queue);
    }

    pub fn create_sparse_queue(&mut self, family_index: u32, priority: f32, index_to_use: u32) {
        let queue = OperationQueue::new(
            family_index,
            priority,
            vk::QueueFlags::SPARSE_BINDING,
            index_to_use,
            false,
        );
        self.sparse_queues.push(queue);
    }
}

#[derive(Debug)]
pub struct OperationQueue {
    family_index: u32,
    priority: f32,
    operations_supported: vk::QueueFlags,
    index_to_use: u32, // Total queues dedicated to graphics operations
    can_present: bool,
}
// TODO: A way to label each queue so that it's easy to get
impl OperationQueue {
    pub fn get_queue(&self) -> u32 {
        self.index_to_use
    }

    pub fn can_present(&self) -> bool {
        self.can_present
    }

    pub fn operations_supported(&self) -> vk::QueueFlags {
        self.operations_supported
    }

    pub fn new(
        family_index: u32,
        priority: f32,
        operations_supported: vk::QueueFlags,
        index_to_use: u32,
        can_present: bool,
    ) -> OperationQueue {
        OperationQueue {
            family_index,
            priority,
            operations_supported,
            index_to_use,
            can_present,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DeviceQueue, OperationQueue, QueueFamily, RendererQueues, RendererQueuesBuilder};
    use crate::config::QueueManager;
    use erupt::vk1_0 as vk;
    #[test]
    fn basic_operation_queue_test() {
        let queue = QueueFamily::new(
            2,
            vk::QueueFlags::GRAPHICS & vk::QueueFlags::COMPUTE,
            4,
            0,
            vk::Extent3D::default(),
            true,
        );

        let mut builder = RendererQueuesBuilder::new();
        builder.create_graphics_queue(2, 1.0, 0, true);
        builder.create_transfer_queue(3, 1.0, 0);
        builder.create_compute_queue(3, 1.0, 1);
        let render_queues = builder.build();
        let total_compute = render_queues.compute_queues.len();
        assert_eq!(total_compute, 1);
        match render_queues.graphics_queues.get(0) {
            Some(gq) => {
                assert_eq!(gq.get_queue(), 0);
                assert_eq!(gq.family_index, 2);
            }
            None => panic!("No graphics queue found"),
        }
        println!("Queues: {:?}", render_queues);
    }

    #[test]
    fn device_queue_test() {
        let mut dq = DeviceQueue::new(2, 5);
        let res = dq.request_queue(1.0);
        assert_eq!(res, Some(0));
        assert_eq!(dq.reserved_queues, 1);
        assert_eq!(dq.available_queues, 5);
    }
}
