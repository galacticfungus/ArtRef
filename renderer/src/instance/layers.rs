use std::collections::HashMap;
use std::ffi::CStr;

use erupt::extensions;

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
    layers_to_enable: Vec<Layers>,
}

impl LayerManager {
    pub fn new(entry: &erupt::DefaultEntryLoader) -> LayerManager {
        let layers = unsafe { entry.enumerate_instance_layer_properties(None) }
            .expect("Failed to get list of layers");
        let mut available_layers = Vec::with_capacity(layers.len());
        for layer in layers {
            available_layers.push(layer.layer_name);
        }
        LayerManager {
            layers_to_enable: Vec::new(),
        }
    }

    pub fn add_layer(&mut self, layer_to_add: Layers) {
        // This can result in a dangling pointer if the &CStr does not have a life time that lasts longer than the LayerManager
        self.layers_to_enable.push(layer_to_add);
    }

    pub fn get_layers_to_load(self) -> Vec<Layers> {
        self.layers_to_enable
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum Layers {
    KhronosValidation,
    NvNsight,
    NvOptimus,
}

impl Layers {
    pub fn get_name(&self) -> &'static CStr {
        match self {
            Self::KhronosValidation => KhronosValidation::name(),
            Self::NvNsight => NvNsight::name(),
            Self::NvOptimus => NvOptimus::name(),
        }
    }
}
