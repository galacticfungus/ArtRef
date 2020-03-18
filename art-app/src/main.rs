
use renderer::{ VulkanConfig, PciVendor, PresentMode};
use renderer::Features;
use ui::Window;
fn main() {
    // Need to clean up stuff, for instance DestroyInstance
    use renderer::FiltersDevices;
    let window = Window::new();
    let (hwnd, hinstance) = window.get_win32_handles();
    let vulkan_api = VulkanConfig::new()
        .api_version(1, 0, 0)
        .application_version(1, 0, 0)
        .engine_version(1, 0, 0)
        .engine_name("ArtRef Renderer v0.1")
        .application_name("ArtRef")
        .with_layers(|mng| {
            mng.add_layer(renderer::Layers::KhronosValidation);
        })
        .optional_extensions(|mng| {
            mng.add_extension(renderer::Extensions::DebugUtils);
        })
        .required_extensions(|mng| {
            mng.add_extension(renderer::Extensions::Surface);
            mng.add_extension(renderer::Extensions::Win32Surface);
        })
        .expect("Failed to load surface extensions")
        .init();
    assert!(vulkan_api.extension_loaded(renderer::Extensions::DebugUtils));
    let mut surface = vulkan_api.create_surface_win32(hwnd, hinstance);
    let mut selector = vulkan_api.create_selector(&mut surface).expect("Failed to create selector");
    let mut device_config = selector.is_discrete()
                                .required_device_extensions(|mng| {
                                    // Filter out any devices that dont support a swap chain
                                    mng.add_extension(renderer::Extensions::Swapchain);
                                })
                                .expect("Failed to load required device extensions")
                                // If the device is an nvidia device
                                .if_vendor(PciVendor::NVidia, |de| {
                                    // Filter out any nividia device that does not have a graphics queue
                                    de.has_graphics_queue();
                                })
                                // Pick one of the remaining devices doesn't matter which
                                .select_device();
    device_config.enable_feature(Features::GeometryShader).expect("Failed to enable feature")
                 .enable_feature(Features::TesselationShader).expect("Failed to enable tesselation")
                 .define_queues(|mng| {
                     mng.create_graphics_queue(1.0, true);
                 }).expect("Failed to create queues")
                 .select_present_mode(|mng|{
                    mng.pick_mode(PresentMode::Mailbox);
                 }).expect("Failed to select mode")
                 .extensions_to_load(|mng| {

                 }).expect("Failed to load extensions");
    let device = device_config.create_device();
    

    // Create the windowand surface, create the swap chain
    // window.init_events();
}
