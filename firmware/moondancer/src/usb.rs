use crate::hal::smolusb;

use smolusb::descriptor::{
    ConfigurationDescriptor, ConfigurationDescriptorHeader, DescriptorType, DeviceDescriptor,
    DeviceQualifierDescriptor, EndpointDescriptor, InterfaceDescriptor, InterfaceDescriptorHeader,
    LanguageId, StringDescriptor, StringDescriptorZero,
};

// - constants ----------------------------------------------------------------

pub const DEVICE_SERIAL_STRING: &str = "moondancer"; // TODO read flash uid

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

        // apollo stub interface claim request
        ApolloClaimInterface = 0xf0, // 240

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
                0xf0 => VendorRequest::ApolloClaimInterface,
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
    bcdUSB: 0x0200,
    bDeviceClass: 0x00,    // Composite
    bDeviceSubClass: 0x00, // Composite
    bDeviceProtocol: 0x00, // Composite
    bMaxPacketSize: 64,
    idVendor: cynthion::shared::usb::bVendorId::cynthion,
    idProduct: cynthion::shared::usb::bProductId::cynthion,
    iManufacturer: 1,
    iProduct: 2,
    iSerialNumber: 3,
    bNumConfigurations: 1,
    ..DeviceDescriptor::new()
};

pub static DEVICE_QUALIFIER_DESCRIPTOR: DeviceQualifierDescriptor = DeviceQualifierDescriptor {
    bcdUSB: 0x0200,
    bDeviceClass: 0x00,    // Composite
    bDeviceSubClass: 0x00, // Composite
    bDeviceProtocol: 0x00, // Composite
    bMaxPacketSize0: 64,
    bNumConfigurations: 1,
    ..DeviceQualifierDescriptor::new()
};

pub static CONFIGURATION_DESCRIPTOR_0: ConfigurationDescriptor = ConfigurationDescriptor::new(
    ConfigurationDescriptorHeader {
        bDescriptorType: DescriptorType::Configuration as u8,
        bConfigurationValue: 1,
        iConfiguration: 4,
        bmAttributes: 0x80, // 0b1000_0000 = bus-powered
        bMaxPower: 250,     // 250 * 2 mA = 500 mA ?
        ..ConfigurationDescriptorHeader::new()
    },
    &[
        InterfaceDescriptor::new(
            InterfaceDescriptorHeader {
                iInterfaceNumber: 0,
                bAlternateSetting: 0,
                bInterfaceClass: 0xff, // Vendor-specific
                bInterfaceSubClass: cynthion::shared::usb::bInterfaceSubClass::moondancer,
                bInterfaceProtocol: cynthion::shared::usb::bInterfaceProtocol::moondancer,
                iInterface: 5,
                ..InterfaceDescriptorHeader::new()
            },
            &[
                EndpointDescriptor {
                    bEndpointAddress: cynthion::shared::libgreat::endpoints::bulk_in_address, // IN
                    bmAttributes: 0x02, // Bulk
                    wMaxPacketSize: 512,
                    bInterval: 0,
                    ..EndpointDescriptor::new()
                },
                EndpointDescriptor {
                    bEndpointAddress: cynthion::shared::libgreat::endpoints::bulk_out_address, // OUT
                    bmAttributes: 0x02, // Bulk
                    wMaxPacketSize: 512,
                    bInterval: 0,
                    ..EndpointDescriptor::new()
                },
            ],
        ),
        // Apollo stub interface
        InterfaceDescriptor::new(
            InterfaceDescriptorHeader {
                iInterfaceNumber: 1,
                bAlternateSetting: 0,
                bInterfaceClass: 0xff, // Vendor-specific
                bInterfaceSubClass: 0,
                bInterfaceProtocol: 0,
                iInterface: 6,
                ..InterfaceDescriptorHeader::new()
            },
            &[],
        ),
    ],
);

pub static OTHER_SPEED_CONFIGURATION_DESCRIPTOR_0: ConfigurationDescriptor =
    ConfigurationDescriptor::new(
        ConfigurationDescriptorHeader {
            bDescriptorType: DescriptorType::OtherSpeedConfiguration as u8,
            bConfigurationValue: 1,
            iConfiguration: 7,
            bmAttributes: 0x80, // 0b1000_0000 = bus-powered
            bMaxPower: 250,     // 250 * 2 mA = 500 mA ?
            ..ConfigurationDescriptorHeader::new()
        },
        &[
            // Moondancer control interface
            InterfaceDescriptor::new(
                InterfaceDescriptorHeader {
                    iInterfaceNumber: 0,
                    bAlternateSetting: 0,
                    bInterfaceClass: 0xff, // Vendor-specific
                    bInterfaceSubClass: cynthion::shared::usb::bInterfaceSubClass::moondancer,
                    bInterfaceProtocol: cynthion::shared::usb::bInterfaceProtocol::moondancer,
                    iInterface: 8,
                    ..InterfaceDescriptorHeader::new()
                },
                &[
                    EndpointDescriptor {
                        bEndpointAddress: cynthion::shared::libgreat::endpoints::bulk_in_address, // IN
                        bmAttributes: 0x02, // Bulk
                        wMaxPacketSize: 64,
                        bInterval: 0,
                        ..EndpointDescriptor::new()
                    },
                    EndpointDescriptor {
                        bEndpointAddress: cynthion::shared::libgreat::endpoints::bulk_out_address, // OUT
                        bmAttributes: 0x02, // Bulk
                        wMaxPacketSize: 64,
                        bInterval: 0,
                        ..EndpointDescriptor::new()
                    },
                ],
            ),
            // Apollo stub interface
            InterfaceDescriptor::new(
                InterfaceDescriptorHeader {
                    iInterfaceNumber: 1,
                    bAlternateSetting: 0,
                    bInterfaceClass: 0xff, // Vendor-specific
                    bInterfaceSubClass: 0,
                    bInterfaceProtocol: 0,
                    iInterface: 9,
                    ..InterfaceDescriptorHeader::new()
                },
                &[],
            ),
        ],
    );

pub static STRING_DESCRIPTOR_0: StringDescriptorZero =
    StringDescriptorZero::new(&[LanguageId::EnglishUnitedStates]);

// manufacturer
pub static STRING_DESCRIPTOR_1: StringDescriptor =
    StringDescriptor::new(cynthion::shared::usb::bManufacturerString::cynthion);
// product
pub static STRING_DESCRIPTOR_2: StringDescriptor =
    StringDescriptor::new(cynthion::shared::usb::bProductString::cynthion);
// serial
pub static STRING_DESCRIPTOR_3: StringDescriptor = StringDescriptor::new(DEVICE_SERIAL_STRING);

// configuration #0
pub static STRING_DESCRIPTOR_4: StringDescriptor = StringDescriptor::new("config0");
// interface #0
pub static STRING_DESCRIPTOR_5: StringDescriptor = StringDescriptor::new("Facedancer Control");
// interface #1
pub static STRING_DESCRIPTOR_6: StringDescriptor = StringDescriptor::new("Apollo Stub");

// other-speed configuration #0
pub static STRING_DESCRIPTOR_7: StringDescriptor = StringDescriptor::new("other config0");
// interface #0
pub static STRING_DESCRIPTOR_8: StringDescriptor = StringDescriptor::new("other interface0");
// interface #1
pub static STRING_DESCRIPTOR_9: StringDescriptor = StringDescriptor::new("other interface1");

pub static STRING_DESCRIPTORS: &[&StringDescriptor] = &[
    &STRING_DESCRIPTOR_1,
    &STRING_DESCRIPTOR_2,
    &STRING_DESCRIPTOR_3,
    &STRING_DESCRIPTOR_4,
    &STRING_DESCRIPTOR_5,
    &STRING_DESCRIPTOR_6,
    &STRING_DESCRIPTOR_7,
    &STRING_DESCRIPTOR_8,
    &STRING_DESCRIPTOR_9,
];
