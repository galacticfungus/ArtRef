// Module responsible for creating a window
use super::{
    DeviceExtensions, Features, InstanceExtensions, PciVendor, VulkanConfig, PresentMode, SwapchainExtent, SwapchainImageCount, SurfaceColourSpace, SurfaceFormat
};
use std::os::raw::c_void;
use erupt::vk1_0::VertexInputRate;
use winit;


pub struct Window {
    window: winit::window::Window,
    event_loop: winit::event_loop::EventLoop<()>,
}



impl Window {
    pub fn new() -> () {
        use renderer::FiltersDevices;
        
        let event_loop = winit::event_loop::EventLoop::new();
        let window = Window::create_window(&event_loop);
        // let (hwnd, hinstance) = window.get_win32_handles();
        let mut lib = VulkanConfig::new().expect("Failed to create Vulkan Config")
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
            .expect("Failed to load surface extensions which are required")
            .init()
            .expect("Failed to initialize the Vulkan API");
        assert!(lib.extension_loaded(renderer::InstanceExtensions::DebugUtils));
        // A window to create a surface is optional when configuring a device?
        // TODO: This should be optional, not supplying it will remove the presentable option as well as remove the ability to configure surface options.
        let device_selector = lib.create_device_selector(&window)
            .expect("Failed to load device selector");
            // Filter devices based on requirements
            // TODO: Wrap this in a SelectedDevice struct
        let (selected_device, configure_presenter) = device_selector.required_device_extensions(|mng| {
            // Filter out any devices that dont support a swap chain
            mng.add_extension(renderer::DeviceExtensions::Swapchain);
            })
            .expect("Failed to load required device extensions")
            // If the device is an nvidia device
            .if_vendor(PciVendor::NVidia, |de| {
                // Filter out any nividia device that does not have a graphics queue
                de.has_graphics_queue();
            })
            // TODO: Largest memory and other performance related queuries
            .is_discrete()
            .select_device();
        
        // Pick one of the remaining devices doesn't matter which as they all have required features
        
        let mut configure_device = lib.configure_device(selected_device);
        // TODO: Debug like options for objects created after device creation if this extension is enabled vkDebugMarkerSetObjectNameEXT - how to expose
            // TODO: Conditional device features
        let device = configure_device.enable_feature(Features::GeometryShader)
            .expect("Failed to enable geometry shader")
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
            }).create_device();
        
        let presenter = configure_presenter.select_present_mode(|mng| {
                mng.pick(PresentMode::Mailbox);
            })
            .expect("Failed to select mode")
            .select_surface_colour_space(|mng| {
                // Colour space in order of preference
                // TODO: Provide more options for selecting a space, like any SRGB etc
                mng.pick(SurfaceColourSpace::SRGBNonlinear);
            })
            .select_surface_format(|surface_picker| {
                // Surface formats in order of preference
                // TODO: Provide more generic methods of selecting a format, ie any rgba format with srgb
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
            .select_presentation_image_count(|image_count| {
                if image_count.min_images() + 1 > image_count.max_images() {
                    // We set the total swapchain images to max_images
                    image_count.set_image_count(image_count.max_images());
                } else {
                    // Otherwise we can safely set it to min + 1
                    image_count.set_image_count(image_count.min_images() + 1);
                }
            }).create_presenter();
        // Image view settings
        // - viewtype
        // - swizzle
        // - sub resource range - specifics
        // TODO: This should point to the ui section not back to the renderer
        let vert_bytes = include_bytes!("../../renderer/src/shaders/vert.spv");
        let frag_bytes = include_bytes!("../../renderer/src/shaders/frag.spv");
        let vertex_shader = erupt::utils::decode_spv(vert_bytes)
            .expect("Failed to vertex decode shader code as &[32]");
        let fragment_shader = erupt::utils::decode_spv(frag_bytes)
            .expect("Failed to decode fragment shader code as &[32]");
        
        let mut configure_pipeline = device.create_pipeline();
        configure_pipeline.configure_shaders(&mut |shaders| {
            shaders.create_fragment_shader("main",fragment_shader.as_slice())?
                   .create_vertex_shader("main",vertex_shader.as_slice())?;
            Ok(())
        }).expect("Failed to configure shaders")
        .configure_vertex_input(&mut|configure_input|{
            configure_input.add_binding(0,erupt::vk1_0::VertexInputRate::VERTEX, 1)
                .add_attribute(0, erupt::vk1_0::Format::A2B10G10R10_SINT_PACK32)
                .add_attribute(1, erupt::vk1_0::Format::A2B10G10R10_SINT_PACK32);
            // configure_input.add_binding(1, VertexInputRate::INSTANCE, 8)
            //     .add_attribute(2, erupt::vk1_0::Format::R8G8B8_SRGB);
        })
        .configure_input_assembely(erupt::vk1_0::PrimitiveTopology::TRIANGLE_LIST, false)
        .configure_viewport(&mut|viewports|{
            // TODO: Support multiple viewports as well as some way to check if its available
            viewports.create_viewport(0.0, 0.0, presenter.get_width() as f32, presenter.get_height() as f32, 0.0, 1.0, 0, 0, presenter.get_width(), presenter.get_height())
        })
        .configure_rasterizer(&mut|config|{

        });
        
        // TODO: This will be multiple renderpasses and should be contained in a frame graph
        let single_pass = device.create_renderpass().create_test_renderpass(presenter.get_format());
        //single_pass.create_test_renderpass();
        
        // Read shaders from files or w/e
        // create shader modules for each shader
        // create pipeline shader
        // create vertexinput pipeline
        // create viewport
        // create rasterizer
        // setup multisampling
        // setup depth test
        // setup color blend
        // pipeline layout
        // create attachments

        // Window {
        //     event_loop,
        //     window,
        //     renderer,
        // }
    }

    // Starts the event loop and responding to user events
    pub fn init_events(self) {
        // Hi jacks the calling thread which means we probably want to call this from another thread
        let Window {
            window,
            event_loop,
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
                            position.to_logical(window.current_monitor().unwrap().scale_factor());
                        println!("{},{} logical position", pos.x, pos.y);
                    }
                    _ => {}
                },
                winit::event::Event::MainEventsCleared => {
                    // Application update code.

                    // Queue a RedrawRequested event.
                    window.request_redraw();
                },
                winit::event::Event::RedrawRequested(_) => {
                    // Redraw the application.
                    //
                    // It's preferrable to render in this event rather than in MainEventsCleared, since
                    // rendering in here allows the program to gracefully handle redraws requested
                    // by the OS.
                    
                },
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

    pub fn get_handle(& self) -> &winit::window::Window {
        &self.window
    }
}
