use super::{ErrorKind, InnerError, Error, DisplayDebug, Context};

impl InnerError {
    pub fn new(kind: ErrorKind, source: Option<Error>) -> InnerError {
        
        InnerError {
            kind,
            context: Context::new(),
            source,
        }
    }
    
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
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