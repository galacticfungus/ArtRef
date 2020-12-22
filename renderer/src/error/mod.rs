use crate::{DeviceExtensions, Features, Gpu, InstanceExtensions};
use std::ffi::CString;

mod context;
mod error;
mod inner;
mod kind;

/// Represents an error returned by the renderer
#[derive(Debug)]
pub struct Error {
    error: Box<InnerError>, // The internals are boxed to minimise the size on the stack
}

pub struct InnerError {
    kind: ErrorKind,
    /// Provides context on where this error occurred
    context: Context,
    // The underlying error if there is one
    // TODO: No need to store the underlying trait object
    source: Option<Error>,
}

// TODO: Rework this, context belongs to a group of errors or at least can instead of just one error return
pub struct Context {
    display: Option<&'static (dyn std::fmt::Display + Send + Sync + 'static)>,
    debug: Option<&'static (dyn std::fmt::Debug + Send + Sync + 'static)>,
}

pub trait DisplayDebug: std::fmt::Display + std::fmt::Debug + Send + Sync {
    fn as_display(&self) -> &(dyn std::fmt::Display + Send + Sync);
    fn as_debug(&self) -> &(dyn std::fmt::Debug + Send + Sync);
}

#[derive(Debug, Clone)]
pub enum ErrorKind {
    OutOfHostMemory,
    OutOfDeviceMemory,
    ExtensionNotFound(CString),
    /// This error is issued whenever a request for an operation included an invalid reference to a layer, ie when creating a vulkan instance or when retrieving a list of device extensions
    LayerNotFound(CString),
    InstanceExtensionsNotFound(Vec<InstanceExtensions>),
    DeviceExtensionsNotFound(Vec<DeviceExtensions>),
    NoGraphicsQueue,
    // TODO: TO derive partialeq we need to wrap this vec in a structure
    MissingRequiredDeviceExtensions(Vec<(Gpu, Vec<CString>)>),
    MissingFeature(Features),
    NoValidQueueFamily,
    // Represents an error returned by the Vulkan API
    VulkanApiError(erupt::vk1_0::Result),
    FailedToRecreateSurface,
    NoDevicesCanPresent,
    NoDevicesFound,
    /// You're unlikely to see this error as its handled by the renderer
    SurfaceLost,
    InitializationFailed,          // Vulkan initialization failed
    SwapchainConfigurationMissing, // TODO: Can we pass Swapchain Configuration here?
    InvalidPipelineConfig, // Is the best we can do a context message about what was misconfigured
    FailedToGetDeviceExtensions(Option<String>),
    VulkanNotInstalled,
    InvalidShaderEntryMethodName(Vec<u8>),
}
