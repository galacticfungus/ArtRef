mod window;
pub use window::Window;
use renderer::Features;
use renderer::{
    DeviceExtensions, InstanceExtensions, PciVendor,
    VulkanConfig, SurfaceColourSpace, SurfaceFormat, PresentMode, SwapchainExtent, SwapchainImageCount, Presenter
};


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
