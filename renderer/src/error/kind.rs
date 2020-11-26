use super::ErrorKind;

impl PartialEq for ErrorKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // TODO: Macro for this
            // TODO: It may be easier to impl PartialEq for the types that are parameters, can't impl PartialEq for CString
            // TODO: Ideally we remove the need to compare these but users of the library probably want to check the results
            (ErrorKind::DeviceExtensionsNotFound(_),  ErrorKind::DeviceExtensionsNotFound(_)) => true,
            (ErrorKind::ExtensionNotFound(_),  ErrorKind::ExtensionNotFound(_)) => true,
            (ErrorKind::FailedToRecreateSurface,  ErrorKind::FailedToRecreateSurface) => true,
            (ErrorKind::InitializationFailed,  ErrorKind::InitializationFailed) => true,
            (ErrorKind::InstanceExtensionsNotFound(_),  ErrorKind::InstanceExtensionsNotFound(_)) => true,
            (ErrorKind::InvalidPipelineConfig,  ErrorKind::InvalidPipelineConfig) => true,
            (ErrorKind::LayerNotFound(_), ErrorKind::LayerNotFound(_)) => true,
            (ErrorKind::MissingFeature(_), ErrorKind::MissingFeature(_)) => true,
            (ErrorKind::MissingRequiredDeviceExtensions(_), ErrorKind::MissingRequiredDeviceExtensions(_)) => true,
            (ErrorKind::NoDevicesCanPresent, ErrorKind::NoDevicesCanPresent) => true,
            (ErrorKind::NoDevicesFound, ErrorKind::NoDevicesFound) => true,
            (ErrorKind::NoGraphicsQueue, ErrorKind::NoGraphicsQueue) => true,
            (ErrorKind::NoValidQueueFamily, ErrorKind::NoValidQueueFamily) => true,
            (ErrorKind::OutOfDeviceMemory, ErrorKind::OutOfDeviceMemory) => true,
            (ErrorKind::OutOfHostMemory, ErrorKind::OutOfHostMemory) => true,
            (ErrorKind::SurfaceLost, ErrorKind::SurfaceLost) => true,
            (ErrorKind::SwapchainConfigurationMissing, ErrorKind::SwapchainConfigurationMissing) => true,
            (ErrorKind::VulkanApiError(error_a), ErrorKind::VulkanApiError(error_b)) => error_a.eq(error_b),
            (ErrorKind::VulkanNotInstalled, ErrorKind::VulkanNotInstalled) => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            ErrorKind::ExtensionNotFound(extension_name) => {
                f.write_fmt(format_args!("Could not load the extension: {:?}", extension_name))
            },
            ErrorKind::InstanceExtensionsNotFound(extensions) => {
                f.write_fmt(format_args!("Could not load the following extensions: {:?}", extensions))
            },
            ErrorKind::DeviceExtensionsNotFound(extensions) => {
                f.write_fmt(format_args!("Could not load the following extensions: {:?}", extensions))
            },
            ErrorKind::LayerNotFound(layer_name) => {
                f.write_fmt(format_args!("Could not load the layer: {:?}", layer_name))
            },
            ErrorKind::VulkanApiError(api_error) => {
                f.write_fmt(format_args!("A Vulkan API call failed, the error was {}", api_error))
            },
            // TODO: Generalise this ie NoQueue(QueueType)
            ErrorKind::NoGraphicsQueue => {
                f.write_fmt(format_args!("While filtering devices no devices with a required graphics queue were found"))
            },
            ErrorKind::NoDevicesCanPresent => {
                f.write_fmt(format_args!("No devices can present to a surface"))
            },
            ErrorKind::FailedToRecreateSurface => {
                f.write_fmt(format_args!("The surface was lost and an attempt to recreate it failed"))
            },
            ErrorKind::MissingRequiredDeviceExtensions(devices_and_extensions) => {
                for (device, missing_extensions) in devices_and_extensions {
                    f.write_fmt(format_args!("{} was missing the extensions {:?}", device, missing_extensions))?;
                }
                Ok(())
            },
            ErrorKind::MissingFeature(feature) => {
                f.write_fmt(format_args!("Device was missing a required feature, feature was {}", feature))
            },
            // TODO: What?
            ErrorKind::NoValidQueueFamily => panic!("This error should only ever be a source error rather than propagated back to the client application"),
            ErrorKind::NoDevicesFound => {
                f.write_fmt(format_args!("No devices supporting Vulkan were found"))
            },
            ErrorKind::InitializationFailed => {
                f.write_fmt(format_args!("Failed to initialize Vulkan"))
            },
            ErrorKind::SwapchainConfigurationMissing => {
                // TODO: Can we pass more info here
                f.write_fmt(format_args!("ConfigureSwapchain was missing which is a required component"))
            },
            ErrorKind::InvalidPipelineConfig => {
                // TODO: Pipelines should have names associated with them
                // TODO: A better way of describing what went wrong
                f.write_fmt(format_args!("The shader being defined in the pipeline was invalid"))
            },
            ErrorKind::SurfaceLost => {
                f.write_fmt(format_args!("The surface that the renderer was using was lost, this should have been handled by the renderer internally."))
            },
            ErrorKind::OutOfHostMemory => {
                f.write_fmt(format_args!("Host is out of memory"))
            },
            ErrorKind::OutOfDeviceMemory => {
                f.write_fmt(format_args!("Device is out of memory."))
            },
            // TODO: Need to have the name of the actual device that failed, a reference is difficult because of lifetime issues
            ErrorKind::FailedToGetDeviceExtensions(device_name) => {
                if let Some(device_name) = device_name {
                    f.write_fmt(format_args!("Failed to get the device extensions for a device called {}", device_name))
                } else {
                    // TODO: Report on why the name could not be retrieved in the context?
                    f.write_fmt(format_args!("Failed to get the device extensions for a device, the devices name could not be retrieved"))
                }
                
            },
            ErrorKind::VulkanNotInstalled => f.write_fmt(format_args!("Failed to load the Vulkan library, is Vulkan not installed?")),
        }
    }
}