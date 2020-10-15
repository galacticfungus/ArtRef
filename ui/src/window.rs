// Module responsible for creating a window
use super::{
    DeviceExtensions, Features, InstanceExtensions, PciVendor, PresentMode, SurfaceColourSpace,
    SurfaceFormat, VulkanConfig,
};
use std::os::raw::c_void;
use winit;

pub struct Window<'a> {
    window: winit::window::Window,
    event_loop: winit::event_loop::EventLoop<()>,
    renderer: renderer::RenderDevice<'a>,
}

impl<'a> Window<'a> {
    pub fn new() -> Window<'a> {
        use renderer::FiltersDevices;

        let event_loop = winit::event_loop::EventLoop::new();
        let window = Window::create_window(&event_loop);
        // let (hwnd, hinstance) = window.get_win32_handles();
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
        let mut surface = vulkan_api.create_surface(&window);
        // let mut surface = vulkan_api.create_surface_win32(window);
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
            // Pick one of the remaining devices doesn't matter which as they all have required features
            .select_device();
        // Configure the selected GPU
        let mut gpu_config = vulkan_api.configure_device(gpu).expect("error");
        gpu_config
            // TODO: Conditional device features
            .enable_feature(Features::GeometryShader)
            .expect("Failed to enable feature")
            .enable_feature(Features::TesselationShader)
            .expect("Failed to enable tesselation")
            .define_queues(|mng| {
                // This creates both a DeviceQueue which is used to create the Queue on the hardware
                // but also creates an entry in the renderer_queue that we use to issue commands to that queue
                mng.create_graphics_queue(1.0, true);
                // TODO: Create a method to conditionally create queues based on total queues available
                // TODO: Create a method to conditionally create a queue based on number of queue families
                // TODO: Create a method to conditionally create a queue based on the idea of only creating a queue if it belongs in a seperate family
                // TODO: To support the above, each create needs to return its family index but we can wrap it
                // TODO: Create a set of queues based on hardware
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
                surface_picker.pick(SurfaceFormat::R8G8B8A8SRGB);
            })
            // This will only be used on devices that dont automatically set the surface resolution equal to the window
            // resolution
            .select_extent(|extent| {
                // TODO: This should use the current window height and width
                extent.set_extent(extent.max_width(), extent.max_height());
            })
            .select_image_count(|image_count| {
                if image_count.min_images() + 1 > image_count.max_images() {
                    // We set the total swapchain images to max_images
                    image_count.set_image_count(image_count.max_images());
                } else {
                    // Otherwise we can safely set it to min + 1
                    image_count.set_image_count(image_count.min_images() + 1);
                }
            });
        let swapchain = configure_swapchain
            .build()
            .expect("Failed to build swapchain");
        // Configure number of images
        // Set sharing / exclusive
        // Create swapchain
        println!("Creating Renderer");
        let renderer = vulkan_api.create_renderer(vulkan_api, device, swapchain);
        // Create the windowand surface, create the swap chain
        // renderer.create_pipeline()
        
        Window {
            event_loop,
            window,
            renderer,
        }
    }

    // Starts the event loop and responding to user events
    pub fn init_events(self) {
        // Hi jacks the calling thread which means we probably want to call this from another thread
        let Window {
            window,
            event_loop,
            renderer,
        } = self;
        event_loop.run(move |event, _, control_flow| {
            // ControlFlow::Wait pauses the event loop if no events are available to process.
            // This is ideal for non-game applications that only update in response to user
            // input, and uses significantly less power/CPU time than ControlFlow::Poll.
            *control_flow = winit::event_loop::ControlFlow::Wait;

            match event {
                winit::event::Event::WindowEvent { event, window_id } => match event {
                    winit::event::WindowEvent::CloseRequested => {
                        println!("The close button was pressed; stopping");
                        *control_flow = winit::event_loop::ControlFlow::Exit
                    }
                    winit::event::WindowEvent::KeyboardInput { input, .. } => match input {
                        winit::event::KeyboardInput { scancode, virtual_keycode,.. } => {
                            println!("Scan code: {}", scancode);
                            match virtual_keycode {
                                Some(virtual_keycode) => println!("Virtual Keycode: {:?}", virtual_keycode),
                                None => println!("No virtual keycode for this scancode"),
                            }
                        }
                    },
                    winit::event::WindowEvent::MouseInput { state, button, .. } => {}
                    winit::event::WindowEvent::CursorMoved { position, .. } => {
                        let pos: winit::dpi::LogicalPosition<f64> =
                            position.to_logical(window.current_monitor().scale_factor());
                        println!("{},{} logical position", pos.x, pos.y);
                    }
                    _ => {}
                },

                winit::event::Event::MainEventsCleared => {
                    // Application update code.

                    // Queue a RedrawRequested event.
                    window.request_redraw();
                }
                winit::event::Event::RedrawRequested(_) => {
                    // Redraw the application.
                    //
                    // It's preferrable to render in this event rather than in MainEventsCleared, since
                    // rendering in here allows the program to gracefully handle redraws requested
                    // by the OS.
                    
                }
                _ => (),
            }
        });
    }

    fn create_window(event_loop: &winit::event_loop::EventLoop<()>) -> winit::window::Window {
        let window = winit::window::WindowBuilder::new()
            .with_inner_size(winit::dpi::LogicalSize::new(640.0, 480.0))
            .with_resizable(false)
            .with_decorations(false)
            .with_visible(true)
            .with_transparent(false)
            .build(&event_loop)
            .unwrap();
        window
    }

    pub fn get_handle(&'a self) -> &winit::window::Window {
        &self.window
    }
}
