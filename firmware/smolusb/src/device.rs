//! `smolusb` device types
//!

/// USB Speed
///
/// Note: These match UTMI xcvr_select constant so the mapping may not be correct for other contexts.
///       See: <https://github.com/greatscottgadgets/luna/blob/main/luna/gateware/usb/usb2/__init__.py>
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
    /// See: <https://github.com/libusb/libusb/blob/6bf2db6feaf3b611c9adedb6c4962a07f5cb07ae/libusb/libusb.h#L1126>
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

//use crate::device::Speed;
use crate::descriptor::*;
use crate::setup::SetupPacket;
use crate::traits::{AsByteSliceIterator, UsbDriver};
use log::{debug, warn};

// /// The set of descriptors describing a USB device.
pub struct Descriptors<'a> {
    pub device_speed: Speed,
    pub device_descriptor: DeviceDescriptor,
    pub configuration_descriptor: ConfigurationDescriptor<'a>,
    pub other_speed_configuration_descriptor: Option<ConfigurationDescriptor<'a>>,
    pub device_qualifier_descriptor: Option<DeviceQualifierDescriptor>,
    pub string_descriptor_zero: StringDescriptorZero<'a>,
    pub string_descriptors: &'a [&'a StringDescriptor<'a>],
}

impl<'a> Descriptors<'a> {
    // TODO ugly hack because I haven't figured out how to do this at compile time yet
    pub fn set_total_lengths(mut self) -> Self {
        self.configuration_descriptor.set_total_length();
        if let Some(other_speed_configuration_descriptor) =
            self.other_speed_configuration_descriptor.as_mut()
        {
            other_speed_configuration_descriptor.set_total_length();
        }
        self
    }

    pub fn write<D>(
        &self,
        usb: &D,
        endpoint_number: u8,
        setup_packet: SetupPacket,
    ) -> Option<SetupPacket>
    where
        D: UsbDriver,
    {
        // extract the descriptor type and number from our SETUP request
        let [descriptor_number, descriptor_type_bits] = setup_packet.value.to_le_bytes();
        let descriptor_type = match DescriptorType::try_from(descriptor_type_bits) {
            Ok(descriptor_type) => descriptor_type,
            Err(_e) => {
                warn!(
                    "Descriptors::write_descriptor() stall - invalid descriptor type: {} {}",
                    descriptor_type_bits, descriptor_number
                );
                usb.stall_endpoint_in(endpoint_number);
                return Some(setup_packet);
            }
        };

        // if the host is requesting less than the maximum amount of data,
        // only respond with the amount requested
        let requested_length = setup_packet.length as usize;

        let bytes_written = match (&descriptor_type, descriptor_number) {
            (DescriptorType::Device, 0) => usb.write(
                endpoint_number,
                self.device_descriptor
                    .as_iter()
                    .copied()
                    .take(requested_length),
            ),
            (DescriptorType::Configuration, _) => usb.write(
                endpoint_number,
                self.configuration_descriptor
                    .iter()
                    .copied()
                    .take(requested_length),
            ),
            (DescriptorType::DeviceQualifier, _) => {
                if self.device_speed == Speed::High {
                    if let Some(descriptor) = &self.device_qualifier_descriptor {
                        usb.write(
                            endpoint_number,
                            descriptor.as_iter().copied().take(requested_length),
                        )
                    } else {
                        // no device qualifier configured, ack HostToDevice instead - TODO check check on mac/windows
                        debug!("  No device qualifier configured for high-speed device");
                        usb.write(endpoint_number, [].into_iter())
                    }
                } else {
                    // for full/low speed devices, ack HostToDevice instead - TODO check on mac/windows
                    debug!(
                        "  Device qualifier request is not supported for full/low-speed devices"
                    );
                    // FIXME we should stall instead
                    usb.write(endpoint_number, [].into_iter())
                }
            }
            (DescriptorType::OtherSpeedConfiguration, _) => {
                if let Some(descriptor) = self.other_speed_configuration_descriptor {
                    usb.write(
                        endpoint_number,
                        descriptor.iter().copied().take(requested_length),
                    )
                } else {
                    // no other speed configuration, ack HostToDevice instead - TODO check check on mac/windows
                    debug!("  Descriptors::write_descriptor() - no other speed configuration descriptor configured");
                    // FIXME we should stall instead
                    usb.write(endpoint_number, [].into_iter())
                }
            }
            (DescriptorType::String, 0) => usb.write(
                endpoint_number,
                self.string_descriptor_zero
                    .iter()
                    .copied()
                    .take(requested_length),
            ),
            (DescriptorType::String, number) => {
                let offset_index: usize = (number - 1).into();
                if offset_index > self.string_descriptors.len() {
                    warn!(
                        "Descriptors::write_descriptor() stall - unknown string descriptor {}",
                        number
                    );
                    return Some(setup_packet);
                }
                usb.write(
                    endpoint_number,
                    self.string_descriptors[offset_index]
                        .iter()
                        .take(requested_length),
                )
            }
            _ => {
                warn!(
                    "  Descriptors::write_descriptor() stall - unhandled descriptor request {:?}, {}",
                    descriptor_type, descriptor_number
                );
                return Some(setup_packet);
            }
        };

        debug!("  wrote {} byte descriptor", bytes_written);

        // consumed
        None
    }
}
