pub struct ExtensionManager<T> {
    extensions_to_load: Vec<T>,
}

// TODO: Extension Manager needs to be able to support checking if an extension is available
// TODO: Both an interface trait as well as a generic trait

impl<T> ExtensionManager<T> {
    pub fn new() -> ExtensionManager<T> {
        ExtensionManager {
            extensions_to_load: Vec::new(),
        }
    }

    pub fn add_extension(&mut self, extension_to_add: T) -> () {
        self.extensions_to_load.push(extension_to_add);
    }

    pub fn get_extensions(self) -> Vec<T> {
        self.extensions_to_load
    }
}

impl<T> std::fmt::Debug for ExtensionManager<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        for extension in self.extensions_to_load.iter() {
            f.write_fmt(format_args!("{:?},", extension))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::InstanceExtensions;

    #[test]
    fn test_add_extension() {
        let mut em = ExtensionManager::new();
        em.add_extension(InstanceExtensions::Surface);
        let ExtensionManager { extensions_to_load } = em;
        assert!(extensions_to_load.contains(&InstanceExtensions::Surface));
    }
}
