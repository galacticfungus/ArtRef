pub struct Version {
    version: u32,
}

impl From<u32> for Version {
    fn from(value: u32) -> Self {
        Version {
            version: value,
        }
    }
}

impl Into<u32> for Version {
    fn into(self) -> u32 {
        self.version
    }
}

impl std::fmt::Debug for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}.{}.0", self.version >> 22, (self.version >> 12) & 0x3ff))
    }
}

impl Default for Version {
    fn default() -> Self {
        Version {
            version: erupt::vk1_0::make_version(1, 0, 0),
        }
    }
}