use std::ffi::c_void;

use ash::vk;

use crate::error;

// FIXME: Each GPU needs a surface associated with it but a GPU has no way of constructing a surface easily since it requires user parameters
// TODO: The above is not quite true, a GPU does not need a surface, just the data structures produced by a Surface object, ie capabilities, surface handle etc...
// TODO: This object is also responsible for recreating the surface if the surface is lost for some reason

#[cfg(target_os = "windows")]
pub struct Surface<'a> {
    pub(super) entry: &'a ash::Entry,
    pub(super) instance: &'a ash::Instance,
    // This is the Surface used by Vulkan
    pub(super) platform_surface: vk::SurfaceKHR,
    pub(super) surface_extension: ash::extensions::khr::Surface,
    pub(super) hwnd: *const c_void,
    pub(super) hinstance: *const c_void,
}

#[cfg(target_os = "windows")]
impl<'a> Surface<'a> {
    pub(crate) fn new(entry: &'a ash::Entry, instance: &'a ash::Instance, hwnd: *const c_void, hinstance: *const c_void) -> Surface<'a> {
        let platform_extension =
            ash::extensions::khr::Win32Surface::new(entry, instance);
        let create_info = vk::Win32SurfaceCreateInfoKHR {
            hinstance,
            hwnd,
            ..Default::default()
        };
        let platform_surface = unsafe { platform_extension.create_win32_surface(&create_info, None) }.expect("Failed to create win32 surface");
        let surface_extension = ash::extensions::khr::Surface::new(entry, instance);
        Surface {
            entry,
            instance,
            surface_extension,
            platform_surface,
            hwnd,
            hinstance,
        }
    }

    pub fn recreate_surface(&mut self) -> Result<(), error::Error> {
        let platform_extension =
            ash::extensions::khr::Win32Surface::new(self.entry, self.instance);
        let create_info = vk::Win32SurfaceCreateInfoKHR {
            hinstance: self.hinstance,
            hwnd: self.hwnd,
            ..Default::default()
        };
        let platform_surface = unsafe { platform_extension.create_win32_surface(&create_info, None) };
        match platform_surface {
            Ok(platform_surface) => {
                self.platform_surface = platform_surface
            },
            Err(error) => return Err(error::Error::VulkanApiError(error)),
        }
        Ok(())
    }
}