use crate::descriptor::{
    ConfigurationDescriptor, ConfigurationDescriptorHeader, DescriptorType, DeviceDescriptor,
    DeviceQualifierDescriptor, EndpointDescriptor, InterfaceDescriptor, InterfaceDescriptorHeader,
    LanguageId, StringDescriptor, StringDescriptorZero,
};

pub const VENDOR_ID: u16 = 0x1a86; // QinHeng Electronics
pub const PRODUCT_ID: u16 = 0x7523; // CH341 in serial mode, usb to serial port converter

pub mod ch34x {
    #[derive(Debug, PartialEq)]
    #[repr(u8)]
    pub enum VendorRequest {
        WriteType = 0x40,  //  64
        ReadType = 0xc0,   // 192
        Read = 0x95,       // 149
        Write = 0x9a,      // 154
        SerialInit = 0xa1, // 161
        ModemOut = 0xa4,   // 164
        Version = 0x5f,    //  95
        Unknown,
    }

    impl From<u8> for VendorRequest {
        fn from(value: u8) -> Self {
            match value {
                0x40 => VendorRequest::WriteType,
                0xc0 => VendorRequest::ReadType,
                0x95 => VendorRequest::Read,
                0x9a => VendorRequest::Write,
                0xa1 => VendorRequest::SerialInit,
                0xa4 => VendorRequest::ModemOut,
                0x5f => VendorRequest::Version,
                _ => VendorRequest::Unknown,
            }
        }
    }
}

pub const DEVICE_DESCRIPTOR: DeviceDescriptor = DeviceDescriptor {
    bcdUSB: 0x0200,
    bDeviceClass: 0xff,    // Vendor-specific
    bDeviceSubClass: 0x00, // Vendor-specific
    bDeviceProtocol: 0x00,
    bMaxPacketSize: 8,
    idVendor: VENDOR_ID,
    idProduct: PRODUCT_ID,
    bcdDevice: 0x0264,
    iManufacturer: 1,
    iProduct: 2,
    iSerialNumber: 3,
    bNumConfigurations: 1,
    ..DeviceDescriptor::new()
};

pub const DEVICE_QUALIFIER_DESCRIPTOR: DeviceQualifierDescriptor = DeviceQualifierDescriptor {
    bcdUSB: 0x0200,
    bDeviceClass: 0xff,
    bDeviceSubClass: 0x00,
    bDeviceProtocol: 0x00,
    bMaxPacketSize0: 8,
    bNumConfigurations: 1,
    bReserved: 0,
    ..DeviceQualifierDescriptor::new()
};

pub const CONFIGURATION_DESCRIPTOR_0: ConfigurationDescriptor = ConfigurationDescriptor::new(
    ConfigurationDescriptorHeader {
        bDescriptorType: DescriptorType::Configuration as u8,
        bConfigurationValue: 1,
        iConfiguration: 1,
        bmAttributes: 0x80, // 0b1000_0000 = bus-powered
        bMaxPower: 50,      // 50 * 2 mA = 100 mA
        ..ConfigurationDescriptorHeader::new()
    },
    &[InterfaceDescriptor::new(
        InterfaceDescriptorHeader {
            iInterfaceNumber: 0,
            bAlternateSetting: 0,
            bInterfaceClass: 0xff,    // Vendor-specific
            bInterfaceSubClass: 0x01, // Vendor-specific
            bInterfaceProtocol: 0x02, // CDC
            iInterface: 2,
            ..InterfaceDescriptorHeader::new()
        },
        &[
            EndpointDescriptor {
                bEndpointAddress: 0x82, // IN
                bmAttributes: 0x02,     // Bulk
                wMaxPacketSize: 512,    // technically 32
                bInterval: 0,
                ..EndpointDescriptor::new()
            },
            EndpointDescriptor {
                bEndpointAddress: 0x02, // OUT
                bmAttributes: 0x02,     // Bulk
                wMaxPacketSize: 512,    // technically 32
                bInterval: 0,
                ..EndpointDescriptor::new()
            },
            EndpointDescriptor {
                bEndpointAddress: 0x81, // IN
                bmAttributes: 0x03,     // Interrupt
                wMaxPacketSize: 8,
                bInterval: 1, // 1ms
                ..EndpointDescriptor::new()
            },
        ],
    )],
);

pub const OTHER_SPEED_CONFIGURATION_DESCRIPTOR_0: ConfigurationDescriptor =
    ConfigurationDescriptor::new(
        ConfigurationDescriptorHeader {
            bDescriptorType: DescriptorType::OtherSpeedConfiguration as u8,
            bConfigurationValue: 1,
            iConfiguration: 1,
            bmAttributes: 0x80, // 0b1000_0000 = bus-powered
            bMaxPower: 50,      // 50 * 2 mA = 100 mA
            ..ConfigurationDescriptorHeader::new()
        },
        &[InterfaceDescriptor::new(
            InterfaceDescriptorHeader {
                iInterfaceNumber: 0,
                bAlternateSetting: 0,
                bInterfaceClass: 0xff,    // Vendor-specific
                bInterfaceSubClass: 0x01, // Vendor-specific
                bInterfaceProtocol: 0x02, // CDC
                iInterface: 2,
                ..InterfaceDescriptorHeader::new()
            },
            &[
                EndpointDescriptor {
                    bEndpointAddress: 0x82, // IN
                    bmAttributes: 0x02,     // Bulk
                    wMaxPacketSize: 64,     // technically 32
                    bInterval: 0,
                    ..EndpointDescriptor::new()
                },
                EndpointDescriptor {
                    bEndpointAddress: 0x02, // OUT
                    bmAttributes: 0x02,     // Bulk
                    wMaxPacketSize: 64,     // technically 32
                    bInterval: 0,
                    ..EndpointDescriptor::new()
                },
                EndpointDescriptor {
                    bEndpointAddress: 0x81, // IN
                    bmAttributes: 0x03,     // Interrupt
                    wMaxPacketSize: 8,
                    bInterval: 1, // 1ms
                    ..EndpointDescriptor::new()
                },
            ],
        )],
    );

pub const STRING_DESCRIPTOR_0: StringDescriptorZero =
    StringDescriptorZero::new(&[LanguageId::EnglishUnitedStates]);

pub const STRING_DESCRIPTOR_1: StringDescriptor = StringDescriptor::new("Great Scott Gadgets");
pub const STRING_DESCRIPTOR_2: StringDescriptor = StringDescriptor::new("CDC-SERIAL Emulation");
pub const STRING_DESCRIPTOR_3: StringDescriptor = StringDescriptor::new("100");

pub const STRING_DESCRIPTORS: &[&StringDescriptor] = &[
    &STRING_DESCRIPTOR_1,
    &STRING_DESCRIPTOR_2,
    &STRING_DESCRIPTOR_3,
];
