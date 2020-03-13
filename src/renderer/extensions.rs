use std::ffi::CStr;

pub struct ExtensionManager {
    // available_extensions: Vec<[i8; ash::vk::MAX_EXTENSION_NAME_SIZE]>,
    // extensions_status: HashMap<&'static CStr, bool>,
    extensions_to_load: Vec<&'static CStr>,
}

impl ExtensionManager {
    pub fn new() -> ExtensionManager {
        ExtensionManager {
            extensions_to_load: Vec::new(),
        }
    }

    pub fn add_extension(&mut self, extension_to_add: Extensions) -> () {
        self.extensions_to_load.push(extension_to_add.get_name());
    }

    pub fn get_extensions(self) -> Vec<&'static CStr> {
        self.extensions_to_load
    }
}

impl std::fmt::Debug for ExtensionManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        for extension_name in self.extensions_to_load.iter() {
            f.write_fmt(format_args!("{:?},", extension_name))?;
        }
        Ok(())
    }
}

pub enum Extensions {
    Surface,
    Win32Surface,
    Swapchain,
    DebugUtils,
}

impl Extensions {
    pub fn get_name(&self) -> &'static CStr {
        match self {
            Self::Surface => ash::extensions::khr::Surface::name(),
            Self::Win32Surface => ash::extensions::khr::Win32Surface::name(),
            Self::Swapchain => ash::extensions::khr::Swapchain::name(),
            Self::DebugUtils => ash::extensions::ext::DebugUtils::name(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_extension() {
        let mut em = ExtensionManager::new();
        em.add_extension(Extensions::Surface);
        let ExtensionManager { extensions_to_load } = em;
        assert!(extensions_to_load.contains(&ash::extensions::khr::Surface::name()));
    }
}