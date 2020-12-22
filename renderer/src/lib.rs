mod config;
mod device;
mod error;
mod extensions;
mod features;
mod gpu;
mod instance;
mod pick;
mod pipeline;
mod presenter;
mod queues;
mod renderpass;
mod select;
mod vendor;
mod version;

pub use crate::error::Error;
pub use extensions::ExtensionManager;
pub use features::{Feature, Features};
pub use instance::InstanceExtensions;
pub use instance::Layers;
pub use instance::VulkanApi;
pub use instance::VulkanConfig;
pub use pick::PickManager;
pub use pipeline::ConfigurePipeline;
pub use version::Version;

pub use config::{ConfigureDevice, DeviceExtensions};
pub use device::VulkanDevice;
pub use pipeline::AttributeFormat;
pub use presenter::{
    ConfigurePresenter, PresentMode, Presenter, SurfaceColourSpace, SurfaceFormat, SwapchainExtent,
    SwapchainImageCount,
};
pub use queues::{DeviceQueue, OperationQueue, QueueFamily, RendererQueues, RendererQueuesBuilder};
pub use renderpass::Renderpass;
pub use select::{DeviceFilter, DeviceSelector, FiltersDevices, SelectedDevice};
pub use vendor::PciVendor;

use erupt::extensions::khr_surface;
use erupt::vk1_0 as vk;

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
