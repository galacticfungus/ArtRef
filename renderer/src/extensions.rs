use std::ffi::CStr;
use ash::vk;

pub struct ExtensionManager {
    extensions_to_load: Vec<Extensions>,
}

// TODO: Extension Manager needs to be able to support checking if an extension is available
// TODO: Both an interface trait as well as a generic trait

impl ExtensionManager {
    pub fn new() -> ExtensionManager {
        ExtensionManager {
            extensions_to_load: Vec::new(),
        }
    }

    pub fn add_extension(&mut self, extension_to_add: Extensions) -> () {
        self.extensions_to_load.push(extension_to_add);
    }

    pub fn get_extensions(self) -> Vec<Extensions> {
        self.extensions_to_load
    }
}

impl std::fmt::Debug for ExtensionManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        for extension in self.extensions_to_load.iter() {
            f.write_fmt(format_args!("{},", extension))?;
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
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

impl std::fmt::Display for Extensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        f.write_fmt(format_args!("{:?},", self.get_name()))
    }
}

impl std::fmt::Debug for Extensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_fmt(format_args!("{:?}, ", self.get_name()))
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
        assert!(extensions_to_load.contains(&Extensions::Surface));
    }
}