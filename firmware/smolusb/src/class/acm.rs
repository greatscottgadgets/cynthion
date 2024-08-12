use crate::descriptor::{
    ClassSpecificDescriptor, ConfigurationDescriptor, ConfigurationDescriptorHeader,
    DescriptorType, DeviceDescriptor, DeviceQualifierDescriptor, EndpointDescriptor,
    InterfaceDescriptor, InterfaceDescriptorHeader, LanguageId, StringDescriptor,
    StringDescriptorZero,
};

pub const VENDOR_ID: u16 = 0x1209; // https://pid.codes/1209/
pub const PRODUCT_ID: u16 = 0x0001; // pid.codes Test PID 1

pub mod serial {
    /// ACM Serial Class Requests
    ///
    /// Just enough of the requests to be usable on the major operating systems.
    #[derive(Debug, PartialEq)]
    #[repr(u8)]
    pub enum ClassRequest {
        SetLineCoding = 0x20,       //  32
        GetLineCoding = 0x21,       //  33
        SetControlLineState = 0x22, //  34
        SendBreak = 0x23,           //  35
        Unknown(u8),
    }

    impl From<u8> for ClassRequest {
        fn from(value: u8) -> Self {
            match value {
                0x20 => ClassRequest::SetLineCoding,
                0x21 => ClassRequest::GetLineCoding,
                0x22 => ClassRequest::SetControlLineState,
                0x23 => ClassRequest::SendBreak,
                _ => ClassRequest::Unknown(value),
            }
        }
    }
}

pub const DEVICE_DESCRIPTOR: DeviceDescriptor = DeviceDescriptor {
    bcdUSB: 0x0200,
    bDeviceClass: 0x00,    // Composite
    bDeviceSubClass: 0x00, // Vendor-specific
    bDeviceProtocol: 0x00,
    bMaxPacketSize: 64,
    idVendor: VENDOR_ID,
    idProduct: PRODUCT_ID,
    bcdDevice: 0x0001,
    iManufacturer: 1,
    iProduct: 2,
    iSerialNumber: 3,
    bNumConfigurations: 1,
    ..DeviceDescriptor::new()
};

pub const DEVICE_QUALIFIER_DESCRIPTOR: DeviceQualifierDescriptor = DeviceQualifierDescriptor {
    bcdUSB: 0x0200,
    bDeviceClass: 0x00,
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
        iConfiguration: 4,
        bmAttributes: 0x80, // 0b1000_0000 = bus-powered
        bMaxPower: 50,      // 50 * 2 mA = 100 mA
        ..ConfigurationDescriptorHeader::new()
    },
    &[
        // Interface #0 - Communications-Control
        InterfaceDescriptor::new_cs(
            InterfaceDescriptorHeader {
                iInterfaceNumber: 0,
                bAlternateSetting: 0,
                bInterfaceClass: 0x02, // Communications-Control
                bInterfaceSubClass: 0x02,
                bInterfaceProtocol: 0x01,
                iInterface: 5,
                ..InterfaceDescriptorHeader::new()
            },
            &[
                // Comm Class Header Functional Descriptor
                ClassSpecificDescriptor {
                    bDescriptorSubtype: 0x00,
                    bmRaw: 0x0110,
                    ..ClassSpecificDescriptor::new()
                },
                // Comm Class Union Functional Descriptor
                ClassSpecificDescriptor {
                    bDescriptorSubtype: 0x06,
                    bmRaw: 0x0100,
                    ..ClassSpecificDescriptor::new()
                },
                // Comm Class Call Management Functional Descriptor
                ClassSpecificDescriptor {
                    bDescriptorSubtype: 0x01,
                    bmRaw: 0x0100,
                    ..ClassSpecificDescriptor::new()
                },
            ],
            &[EndpointDescriptor {
                bEndpointAddress: 0x83, // IN
                bmAttributes: 0x03,     // Interrupt
                wMaxPacketSize: 64,
                bInterval: 11,
                ..EndpointDescriptor::new()
            }],
        ),
        // Interface #1 - Communications-Data/Unknown Comm Class Model
        InterfaceDescriptor::new(
            InterfaceDescriptorHeader {
                iInterfaceNumber: 1,
                bAlternateSetting: 0,
                bInterfaceClass: 0x0a,    // Communications-Data
                bInterfaceSubClass: 0x00, // Unknown Comm Class Model
                bInterfaceProtocol: 0x00,
                iInterface: 6,
                ..InterfaceDescriptorHeader::new()
            },
            &[
                EndpointDescriptor {
                    bEndpointAddress: 0x84, // IN
                    bmAttributes: 0x02,     // Bulk
                    wMaxPacketSize: 64,
                    bInterval: 255,
                    ..EndpointDescriptor::new()
                },
                EndpointDescriptor {
                    bEndpointAddress: 0x04, // OUT
                    bmAttributes: 0x02,     // Bulk
                    wMaxPacketSize: 512,    // 512 ?
                    bInterval: 255,
                    ..EndpointDescriptor::new()
                },
            ],
        ),
    ],
);

pub const OTHER_SPEED_CONFIGURATION_DESCRIPTOR_0: ConfigurationDescriptor =
    ConfigurationDescriptor::new(
        ConfigurationDescriptorHeader {
            bDescriptorType: DescriptorType::Configuration as u8,
            bConfigurationValue: 1,
            iConfiguration: 4,
            bmAttributes: 0x80, // 0b1000_0000 = bus-powered
            bMaxPower: 50,      // 50 * 2 mA = 100 mA
            ..ConfigurationDescriptorHeader::new()
        },
        &[
            // Interface #0 - Communications-Control
            InterfaceDescriptor::new_cs(
                InterfaceDescriptorHeader {
                    iInterfaceNumber: 0,
                    bAlternateSetting: 0,
                    bInterfaceClass: 0x02, // Communications-Control
                    bInterfaceSubClass: 0x02,
                    bInterfaceProtocol: 0x01,
                    iInterface: 5,
                    ..InterfaceDescriptorHeader::new()
                },
                &[
                    // Comm Class Header Functional Descriptor
                    ClassSpecificDescriptor {
                        bDescriptorSubtype: 0x00,
                        bmRaw: 0x0110,
                        ..ClassSpecificDescriptor::new()
                    },
                    // Comm Class Union Functional Descriptor
                    ClassSpecificDescriptor {
                        bDescriptorSubtype: 0x06,
                        bmRaw: 0x0100,
                        ..ClassSpecificDescriptor::new()
                    },
                    // Comm Class Call Management Functional Descriptor
                    ClassSpecificDescriptor {
                        bDescriptorSubtype: 0x01,
                        bmRaw: 0x0100,
                        ..ClassSpecificDescriptor::new()
                    },
                ],
                &[EndpointDescriptor {
                    bEndpointAddress: 0x83, // IN
                    bmAttributes: 0x02,     // Interrupt
                    wMaxPacketSize: 64,
                    bInterval: 11,
                    ..EndpointDescriptor::new()
                }],
            ),
            // Interface #1 - Communications-Data/Unknown Comm Class Model
            InterfaceDescriptor::new(
                InterfaceDescriptorHeader {
                    iInterfaceNumber: 1,
                    bAlternateSetting: 0,
                    bInterfaceClass: 0x0a,    // Communications-Data
                    bInterfaceSubClass: 0x00, // Unknown Comm Class Model
                    bInterfaceProtocol: 0x00,
                    iInterface: 6,
                    ..InterfaceDescriptorHeader::new()
                },
                &[
                    EndpointDescriptor {
                        bEndpointAddress: 0x84, // IN
                        bmAttributes: 0x02,     // Bulk
                        wMaxPacketSize: 64,
                        bInterval: 255,
                        ..EndpointDescriptor::new()
                    },
                    EndpointDescriptor {
                        bEndpointAddress: 0x04, // OUT
                        bmAttributes: 0x02,     // Bulk
                        wMaxPacketSize: 64,
                        bInterval: 255,
                        ..EndpointDescriptor::new()
                    },
                ],
            ),
        ],
    );

pub const STRING_DESCRIPTOR_0: StringDescriptorZero =
    StringDescriptorZero::new(&[LanguageId::EnglishUnitedStates]);

pub const STRING_DESCRIPTOR_1: StringDescriptor = StringDescriptor::new("Cynthion Project");
pub const STRING_DESCRIPTOR_2: StringDescriptor = StringDescriptor::new("USB-to-serial");
pub const STRING_DESCRIPTOR_3: StringDescriptor = StringDescriptor::new("100");
pub const STRING_DESCRIPTOR_4: StringDescriptor = StringDescriptor::new("iConfiguration 0"); // iConfiguration #0
pub const STRING_DESCRIPTOR_5: StringDescriptor = StringDescriptor::new("iInterface 0"); // iInterface #0
pub const STRING_DESCRIPTOR_6: StringDescriptor = StringDescriptor::new("iInterface 1"); // iInterface #1

pub const STRING_DESCRIPTORS: &[&StringDescriptor] = &[
    &STRING_DESCRIPTOR_1,
    &STRING_DESCRIPTOR_2,
    &STRING_DESCRIPTOR_3,
    &STRING_DESCRIPTOR_4,
    &STRING_DESCRIPTOR_5,
    &STRING_DESCRIPTOR_6,
];
