use crate::descriptor::*;

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
    descriptor_version: 0x0200,
    device_class: 0xff,    // Vendor-specific
    device_subclass: 0x00, // Vendor-specific
    device_protocol: 0x00,
    max_packet_size: 8,
    vendor_id: 0x1a86,
    product_id: 0x7523,
    device_version_number: 0x0264,
    manufacturer_string_index: 1,
    product_string_index: 2,
    serial_string_index: 3,
    num_configurations: 1,
    ..DeviceDescriptor::new()
};

pub const DEVICE_QUALIFIER_DESCRIPTOR: DeviceQualifierDescriptor = DeviceQualifierDescriptor {
    descriptor_version: 0x0200,
    device_class: 0xff,
    device_subclass: 0x00,
    device_protocol: 0x00,
    max_packet_size: 8,
    num_configurations: 1,
    reserved: 0,
    ..DeviceQualifierDescriptor::new()
};

pub const CONFIGURATION_DESCRIPTOR_0: ConfigurationDescriptor = ConfigurationDescriptor::new(
    ConfigurationDescriptorHeader {
        descriptor_type: DescriptorType::Configuration as u8,
        configuration_value: 1,
        configuration_string_index: 1,
        attributes: 0x80, // 0b1000_0000 = bus-powered
        max_power: 50,    // 50 * 2 mA = 100 mA
        ..ConfigurationDescriptorHeader::new()
    },
    &[InterfaceDescriptor::new(
        InterfaceDescriptorHeader {
            interface_number: 0,
            alternate_setting: 0,
            interface_class: 0xff,    // Vendor-specific
            interface_subclass: 0x01, // Vendor-specific
            interface_protocol: 0x02, // CDC
            interface_string_index: 2,
            ..InterfaceDescriptorHeader::new()
        },
        &[
            EndpointDescriptor {
                endpoint_address: 0x82, // IN
                attributes: 0x02,       // Bulk
                max_packet_size: 512,   // technically 32
                interval: 0,
                ..EndpointDescriptor::new()
            },
            EndpointDescriptor {
                endpoint_address: 0x02, // OUT
                attributes: 0x02,       // Bulk
                max_packet_size: 512,   // technically 32
                interval: 0,
                ..EndpointDescriptor::new()
            },
            EndpointDescriptor {
                endpoint_address: 0x81, // IN
                attributes: 0x03,       // Interrupt
                max_packet_size: 8,
                interval: 1, // 1ms
                ..EndpointDescriptor::new()
            },
        ],
    )],
);

pub const OTHER_SPEED_CONFIGURATION_DESCRIPTOR_0: ConfigurationDescriptor =
    ConfigurationDescriptor::new(
        ConfigurationDescriptorHeader {
            descriptor_type: DescriptorType::OtherSpeedConfiguration as u8,
            configuration_value: 1,
            configuration_string_index: 1,
            attributes: 0x80, // 0b1000_0000 = bus-powered
            max_power: 50,    // 50 * 2 mA = 100 mA
            ..ConfigurationDescriptorHeader::new()
        },
        &[InterfaceDescriptor::new(
            InterfaceDescriptorHeader {
                interface_number: 0,
                alternate_setting: 0,
                interface_class: 0xff,    // Vendor-specific
                interface_subclass: 0x01, // Vendor-specific
                interface_protocol: 0x02, // CDC
                interface_string_index: 2,
                ..InterfaceDescriptorHeader::new()
            },
            &[
                EndpointDescriptor {
                    endpoint_address: 0x82, // IN
                    attributes: 0x02,       // Bulk
                    max_packet_size: 64,    // technically 32
                    interval: 0,
                    ..EndpointDescriptor::new()
                },
                EndpointDescriptor {
                    endpoint_address: 0x02, // OUT
                    attributes: 0x02,       // Bulk
                    max_packet_size: 64,    // technically 32
                    interval: 0,
                    ..EndpointDescriptor::new()
                },
                EndpointDescriptor {
                    endpoint_address: 0x81, // IN
                    attributes: 0x03,       // Interrupt
                    max_packet_size: 8,
                    interval: 1, // 1ms
                    ..EndpointDescriptor::new()
                },
            ],
        )],
    );

pub const USB_STRING_DESCRIPTOR_0: StringDescriptorZero =
    StringDescriptorZero::new(&[LanguageId::EnglishUnitedStates]);

pub const USB_STRING_DESCRIPTOR_1: StringDescriptor = StringDescriptor::new("Great Scott Gadgets");
pub const USB_STRING_DESCRIPTOR_2: StringDescriptor = StringDescriptor::new("CDC-SERIAL Emulation");
pub const USB_STRING_DESCRIPTOR_3: StringDescriptor = StringDescriptor::new("100");

pub const USB_STRING_DESCRIPTORS: &[&StringDescriptor] = &[
    &USB_STRING_DESCRIPTOR_1,
    &USB_STRING_DESCRIPTOR_2,
    &USB_STRING_DESCRIPTOR_3,
];
