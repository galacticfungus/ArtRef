// Module responsible for creating a window
use std::os::raw::c_void;
use winit;

pub struct Window {
    window: winit::window::Window,
    event_loop: winit::event_loop::EventLoop<()>,
}

impl Window {
    pub fn new() -> Window {
        let event_loop = winit::event_loop::EventLoop::new();
        let window = Window::create_window(&event_loop);
        Window { event_loop, window }
    }

    // Starts the event loop and responding to user events
    pub fn init_events(self) {
        // Hi jacks the calling thread which means we probably want to call this from another thread
        let Window { window, event_loop } = self;
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
                        winit::event::KeyboardInput { scancode, .. } => {
                            println!("Scan code: {}", scancode);
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

    pub fn get_win32_handles(&self) -> (*mut c_void, *mut c_void) {
        use winit::platform::windows::WindowExtWindows;
        // These values must not outlive the window itself
        let hwnd = self.window.hwnd();
        let hinstance = self.window.hinstance();
        (hwnd, hinstance)
    }
}
