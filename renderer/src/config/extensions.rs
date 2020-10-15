use std::ffi::CStr;
#[derive(PartialEq, Eq, Hash, Clone)]
pub enum DeviceExtensions {
    Swapchain,
    IndirectCount,
}

impl DeviceExtensions {
    pub fn get_name(&self) -> &'static CStr {
        // SAFE: These are defined as static c strings and their type set to const * c_char, so it's safe to cast to CStr
        match self {
            Self::Swapchain => unsafe {CStr::from_ptr(erupt::extensions::khr_swapchain::KHR_SWAPCHAIN_EXTENSION_NAME)},
            Self::IndirectCount => unsafe {CStr::from_ptr(erupt::extensions::khr_draw_indirect_count::KHR_DRAW_INDIRECT_COUNT_EXTENSION_NAME)},
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

