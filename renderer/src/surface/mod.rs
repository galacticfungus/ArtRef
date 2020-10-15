// This module is responsible for creating the platform specific objects that Vulkan needs to create a surface to draw onto
use erupt::vk1_0 as vk;
use erupt::extensions::khr_surface as surface;
use crate::error;

mod platform;

pub use platform::Surface as Surface;

impl<'a> Surface<'a> {
    // Gets the surface capabilities that are selected when creating a swapchain
    pub fn get_surface_capabilities(&mut self, physical_device: vk::PhysicalDevice) -> Result<surface::SurfaceCapabilitiesKHR, error::Error> {
        let surface_capabilities = unsafe {self.instance.get_physical_device_surface_capabilities_khr(physical_device, self.platform_surface, None) }.result();
        match surface_capabilities {
            Ok(surface_capabilities) => Ok(surface_capabilities),
            Err(error) if error == vk::Result::ERROR_SURFACE_LOST_KHR => {
                for _ in 0..4 {
                    // This function can only return out of host or device memory errors so its fine to just return the error
                    self.recreate_surface()?;
                    let surface_capabilities = unsafe { self.instance.get_physical_device_surface_capabilities_khr(physical_device, self.platform_surface, None) }.result();
                    match surface_capabilities {
                        Ok(surface_capabilities) => return Ok(surface_capabilities),
                        Err(error) if error == vk::Result::ERROR_SURFACE_LOST_KHR => continue,
                        Err(error) => return Err(error::Error::VulkanApiError(error)), // TODO: This can use the from trait, just return the error
                    }
                }
                // TODO: Make a unable to recreate surface error
                return Err(error::Error::FailedToRecreateSurface);
            },
            Err(error) => Err(error::Error::VulkanApiError(error)),
        }
    }

    pub fn get_surface_support(&self, physical_device: vk::PhysicalDevice, family_index: u32) -> bool {
        unsafe {self.instance.get_physical_device_surface_support_khr(physical_device, family_index, self.platform_surface, None) }
            .expect(format!("Failed to retrieve surface support for family index {}", family_index))
    }

    // Get the surface formats supported, used in swap chain creation
    pub fn get_surface_formats(&mut self, physical_device: vk::PhysicalDevice) -> Result<Vec<surface::SurfaceFormatKHR>, error::Error> {
        let surface_formats = unsafe { self.instance.get_physical_device_surface_formats_khr(physical_device, self.platform_surface, None) }.result();
        match surface_formats {
            Ok(surface_formats) => Ok(surface_formats),
            Err(error) if error == vk::Result::ERROR_SURFACE_LOST_KHR => {
                for _ in 0..4 {
                    // This function can only return out of host or device memory errors so its fine to just return the error
                    self.recreate_surface()?;
                    // let Self {surface_extension, platform_surface, ..} = self;
                    let surface_formats = unsafe { self.instance.get_physical_device_surface_formats_khr(physical_device, self.platform_surface, None) }.result();
                    match surface_formats {
                        Ok(surface_formats) => return Ok(surface_formats),
                        Err(error) if error == vk::Result::ERROR_SURFACE_LOST_KHR => continue,
                        Err(error) => return Err(error::Error::VulkanApiError(error)),
                    }
                }
                // TODO: impl From to convert errors automatically, should we attempt to recreate the surface here
                return Err(error::Error::FailedToRecreateSurface);
            },
            Err(error) => Err(error::Error::VulkanApiError(error)),
        }
    }

    // 
    pub fn get_surface_presentation_modes(&mut self, physical_device: erupt::vk1_0::PhysicalDevice) -> Result<Vec<surface::PresentModeKHR>, error::Error> {
        
        let present_modes = unsafe { self.instance.get_physical_device_surface_present_modes_khr(physical_device, self.platform_surface, None) }.result();
        match present_modes {
            Ok(present_modes) => Ok(present_modes),
            Err(error) if error == vk::Result::ERROR_SURFACE_LOST_KHR => {
                for _ in 0..4 {
                    // This function can only return out of host or device memory errors so its fine to just return the error
                    self.recreate_surface()?;
                    // let Self {surface_extension, platform_surface, ..} = self;
                    let present_modes = unsafe { self.instance.get_physical_device_surface_present_modes_khr(physical_device, self.platform_surface, None).result() };
                    match present_modes {
                        Ok(present_modes) => return Ok(present_modes),
                        Err(error) if error == vk::Result::ERROR_SURFACE_LOST_KHR => continue,
                        Err(error) => return Err(error::Error::VulkanApiError(error)),
                    }
                }
                // TODO: Make a unable to recreate surface error
                return Err(error::Error::FailedToRecreateSurface);
            }
            Err(error) => Err(error::Error::VulkanApiError(error)),
        }
    }
}
