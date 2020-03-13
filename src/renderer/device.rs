use ash::vk;
use super::Gpu;

/// A fully configured device ready for use
pub struct VulkanDevice {
    // TODO: Extract hte relevant daa for a configured device from gpu
    gpu: Gpu,
    queues: Vec<vk::Queue>,
    device: ash::Device,
}

impl VulkanDevice {}