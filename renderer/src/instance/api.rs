use ash::version::{EntryV1_0};
use ash::vk;
use ash::vk_make_version;

// use winit::{
//     dpi::{LogicalPosition, LogicalSize},
//     event::{Event, KeyboardInput, ScanCode, WindowEvent},
//     event_loop::{ControlFlow, EventLoop},
//     window::{Window, WindowBuilder},
// };

use std::collections::{HashMap, HashSet};
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::{c_char, c_void};

use super::layers::LayerManager;
use super::{DeviceSelector, VulkanDevice, Swapchain, ConfigureSwapchain, Surface, RenderDevice, ExtensionManager, InstanceExtensions, Layers};
use crate::ConfigureDevice;
use crate::Gpu;
use crate::error;

pub struct VulkanConfig {
    entry: ash::Entry,
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
    pub fn new() -> Self {
        let entry = ash::Entry::new().expect("Failed to load Vulkan");
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
        let available_extensions = entry
            .enumerate_instance_extension_properties()
            .expect("Failed to load list of extensions");
        let available_layers = entry.enumerate_instance_layer_properties().expect("Failed to load list of layers");
        VulkanConfig {
            entry,
            application_name: None,
            engine_name: None,
            engine_version: 0,
            application_version: 0,
            api_version: vk_make_version!(1, 0, 0),
            requested_extensions: HashMap::new(),
            available_extensions,
            layers_to_load: HashMap::new(),
            available_layers,
        }
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
        self.api_version = vk_make_version!(major, minor, patch);
        self
    }

    pub fn application_version(mut self, major: u32, minor: u32, patch: u32) -> Self {
        self.application_version = vk_make_version!(major, minor, patch);
        self
    }

    pub fn engine_version(mut self, major: u32, minor: u32, patch: u32) -> Self {
        self.engine_version = vk_make_version!(major, minor, patch);
        self
    }

    // call_with_one<F>(some_closure: F) -> i32
    // where F: Fn(i32) -> i32 {

    pub fn required_extensions<F>(mut self, required_extensions: F) -> Result<Self, error::Error>
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
                .into_iter()
                .filter(|(_, present)| *present == false)
                .map(|(ext, _)| ext)
                .collect();
            return Err(error::Error::InstanceExtensionsNotFound(missing_extensions));
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
                // TODO: Ensure these pointers are valid as they should point to static strings inside the ash library
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

    // Create an instance of Vulkan and begin device selection
    pub fn init(self) -> VulkanApi {
        let VulkanConfig {
            application_name,
            engine_name,
            application_version,
            engine_version,
            api_version,
            requested_extensions,
            layers_to_load,
            entry,
            .. // We no longer have any need of the available extensions
        } = self;
        // TODO: These can be static references
        // Must be NULL or a c string
        let c_app_name = match application_name {
            Some(app_name) => CString::new(app_name).expect("Failed to create c string"),
            None => CString::default(),
        };
        // Must be null or a C string
        let c_engine_name = match engine_name {
            Some(eng_name) => CString::new(eng_name).expect("Failed to create c string"),
            None => CString::default(),
        };
        let app_info = vk::ApplicationInfo {
            api_version,
            p_application_name: c_app_name.as_ptr(),
            p_engine_name: c_engine_name.as_ptr(),
            engine_version,
            application_version,
            ..Default::default()
        };
        // Convert the Extensions enum to raw pointers to c strings
        let extensions_to_load: Vec<*const c_char> = requested_extensions
            .iter()
            .filter(|(_, &present)| present == true)
            .map(|(ext, _)| ext.get_name().as_ptr())
            .collect();
        let available_layers_to_load: Vec<*const c_char> = layers_to_load.iter()
            .filter(|(_, &present)| present == true)
            .map(|(layer, _)| layer.get_name().as_ptr())
            .collect();
        let create_info = vk::InstanceCreateInfo {
            p_application_info: &app_info,
            pp_enabled_extension_names: extensions_to_load.as_ptr(),
            enabled_extension_count: extensions_to_load.len() as u32,
            pp_enabled_layer_names: available_layers_to_load.as_ptr(),
            enabled_layer_count: available_layers_to_load.len() as u32,
            ..Default::default()
        };
        let instance = unsafe {
            entry
                .create_instance(&create_info, None)
                .expect("Failed to create instance")
        };
        // TODO: Store the extensions and layers that were loaded
        let extensions_loaded = extensions_to_load.iter()
                                                    .map(|&ext| unsafe { CStr::from_ptr(ext) })
                                                    .collect::<HashSet<&'static CStr>>();
        VulkanApi { entry, instance, extensions_loaded }
    }
}

pub struct VulkanApi {
    entry: ash::Entry,
    instance: ash::Instance,
    extensions_loaded: HashSet<&'static CStr>,
}

impl VulkanApi {
    pub fn select_device(&self, surface: &mut Surface) -> Result<DeviceSelector, error::Error> {
        DeviceSelector::new(&self.instance, surface)
    }

    pub fn configure_device(&self, gpu: Gpu) -> Result<ConfigureDevice, error::Error> {
        let Gpu {api_version, device_handle, device_features, queue_families, driver_version, vendor_id, device_id, device_name, device_type, available_extensions, surface_capabilities, surface_formats, present_modes} = gpu;
        let config = ConfigureDevice::new(&self.instance, device_handle, queue_families, api_version, driver_version, vendor_id, device_id, device_name, device_type, available_extensions, device_features, surface_capabilities, surface_formats, present_modes);
        Ok(config)
    }

    pub fn extension_loaded(&self, extension: super::InstanceExtensions) -> bool {
        self.extensions_loaded.contains(extension.get_name())
    }

    // TODO: Swapping to a enum based solution will still require something like the following, this code is safe as long as the CStr is validated
    // pub fn raw_extension_loaded(&self, extension_name: &'static CStr) -> bool {
    //     // How do we store the extensions that were loaded, since all the strings are static we can simply store the pointer
    //     self.extensions_loaded.contains(extension_name)
    // }

    // TODO: Here we should recieve a Window and extract the hwnd and hinstance from there
    pub fn create_surface_win32(&self, hwnd: *const c_void, hinstance: *const c_void) -> Surface {
        Surface::new(&self.entry, &self.instance, hwnd, hinstance)
    }

    pub fn configure_swapchain<'a, 'b>(&self, device: &'a VulkanDevice, surface: Surface<'b>) -> ConfigureSwapchain<'a, 'b> {
        ConfigureSwapchain::new(&self.instance, device, surface)
    }

    pub fn create_renderer(&self, device: VulkanDevice, swapchain: Swapchain) -> RenderDevice {
        RenderDevice {

        }
    }
}

impl Drop for VulkanApi {
    fn drop(&mut self) {
        use ash::version::InstanceV1_0;
        unsafe { self.instance.destroy_instance(None) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layers() {
        let config = VulkanConfig::new()
            .api_version(1, 0, 0)
            .application_name("Bug")
            .application_version(1, 0, 0)
            .engine_version(1, 0, 0)
            .with_layers(|mng| {
                mng.add_layer(crate::Layers::KhronosValidation);
            });
        assert_eq!(config.layers_to_load.len(), 1);
        let result = config.layers_to_load[&super::super::layers::Layers::KhronosValidation];
        assert!(result);
    }

    #[test]
    fn test_required_extensions() {
        let config = VulkanConfig::new()
            .api_version(1, 0, 0)
            .application_name("Bug")
            .application_version(1, 0, 0)
            .engine_version(1, 0, 0)
            .required_extensions(|mng| {
                mng.add_extension(crate::InstanceExtensions::Win32Surface);
                mng.add_extension(crate::InstanceExtensions::Surface);
            })
            .expect("Failed to load extensions");

        // assert!(config.is_ok());
        assert_eq!(config.requested_extensions.len(), 2);
        let name_to_test = ash::extensions::khr::Win32Surface::name().as_ptr();
        // Requires a reference to a reference since normally this points to an array of pointers to static &CStr
        assert_eq!(config.requested_extensions.get(&crate::InstanceExtensions::Win32Surface), Some(&true));
        // println!("Loaded extensions are {:?}", config.requested_extensions);
    }

    #[test]
    fn test_optional_extensions() {
        let config = VulkanConfig::new()
            .api_version(1, 0, 0)
            .application_name("Test")
            .application_version(1, 0, 0)
            .engine_version(1, 0, 0)
            .optional_extensions(|mng| {
                mng.add_extension(crate::InstanceExtensions::Win32Surface);
            });
        assert_eq!(config.requested_extensions.len(), 1);
        let vulkan = config.init();
        assert!(vulkan.extension_loaded(InstanceExtensions::Win32Surface), true);
    }
}
