// use ash::version::{EntryV1_0};
// use ash::vk;
// use ash::vk_make_version;

use erupt::vk1_0 as vk;
use erupt::vk1_0::make_version as make_version;

use std::collections::{HashMap, HashSet};
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::{c_char};

use super::layers::LayerManager;
use super::{DeviceSelector, ExtensionManager, InstanceExtensions, Layers};
use crate::{ConfigureDevice, SelectedDevice};
use crate::error::{Error, ErrorKind, DisplayDebug};

pub struct VulkanConfig {
    entry: erupt::DefaultEntryLoader,
    api_version: u32,
    engine_version: u32,
    application_version: u32,
    application_name: Option<String>,
    engine_name: Option<String>,
    requested_extensions: HashMap<InstanceExtensions, bool>,
    available_extensions: Vec<vk::ExtensionProperties>,
    layers_to_load: HashMap<Layers, bool>,
    available_layers: Vec<vk::LayerProperties>,
}

impl VulkanConfig {
    pub fn new() -> Result<Self, Error> {
        use erupt::utils::loading::{EntryLoaderError, LibraryError};
        use std::fmt::Display;
        use std::error::Error as ErrorTrait;
        let entry = match erupt::EntryLoader::new() {
            
            Ok(entry) => entry,
            // These two error objects both have debug and display traits so perhaps I should include them as source or as context?
            // TODO: TO get better error information we need to override the default library loading code
            Err(EntryLoaderError::EntryLoad(_)) => return Err(Error::new(ErrorKind::InitializationFailed, None)),
            // This is an error from the libloading crate but accessing it from here is impossible as it has been wrapped in a LibraryError with private details
            Err(EntryLoaderError::Library(_)) => {
                return Err(Error::new(ErrorKind::VulkanNotInstalled, None));
            },
        };
        // Remember instance version differs from device version
        // let entry = Entry::new()?;
        // match entry.try_enumerate_instance_version()? {
        // Vulkan 1.1+
            // Some(version) => {
            //     let major = vk_version_major!(version);
            //     let minor = vk_version_minor!(version);
            //     let patch = vk_version_patch!(version);
            // },
        // Vulkan 1.0
        //     None => {},
        // }
        
        let available_extensions = unsafe {entry
            .enumerate_instance_extension_properties(None, None) }
            .expect("Failed to load list of extensions");
        // VK_ERROR_OUT_OF_HOST_MEMORY
        // VK_ERROR_OUT_OF_DEVICE_MEMORY
        // VK_ERROR_LAYER_NOT_PRESENT

        let available_layers = unsafe { entry.enumerate_instance_layer_properties(None) }.expect("Failed to load list of layers");
        // VK_ERROR_OUT_OF_HOST_MEMORY
        // VK_ERROR_OUT_OF_DEVICE_MEMORY

        let config = VulkanConfig {
            entry,
            application_name: None,
            engine_name: None,
            engine_version: 0,
            application_version: 0,
            api_version: make_version(1, 0, 0),
            requested_extensions: HashMap::new(),
            available_extensions,
            layers_to_load: HashMap::new(),
            available_layers,
        };
        Ok(config)
    }

    pub fn application_name<S: Into<String>>(mut self, app_name: S) -> Self {
        self.application_name = Some(app_name.into());
        self
    }

    pub fn engine_name<S: Into<String>>(mut self, engine_name: S) -> Self {
        self.engine_name = Some(engine_name.into());
        self
    }

    pub fn api_version(mut self, major: u32, minor: u32, patch: u32) -> Self {
        self.api_version = make_version(major, minor, patch);
        self
    }

    pub fn application_version(mut self, major: u32, minor: u32, patch: u32) -> Self {
        self.application_version = make_version(major, minor, patch);
        self
    }

    pub fn engine_version(mut self, major: u32, minor: u32, patch: u32) -> Self {
        self.engine_version = make_version(major, minor, patch);
        self
    }

    // call_with_one<F>(some_closure: F) -> i32
    // where F: Fn(i32) -> i32 {

    pub fn required_extensions<F>(mut self, required_extensions: F) -> Result<Self, Error>
    where
        F: Fn(&mut ExtensionManager<InstanceExtensions>) -> (),
    {
        
        let mut mng = ExtensionManager::new();
        required_extensions(&mut mng);
        // If any of the extensions couldn't be found return an error
        let requested_extensions = mng.get_extensions();
        let mut extensions_to_add: HashMap<InstanceExtensions, bool> = HashMap::with_capacity(requested_extensions.len());
        for extension in requested_extensions {
            if self.is_extension_available(&extension) {
                // TODO: Ensure these pointers are valid as they should point to static strings inside the ash library
                extensions_to_add.insert(extension, true);
            } else {
                extensions_to_add.insert(extension, false);
            }
        }
        if extensions_to_add.iter().any(|(_, present)| *present == false) {
            let missing_extensions: Vec<InstanceExtensions> = self
                .requested_extensions
                .iter()
                .filter(|(_, present)| **present == false)
                .map(|(ext, _)| ext.clone())
                .collect();
            return Err(Error::new(ErrorKind::InstanceExtensionsNotFound(missing_extensions), None));
        }
        
        self.requested_extensions.extend(extensions_to_add.into_iter());
        Ok(self)
    }
    /// Checks if a layer is available to Vulkan
    fn is_layer_available(&self, layer: &Layers) -> bool {
        self.available_layers.iter()
            // This is safe since the pointer is never left dangling and layer_name must be a c string
            .map(|layer| unsafe { CStr::from_ptr(layer.layer_name.as_ptr()) })
            .any(|layer_name| layer_name == layer.get_name())
    }

    fn is_extension_available(
        &self,
        extension: &InstanceExtensions,
    ) -> bool {
        for available_extension in self.available_extensions.iter().map(|ext| unsafe { CStr::from_ptr(ext.extension_name.as_ptr()) } ) {
            if available_extension == extension.get_name() {
                return true;
            }
        }
        false
    }

    pub fn optional_extensions<F>(mut self, optional_extensions: F) -> Self
    where
        F: Fn(&mut ExtensionManager<InstanceExtensions>) -> (),
    {
        let mut mng = ExtensionManager::new();
        optional_extensions(&mut mng);
        let requested_extensions = mng.get_extensions();
        for extension in requested_extensions {
            if self.is_extension_available(&extension) {
                self.requested_extensions.insert(extension, true);
            } else {
                self.requested_extensions.insert(extension, false);
            }
        }
        self
    }

    pub fn with_layers<F>(mut self, layers_to_load: F) -> Self
    where
        F: Fn(&mut LayerManager) -> (),
    {
        let mut mng = LayerManager::new(&self.entry);
        layers_to_load(&mut mng);
        let layers = mng.get_layers_to_load();
        for layer in layers {
            if self.is_layer_available(&layer) {
                // TODO: Ensure these pointers are valid as they should point to static strings inside the ash library
                self.layers_to_load.insert(layer, true);
            } else {
                self.layers_to_load.insert(layer, false);
            }
        }
        self
    }

    // Create an instance of the Vulkan API
    pub fn init(self) -> Result<VulkanApi, Error> {
        
        // TODO: These can be static references
        // Must be NULL or a c string
        // let c_app_name = match &self.application_name {
        //     Some(app_name) => CStr::from_bytes_with_nul(app_name.as_bytes()).expect(""),
        //     None => Default::default(),
        // };
        // Must be null or a C string
        let app_name = match &self.application_name {
                Some(app_name) => CString::new(app_name.as_str())
                    .expect("Failed to create a c type string from the application name"),
                None => CString::default(),
        };
        let engine_name = match &self.engine_name {
                Some(eng_name) => CString::new(eng_name.as_str())
                    .expect("Failed to create a c type string from the engin name"),
                None => CString::default(),
        };
        let app_info = erupt::vk1_0::ApplicationInfo {
            api_version: self.api_version,
            p_application_name: app_name.as_c_str().as_ptr(),
            p_engine_name: engine_name.as_c_str().as_ptr(),
            engine_version: self.engine_version,
            application_version: self.application_version,
            ..Default::default()
        };
        // TODO: Support required instance extensions and layers
        // TODO: Double check all handling of c strings
        // Convert the Extensions enum to raw pointers to c strings, only include extensions that are available
        let extensions_to_load: Vec<*const c_char> = self.requested_extensions
            .iter()
            .filter(|(_, &present)| present == true)
            .map(|(ext, _)| ext.get_name().as_ptr())
            .collect();
        // Filter layers that are not available
        let available_layers_to_load: Vec<*const c_char> = self.layers_to_load.iter()
            .filter(|(_, &present)| present == true)
            .map(|(layer, _)| layer.get_name().as_ptr())
            .collect();
        let create_info = erupt::vk1_0::InstanceCreateInfo {
            p_application_info: &app_info,
            pp_enabled_extension_names: extensions_to_load.as_ptr(),
            enabled_extension_count: extensions_to_load.len() as u32,
            pp_enabled_layer_names: available_layers_to_load.as_ptr(),
            enabled_layer_count: available_layers_to_load.len() as u32,
            ..Default::default()
        };
        //let instance = InstanceLoader::new(&entry, &create_info, None).unwrap();
        let instance = erupt::InstanceLoader::new(&self.entry, &create_info, None)
                .expect("Failed to create instance");
        // TODO: Handle possible errors
        
        // instance = Some(instance);
        let extensions_loaded = extensions_to_load.iter()
                                                    .map(|&ext| unsafe { CStr::from_ptr(ext) })
                                                    .collect::<HashSet<&'static CStr>>();
        // TODO: store loaded layers
        Ok(VulkanApi::new(self.entry, instance, extensions_loaded))
    }
}

pub struct VulkanApi {
    _entry: erupt::DefaultEntryLoader,
    instance: erupt::InstanceLoader,
    extensions_loaded: HashSet<&'static CStr>,
}

impl VulkanApi {
    pub fn new(entry: erupt::DefaultEntryLoader, instance: erupt::InstanceLoader, extensions_loaded: HashSet<&'static CStr>) -> VulkanApi {
        
        VulkanApi {
            _entry: entry, 
            instance, 
            extensions_loaded,
        }
    }

    pub fn extension_loaded(&self, extension: crate::InstanceExtensions) -> bool {
        self.extensions_loaded.contains(extension.get_name())
    }

    pub fn create_device_selector(&self, window: &winit::window::Window) -> Result<DeviceSelector, Error> {
        let surface = crate::presenter::create_surface(&self.instance, window);
        DeviceSelector::new(&self.instance, surface)    
    }

    pub fn configure_device(&self, selected_device: SelectedDevice) -> ConfigureDevice {
        // let SelectedDevice {api_version, device_handle, device_features, queue_families, driver_version, vendor_id, device_id, device_name, device_type, available_extensions} = selected_device;
        ConfigureDevice::new(&self.instance, selected_device)
    }
}

// impl Drop for VulkanApi {
//     fn drop(&mut self) {
//         unsafe { self.instance.destroy_instance(None) };
//     }
// }
