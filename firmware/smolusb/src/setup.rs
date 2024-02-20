//! Types for working with the SETUP packet.

/// Represents a USB setup packet.
#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct SetupPacket {
    // 0..4 Recipient: 0=Device, 1=Interface, 2=Endpoint, 3=Other, 4-31=Reserved
    // 5..6 Type: 0=Standard, 1=Class, 2=Vendor, 3=Reserved
    // 7    Data Phase Transfer Direction: 0=Host to Device, 1=Device to Host
    pub request_type: u8,
    // values 0..=9 are standard, others are class or vendor
    pub request: u8,
    pub value: u16,
    pub index: u16,
    pub length: u16,
}

impl From<[u8; 8]> for SetupPacket {
    fn from(buffer: [u8; 8]) -> Self {
        // Deserialize into a SetupRequest in the most cursed manner available to us
        // TODO do this properly
        unsafe { core::mem::transmute::<[u8; 8], SetupPacket>(buffer) }
    }
}

impl SetupPacket {
    #[must_use]
    pub fn as_bytes(setup_packet: SetupPacket) -> [u8; 8] {
        // Serialize into bytes in the most cursed manner available to us
        // TODO do this properly
        unsafe { core::mem::transmute::<SetupPacket, [u8; 8]>(setup_packet) }
    }
}

impl SetupPacket {
    #[must_use]
    pub fn request_type(&self) -> RequestType {
        RequestType::from(self.request_type)
    }

    #[must_use]
    pub fn recipient(&self) -> Recipient {
        Recipient::from(self.request_type)
    }

    #[must_use]
    pub fn direction(&self) -> Direction {
        Direction::from(self.request_type)
    }

    #[must_use]
    pub fn request(&self) -> Request {
        Request::from(self.request)
    }
}

/// Represents bits 0..=4 of the `[SetupPacket]` `request_type` field.
#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum Recipient {
    Device = 0,
    Interface = 1,
    Endpoint = 2,
    Other = 3,
    Reserved = 4,
}

impl From<u8> for Recipient {
    fn from(value: u8) -> Self {
        match value & 0b0001_1111 {
            0 => Recipient::Device,
            1 => Recipient::Interface,
            2 => Recipient::Endpoint,
            3 => Recipient::Other,
            4..=u8::MAX => Recipient::Reserved,
        }
    }
}

/// Represents bit 5..=6 of the `[SetupPacket]` `request`_type field.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RequestType {
    Standard = 0,
    Class = 1,
    Vendor = 2,
    Reserved(u8) = 3,
}

impl From<u8> for RequestType {
    fn from(value: u8) -> Self {
        match (value >> 5) & 0b0000_0011 {
            0 => RequestType::Standard,
            1 => RequestType::Class,
            2 => RequestType::Vendor,
            3..=u8::MAX => RequestType::Reserved(value),
        }
    }
}

/// Represents bit 7 of the `[SetupPacket]` `request`_type field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Direction {
    /// Host to device (OUT)
    HostToDevice = 0x00,
    /// Device to host (IN)
    DeviceToHost = 0x80, // 0b1000_0000,
}

impl Direction {
    pub const OUT: Direction = Direction::HostToDevice;
    pub const IN: Direction = Direction::DeviceToHost;
}

impl From<u8> for Direction {
    fn from(endpoint_or_request_type: u8) -> Self {
        if endpoint_or_request_type & 0b1000_0000 == 0 {
            Direction::HostToDevice
        } else {
            Direction::DeviceToHost
        }
    }
}

/// Represents the `SetupPacket` `request` field.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum Request {
    GetStatus = 0,
    ClearFeature = 1,
    SetFeature = 3,
    SetAddress = 5,
    GetDescriptor = 6,
    SetDescriptor = 7,
    GetConfiguration = 8,
    SetConfiguration = 9,
    GetInterface = 10,
    SetInterface = 11,
    SynchronizeFrame = 12,
    ClassOrVendor(u8),
    Reserved(u8),
}

impl From<u8> for Request {
    fn from(value: u8) -> Self {
        match value {
            0 => Request::GetStatus,
            1 => Request::ClearFeature,
            2 => Request::Reserved(2),
            3 => Request::SetFeature,
            4 => Request::Reserved(4),
            5 => Request::SetAddress,
            6 => Request::GetDescriptor,
            7 => Request::SetDescriptor,
            8 => Request::GetConfiguration,
            9 => Request::SetConfiguration,
            10 => Request::GetInterface,
            11 => Request::SetInterface,
            12 => Request::SynchronizeFrame,
            13..=u8::MAX => Request::ClassOrVendor(value),
        }
    }
}

/// Represents standard values for `Request::SetFeature` and `Request::ClearFeature`.
#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum Feature {
    EndpointHalt = 0,
    DeviceRemoteWakeup = 1,
    DeviceTestMode = 2,
    DeviceBHnpEnable = 3,
    DeviceAHnpSupport = 4,
    DeviceAAltHnpSupport = 5,
    Unknown(u16) = 255,
}

impl From<u16> for Feature {
    fn from(value: u16) -> Self {
        match value {
            0 => Feature::EndpointHalt,
            1 => Feature::DeviceRemoteWakeup,
            2 => Feature::DeviceTestMode,
            3 => Feature::DeviceBHnpEnable,
            4 => Feature::DeviceAHnpSupport,
            5 => Feature::DeviceAAltHnpSupport,
            _ => Feature::Unknown(value),
        }
    }
}
