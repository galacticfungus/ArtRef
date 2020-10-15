use std::ffi::c_void;
use erupt::extensions::khr_surface as surface;
use erupt::extensions::khr_win32_surface as win32_surface;

use crate::error;

// FIXME: Each GPU needs a surface associated with it but a GPU has no way of constructing a surface easily since it requires user parameters
// TODO: The above is not quite true, a GPU does not need a surface, just the data structures produced by a Surface object, ie capabilities, surface handle etc...
// TODO: This object is also responsible for recreating the surface if the surface is lost for some reason

#[cfg(target_os = "windows")]
#[derive(Debug)]
pub struct Surface<'a> {
    pub(super) entry: &'a erupt::DefaultEntryLoader,
    pub(super) instance: &'a erupt::InstanceLoader,
    // This is the Surface used by Vulkan
    pub(crate) platform_surface: erupt::extensions::khr_surface::SurfaceKHR,
    pub(super) hwnd: *mut std::ffi::c_void,
    pub(super) hinstance: *mut std::ffi::c_void,
}

#[cfg(target_os = "windows")]
impl<'a> Surface<'a> {
    pub(crate) fn new(entry: &'a erupt::DefaultEntryLoader, instance: &'a erupt::InstanceLoader, window: &winit::window::Window) -> Surface<'a> {
        // let f = handle as &dyn HasRawWindowHandle;
        use winit::platform::windows::WindowExtWindows;
        let hinstance = window.hinstance();
        let hwnd = window.hwnd();
        
        use erupt::extensions::khr_win32_surface;

        let create_info = khr_win32_surface::Win32SurfaceCreateInfoKHR {
            hinstance,
            hwnd,
            ..Default::default()
        };

        let platform_surface = unsafe {instance.create_win32_surface_khr(&create_info, None, None)}
            .expect("Failed to create windows surface");

        Surface {
            entry,
            instance,
            platform_surface,
            hwnd,
            hinstance,
        }
    }

    pub fn recreate_surface(&mut self) -> Result<(), error::Error> {
        
        use erupt::extensions::khr_win32_surface;
        let create_info = khr_win32_surface::Win32SurfaceCreateInfoKHR {
            hinstance: self.hinstance,
            hwnd: self.hwnd,
            ..Default::default()
        };

        let platform_surface = unsafe {self.instance.create_win32_surface_khr(&create_info, None, None)}
            .expect("Failed to create windows surface");
        self.platform_surface = platform_surface;
        Ok(())
    }
}

impl<'a> Drop for Surface<'a> {
    fn drop(&mut self) {
        println!("Surface is being dropped");
        unsafe { self.instance.destroy_surface_khr(Some(self.platform_surface), None) };
    }
}