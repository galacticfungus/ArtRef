#[derive(Clone, Copy)]
pub enum Features {
    /// Enables the geoetry shader stage in the graphis pipeline
    GeometryShader,
    /// Enables the tesselation shader stage in the graphics pipeline
    TesselationShader,
}

impl std::fmt::Debug for Features {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Features::GeometryShader => f.write_str("Geometry Shader"),
            Features::TesselationShader => f.write_str("Tesselation Shader"),
        }
    }
}

impl std::fmt::Display for Features {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Features::GeometryShader => f.write_str("Geometry Shader"),
            Features::TesselationShader => f.write_str("Tesselation Shader"),
        }
    }
}

pub struct Feature<'a> {
    is_available: bool,
    enabled: &'a mut u32,
}

impl<'a> Feature<'a> {
    // Initialise a feature, first value is a boolean that
    pub fn new(is_available: bool, ptr_to_enabled: &'a mut u32) -> Feature<'a> {
        Feature {
            is_available: is_available,
            enabled: ptr_to_enabled,
        }
    }
}

impl<'a> Feature<'a> {
    // Returns true if the feature is available
    pub fn is_available(&self) -> bool {
        self.is_available
    }

    // Enable the feature
    pub fn enable(self) {
        *self.enabled = 1;
    }

    // If the feature is supported enable it
    pub fn enable_if_able(self) {
        if self.is_available {
            self.enable();
        }
    }
}
