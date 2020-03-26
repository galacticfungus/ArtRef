use std::ffi::CString;

use crate::{Features, InstanceExtensions, DeviceExtensions, Gpu, ConfigureSwapchain};

use ash::vk;

pub enum Error {
    ExtensionNotFound(CString),
    LayerNotFound(CString),
    InstanceExtensionsNotFound(Vec<InstanceExtensions>),
    DeviceExtensionsNotFound(Vec<DeviceExtensions>),
    NoGraphicsQueue,
    MissingRequiredDeviceExtensions(Vec<(Gpu, Vec<CString>)>),
    MissingFeature(Features),
    NoValidQueueFamily,
    // Represents an error returned by the Vulkan API
    VulkanApiError(vk::Result),
    FailedToRecreateSurface,
    NoDevicesCanPresent,
    NotPresentableDevice,
    NoDevicesFound,
    InitializationFailed,
    // TODO: This should probably take a ConfigureSwapchain but it proliferates its lifetime through multiple types
    SwapchainConfigurationMissing(&'static str),
}

// impl PartialEq for Error {

//     fn eq(&self, other: &Self) -> bool {
//         match self {
//             Error::ExtensionNotFound(name) =>
//         }
//     }
// }

impl<'a> std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Error::ExtensionNotFound(extension_name) => {
                f.write_fmt(format_args!("Could not load the extension: {:?}", extension_name))
            },
            Error::LayerNotFound(layer_name) => {
                f.write_fmt(format_args!("Could not load the layer: {:?}", layer_name))
            },
            Error::InstanceExtensionsNotFound(extensions) => {
                f.write_fmt(format_args!("Could not load the following extensions: {:?}", extensions))
            },
            Error::DeviceExtensionsNotFound(extensions) => {
                f.write_fmt(format_args!("Could not load the following extensions: {:?}", extensions))
            },
            Error::VulkanApiError(api_error) => {
                f.write_fmt(format_args!("A Vulkan API call failed, the error was {}", api_error))
            },
            Error::NoGraphicsQueue => {
                f.write_fmt(format_args!("No devices with a graphics queue were found"))
            },
            Error::NoDevicesCanPresent => {
                f.write_fmt(format_args!("No devices can present to a surface"))
            },
            Error::FailedToRecreateSurface => {
                f.write_fmt(format_args!("The surface was lost and an attempt to recreate it failed"))
            },
            Error::MissingRequiredDeviceExtensions(devices_and_extensions) => {
                for (device, missing_extensions) in devices_and_extensions {
                    f.write_fmt(format_args!("{} was missing the extensions {:?}", device, missing_extensions))?;
                }
                Ok(())
            },
            Error::MissingFeature(feature) => {
                f.write_fmt(format_args!("Device was missing a required feature, feature was {}", feature))
            },
            Error::NoValidQueueFamily => panic!("This error should only ever be a source error rather than propagated back to the client application"),
            Error::NotPresentableDevice => {
                f.write_fmt(format_args!("The device is can't present to the supplied surface and so is implicitly invalid"))
            },
            Error::NoDevicesFound => {
                f.write_fmt(format_args!("No devices supporting Vulkan were found"))
            },
            Error::InitializationFailed => {
                f.write_fmt(format_args!("Failed to initialize Vulkan"))
            },
            Error::SwapchainConfigurationMissing(config) => {
                f.write_fmt(format_args!("ConfigureSwapchain was missing {} which is a required component", config))
            }
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Error::ExtensionNotFound(extension_name) => {
                f.write_fmt(format_args!("Could not load the extension: {:?}", extension_name))
            },
            Error::InstanceExtensionsNotFound(extensions) => {
                f.write_fmt(format_args!("Could not load the following extensions: {:?}", extensions))
            },
            Error::DeviceExtensionsNotFound(extensions) => {
                f.write_fmt(format_args!("Could not load the following extensions: {:?}", extensions))
            },
            Error::LayerNotFound(layer_name) => {
                f.write_fmt(format_args!("Could not load the layer: {:?}", layer_name))
            },
            Error::VulkanApiError(api_error) => {
                f.write_fmt(format_args!("A Vulkan API call failed, the error was {}", api_error))
            },
            // TODO: Generalise this ie NoQueue(QueueType)
            Error::NoGraphicsQueue => {
                f.write_fmt(format_args!("While filtering devices no devices with a required graphics queue were found"))
            },
            Error::NoDevicesCanPresent => {
                f.write_fmt(format_args!("No devices can present to a surface"))
            },
            Error::FailedToRecreateSurface => {
                f.write_fmt(format_args!("The surface was lost and an attempt to recreate it failed"))
            },
            Error::MissingRequiredDeviceExtensions(devices_and_extensions) => {
                for (device, missing_extensions) in devices_and_extensions {
                    f.write_fmt(format_args!("{} was missing the extensions {:?}", device, missing_extensions))?;
                }
                Ok(())
            },
            Error::MissingFeature(feature) => {
                f.write_fmt(format_args!("Device was missing a required feature, feature was {}", feature))
            },
            Error::NoValidQueueFamily => panic!("This error should only ever be a source error rather than propagated back to the client application"),
            Error::NotPresentableDevice => {
                f.write_fmt(format_args!("The device is can't present to the supplied surface and so is implicitly invalid"))
            },
            Error::NoDevicesFound => {
                f.write_fmt(format_args!("No devices supporting Vulkan were found"))
            },
            Error::InitializationFailed => {
                f.write_fmt(format_args!("Failed to initialize Vulkan"))
            },
            Error::SwapchainConfigurationMissing(config) => {
                f.write_fmt(format_args!("ConfigureSwapchain was missing {} which is a required component", config))
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ExtensionNotFound(_) => None,
            Self::InstanceExtensionsNotFound(_) => None,
            Self::DeviceExtensionsNotFound(_) => None,
            Self::LayerNotFound(_) => None,
            Self::MissingRequiredDeviceExtensions(_) => None,
            Self::FailedToRecreateSurface => None,
            Self::NoGraphicsQueue => None,
            Self::MissingFeature(_) => None,
            Self::VulkanApiError(_) => None,
            Self::NoValidQueueFamily => None,
            Self::NoDevicesCanPresent => None,
            Error::NotPresentableDevice => None,
            Error::InitializationFailed => None,
            Error::NoDevicesFound => None,
            Error::SwapchainConfigurationMissing(_) => None,
        }
    }
}

impl From<vk::Result> for Error {
    fn from(result: vk::Result) -> Self {
        match result {
            vk::Result::ERROR_INITIALIZATION_FAILED => Error::InitializationFailed,
            error => Error::VulkanApiError(error),
        }
    }
}