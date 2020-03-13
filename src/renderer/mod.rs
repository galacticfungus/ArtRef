mod api;
mod create;
mod extensions;
mod features;
mod layers;
mod queues;
mod select;
mod surface;
mod vendor;
mod device;
mod render;
mod swapchain;

pub use crate::error::Error;
pub use api::VulkanApi;
pub use api::VulkanConfig;
pub use extensions::ExtensionManager;
pub use extensions::Extensions;
pub use layers::Layers;
pub use features::{Features, Feature};

pub use queues::{QueueFamily, QueueToCreate};
pub use select::{DeviceSelector, DeviceFilter, FiltersDevices, Gpu};
pub use surface::Surface;
pub use vendor::PciVendor;
pub use render::RenderDevice;
pub use device::VulkanDevice;
pub use create::ConfigureDevice;
pub use swapchain::{Swapchain, ConfigureSwapchain};

#[cfg(test)]
pub use select::TestGpuBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_config() {
        let entry = ash::Entry::new().expect("Failed to load Vulkan");
        //entry.try_enumerate_instance_version();
        let vulkan_api = VulkanConfig::new()
            .api_version(1, 0, 0)
            .application_name("ArtRef")
            .engine_name("RefRenderer")
            .engine_version(1, 0, 0)
            .application_version(1, 0, 0)
            .required_extensions(|mng| {
                mng.add_extension(Extensions::Surface);
                mng.add_extension(Extensions::Win32Surface);
            })
            .expect("Failed to load extensions")
            // .optional_extensions(|mng| {
            // })
            .with_layers(|mng| {
                mng.add_layer(Layers::KhronosValidation)
                    .expect("Failed to load layer");
            })
            .init();
        // TODO: Test something
    }

    #[test]
    fn test_device_selection() {}

    #[test]
    fn test_device_creation() {}
}
