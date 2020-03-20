#[derive(Clone, PartialEq)]
pub enum PciVendor {
    NVidia, // 0x10DE
    AMD,    // 0x1002
    Intel,  // 0x8086
    KhronosId(KhronosVendor),
    Unknown(u32),
    Default,
}

#[derive(Clone, PartialEq)]
pub enum KhronosVendor {
    Vivante,
    VeriSilicon,
    KazanSoftwareRenderer,
    CodePlay,
    Unknown(u32),
}

impl From<u32> for KhronosVendor {
    fn from(value: u32) -> Self {
        match value {
            0x10001 => KhronosVendor::Vivante,
            0x10002 => KhronosVendor::VeriSilicon,
            0x10003 => KhronosVendor::KazanSoftwareRenderer,
            0x10004 => KhronosVendor::CodePlay,
            value => KhronosVendor::Unknown(value),
        }
    }
}

impl std::fmt::Display for KhronosVendor {    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            KhronosVendor::Vivante => f.write_str("Vivante "),
            KhronosVendor::VeriSilicon => f.write_str("VeriSilicon"),
            KhronosVendor::KazanSoftwareRenderer => f.write_str("Kazan Software Renderer"),
            KhronosVendor::CodePlay => f.write_str("Codeplay Software Ltd"),
            KhronosVendor::Unknown(value) => f.write_fmt(format_args!("Unknown Khronos Vendor: {:#x}", value)),
        }
    }
}

impl From<u32> for PciVendor {
    fn from(value: u32) -> Self {
        // Low 16 bits for vendor ID's - Khronos ID's start at 0x10000
        if value > 0x10000 {
            return PciVendor::KhronosId(KhronosVendor::from(value));
        } else {
            // These values match PCI Company ID's
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
            PciVendor::KhronosId(id) => f.write_fmt(format_args!("Khronos ID: {}", id)),
            PciVendor::Default => f.write_fmt(format_args!("Default Test Vendor")),
            
        }
    }
}
