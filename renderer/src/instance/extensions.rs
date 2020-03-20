use std::ffi::CStr;





#[derive(PartialEq, Eq, Hash, Clone)]
pub enum InstanceExtensions {
    Surface,
    Win32Surface,
    DebugUtils,
}

impl InstanceExtensions {
    pub fn get_name(&self) -> &'static CStr {
        match self {
            Self::Surface => ash::extensions::khr::Surface::name(),
            Self::Win32Surface => ash::extensions::khr::Win32Surface::name(),
            Self::DebugUtils => ash::extensions::ext::DebugUtils::name(),
        }
    }
}



impl std::fmt::Display for InstanceExtensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        f.write_fmt(format_args!("{:?},", self.get_name()))
    }
}

impl std::fmt::Debug for InstanceExtensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_fmt(format_args!("{:?}, ", self.get_name()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ExtensionManager;
    #[test]
    fn test_add_extension() {
        let mut em = ExtensionManager::new();
        em.add_extension(InstanceExtensions::Surface);
        // let ExtensionManager { extensions_to_load } = em;
        let extensions_to_load = em.get_extensions();
        assert!(extensions_to_load.contains(&InstanceExtensions::Surface));
    }
}