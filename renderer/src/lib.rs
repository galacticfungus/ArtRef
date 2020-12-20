mod instance;
mod error;
mod pick;
mod gpu;
mod version;
mod config;
mod device;
mod extensions;
mod features;
mod queues;
mod select;
mod vendor;
mod presenter;
mod pipeline;
mod renderpass;

pub use pipeline::ConfigurePipeline;
pub use crate::error::Error;
pub use pick::PickManager;
pub use version::Version;
pub use instance::VulkanApi;
pub use instance::VulkanConfig;
pub use extensions::ExtensionManager;
pub use instance::InstanceExtensions;
pub use instance::Layers;
pub use features::{Features, Feature};

pub use queues::{QueueFamily, DeviceQueue, RendererQueues, OperationQueue, RendererQueuesBuilder};
pub use select::{DeviceSelector, DeviceFilter, FiltersDevices, SelectedDevice};
pub use vendor::PciVendor;
pub use device::VulkanDevice;
pub use presenter::{Presenter, ConfigurePresenter, PresentMode, SurfaceFormat, SurfaceColourSpace, SwapchainExtent, SwapchainImageCount};
pub use config::{ConfigureDevice, DeviceExtensions};
pub use renderpass::Renderpass;
pub use pipeline::AttributeFormat;

use erupt::vk1_0 as vk;
use erupt::extensions::khr_surface;

// Must be clonable so that errors can access a list of Gpu's
#[derive(Clone)]
pub struct Gpu {
    device_handle: vk::PhysicalDevice,
    queue_families: Vec<QueueFamily>,
    api_version: Version,
    driver_version: u32,
    vendor_id: PciVendor,
    device_id: u32,
    device_name: [i8; erupt::vk1_0::MAX_PHYSICAL_DEVICE_NAME_SIZE as usize],
    device_type: vk::PhysicalDeviceType,
    available_extensions: Vec<vk::ExtensionProperties>,
    device_features: vk::PhysicalDeviceFeatures,
    surface_capabilities: khr_surface::SurfaceCapabilitiesKHR,
    surface_formats: Vec<khr_surface::SurfaceFormatKHR>,
    present_modes: Vec<khr_surface::PresentModeKHR>,
    presentable: bool,
    // pipelinecacheID,
    // limits,
    // sparse_properties,
}

#[cfg(test)]
mod tests {
    use super::*;
    use winit;

    #[test]
    fn test_device_selection() {}

    #[test]
    fn test_device_creation() {}
}
