mod window;
pub use window::Window;
use renderer::Features;
use renderer::{
    DeviceExtensions, InstanceExtensions, PciVendor, PresentMode, SurfaceColourSpace,
    SurfaceFormat, VulkanConfig,
};


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
