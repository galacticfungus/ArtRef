mod instance;
mod error;
mod pick;
mod version;
mod device;
mod extensions;
mod features;
mod queues;
mod select;
mod surface;
mod vendor;
mod render;
mod swapchain;

pub use crate::error::Error;
pub use pick::PickManager;
pub use version::Version;
pub use instance::VulkanApi;
pub use instance::VulkanConfig;
pub use extensions::ExtensionManager;
pub use instance::InstanceExtensions;
pub use device::DeviceExtensions;
pub use instance::Layers;
pub use features::{Features, Feature};

pub use queues::{QueueFamily, QueueToCreate};
pub use select::{DeviceSelector, DeviceFilter, FiltersDevices, Gpu};
pub use surface::Surface;
pub use vendor::PciVendor;
pub use render::RenderDevice;
pub use device::{VulkanDevice, ConfigureDevice, PresentMode};
pub use swapchain::{Swapchain, ConfigureSwapchain};

#[cfg(test)]
pub use select::TestGpuBuilder;

#[cfg(test)]
mod tests {
    use super::*;
    use winit;
    #[test]
    fn test_api_config() {
        //entry.try_enumerate_instance_version();
        let vulkan_api = VulkanConfig::new()
            .api_version(1, 0, 0)
            .application_name("ArtRef")
            .engine_name("RefRenderer")
            .engine_version(1, 0, 0)
            .application_version(1, 0, 0)
            .required_extensions(|mng| {
                mng.add_extension(InstanceExtensions::Surface);
                mng.add_extension(InstanceExtensions::Win32Surface);
            }).expect("Failed to load required instance extensions")
            .optional_extensions(|mng| {
                mng.add_extension(InstanceExtensions::DebugUtils);
            })
            .with_layers(|mng| {
                mng.add_layer(Layers::KhronosValidation);
            })
            .init();
        use winit::platform::windows::EventLoopExtWindows;
        use winit::platform::windows::WindowExtWindows;
        // let event_loop = winit::platform::windows::EventLoopExtWindows::new_any_thread();
        let event_loop = winit::event_loop::EventLoop::<()>::new_any_thread();
        let window = winit::window::Window::new(&event_loop).expect("Failed to create window");
        let hwnd = window.hwnd();
        let hinstance = window.hinstance();
        let mut surface = vulkan_api.create_surface_win32(hwnd, hinstance);
        let mut selector = vulkan_api.create_selector(&mut surface).expect("Test Selector");
        let mut config = selector.required_device_extensions(|mng| {
                mng.add_extension(DeviceExtensions::Swapchain);
            }).expect("Required Device Extensions Missing")
            .select_device();
        let device = config.extensions_to_load(|mng| {
            mng.add_extension(DeviceExtensions::Swapchain);
        });
        println!("{:?}", device);
        // TODO: Test something
    }

    #[test]
    fn test_device_selection() {}

    #[test]
    fn test_device_creation() {}
}
