use ash::version::EntryV1_0;

use std::ffi::{CStr};

use super::Error;
#[feature(const_str_as_bytes)]
macro_rules! make_layer {
    ($layer_name:ident, $rust_id:ident) => {
        pub struct $rust_id;
        impl $rust_id {
            pub fn name() -> &'static ::std::ffi::CStr {
                ::std::ffi::CStr::from_bytes_with_nul(
                    concat!(stringify!($layer_name), "\0").as_bytes(),
                )
                .expect("Invalid layer string")
            }
        }
    };
}

make_layer!(VK_LAYER_NV_nsight, NvNsight);
make_layer!(VK_LAYER_NV_optimus, NvOptimus);
make_layer!(VK_LAYER_RENDERDOC_Capture, RenderdocCapture);
// The default and recommended validation layer to enable
make_layer!(VK_LAYER_KHRONOS_validation, KhronosValidation);
// Displays FPS
make_layer!(VK_LAYER_LUNARG_monitor, LunargMonitor);
// This layer will crash if VkTrace is not running
make_layer!(VK_LAYER_LUNARG_vktrace, VkTrace);

// TODO: Using an enum to load the appropriate layer would probably be better
// TODO: The ability to load a custom layer should be provided

pub struct LayerManager {
    available_layers: Vec<[i8; ash::vk::MAX_EXTENSION_NAME_SIZE]>,
    layers_to_enable: Vec<&'static CStr>,
}

impl LayerManager {
    pub fn new(entry: &ash::Entry) -> LayerManager {
        let layers = entry
            .enumerate_instance_layer_properties()
            .expect("Failed to get list of layers");
        let mut available_layers = Vec::with_capacity(layers.len());
        for layer in layers {
            available_layers.push(layer.layer_name);
        }
        LayerManager {
            available_layers,
            layers_to_enable: Vec::new(),
        }
    }

    pub fn add_layer(&mut self, layer_to_add: Layers) -> Result<(), Error> {
        // This can result in a dangling pointer if the &CStr does not have a life time that lasts longer than the LayerManager
        // All generated Vulkan layers part of this module have a lifetime of static
        let layer_name = layer_to_add.get_name();
        if self.is_available(layer_to_add) == false {
            return Err(Error::LayerNotFound(layer_name.to_owned()));
        }
        self.layers_to_enable.push(layer_name);
        Ok(())
    }

    pub fn is_available(&self, layer_name: Layers) -> bool {
        self.available_layers
            .iter()
            // This is safe since available_layer must be a c string and be valid for the scope of its use
            .map(|available_layer| unsafe { CStr::from_ptr(available_layer.as_ptr()) })
            .any(|available_layer| available_layer == layer_name.get_name())
    }

    pub fn get_layers_to_load(self) -> Vec<&'static CStr> {
        self.layers_to_enable
    }
}

pub enum Layers {
    KhronosValidation,
}

impl Layers {
    pub fn get_name(&self) -> &'static CStr {
        match self {
            Self::KhronosValidation => KhronosValidation::name(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_manager() {
        let entry = ash::Entry::new().expect("Failed to load Vulkan");
        let mng = LayerManager::new(&entry);
        assert!(mng.available_layers.len() > 0);
    }

    #[test]
    fn test_is_available() {
        let entry = ash::Entry::new().expect("Failed to load Vulkan");
        let mng = LayerManager::new(&entry);
        assert!(mng.is_available(Layers::KhronosValidation));
    }

    #[test]
    fn test_add_layer() {
        // TODO: This really doesn't test much, just pushing a value onto a vec
        let entry = ash::Entry::new().expect("Failed to load Vulkan");
        let mut mng = LayerManager::new(&entry);
        mng.add_layer(Layers::KhronosValidation).expect("Failed to add a layer");
        assert_eq!(mng.layers_to_enable.len(), 1);
        assert_eq!(mng.layers_to_enable[0], unsafe { CStr::from_ptr(KhronosValidation::name().as_ptr()) });
    }
}
