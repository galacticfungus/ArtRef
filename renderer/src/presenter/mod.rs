mod config;
mod presenter;
mod types;

use winit::window::Window;
use erupt::extensions::khr_surface as surface;
use erupt::vk1_0 as vk;

pub use types::{PresentMode, SurfaceColourSpace, SurfaceFormat, SwapchainExtent, SwapchainImageCount};

pub fn create_surface(instance: &erupt::InstanceLoader, window: &Window) -> surface::SurfaceKHR {
    // Gets the surface capabilities that are selected when creating a swapchain
    use erupt::extensions::khr_win32_surface as win32_surface;
    use winit::platform::windows::WindowExtWindows;
    let hinstance = window.hinstance();
    let hwnd = window.hwnd();

    let create_info = win32_surface::Win32SurfaceCreateInfoKHR {
        hinstance,
        hwnd,
        ..Default::default()
    };
    // SAFE: Only incorrect values from window can result in undefined behaviour and its too late to detect that
    unsafe { instance.create_win32_surface_khr(&create_info, None, None) }
        .expect("Failed to create windows surface")
}

pub struct ConfigurePresenter {
    surface_capabilities: surface::SurfaceCapabilitiesKHR,
    surface_formats: Vec<surface::SurfaceFormatKHR>,
    present_modes: Vec<surface::PresentModeKHR>,
    // Create the surface
    surface: surface::SurfaceKHR,
    surface_format: Option<surface::SurfaceFormatKHR>,
    present_mode: Option<surface::PresentModeKHR>,
    swapchain_extent: Option<vk::Extent2D>,
    image_count: Option<u32>,
    // This needs to accept a bunch of borrows of surface capabilities since the are obtained during device creation
    // Create the swapchain
    // Create the ImageViews
    // Create the Framebuffers
}

/// Responsible for maintaining the swapchain and its backing imageviews and framebuffers, 
/// creating the surface and presenting the image to the surface
pub struct Presenter {
    surface: surface::SurfaceKHR,
    surface_format: surface::SurfaceFormatKHR,
    extent: vk::Extent2D,
}