use super::{ErrorKind, InnerError, Error, DisplayDebug, Context};

impl InnerError {
    pub fn new(kind: ErrorKind, source: Option<Error>) -> InnerError {
        
        InnerError {
            kind,
            context: Context::new(),
            source,
        }
    }

    pub fn add_context(&mut self, context: &'static (dyn DisplayDebug + Send + Sync)) {
        self.context.add_context(context);
    }

    pub fn add_debug_context(&mut self, context: &'static (dyn std::fmt::Debug + Send + Sync)) {
        self.context.add_debug_context(context);
    }

    pub fn add_display_context(&mut self, context: &'static (dyn std::fmt::Display + Send + Sync)) {
        self.context.add_display_context(context);
    }
    
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    // TODO: There is no need to store the source as a trait object we can just cast to the appropriate type
    pub fn source(&self) -> Option<&Error> {
        self.source.as_ref()

    }
}

impl std::fmt::Display for InnerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.kind))?;
        self.context.fmt(f)
    }
}

impl std::fmt::Debug for InnerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.kind))?;
        self.context.fmt(f)
    }
}