use super::{ErrorKind, InnerError, Error, DisplayDebug};

impl Error {
    pub fn new(kind: ErrorKind, source: Option<Error>) -> Error {
        Error {
            error: Box::new(InnerError::new(kind, source)),
        }
    }

    pub fn kind(&self) -> &ErrorKind {
        self.error.kind()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.error.kind))
    }
}

impl std::error::Error for Error {
    // TODO: Provide a non-trait version of this method that can return a render::Error
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl std::convert::From<erupt::vk1_0::Result> for Error {
    fn from(result: erupt::vk1_0::Result) -> Self {
        match result {
            erupt::vk1_0::Result::ERROR_INITIALIZATION_FAILED => Error::new(ErrorKind::InitializationFailed, None),
            erupt::vk1_0::Result::ERROR_OUT_OF_HOST_MEMORY => Error::new(ErrorKind::OutOfHostMemory, None),
            erupt::vk1_0::Result::ERROR_OUT_OF_DEVICE_MEMORY => Error::new(ErrorKind::OutOfDeviceMemory, None),
            erupt::vk1_0::Result::ERROR_LAYER_NOT_PRESENT => unreachable!("ERROR_LAYER_NOT_PRESENT must be converted in place as it has a parameter"),
            // Generic catch all Vulkan error
            error => Error::new(ErrorKind::VulkanApiError(error), None),
        }
    }
}