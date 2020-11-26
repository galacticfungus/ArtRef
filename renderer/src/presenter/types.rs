use erupt::vk1_0 as vk;
use erupt::extensions::khr_surface as surface;
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
    SRGBNonlinear,
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

pub struct SwapchainImageCount {
    min_images: u32,
    max_images: u32,
    image_count: u32,
}

impl SwapchainImageCount {
    pub fn new(min_images: u32, max_images: u32) -> SwapchainImageCount {
        SwapchainImageCount {
            min_images,
            max_images,
            image_count: 0,
        }
    }

    pub fn min_images(&self) -> u32 {
        self.min_images
    }

    pub fn max_images(&self) -> u32 {
        self.max_images
    }

    pub fn set_image_count(&mut self, image_count: u32) {
        self.image_count = image_count;
    }

    pub fn image_count(image_count: SwapchainImageCount) -> u32 {
        image_count.image_count
    }
}

impl std::convert::From<surface::PresentModeKHR> for PresentMode {
    fn from(present_mode: surface::PresentModeKHR) -> Self {
        match present_mode {
            surface::PresentModeKHR::IMMEDIATE_KHR => PresentMode::Immediate,
            surface::PresentModeKHR::MAILBOX_KHR => PresentMode::Mailbox,
            surface::PresentModeKHR::FIFO_KHR => PresentMode::Fifo,
            surface::PresentModeKHR::FIFO_RELAXED_KHR => PresentMode::FifoRelaxed,
            surface::PresentModeKHR::SHARED_DEMAND_REFRESH_KHR => PresentMode::SharedDemandRefresh,
            surface::PresentModeKHR::SHARED_CONTINUOUS_REFRESH_KHR => {
                PresentMode::SharedContinuousRefresh
            }
            _ => unreachable!("Unknown present mode found when converting a PresentModeKHR"),
        }
    }
}

impl std::convert::From<&PresentMode> for erupt::extensions::khr_surface::PresentModeKHR {
    fn from(present_mode: &PresentMode) -> Self {
        match present_mode {
            PresentMode::Immediate => surface::PresentModeKHR::IMMEDIATE_KHR,
            PresentMode::Mailbox => surface::PresentModeKHR::MAILBOX_KHR,
            PresentMode::Fifo => surface::PresentModeKHR::FIFO_KHR,
            PresentMode::FifoRelaxed => surface::PresentModeKHR::FIFO_RELAXED_KHR,
            PresentMode::SharedDemandRefresh => surface::PresentModeKHR::SHARED_DEMAND_REFRESH_KHR,
            PresentMode::SharedContinuousRefresh => {
                surface::PresentModeKHR::SHARED_CONTINUOUS_REFRESH_KHR
            }
        }
    }
}

impl From<&SurfaceFormat> for erupt::vk1_0::Format {
    fn from(format: &SurfaceFormat) -> erupt::vk1_0::Format {
        match format {
            SurfaceFormat::B8G8R8A8UNorm => erupt::vk1_0::Format::B8G8R8A8_UNORM,
            SurfaceFormat::B8G8R8A8SRGB => erupt::vk1_0::Format::B8G8R8A8_SRGB,
            SurfaceFormat::R8G8B8A8UNorm => erupt::vk1_0::Format::R8G8B8A8_UNORM,
            SurfaceFormat::R8G8B8A8SRGB => erupt::vk1_0::Format::R8G8B8A8_SRGB,
            SurfaceFormat::R5G6B5UNormPack16 => erupt::vk1_0::Format::R5G6B5_UNORM_PACK16,
        }
    }
}

impl From<&SurfaceColourSpace> for surface::ColorSpaceKHR {
    fn from(colour_space: &SurfaceColourSpace) -> surface::ColorSpaceKHR {
        match colour_space {
            SurfaceColourSpace::SRGBNonlinear => surface::ColorSpaceKHR::SRGB_NONLINEAR_KHR,
        }
    }
}