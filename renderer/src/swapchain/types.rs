use ash::vk;

use super::{PresentMode, SurfaceFormat, SurfaceColourSpace};

impl std::convert::From<ash::vk::PresentModeKHR> for PresentMode {
    fn from(present_mode: vk::PresentModeKHR) -> Self {
        match present_mode {
            vk::PresentModeKHR::IMMEDIATE => PresentMode::Immediate,
            vk::PresentModeKHR::MAILBOX => PresentMode::Mailbox,
            vk::PresentModeKHR::FIFO => PresentMode::Fifo,
            vk::PresentModeKHR::FIFO_RELAXED => PresentMode::FifoRelaxed,
            vk::PresentModeKHR::SHARED_DEMAND_REFRESH => PresentMode::SharedDemandRefresh,
            vk::PresentModeKHR::SHARED_CONTINUOUS_REFRESH => PresentMode::SharedContinuousRefresh,
            _ => unreachable!("Unknown present mode found when converting a PresentModeKHR"),
        }
    }
}

impl std::convert::From<&PresentMode> for ash::vk::PresentModeKHR {
    fn from(present_mode: &PresentMode) -> Self {
        match present_mode {
            PresentMode::Immediate => vk::PresentModeKHR::IMMEDIATE,
            PresentMode::Mailbox => vk::PresentModeKHR::MAILBOX,
            PresentMode::Fifo => vk::PresentModeKHR::FIFO,
            PresentMode::FifoRelaxed => vk::PresentModeKHR::FIFO_RELAXED,
            PresentMode::SharedDemandRefresh => vk::PresentModeKHR::SHARED_DEMAND_REFRESH,
            PresentMode::SharedContinuousRefresh => vk::PresentModeKHR::SHARED_CONTINUOUS_REFRESH,
        }
    }
}

impl From<&SurfaceFormat> for vk::Format {
    fn from(format: &SurfaceFormat) -> vk::Format {
        match format {
            SurfaceFormat::B8G8R8A8UNorm => vk::Format::B8G8R8_UNORM,
            SurfaceFormat::B8G8R8A8SRGB => vk::Format::B8G8R8_SRGB,
            SurfaceFormat::R8G8B8A8UNorm => vk::Format::R8G8B8_UNORM,
            SurfaceFormat::R8G8B8A8SRGB => vk::Format::R8G8B8_SRGB,
            SurfaceFormat::R5G6B5UNormPack16 => vk::Format::R5G6B5_UNORM_PACK16,
        }
    }
}

impl From<&SurfaceColourSpace> for vk::ColorSpaceKHR {
    
    fn from(colour_space: &SurfaceColourSpace) -> vk::ColorSpaceKHR {
        match colour_space {
             SurfaceColourSpace::SRGBNonlinear => vk::ColorSpaceKHR::SRGB_NONLINEAR,
        }
    }
}