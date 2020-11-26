use super::{ErrorKind, InnerError, Error, DisplayDebug};



// TODO: impl Box<Error> for Error - ie just return Box<InnerError>
impl Error {
    pub fn new(kind: ErrorKind, source: Option<Error>) -> Error {
        Error {
            error: Box::new(InnerError::new(kind, source)),
        }
    }

    pub fn kind(&self) -> &ErrorKind {
        self.error.kind()
    }

    pub fn with_display_context(mut self, context: &'static (dyn std::fmt::Display + Send + Sync + 'static)) -> Self {
        self.error.add_display_context(context);
        self
    }

    pub fn with_debug_context(mut self, context: &'static (dyn std::fmt::Debug + Send + Sync + 'static)) -> Self {
        self.error.add_debug_context(context);
        self
    }

    pub fn with_context(mut self, context: &'static (dyn DisplayDebug + Send + Sync)) -> Self {
        self.error.add_context(context);
        self
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
        let data = self.error.source();
        // If there is an error return it
        if let Some(error) = data {
            // We can't directly cast to a Optional trait object only a trait object
            // so we need to remove the option then cast
            // TODO: Trait not implemented for a reference type
            // Dereference the box and return a pointer to a trait object
            // let h = error.as_ref();
            let error_trait = error as &(dyn std::error::Error + 'static);
            return Some(error_trait);
        }
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