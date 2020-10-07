use renderer::Features;
use renderer::{
    DeviceExtensions, InstanceExtensions, PciVendor, PresentMode, SurfaceColourSpace,
    SurfaceFormat, VulkanConfig,
};
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
            mng.add_extension(InstanceExtensions::DebugUtils);
        })
        .required_extensions(|mng| {
            mng.add_extension(InstanceExtensions::Surface);
            mng.add_extension(InstanceExtensions::Win32Surface);
        })
        .expect("Failed to load surface extensions")
        .init();
    assert!(vulkan_api.extension_loaded(renderer::InstanceExtensions::DebugUtils));
    let mut surface = vulkan_api.create_surface_win32(hwnd, hinstance);
    let mut selector = vulkan_api
        .select_device(&mut surface)
        .expect("Failed to create selector");
    // Filter devices based on requirements
    let gpu = selector
        .required_device_extensions(|mng| {
            // Filter out any devices that dont support a swap chain
            mng.add_extension(renderer::DeviceExtensions::Swapchain);
        })
        .expect("Failed to load required device extensions")
        // If the device is an nvidia device
        .if_vendor(PciVendor::NVidia, |de| {
            // Filter out any nividia device that does not have a graphics queue
            de.has_graphics_queue();
        })
        .is_discrete()
        .select_device();

    // Pick one of the remaining devices doesn't matter which

    // Configure the selected GPU
    let mut gpu_config = vulkan_api.configure_device(gpu).expect("error");
    gpu_config
        .enable_feature(Features::GeometryShader)
        .expect("Failed to enable feature")
        .enable_feature(Features::TesselationShader)
        .expect("Failed to enable tesselation")
        .define_queues(|mng| {
            mng.create_graphics_queue(1.0, true);
        })
        .expect("Failed to create queues")
        .extensions_to_load(|mng| {
            // Load the swapchain extension
            mng.add_extension(DeviceExtensions::Swapchain);
        });
    let device = gpu_config.create_device();
    let mut configure_swapchain = vulkan_api.configure_swapchain(&device, surface);
    configure_swapchain
        .select_present_mode(|mng| {
            mng.pick(PresentMode::Mailbox);
        })
        .expect("Failed to select mode")
        .select_surface_colour_space(|mng| {
            // Colour space in order of preference
            mng.pick(SurfaceColourSpace::SRGBNonlinear);
        })
        .select_surface_format(|surface_picker| {
            // Surface formats in order of preference
            surface_picker.pick(SurfaceFormat::B8G8R8A8SRGB);
            surface_picker.pick(SurfaceFormat::B8G8R8A8UNorm);
        })
        // This will only be used on devices that dont automatically set the surface resolution equal to the window
        // resolution
        .select_extent(|extent| {
            // TODO: This should use the current window height and width
            extent.set_extent(extent.max_width(), extent.max_height());
        })
        .select_image_count(|image_count| {
            if image_count.min_images() + 1 > image_count.max_images() {
                // We set the total swapchain images to max_images if min + 1
                image_count.set_image_count(image_count.max_images());
            } else {
                // This includes the case where the
                image_count.set_image_count(image_count.min_images() + 1);
            }
        });
    let swapchain = configure_swapchain
        .build()
        .expect("Failed to build swapchain");
    // Configure number of images
    // Set sharing / exclusive
    // Create swapchain
    let renderer = vulkan_api.create_renderer(device, swapchain);
    // Create the windowand surface, create the swap chain
    //renderer.create_pipeline()

    window.init_events();
}
