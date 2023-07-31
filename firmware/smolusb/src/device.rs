#![allow(dead_code, unused_imports, unused_variables)] // TODO

use crate::control::{Direction, Feature, Recipient, Request, RequestType, SetupPacket};
use crate::descriptor::*;
use crate::error::{SmolError, SmolResult};
use crate::traits::AsByteSliceIterator;
use crate::traits::{
    ControlRead, EndpointRead, EndpointWrite, EndpointWriteRef, UnsafeUsbDriverOperations,
    UsbDriverOperations,
};

use log::{debug, error, info, trace, warn};

use core::cell::RefCell;

///! `smolusb` device implementation for Luna USB peripheral
///!
///! TODO probably not all of this should live in the smolusb crate,
///! it should rather be split into generic and
///! implementation-specific parts

/// USB Speed
///
/// Note: These match the gateware peripheral so the mapping isn't particularly meaningful in other contexts.
///
/// TODO also, these don't match what I'm seeing from the host side ???
#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum Speed {
    Low = 2,        // 1.5 Mbps
    Full = 1,       //  12 Mbps
    High = 0,       // 480 Mbps
    SuperSpeed = 3, // 5/10 Gbps (includes SuperSpeed+)
}

impl From<u8> for Speed {
    fn from(value: u8) -> Self {
        match value & 0b11 {
            0 => Speed::High,
            1 => Speed::Full,
            2 => Speed::Low,
            3 => Speed::SuperSpeed,
            _ => unimplemented!(),
        }
    }
}

/// USB device state
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DeviceState {
    Reset,
    Address,
    Configured,
    Suspend,
}

/// A USB device
///
/// `UsbDevice` implements the control portion of the USB
/// specification and consists of:
///
///     * a hal driver
///     * a device descriptor
///     * a configuration descriptor
///     * a set of string descriptors
///
pub struct UsbDevice<'a, D> {
    pub hal_driver: D,
    device_descriptor: &'a DeviceDescriptor,
    configuration_descriptor: ConfigurationDescriptor<'a>,
    pub device_qualifier_descriptor: Option<&'a DeviceQualifierDescriptor>,
    pub other_speed_configuration_descriptor: Option<ConfigurationDescriptor<'a>>,
    string_descriptor_zero: &'a StringDescriptorZero<'a>,
    string_descriptors: &'a [&'a StringDescriptor<'a>],
    pub state: RefCell<DeviceState>,
    pub reset_count: usize,
    pub feature_remote_wakeup: bool,

    pub cb_class_request:
        Option<fn(device: &UsbDevice<'a, D>, setup_packet: &SetupPacket, request: u8)>,
    pub cb_vendor_request:
        Option<fn(device: &UsbDevice<'a, D>, setup_packet: &SetupPacket, request: u8)>,
    pub cb_string_request:
        Option<fn(device: &UsbDevice<'a, D>, setup_packet: &SetupPacket, index: u8)>,
}

impl<'a, D> UsbDevice<'a, D>
where
    D: ControlRead + EndpointRead + EndpointWrite + EndpointWriteRef + UsbDriverOperations,
{
    pub fn new(
        hal_driver: D,
        device_descriptor: &'a DeviceDescriptor,
        configuration_descriptor: &'a ConfigurationDescriptor<'a>,
        string_descriptor_zero: &'a StringDescriptorZero<'a>,
        string_descriptors: &'a [&'a StringDescriptor<'a>],
    ) -> Self {
        // Calculate and update descriptor length fields
        // TODO this ain't great but it will do for now
        let mut configuration_descriptor = configuration_descriptor.clone();
        let total_length = configuration_descriptor.set_total_length();

        Self {
            hal_driver,
            device_descriptor,
            configuration_descriptor,
            device_qualifier_descriptor: None,
            other_speed_configuration_descriptor: None,
            string_descriptor_zero,
            string_descriptors,
            state: DeviceState::Reset.into(),
            reset_count: 0,
            feature_remote_wakeup: false,

            cb_class_request: None,
            cb_vendor_request: None,
            cb_string_request: None,
        }
    }

    pub fn state(&self) -> DeviceState {
        *self.state.borrow()
    }
}

// Device functions
impl<'a, D> UsbDevice<'a, D>
where
    D: ControlRead + EndpointRead + EndpointWrite + EndpointWriteRef + UsbDriverOperations,
{
    pub fn connect(&self) -> Speed {
        self.hal_driver.connect().into()
    }

    pub fn disconnect(&self) {
        self.hal_driver.disconnect()
    }

    pub fn reset(&self) -> Speed {
        let speed = self.hal_driver.reset().into();
        // TODO self.reset_count += 1;
        self.state.replace(DeviceState::Reset.into());
        speed
    }

    pub fn bus_reset(&self) -> Speed {
        let speed = self.hal_driver.bus_reset().into();
        // TODO self.reset_count += 1;
        self.state.replace(DeviceState::Reset.into());
        speed
    }
}

// Handle SETUP packet
impl<'a, D> UsbDevice<'a, D>
where
    D: ControlRead
        + EndpointRead
        + EndpointWrite
        + EndpointWriteRef
        + UsbDriverOperations
        + UnsafeUsbDriverOperations,
{
    pub fn handle_setup_request(&self, _endpoint_number: u8, setup_packet: &SetupPacket) -> SmolResult<()> {
        let request_type = setup_packet.request_type();
        let request = setup_packet.request();

        match (&request_type, &request) {
            (RequestType::Standard, Request::SetAddress) => {
                self.handle_set_address(setup_packet)?;
            }
            (RequestType::Standard, Request::GetDescriptor) => {
                self.handle_get_descriptor(setup_packet)?;
            }
            (RequestType::Standard, Request::SetConfiguration) => {
                self.handle_set_configuration(setup_packet)?;
            }
            (RequestType::Standard, Request::GetConfiguration) => {
                self.handle_get_configuration(setup_packet)?;
            }
            (RequestType::Standard, Request::ClearFeature) => {
                self.handle_clear_feature(setup_packet)?;
            }
            (RequestType::Standard, Request::SetFeature) => {
                self.handle_set_feature(setup_packet)?;
            }
            (RequestType::Class, Request::ClassOrVendor(request)) => {
                if let Some(cb) = self.cb_class_request {
                    cb(self, setup_packet, *request);
                } else {
                    warn!(
                        "SETUP stall: unhandled class request {:?} {:?}",
                        request_type, request
                    );
                    self.hal_driver.stall_request();
                }
            }
            (RequestType::Vendor, Request::ClassOrVendor(request)) => {
                if let Some(cb) = self.cb_vendor_request {
                    cb(self, setup_packet, *request);
                } else {
                    warn!(
                        "SETUP stall: unhandled vendor request {:?} {:?}",
                        request_type, request
                    );
                    self.hal_driver.stall_request();
                }
            }
            _ => {
                warn!(
                    "SETUP stall: unhandled request {:?} {:?}",
                    request_type, request
                );
                self.hal_driver.stall_request();
            }
        }

        debug!(
            "SETUP {:?} {:?} {:?} {:?} 0x{:x} 0x{:x} {}",
            setup_packet.recipient(),
            setup_packet.direction(),
            request_type,
            request,
            setup_packet.value,
            setup_packet.index,
            setup_packet.length
        );

        Ok(())
    }

    // TODO move tx_ack_active flag logic to hal_driver
    fn handle_set_address(&self, setup_packet: &SetupPacket) -> SmolResult<()> {
        // set tx_ack_active flag
        // TODO a slighty safer approach would be nice
        unsafe {
            self.hal_driver.set_tx_ack_active();
        }

        // respond with ack status first before changing device address
        //self.hal_driver.ack_status_stage(setup_packet);
        self.hal_driver.ack(0, Direction::HostToDevice);

        // wait for the response packet to get sent
        // TODO a slightly safer approach would be nice
        loop {
            let active = unsafe { self.hal_driver.is_tx_ack_active() };
            if active == false {
                break;
            }
        }

        // activate new address
        let address: u8 = (setup_packet.value & 0x7f) as u8;
        self.hal_driver.set_address(address);
        self.state.replace(DeviceState::Address.into());

        Ok(())
    }

    fn handle_get_descriptor(&self, setup_packet: &SetupPacket) -> SmolResult<()> {
        // extract the descriptor type and number from our SETUP request
        let [descriptor_number, descriptor_type_bits] = setup_packet.value.to_le_bytes();
        let descriptor_type = match DescriptorType::try_from(descriptor_type_bits) {
            Ok(descriptor_type) => descriptor_type,
            Err(e) => {
                warn!(
                    "SETUP stall: invalid descriptor type: {} {}",
                    descriptor_type_bits, descriptor_number
                );
                self.hal_driver.stall_request();
                return Ok(());
            }
        };

        // if the host is requesting less than the maximum amount of data,
        // only respond with the amount requested
        let requested_length = setup_packet.length as usize;

        match (&descriptor_type, descriptor_number) {
            (DescriptorType::Device, 0) => self
                .hal_driver
                .write_ref(0, self.device_descriptor.as_iter().take(requested_length)),
            (DescriptorType::Configuration, 0) => self.hal_driver.write_ref(
                0,
                self.configuration_descriptor.iter().take(requested_length),
            ),
            (DescriptorType::DeviceQualifier, 0) => {
                if let Some(descriptor) = &self.device_qualifier_descriptor {
                    self.hal_driver
                        .write_ref(0, descriptor.as_iter().take(requested_length));
                } else {
                    warn!("SETUP stall: no device qualifier descriptor configured");
                    // TODO stall?
                }
            }
            (DescriptorType::OtherSpeedConfiguration, 0) => {
                if let Some(descriptor) = self.other_speed_configuration_descriptor {
                    self.hal_driver
                        .write_ref(0, descriptor.iter().take(requested_length));
                } else {
                    warn!("SETUP stall: no other speed configuration descriptor configured");
                    // TODO stall?
                }
            }
            (DescriptorType::String, 0) => self
                .hal_driver
                .write_ref(0, self.string_descriptor_zero.iter().take(requested_length)),
            (DescriptorType::String, index) => {
                let offset_index: usize = (index - 1).into();

                if offset_index > self.string_descriptors.len() {
                    if let Some(cb) = self.cb_string_request {
                        cb(self, setup_packet, index);
                    } else {
                        warn!("SETUP stall: unknown string descriptor {}", index);
                        self.hal_driver.stall_request();
                    }
                    return Ok(());
                }

                self.hal_driver.write(
                    0,
                    self.string_descriptors[offset_index]
                        .iter()
                        .take(requested_length),
                )
            }
            _ => {
                warn!(
                    "SETUP stall: unhandled descriptor {:?}, {}",
                    descriptor_type, descriptor_number
                );
                self.hal_driver.stall_request();
                return Ok(());
            }
        }

        self.hal_driver.ack_status_stage(setup_packet);

        trace!(
            "SETUP handle_get_descriptor({:?}({}), {}, {})",
            descriptor_type,
            descriptor_type_bits,
            descriptor_number,
            requested_length
        );

        Ok(())
    }

    fn handle_set_configuration(&self, setup_packet: &SetupPacket) -> SmolResult<()> {
        self.hal_driver.ack_status_stage(setup_packet);

        trace!("SETUP handle_set_configuration()");

        let configuration = setup_packet.value;
        if configuration > 1 {
            warn!("SETUP stall: unknown configuration {}", configuration);
            self.hal_driver.stall_request();
            return Ok(());
        }
        self.state.replace(DeviceState::Configured.into());

        Ok(())
    }

    fn handle_get_configuration(&self, setup_packet: &SetupPacket) -> SmolResult<()> {
        trace!("SETUP handle_get_configuration()");

        let requested_length = setup_packet.length as usize;

        self.hal_driver
            .write(0, [1].into_iter().take(requested_length));
        self.hal_driver.ack_status_stage(setup_packet);

        Ok(())
    }

    fn handle_clear_feature(&self, setup_packet: &SetupPacket) -> SmolResult<()> {
        // parse request
        let recipient = setup_packet.recipient();
        let feature_bits = setup_packet.value;
        let feature = match Feature::try_from(feature_bits) {
            Ok(feature) => feature,
            Err(e) => {
                warn!("SETUP stall: invalid clear feature type: {}", feature_bits);
                self.hal_driver.stall_request();
                return Ok(());
            }
        };

        match (&recipient, &feature) {
            (Recipient::Device, Feature::DeviceRemoteWakeup) => {
                // TODO self.feature_remote_wakeup = false;
            }
            (Recipient::Endpoint, Feature::EndpointHalt) => {
                let endpoint_address = setup_packet.index as u8;
                self.hal_driver
                    .clear_feature_endpoint_halt(endpoint_address);
                self.hal_driver.ack_status_stage(setup_packet);
                debug!(
                    "SETUP handle_clear_feature EndpointHalt: 0x{:x}",
                    endpoint_address
                );
            }
            _ => {
                warn!(
                    "SETUP stall: unhandled clear feature {:?}, {:?}",
                    recipient, feature
                );
                self.hal_driver.stall_request();
                return Ok(());
            }
        };

        Ok(())
    }

    fn handle_set_feature(&self, setup_packet: &SetupPacket) -> SmolResult<()> {
        trace!("SETUP handle_set_feature()");

        // parse request
        let recipient = setup_packet.recipient();
        let feature_bits = setup_packet.value;
        let feature = match Feature::try_from(feature_bits) {
            Ok(feature) => feature,
            Err(e) => {
                warn!("SETUP stall: invalid set feature type: {}", feature_bits);
                self.hal_driver.stall_request();
                return Ok(());
            }
        };

        match (&recipient, &feature) {
            (Recipient::Device, Feature::DeviceRemoteWakeup) => {
                // TODO self.feature_remote_wakeup = true;
            }
            _ => {
                warn!(
                    "SETUP stall: unhandled set feature {:?}, {:?}",
                    recipient, feature
                );
                self.hal_driver.stall_request();
                return Ok(());
            }
        };

        Ok(())
    }
}

/*
# Reference enumeration process (quirks merged from Linux, macOS, and Windows):
# - Read 8 bytes of device descriptor.
# + Read 64 bytes of device descriptor.
# + Set address.
# + Read exact device descriptor length.
# - Read device qualifier descriptor, three times.
# - Read config descriptor (without subordinates).
# - Read language descriptor.
# - Read Windows extended descriptors. [optional]
# - Read string descriptors from device descriptor (wIndex=language id).
# - Set configuration.
# - Read back configuration number and validate.

*/
