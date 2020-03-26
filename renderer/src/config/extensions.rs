use std::ffi::CStr;

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum DeviceExtensions {
    Swapchain,
    IndirectCount,
}

impl DeviceExtensions {
    pub fn get_name(&self) -> &'static CStr {
        match self {
            Self::Swapchain => ash::extensions::khr::Swapchain::name(),
            Self::IndirectCount => super::DrawIndirectCount::name(),
        }
    }
}

impl std::fmt::Display for DeviceExtensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        f.write_fmt(format_args!("{:?},", self.get_name()))
    }
}

impl std::fmt::Debug for DeviceExtensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_fmt(format_args!("{:?}, ", self.get_name()))
    }
}

