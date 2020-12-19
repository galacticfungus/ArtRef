use erupt::vk1_0 as vk;
use std::collections::HashMap;
use std::ffi::CStr;
// use super::Gpu;
use super::{
    ConfigureDevice, DeviceExtensions, ExtensionManager, Feature, Features, PciVendor, QueueFamily,
    QueueManager,
};
use crate::error::{Error, ErrorKind};
use crate::{Version, VulkanDevice, SelectedDevice};

// Notes from Nvidia: Don’t overlap compute work on the graphics queue with compute work on a
// dedicated asynchronous compute queue. This may lead to gaps in execution of the
// asynchronous compute queue

impl<'a> ConfigureDevice<'a> {
    pub fn new(
        instance: &'a erupt::InstanceLoader,
        selected_device: SelectedDevice,
    ) -> ConfigureDevice {
        let SelectedDevice {api_version, device_handle, device_features, queue_families, driver_version, vendor_id, device_id, device_name, device_type, available_extensions} = selected_device;
        
        ConfigureDevice {
            instance,
            device_handle,
            available_queues: queue_families,
            render_queues: None,
            api_version: Version::from(api_version),
            driver_version,
            vendor_id,
            device_id,
            device_name,
            device_type,
            // device extensions
            available_device_extensions: available_extensions,
            extensions_to_load: HashMap::new(),
            device_features,
            enabled_features: vk::PhysicalDeviceFeatures::default(),
            queues_to_create: Vec::new(),
        }
    }

    // Retrieve a handle to a feature to either check its availability or enable that feature
    pub fn feature(&mut self, feature: &Features) -> Feature {
        match feature {
            Features::GeometryShader => Feature::new(
                self.device_features.geometry_shader > 0,
                &mut self.enabled_features.geometry_shader,
            ),
            Features::TesselationShader => Feature::new(
                self.device_features.tessellation_shader > 0,
                &mut self.enabled_features.tessellation_shader,
            ),
        }
    }

    // Will enable a device feature or return an error
    pub fn enable_feature(mut self, requested_feature: Features) -> Result<Self, Error> {
        let gpu_feature = self.feature(&requested_feature);
        if gpu_feature.is_available() {
            gpu_feature.enable();
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::MissingFeature(requested_feature), None))
        }
    }

    // Will see if a feature can be enabled and enable it if it is supported
    pub fn try_enable_feature(mut self, feature: Features) -> Self {
        let feature = self.feature(&feature);
        feature.enable_if_able();
        self
    }

    pub fn extensions_to_load<F>(mut self, select_extensions: F) -> Self
    where
        F: Fn(&mut ExtensionManager<DeviceExtensions>) -> (),
    {
        let mut mng = ExtensionManager::new();
        select_extensions(&mut mng);
        let extensions_to_load = mng.get_extensions();
        // Add the extensions selected to the list of extensions to load marking whether that extension is available
        for extension in extensions_to_load {
            if self.is_extension_available(&extension) {
                self.extensions_to_load.insert(extension, true);
            } else {
                self.extensions_to_load.insert(extension, false);
            }
        }
        self
    }

    fn is_extension_available(&self, extension: &DeviceExtensions) -> bool {
        // self.available_extensions.iter().map(|ext| unsafe { CStr::from_ptr(ext.extension_name.as_ptr()) } ).any(|ext_name| ext_name == extension.get_name())

        for available_extension in self.available_device_extensions.iter() {
            let available_name =
                unsafe { CStr::from_ptr(available_extension.extension_name.as_ptr()) };
            if available_name == extension.get_name() {
                return true;
            }
        }
        println!("Exension not available");
        false
    }

    // This function will return an error when a queue is requested that is not available
    pub fn define_queues<F>(mut self, get_queues_to_create: F) -> Result<Self, Error>
    where
        F: Fn(&mut QueueManager) -> (),
    {
        let mut qm = QueueManager::new(self.available_queues.as_slice());
        get_queues_to_create(&mut qm);
        let QueueManager {
            queues_to_create,
            render_queues,
            ..
        } = qm;
        println!("Queues to create: {:?}", queues_to_create);
        self.queues_to_create = queues_to_create
            .into_iter()
            .map(|(_, queue)| queue)
            .collect();
        self.render_queues = Some(render_queues.build());
        Ok(self)
    }

    pub fn create_device(self) -> VulkanDevice {
        let mut queues_to_submit = Vec::new();
        println!("Creating Queues");
        use std::convert::TryFrom;
        for queue_to_create in self.queues_to_create.as_slice().iter() {
            println!("Processing {:?}", queue_to_create);
            // TODO: Ensure that family_index can fit in a u32
            // TODO: Ensure that queue_count can fit in a u32
            let family_index = u32::try_from(queue_to_create.family_index())
                .expect("Family index exceeded the max u32 value");
            let queue_info = vk::DeviceQueueCreateInfo {
                p_queue_priorities: queue_to_create.priorities().as_ptr(),
                queue_family_index: family_index,
                queue_count: queue_to_create.reserved_queues(),
                ..Default::default()
            };
            queues_to_submit.push(queue_info);
        }
        println!("Submitting Queues: {:?}", queues_to_submit);
        // The queue map lets us map from creation index to queue index
        let device_extensions: Vec<*const std::os::raw::c_char> = self
            .extensions_to_load
            .iter()
            .filter(|(_, &present)| present == true)
            .map(|(ext, _)| ext.get_name().as_ptr())
            .collect();
        let create_info = vk::DeviceCreateInfo {
            enabled_extension_count: device_extensions.len() as u32,
            pp_enabled_extension_names: device_extensions.as_ptr(),
            p_enabled_features: &self.enabled_features,
            queue_create_info_count: queues_to_submit.len() as u32,
            p_queue_create_infos: queues_to_submit.as_ptr(),
            ..Default::default()
        };
        println!("Creating Device using: {:?}", create_info);
        // This should be safe as all data structures are in scope and there are no user parameters
        // let device = unsafe { self.instance.create_device(self.device_handle, &create_info, None, None) }
        //     .expect("Failed to create device");
        let device =
            erupt::DeviceLoader::new(&self.instance, self.device_handle, &create_info, None)
                .expect("Failed to create device");
        // We no longer need access to the vector of DeviceQueues as they are only used to create the queues
        // Instead we can get information about the queues using the RenderQueues object
        let render_queues = self
            .render_queues
            .expect("Failed to create the RenderQueues");

        VulkanDevice::new(
            self.device_handle,
            render_queues,
            self.enabled_features,
            self.extensions_to_load,
            device,
            self.vendor_id,
            self.device_id,
            self.api_version,
            self.driver_version,
            self.device_name,
        )
    }
}

impl<'a> std::fmt::Debug for ConfigureDevice<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Create a VulkanVersion struct
        fmt.debug_struct("DeviceConfigure")
            .field("available_extensions", &self.available_device_extensions)
            .field("api_version", &self.api_version)
            // Safe since device_name must be a CStr and the string itself will always be valid for the lifetime of the pointer
            .field("device_name", unsafe {
                &CStr::from_ptr(self.device_name.as_ptr())
            })
            .field("device_features", &self.device_features)
            .field("device_type", &self.device_type)
            .field("driver_version", &self.driver_version)
            .field("enabled_features", &self.enabled_features)
            .finish()
    }
}

// impl<'a> From<crate::SelectedDevice> for ConfigureDevice<'a> {
//     fn from(selected_device: crate::SelectedDevice) -> Self {
//         use crate::SelectedDevice;
//         let SelectedDevice {
//             device_type,
//             device_handle,
//             queue_families,
//             api_version,
//             driver_version,
//             vendor_id,
//             device_id,
//             device_name,
//             available_extensions,
//             device_features,
//         } = selected_device;

//         ConfigureDevice {
//             device_type,
//             device_handle,
//             available_queues: queue_families,
//             api_version: Version::from(api_version),
//             driver_version,
//             vendor_id,
//             device_id,
//             device_name,
//             available_device_extensions: available_extensions,
//             device_features,
//             extensions_to_load: HashMap::new(),
//             enabled_features: erupt::vk1_0::PhysicalDeviceFeatures::default(),
//             queues_to_create: Vec::new(),
//             render_queues: None,
//             instance: 
//         }
//     }
// }
