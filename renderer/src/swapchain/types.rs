use erupt::vk1_0 as vk;
use erupt::extensions::khr_surface as surface;

use super::{PresentMode, SurfaceFormat, SurfaceColourSpace};

impl std::convert::From<surface::PresentModeKHR> for PresentMode {
    fn from(present_mode: surface::PresentModeKHR) -> Self {
        match present_mode {
            surface::PresentModeKHR::IMMEDIATE_KHR => PresentMode::Immediate,
            surface::PresentModeKHR::MAILBOX_KHR => PresentMode::Mailbox,
            surface::PresentModeKHR::FIFO_KHR => PresentMode::Fifo,
            surface::PresentModeKHR::FIFO_RELAXED_KHR => PresentMode::FifoRelaxed,
            surface::PresentModeKHR::SHARED_DEMAND_REFRESH_KHR => PresentMode::SharedDemandRefresh,
            surface::PresentModeKHR::SHARED_CONTINUOUS_REFRESH_KHR => PresentMode::SharedContinuousRefresh,
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
            PresentMode::SharedContinuousRefresh => surface::PresentModeKHR::SHARED_CONTINUOUS_REFRESH_KHR,
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