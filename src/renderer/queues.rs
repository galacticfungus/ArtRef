use ash::vk;

#[derive(Debug, Clone)]
pub struct QueueFamily {
    index: usize,
    flags: vk::QueueFlags,
    queue_count: u32,
    time_stamp: u32,
    image_granularity: vk::Extent3D,
    total_queue_types: u32,
    queues_to_create: Vec<QueueToCreate>,
    presentable: bool,
}

impl QueueFamily {
    pub fn new(
        index: usize,
        flags: vk::QueueFlags,
        queue_count: u32,
        time_stamp: u32,
        image_granularity: vk::Extent3D,
        presentable: bool,
    ) -> QueueFamily {
        let mut supported_operations = 0;
        let mut data = flags.as_raw();
        while data > 0 {
            if data & 1 == 1 {
                supported_operations += 1;
            }
            data >>= 1;
        }

        QueueFamily {
            index,
            flags,
            queue_count,
            time_stamp,
            image_granularity,
            total_queue_types: supported_operations,
            queues_to_create: Vec::default(),
            presentable,
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

    pub fn is_full(&self) -> bool {
        if self.queue_count > self.queues_to_create.len() as u32 {
            return false;
        }
        true
    }

    pub fn supports_compute(&self) -> bool {
        if self.flags & vk::QueueFlags::COMPUTE == vk::QueueFlags::COMPUTE {
            return true;
        }
        false
    }

    pub fn presentable(&self) -> bool {
        self.presentable
    }

    pub fn flags(&self) -> vk::QueueFlags {
        self.flags
    }

    pub fn total_queues(&self) -> u32 {
        self.queue_count
    }

    pub fn total_queue_types(&self) -> usize {
        let mut current_count = 0;
        let mut data = self.flags.as_raw();
        while data > 0 {
            if data & 1 == 1 {
                current_count += 1;
            }
            data >>= 1;
        }
        current_count
    }

    pub fn add_queue_to_create(&mut self, queue_to_create: QueueToCreate) {
        self.queues_to_create.push(queue_to_create);
    }

    pub fn queues_to_create(&self) -> &[QueueToCreate] {
        self.queues_to_create.as_slice()
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

#[derive(Debug, Clone)]
pub struct QueueToCreate {
    operations_supported: vk::QueueFlags,
    priority: f32,
    // This is the index of the queue in order of creation this is used to ensure
    // that the vector of queues matches the order that they were created in
    index: usize,
    must_present: bool,
}

impl QueueToCreate {
    pub fn new(operations_supported: vk::QueueFlags, priority: f32, index: usize, must_present: bool) -> QueueToCreate {
        QueueToCreate {
            operations_supported,
            priority,
            index,
            must_present,
        }
    }

    pub fn must_present(&self) -> bool {
        self.must_present
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn priority(&self) -> f32 {
        self.priority
    }

    pub fn supported_operations(&self) -> vk::QueueFlags {
        self.operations_supported
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    impl QueueFamily {
        pub fn create_test_family(
            index: usize,
            queue_types: vk::QueueFlags,
            queue_count: u32,
            presentable: bool,
        ) -> QueueFamily {
            QueueFamily::new(index, queue_types, queue_count, 0, vk::Extent3D::default(), presentable)
        }
    }

    #[test]
    fn test_get_queue_count() {
        let test_data = QueueFamily::new(0, vk::QueueFlags::COMPUTE, 1, 0, vk::Extent3D::default(), false);
        assert_eq!(test_data.total_queue_types(), 1);
        let test_data = QueueFamily::new(
            0,
            vk::QueueFlags::COMPUTE | vk::QueueFlags::GRAPHICS,
            1,
            0,
            vk::Extent3D::default(),
            true,
        );
        assert_eq!(test_data.total_queue_types(), 2);
        let test_data = QueueFamily::new(
            0,
            vk::QueueFlags::TRANSFER | vk::QueueFlags::COMPUTE | vk::QueueFlags::GRAPHICS,
            1,
            0,
            vk::Extent3D::default(),
            false,
        );
        assert_eq!(test_data.total_queue_types(), 3);
    }
}
