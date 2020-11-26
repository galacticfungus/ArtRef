use super::{DisplayDebug, Context};

impl Context {
    pub fn new() -> Context {
        Context {
            display: None,
            debug: None,
        }
    }

    pub fn add_display_context(&mut self, context: &'static (dyn std::fmt::Display + Send + Sync + 'static)) {
        self.display = Some(context);
    }

    pub fn add_debug_context(&mut self, context: &'static (dyn std::fmt::Debug + Send + Sync + 'static)) {
        self.debug = Some(context);
    }

    pub fn add_context(&mut self, context: &'static (dyn DisplayDebug + Send + Sync + 'static)) {
        self.display = Some(context.as_display());
        self.debug = Some(context.as_debug());
    }
}

impl std::fmt::Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(display) = self.display {
            display.fmt(f)?;
        }
        Ok(())
    }
}

impl std::fmt::Debug for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(debug) = self.debug {
            debug.fmt(f)?;
        }
        Ok(())
    }
}

impl<T: std::fmt::Display + std::fmt::Debug + Send + Sync + 'static> DisplayDebug for T {
    fn as_debug(&self) -> & (dyn std::fmt::Debug + Send + Sync) {
        self
    }

    fn as_display(&self) -> & (dyn std::fmt::Display + Send + Sync) {
        self
    }
}