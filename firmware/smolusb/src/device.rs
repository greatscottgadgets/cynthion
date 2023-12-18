///! `smolusb` device types

/// USB Speed
///
/// Note: These match UTMI xcvr_select constant so the mapping may not be correct for other contexts.
///       See: https://github.com/greatscottgadgets/luna/blob/main/luna/gateware/usb/usb2/__init__.py
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum Speed {
    /// High speed (480 Mbps)
    High = 0,
    /// Full speed (12 Mbps)
    Full = 1,
    /// Low speed (1.5 Mbps)
    Low = 2,
    /// Super Speed (5/10 Gbps - includes SuperSpeed+)
    Super = 3,
    /// unsupported: compatibility fallback
    SuperPlus = 4,
    /// unsupported: compatibility fallback
    Unknown = 0xff,
}

impl From<u8> for Speed {
    fn from(value: u8) -> Self {
        match value & 0b11 {
            0 => Speed::High, // gateware gives 1
            1 => Speed::Full,
            2 => Speed::Low,
            3 => Speed::Super,
            _ => unimplemented!(),
        }
    }
}

impl Speed {
    /// Convert from a libusb speed constant to smolusb
    ///
    /// See: https://github.com/libusb/libusb/blob/6bf2db6feaf3b611c9adedb6c4962a07f5cb07ae/libusb/libusb.h#L1126
    pub fn from_libusb(value: u8) -> Self {
        match value {
            0 => Speed::Unknown,
            1 => Speed::Low,
            2 => Speed::Full,
            3 => Speed::High,
            4 => Speed::Super,
            5 => Speed::Super,
            _ => Speed::Unknown,
        }
    }

    pub fn to_libusb(&self) -> u8 {
        match self {
            Speed::Low => 1,
            Speed::Full => 2,
            Speed::High => 3,
            Speed::Super => 4,
            _ => 0,
        }
    }
}
