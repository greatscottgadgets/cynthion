//! `smolusb` device types
//!

use crate::descriptor::microsoft10;
use crate::descriptor::{
    ConfigurationDescriptor, DescriptorType, DeviceDescriptor, DeviceQualifierDescriptor,
    StringDescriptor, StringDescriptorNumber, StringDescriptorZero,
};
use crate::setup::SetupPacket;
use crate::traits::{AsByteSliceIterator, UsbDriver};
use log::{debug, trace, warn};

/// The set of descriptors describing a USB device.
pub struct Descriptors<'a> {
    // required
    pub device_speed: Speed,
    pub device_descriptor: DeviceDescriptor,
    pub configuration_descriptor: ConfigurationDescriptor<'a>,
    pub string_descriptor_zero: StringDescriptorZero<'a>,
    pub string_descriptors: &'a [&'a StringDescriptor<'a>],
    // optional
    pub device_qualifier_descriptor: Option<DeviceQualifierDescriptor>,
    pub other_speed_configuration_descriptor: Option<ConfigurationDescriptor<'a>>,
    pub microsoft10: Option<microsoft10::Descriptors<'a>>,
}

impl Descriptors<'_> {
    /// Calculates the total length of the descriptor and returns an updated instance.
    ///
    /// TODO ugly hack because I haven't figured out how to do this at compile time yet
    #[must_use]
    pub fn set_total_lengths(mut self) -> Self {
        self.configuration_descriptor.set_total_length();
        if let Some(other_speed_configuration_descriptor) =
            self.other_speed_configuration_descriptor.as_mut()
        {
            other_speed_configuration_descriptor.set_total_length();
        }
        self
    }

    /// Writes the descriptor corresponding to the request.
    ///
    /// Returns the given [`SetupPacket`] if the descriptor request could not be handled.
    #[allow(clippy::too_many_lines)] // ...and sometimes clippy has opinions it should keep to itself!
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
        let descriptor_type = DescriptorType::from(descriptor_type_bits);

        // if the host is requesting less than the maximum amount of data,
        // only respond with the amount requested
        let requested_length = setup_packet.length as usize;

        let bytes_written = match (&descriptor_type, descriptor_number) {
            (DescriptorType::Device, 0) => usb.write_requested(
                endpoint_number,
                requested_length,
                self.device_descriptor
                    .as_iter()
                    .copied()
                    .take(requested_length),
            ),
            (DescriptorType::Configuration, _) => usb.write_requested(
                endpoint_number,
                requested_length,
                self.configuration_descriptor
                    .iter()
                    .copied()
                    .take(requested_length),
            ),
            (DescriptorType::DeviceQualifier, _) => {
                if self.device_speed == Speed::High {
                    if let Some(descriptor) = &self.device_qualifier_descriptor {
                        usb.write_requested(
                            endpoint_number,
                            requested_length,
                            descriptor.as_iter().copied().take(requested_length),
                        )
                    } else {
                        // no device qualifier configured, ack HostToDevice instead - TODO check check on mac/windows
                        debug!("  No device qualifier configured for high-speed device");
                        usb.write(endpoint_number, [].into_iter())
                    }
                } else {
                    // for full/low speed devices, ack HostToDevice instead - TODO check on mac/windows
                    trace!(
                        "  Device qualifier request is not supported for full/low-speed devices"
                    );
                    // FIXME we should stall instead
                    usb.write(endpoint_number, [].into_iter())
                }
            }
            (DescriptorType::OtherSpeedConfiguration, _) => {
                if let Some(descriptor) = self.other_speed_configuration_descriptor {
                    usb.write_requested(
                        endpoint_number,
                        requested_length,
                        descriptor.iter().copied().take(requested_length),
                    )
                } else {
                    // no other speed configuration, ack HostToDevice instead - TODO check check on mac/windows
                    debug!("  Descriptors::write_descriptor() - no other speed configuration descriptor configured");
                    // FIXME we should stall instead
                    usb.write(endpoint_number, [].into_iter())
                }
            }
            (DescriptorType::String, StringDescriptorNumber::Zero) => usb.write_requested(
                endpoint_number,
                requested_length,
                self.string_descriptor_zero
                    .iter()
                    .copied()
                    .take(requested_length),
            ),
            (DescriptorType::String, StringDescriptorNumber::Microsoft) => {
                match &self.microsoft10 {
                    Some(descriptors) => usb.write_requested(
                        endpoint_number,
                        requested_length,
                        descriptors.string_descriptor.iter(),
                    ),
                    _ => {
                        warn!(
                            "Descriptors::write_descriptor() - no ms os 1.0 string descriptor defined",
                        );
                        usb.stall_endpoint_in(endpoint_number);
                        return Some(setup_packet);
                    }
                }
            }
            (DescriptorType::String, number) => {
                let offset_index: usize = (number - 1).into();
                if offset_index > self.string_descriptors.len() {
                    warn!(
                        "Descriptors::write_descriptor() - unknown string descriptor {}",
                        number
                    );
                    return Some(setup_packet);
                }
                usb.write_requested(
                    endpoint_number,
                    requested_length,
                    self.string_descriptors[offset_index]
                        .iter()
                        .take(requested_length),
                )
            }
            _ => {
                warn!(
                    "  Descriptors::write_descriptor() - unhandled descriptor request {:?}, {}",
                    descriptor_type, descriptor_number
                );
                return Some(setup_packet);
            }
        };

        trace!("  wrote {} byte descriptor", bytes_written);

        // consumed
        None
    }
}

/// USB device speed
///
/// Note: These match UTMI's `xcvr_select` constant so the mapping may not be correct for other contexts.
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
    /// Super Speed (5/10 Gbps - includes `SuperSpeed+`)
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
    #[must_use]
    pub fn from_libusb(value: u8) -> Self {
        match value {
            1 => Speed::Low,
            2 => Speed::Full,
            3 => Speed::High,
            4 | 5 => Speed::Super,
            _ => Speed::Unknown,
        }
    }

    #[must_use]
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
