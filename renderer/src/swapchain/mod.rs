use erupt::vk1_0 as vk;
use erupt::extensions::khr_surface as surface;
use erupt::extensions::khr_swapchain;
use crate::Surface;
use crate::device;
mod config;
mod types;
mod swapchain;
// TODO: Should we keep surface capabilities, we certainly need to compute a new size, we are able to just get a new surface capabilities
#[derive(Debug)]
pub struct Swapchain<'a> {
    image_count: u32,
    present_mode: surface::PresentModeKHR,
    surface_format: surface::SurfaceFormatKHR,
    surface: Surface<'a>,
    swapchain_extent: vk::Extent2D,
    image_usage: vk::ImageUsageFlags,
    sharing_mode: vk::SharingMode,
    swapchain: khr_swapchain::SwapchainKHR,
    clipped: bool,
    composite_alpha: surface::CompositeAlphaFlagsKHR,
    transform: surface::SurfaceTransformFlagBitsKHR,
    previous_swapchain: Option<khr_swapchain::SwapchainKHR>,
    images: Vec<vk::Image>
    // queue indicies and count if in Sharing Mode
}

pub struct ConfigureSwapchain<'a, 'b> {
    surface_format: Option<surface::SurfaceFormatKHR>,
    present_mode: Option<surface::PresentModeKHR>,
    present_modes: &'a [surface::PresentModeKHR],
    surface_formats: &'a [surface::SurfaceFormatKHR],
    surface_capabilities: &'a surface::SurfaceCapabilitiesKHR,
    surface: Surface<'b>,
    swapchain_extent: Option<vk::Extent2D>,
    image_count: Option<u32>,
    device: &'a device::VulkanDevice
}
#[derive(Debug)]
pub enum PresentMode {
    /// Specifies that the presentation engine does not wait for a vertical blanking period to update the current image, meaning 
    /// this mode may result in visible tearing. No internal queuing of presentation requests is needed, as the requests are applied immediately.
    Immediate,
    /// Mailbox specifies that the presentation engine waits for the next vertical blanking period to update the current image.
    /// Tearing cannot be observed. An internal single-entry queue is used to hold pending presentation requests. If the queue is full when a 
    /// new presentation request is received, the new request replaces the existing entry, and any images associated with the prior entry become 
    /// available for re-use by the application. One request is removed from the queue and processed during each vertical blanking period in which 
    /// the queue is non-empty.
    Mailbox,
    /// Fifo specifies that the presentation engine waits for the next vertical blanking period to update the current image.
    /// Tearing cannot be observed. An internal queue is used to hold pending presentation requests. New requests are appended to the end of the queue,
    /// and one request is removed from the beginning of the queue and processed during each vertical blanking period in which the queue is non-empty.
    /// This is the only value of presentMode that is required to be supported.
    Fifo,
    /// FifoRelaxed specifies that the presentation engine generally waits for the next vertical blanking period to update the
    /// current image. If a vertical blanking period has already passed since the last update of the current image then the presentation engine does
    /// not wait for another vertical blanking period for the update, meaning this mode may result in visible tearing in this case. This mode is useful
    /// for reducing visual stutter with an application that will mostly present a new image before the next vertical blanking period, but may occasionally
    /// be late, and present a new image just after the next vertical blanking period. An internal queue is used to hold pending presentation requests.
    /// New requests are appended to the end of the queue, and one request is removed from the beginning of the queue and processed during or after each
    /// vertical blanking period in which the queue is non-empty.
    FifoRelaxed,
    /// SharedDemandRefresh specifies that the presentation engine and application have concurrent access to a single image, which is
    /// referred to as a shared presentable image. The presentation engine is only required to update the current image after a new presentation request is
    /// received. Therefore the application must make a presentation request whenever an update is required. However, the presentation engine may update the
    /// current image at any point, meaning this mode may result in visible tearing.
    SharedDemandRefresh,
    /// SharedContinuousRefresh specifies that the presentation engine and application have concurrent access to a single image, which is referred to as a
    /// shared presentable image. The presentation engine periodically updates the current image on its regular refresh cycle. The application is only required
    /// to make one initial presentation request, after which the presentation engine must update the current image without any need for further presentation
    /// requests. The application can indicate the image contents have been updated by making a presentation request, but this does not guarantee the timing of
    /// when it will be updated. This mode may result in visible tearing if rendering to the image is not timed correctly.
    SharedContinuousRefresh,
}
#[derive(Debug)]
pub enum SurfaceFormat {
    B8G8R8A8UNorm,
    B8G8R8A8SRGB,
    R8G8B8A8UNorm,
    R8G8B8A8SRGB,
    R5G6B5UNormPack16,
}
#[derive(Debug)]
pub enum SurfaceColourSpace {
    SRGBNonlinear
}
#[derive(Debug)]
pub struct SwapchainExtent {
    min: vk::Extent2D,
    max: vk::Extent2D,
    width: u32,
    height: u32,
}

// Choosing a swapchain extent is capabilities.currentExtent is 32 MAX then we can pick our own
// Otherwise we need to simply return capabilities.currentExtent

impl SwapchainExtent {
    pub fn new(min_extent: &vk::Extent2D, max_extent: &vk::Extent2D) -> SwapchainExtent {
        SwapchainExtent {
            min: *min_extent,
            max: *max_extent,
            height: 0,
            width: 0,
        }
    }

    pub fn min_width(&self) -> u32 {
        self.min.width
    }

    pub fn min_height(&self) -> u32 {
        self.min.height
    }

    pub fn max_height(&self) -> u32 {
        self.max.height
    }

    pub fn max_width(&self) -> u32 {
        self.max.width
    }

    pub fn set_extent(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}