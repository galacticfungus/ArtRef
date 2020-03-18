#[derive(Clone, PartialEq)]
pub enum PciVendor {
    NVidia, // 0x10DE
    AMD,    // 0x1002
    Intel,  // 0x8086
    KhronosId(u32),
    Unknown(u32),
    Default,
}

impl From<u32> for PciVendor {
    fn from(value: u32) -> Self {
        // TODO: Deal with Khronos ID's properly
        // Low 16 bits for vendor ID's - Khronos ID's start at 0x10000
        if value > 0x10000 {
            return PciVendor::KhronosId(value);
        } else {
            match value {
                0x1002 => PciVendor::AMD,
                0x8086 => PciVendor::Intel,
                0x10DE => PciVendor::NVidia,
                _ => PciVendor::Unknown(value),
            }
        }
    }
}

impl std::default::Default for PciVendor {
    fn default() -> Self { 
        PciVendor::Default
    }
}

impl std::fmt::Display for PciVendor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            PciVendor::AMD => f.write_str("Advanced Micro Devices Inc"),
            PciVendor::NVidia => f.write_str("NVidia"),
            PciVendor::Intel => f.write_str("Intel"),
            PciVendor::Unknown(value) => {
                f.write_fmt(format_args!("Unknown - Vendor ID: {:#x}", value))
            }
            PciVendor::KhronosId(id) => f.write_fmt(format_args!("Khronos ID: {:#x}", id)),
            PciVendor::Default => f.write_fmt(format_args!("Default Test Vendor")),
            
        }
    }
}
