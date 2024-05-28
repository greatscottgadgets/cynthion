use crate::hal::smolusb;

use smolusb::descriptor::{
    ConfigurationDescriptor, ConfigurationDescriptorHeader, DescriptorType, DeviceDescriptor,
    DeviceQualifierDescriptor, EndpointDescriptor, InterfaceDescriptor, InterfaceDescriptorHeader,
    LanguageId, StringDescriptor, StringDescriptorZero,
};

// - constants ----------------------------------------------------------------

pub const DEVICE_VERSION_NUMBER: u16 = 0x0004; // Cynthion r0.4 TODO read from?
pub const DEVICE_SERIAL_STRING: &str = "r0.4"; // TODO read from?

// - vendor request -----------------------------------------------------------

pub mod vendor {
    #[repr(u8)]
    #[derive(Debug, PartialEq)]
    pub enum VendorRequest {
        // libgreat/firmware/platform/lpc43xx/include/drivers/usb/comms_backend.h
        //   11:  #define LIBGREAT_USB_COMMAND_REQUEST 0x65
        // libgreat/host/pygreat/comms_backends/usb.py
        //   30:  LIBGREAT_REQUEST_NUMBER = 0x65
        UsbCommandRequest = 0x65, // 101

        // legacy commands - see: host/greatfet/boards/legacy.py
        LegacyReadBoardId = 0x04,
        LegacyReadVersionString = 0x05,
        LegacyReadPartId = 0x06,
        LegacyReset = 0x16,     // 22
        LegacyReadDmesg = 0x40, // 64

        Unknown(u8),
    }

    impl From<u8> for VendorRequest {
        fn from(value: u8) -> Self {
            match value {
                0x04 => VendorRequest::LegacyReadBoardId,
                0x05 => VendorRequest::LegacyReadVersionString,
                0x06 => VendorRequest::LegacyReadPartId,
                0x16 => VendorRequest::LegacyReset,
                0x40 => VendorRequest::LegacyReadDmesg,
                0x65 => VendorRequest::UsbCommandRequest,
                _ => VendorRequest::Unknown(value),
            }
        }
    }

    #[repr(u16)]
    #[derive(Debug, PartialEq)]
    pub enum VendorValue {
        Execute = 0x0000,
        Cancel = 0xdead,
        Unknown(u16),
    }

    impl From<u16> for VendorValue {
        fn from(value: u16) -> Self {
            match value {
                0x0000 => VendorValue::Execute,
                0xdead => VendorValue::Cancel,
                _ => VendorValue::Unknown(value),
            }
        }
    }
}

// - descriptors --------------------------------------------------------------

pub static DEVICE_DESCRIPTOR: DeviceDescriptor = DeviceDescriptor {
    descriptor_version: 0x0200,
    device_class: 0x00,    // Composite
    device_subclass: 0x00, // Composite
    device_protocol: 0x00, // Composite
    max_packet_size: 64,
    vendor_id: cynthion::shared::usb::bVendorId::cynthion,
    product_id: cynthion::shared::usb::bProductId::cynthion,
    device_version_number: DEVICE_VERSION_NUMBER,
    manufacturer_string_index: 1,
    product_string_index: 2,
    serial_string_index: 3,
    num_configurations: 1,
    ..DeviceDescriptor::new()
};

pub static DEVICE_QUALIFIER_DESCRIPTOR: DeviceQualifierDescriptor = DeviceQualifierDescriptor {
    descriptor_version: 0x0200,
    device_class: 0x00,    // Composite
    device_subclass: 0x00, // Composite
    device_protocol: 0x00, // Composite
    max_packet_size: 64,
    num_configurations: 1,
    ..DeviceQualifierDescriptor::new()
};

pub static CONFIGURATION_DESCRIPTOR_0: ConfigurationDescriptor = ConfigurationDescriptor::new(
    ConfigurationDescriptorHeader {
        descriptor_type: DescriptorType::Configuration as u8,
        configuration_value: 1,
        configuration_string_index: 4,
        attributes: 0x80, // 0b1000_0000 = bus-powered
        max_power: 250,   // 250 * 2 mA = 500 mA ?
        ..ConfigurationDescriptorHeader::new()
    },
    &[InterfaceDescriptor::new(
        InterfaceDescriptorHeader {
            interface_number: 0,
            alternate_setting: 0,
            interface_class: 0xff, // Vendor-specific
            interface_subclass: cynthion::shared::usb::bInterfaceSubClass::moondancer,
            interface_protocol: cynthion::shared::usb::bInterfaceProtocol::moondancer,
            interface_string_index: 5,
            ..InterfaceDescriptorHeader::new()
        },
        &[
            EndpointDescriptor {
                endpoint_address: cynthion::shared::libgreat::endpoints::bulk_in_address, // IN
                attributes: 0x02,                                                         // Bulk
                max_packet_size: 512,
                interval: 0,
                ..EndpointDescriptor::new()
            },
            EndpointDescriptor {
                endpoint_address: cynthion::shared::libgreat::endpoints::bulk_out_address, // OUT
                attributes: 0x02,                                                          // Bulk
                max_packet_size: 512,
                interval: 0,
                ..EndpointDescriptor::new()
            },
        ],
    )],
);

pub static OTHER_SPEED_CONFIGURATION_DESCRIPTOR_0: ConfigurationDescriptor =
    ConfigurationDescriptor::new(
        ConfigurationDescriptorHeader {
            descriptor_type: DescriptorType::OtherSpeedConfiguration as u8,
            configuration_value: 1,
            configuration_string_index: 7,
            attributes: 0x80, // 0b1000_0000 = bus-powered
            max_power: 250,   // 250 * 2 mA = 500 mA ?
            ..ConfigurationDescriptorHeader::new()
        },
        &[InterfaceDescriptor::new(
            InterfaceDescriptorHeader {
                interface_number: 0,
                alternate_setting: 0,
                interface_class: 0xff, // Vendor-specific
                interface_subclass: cynthion::shared::usb::bInterfaceSubClass::moondancer,
                interface_protocol: cynthion::shared::usb::bInterfaceProtocol::moondancer,
                interface_string_index: 5,
                ..InterfaceDescriptorHeader::new()
            },
            &[
                EndpointDescriptor {
                    endpoint_address: cynthion::shared::libgreat::endpoints::bulk_in_address, // IN
                    attributes: 0x02, // Bulk
                    max_packet_size: 64,
                    interval: 0,
                    ..EndpointDescriptor::new()
                },
                EndpointDescriptor {
                    endpoint_address: cynthion::shared::libgreat::endpoints::bulk_out_address, // OUT
                    attributes: 0x02, // Bulk
                    max_packet_size: 64,
                    interval: 0,
                    ..EndpointDescriptor::new()
                },
            ],
        )],
    );

pub static STRING_DESCRIPTOR_0: StringDescriptorZero =
    StringDescriptorZero::new(&[LanguageId::EnglishUnitedStates]);

pub static STRING_DESCRIPTOR_1: StringDescriptor =
    StringDescriptor::new(cynthion::shared::usb::bManufacturerString::cynthion); // manufacturer
pub static STRING_DESCRIPTOR_2: StringDescriptor =
    StringDescriptor::new(cynthion::shared::usb::bProductString::cynthion); // product
pub static STRING_DESCRIPTOR_3: StringDescriptor = StringDescriptor::new(DEVICE_SERIAL_STRING); // serial
pub static STRING_DESCRIPTOR_4: StringDescriptor = StringDescriptor::new("config0"); // configuration #0
pub static STRING_DESCRIPTOR_5: StringDescriptor = StringDescriptor::new("interface0"); // interface #0
pub static STRING_DESCRIPTOR_6: StringDescriptor = StringDescriptor::new("interface1"); // interface #1
pub static STRING_DESCRIPTOR_7: StringDescriptor = StringDescriptor::new("config1"); // configuration #1

pub static STRING_DESCRIPTORS: &[&StringDescriptor] = &[
    &STRING_DESCRIPTOR_1,
    &STRING_DESCRIPTOR_2,
    &STRING_DESCRIPTOR_3,
    &STRING_DESCRIPTOR_4,
    &STRING_DESCRIPTOR_5,
    &STRING_DESCRIPTOR_6,
    &STRING_DESCRIPTOR_7,
];
