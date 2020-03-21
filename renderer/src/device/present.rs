use ash::vk;

use super::{PresentModeManager, PresentMode};

impl<'a> PresentModeManager<'a> {
    pub fn new(modes_picked: &'a mut Vec<PresentMode>) -> PresentModeManager {
        PresentModeManager {
            modes_picked,
        }
    }

    pub fn pick_mode(&mut self, mode: PresentMode) -> &mut Self {
        self.modes_picked.push(mode);
        self
    }
}

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

impl std::convert::From<PresentMode> for ash::vk::PresentModeKHR {
    fn from(present_mode: PresentMode) -> Self {
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