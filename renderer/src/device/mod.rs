mod config;
mod device;
mod indirect_count;
mod extensions;
mod queue;
mod present;

pub use indirect_count::DrawIndirectCount;
pub use extensions::DeviceExtensions;
pub use config::ConfigureDevice;
use super::{QueueFamily, QueueToCreate, PciVendor, Features, Feature, ExtensionManager};
use crate::Version;

use ash::vk;

use std::collections::HashMap;



/// A fully configured device ready for use
pub struct VulkanDevice {
    // TODO: Extract hte relevant daa for a configured device from gpu
    physical_device: vk::PhysicalDevice,
    // A vector of the vulkan queues we created
    queues: Vec<vk::Queue>,
    device: ash::Device,
    enabled_features: vk::PhysicalDeviceFeatures,
    extensions_loaded: HashMap<DeviceExtensions, bool>,
    surface_capabilities: vk::SurfaceCapabilitiesKHR,
    surface_formats: Vec<vk::SurfaceFormatKHR>,
    present_modes: Vec<vk::PresentModeKHR>,
    api_version: Version,
    driver_version: u32,
    vendor_id: PciVendor,
    device_id: u32,
    device_name: [i8; ash::vk::MAX_PHYSICAL_DEVICE_NAME_SIZE],
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

pub struct PresentModeManager<'a> {
    modes_picked: &'a mut Vec<PresentMode>,
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