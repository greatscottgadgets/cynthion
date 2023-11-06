#![allow(dead_code, unused_imports, unused_mut, unused_variables)]

use core::marker::PhantomData;

use log::{error, info, warn};

use crate::device::Speed;
use crate::event::UsbEvent;
use crate::descriptor::*;
use crate::setup::{Direction, Request, RequestType, SetupPacket};
use crate::traits::{AsByteSliceIterator, UsbDriver};

use ladybug::Channel;

// - Control ------------------------------------------------------------------

pub enum Callback {
    SetAddress(u8),
    EndpointOutPrimeReceive(u8),
    EndpointInSendZLP(u8),
    Ack(u8, Direction),
}

impl Callback {
    pub fn call<D>(&self, usb: &D, control_state: ControlState) -> ControlState
    where
        D: UsbDriver
    {
        use Callback::*;
        match *self {
            SetAddress(address) => {
                usb.set_address(address);
                ControlState::Idle
            }
            Ack(endpoint_number, Direction::DeviceToHost) |
            EndpointOutPrimeReceive(endpoint_number) => {
                // DeviceToHost - IN request,  prime the endpoint because the host will send a zlp to the device
                ladybug::trace(Channel::B, 2, || {
                    usb.ep_out_prime_receive(endpoint_number);
                });
                control_state
            }
            Ack(endpoint_number, Direction::HostToDevice) |
            EndpointInSendZLP(endpoint_number) => {
                // HostToDevice - OUT request, send a ZLP from the device to the host
                ladybug::trace(Channel::B, 1, || {
                    usb.write(endpoint_number, [].into_iter());
                });
                control_state
            }
        }
    }
}

// debug enabled wrappers
#[inline(always)]
fn usb_ack<D>(usb: &D, endpoint_number: u8, direction: Direction)
where
    D: UsbDriver
{
    ladybug::trace(Channel::B, 5, || {
        match direction {
            Direction::DeviceToHost => {
                // DeviceToHost - IN request,  prime the endpoint because the host will send a zlp to the device
                ladybug::trace(Channel::B, 2, || {
                    usb.ep_out_prime_receive(endpoint_number);
                });
            }
            Direction::HostToDevice => {
                // HostToDevice - OUT request, send a ZLP from the device to the host
                ladybug::trace(Channel::B, 1, || {
                    usb.write(endpoint_number, [].into_iter());
                });
            }
        }
    });
}

#[derive(Clone, Copy, PartialEq)]
pub enum ControlState {
    Idle,
    Setup,
    Data,
    Status,
    Stalled,
    Error, // what is Error if not Stalled?
}

pub struct Control<'a, D, const MAX_RECEIVE_SIZE: usize> {
    endpoint_number: u8,
    descriptors: Descriptors<'a>,

    state: ControlState,
    cb_send_complete: Option<Callback>,
    cb_receive_packet: Option<Callback>,
    configuration: Option<u8>,

    _marker: PhantomData<&'a D>,
}


impl<'a, D, const MAX_RECEIVE_SIZE: usize> Control<'a, D, MAX_RECEIVE_SIZE>
where
    D: UsbDriver
{
    pub fn new(endpoint_number: u8, descriptors: Descriptors<'a>) -> Self {
        Self {
            endpoint_number,
            descriptors: descriptors.set_total_lengths(), // TODO figure out a better solution

            state: ControlState::Idle,
            cb_send_complete: None,
            cb_receive_packet: None,
            configuration: None,

            _marker: PhantomData,
        }
    }

    pub fn handle_event(&mut self, usb: &D, event: UsbEvent) -> Option<(SetupPacket, &[u8])> {
        if self.state == ControlState::Error {
            // stall endpoint and drop event I'd assume ?
            return None;
        }

        // TODO sanity check endpoint_numbers here
        use UsbEvent::*;
        match event {
            BusReset => {
                self.handle_bus_reset();
            }
            ReceiveControl(endpoint_number) => {
                if endpoint_number != self.endpoint_number {
                    error!("event endpoint does not match control endpoint");
                }
                let mut buffer = [0_u8; 8];
                let bytes_read = usb.read_control(&mut buffer);
                if bytes_read != 8 {
                    error!("Received {} bytes for Setup packet.", bytes_read);
                    error!("Dropping control event");
                    // FIXME are we in error state now?
                    // won't know until we can replicate the double ep_control irq...
                    return None;
                }
                let setup_packet = SetupPacket::from(buffer);
                return self.handle_receive_setup_packet(usb, setup_packet).map(|setup_packet| {
                    (setup_packet, &[] as &[u8])
                });
            }
            ReceiveSetupPacket(endpoint_number, setup_packet) => {
                if endpoint_number != self.endpoint_number {
                    error!("event endpoint does not match control endpoint");
                }
                return self.handle_receive_setup_packet(usb, setup_packet).map(|setup_packet| {
                    (setup_packet, &[] as &[u8])
                });
            }
            ReceivePacket(endpoint_number) => {
                if endpoint_number != self.endpoint_number {
                    error!("event endpoint does not match control endpoint");
                }
                match self.handle_receive_packet(usb) {
                    None => {
                        // consumed
                    }
                    Some((setup_packet, rx_buffer)) => {
                        // this was a control transfer with data, give it back
                        return Some((setup_packet, rx_buffer));
                    }
                }
            }
            SendComplete(endpoint_number) => {
                if endpoint_number != self.endpoint_number {
                    error!("event endpoint does not match control endpoint");
                }
                self.handle_send_complete(usb);
            }
        }

        None
    }

    fn handle_bus_reset(&mut self) {
        self.state = ControlState::Idle;
        self.cb_send_complete = None;
        self.cb_receive_packet = None;
        self.configuration = None;
    }

    fn handle_receive_setup_packet(&mut self, usb: &D, setup_packet: SetupPacket) -> Option<SetupPacket> {
        // TODO if not idle, stall ?
        /*if self.state != ControlState::Idle {
            error!("Control::handle_receive_setup_packet() stall - not idle");
            self.state = ControlState::Stalled;
            return Some(setup_packet);
        }*/

        // enter the setup stage
        self.state = ControlState::Setup;

        // parse setup packet
        let request_type = setup_packet.request_type();
        let request = setup_packet.request();

        // handle request
        match (&request_type, &request) {
            (RequestType::Standard, Request::GetDescriptor) => { // DeviceToHost
                // enter data stage
                self.state = ControlState::Data;

                // register callback for successful transmission to host -> Prime ep_out for host zlp
                self.cb_send_complete = Some(Callback::EndpointOutPrimeReceive(self.endpoint_number));

                // write descriptor and enter data stage
                ladybug::trace(Channel::B, 1, || {
                    match self.descriptors.write(usb, self.endpoint_number, setup_packet) {
                        None => {
                            // request handled, consumed
                            None
                        }
                        Some(setup_packet) => {
                            // TODO here we can either stall or even give it back to the caller for handling...
                            // hrmmm... decisions...
                            return Some(setup_packet);
                        }
                    }
                    // check state
                });
            }

            (RequestType::Standard, Request::SetAddress) => { // HostToDevice
                // register callback for successful zlp to host -> Set device address
                let address: u8 = (setup_packet.value & 0x7f) as u8;
                self.cb_send_complete = Some(Callback::SetAddress(address));

                // send ZLP to host to end status stage
                self.state = ControlState::Status;
                usb_ack(usb, 0, Direction::HostToDevice);
            }

            (RequestType::Standard, Request::SetConfiguration) => { // HostToDevice
                let configuration: u8 = setup_packet.value as u8;
                if configuration > 1 {
                    warn!(
                        "Request::SetConfiguration stall - unknown configuration {}",
                        configuration
                    );
                    self.configuration = None;
                    usb.stall_control_request();
                    self.state = ControlState::Stalled; // TODO is any of this right?
                    return None;
                } else {
                    self.configuration = Some(configuration);
                }

                // send ZLP to host to end status stage
                self.state = ControlState::Status;
                usb_ack(usb, 0, Direction::HostToDevice);
            }

            (RequestType::Standard, Request::GetConfiguration) => { // DeviceToHost
                ladybug::trace(Channel::B, 1, || {
                    if let Some(configuration) = self.configuration {
                        usb.write_ref(0, [configuration].iter());
                    } else {
                        usb.write_ref(0, [0].iter());
                    }
                });

                // prepare to receive ZLP from host to end status stage
                self.state = ControlState::Status;
                usb_ack(usb, 0, Direction::DeviceToHost);
            }

            (RequestType::Standard, Request::ClearFeature) => { // TODO Direction ?
                info!("Request::ClearFeature {:?}", setup_packet.direction());
                // TODO
            }

            (RequestType::Standard, Request::SetFeature) => { // TODO Direction ?
                info!("Request::SetFeature {:?}", setup_packet.direction());
                // TODO
            }

            (RequestType::Standard, Request::GetStatus) => { // TODO Direction ?
                info!("Request::GetStatus {:?}", setup_packet.direction());
                // TODO
            }

            _ => {
                // not supported, pass it back to the caller for handling
                log::debug!(
                    "Control::handle_receive_setup_packet() - unsupported request {:?} {:?}",
                    request_type,
                    request
                );
                return Some(setup_packet);
            }
        }

        // consumed
        None

/*
        // if we have a response, we can now enter the Data stage {
            self.state = ControlState::Data;
            // ... and we can send our response
            // usb.write()
        // } else { // otherwise, enter the Status stage
            self.state = ControlState::Status;
        //}
*/
    }

    fn handle_receive_packet(&mut self, usb: &D) -> Option<(SetupPacket, &[u8])> {
        ladybug::trace(Channel::B, 3, || {
            let mut rx_buffer: [u8; MAX_RECEIVE_SIZE] = [0; MAX_RECEIVE_SIZE];
            let bytes_read = usb.read(self.endpoint_number, &mut rx_buffer);
            if bytes_read == 0 {
                // it's an ack
            } else {
                info!(
                    "USB0_EP_OUT received packet on endpoint:{} bytes_read:{}",
                    self.endpoint_number, bytes_read
                );
            }
        });

        // FIXME make sure we always prime for next packet, at the moment it's getting lost
        ladybug::trace(Channel::B, 2, || {
            usb.ep_out_prime_receive(self.endpoint_number);
        });

        // execute any receive packet callback we may have registered
        if let Some(callback) = self.cb_receive_packet.take() {
            ladybug::trace(Channel::B, 4, || {
                self.state = callback.call(usb, self.state);
            });
            return None;
        }

        // TODO check below

        // if the host has finished sending data we can enter status stage
        if /* packet.len() == 0 || packet.len() < max_packet_size */ self.state == ControlState::Data {
            self.state = ControlState::Status;
        }

        if /*packet.len() == max_packet_size && */ self.state == ControlState::Data {
            // the host is still sending data, buffer it and carry on
            self.state  = ControlState::Data;
        }

        // if the host ended the status stage by sending a ZLP we can end the status stage
        if /* packet.len() == 0  */ self.state == ControlState::Status {
            // all done
            self.state = ControlState::Idle;
        }

        None

    }


    fn handle_send_complete(&mut self, usb: &D) {
        // execute any send complete callback we may have registered
        if let Some(callback) = self.cb_send_complete.take() {
            ladybug::trace(Channel::B, 4, || {
                self.state = callback.call(usb, self.state);
            });
            return;
        }

        // TODO check below

        // we sent a ZLP to end the status stage
        if self.state == ControlState::Status {
            self.state = ControlState::Idle;
        }

        // we sent some data and now we can enter the Status stage
        if self.state == ControlState::Data {
            self.state = ControlState::Status;
        }
    }

}



// - Descriptors --------------------------------------------------------------

//#[derive(Clone, Copy)]
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
        if let Some(mut other_speed_configuration_descriptor) = self.other_speed_configuration_descriptor.as_mut() {
            other_speed_configuration_descriptor.set_total_length();
        }
        self
    }

    pub fn write<D>(&self, usb: &D, endpoint_number: u8, setup_packet: SetupPacket) -> Option<SetupPacket>
    where
        D: UsbDriver
    {
        // extract the descriptor type and number from our SETUP request
        let [descriptor_number, descriptor_type_bits] = setup_packet.value.to_le_bytes();
        let descriptor_type = match DescriptorType::try_from(descriptor_type_bits) {
            Ok(descriptor_type) => descriptor_type,
            Err(e) => {
                warn!(
                    "Descriptors::write_descriptor() stall - invalid descriptor type: {} {}",
                    descriptor_type_bits, descriptor_number
                );
                usb.stall_control_request();
                return Some(setup_packet);
            }
        };

        //info!("  {:?} #{}", descriptor_type, descriptor_number);

        // if the host is requesting less than the maximum amount of data,
        // only respond with the amount requested
        let requested_length = setup_packet.length as usize;

        match (&descriptor_type, descriptor_number) {
            (DescriptorType::Device, 0) => {
                usb.write_ref(
                    endpoint_number,
                    self.device_descriptor.as_iter().take(requested_length)
                );
            },
            (DescriptorType::Configuration, _) => {
                usb.write_ref(
                    endpoint_number,
                    self.configuration_descriptor.iter().take(requested_length),
                );
            },
            (DescriptorType::DeviceQualifier, _) => {
                if self.device_speed == Speed::High {
                    if let Some(descriptor) = &self.device_qualifier_descriptor {
                        usb.write_ref(
                            endpoint_number,
                            descriptor.as_iter().take(requested_length)
                        );
                    } else {
                        // no device qualifier configured, ack HostToDevice instead - TODO check check on mac/windows
                        warn!("No device qualifier configured for high-speed device");
                        usb.write(endpoint_number, [].into_iter());
                    }
                } else {
                    // for full/low speed devices, ack HostToDevice instead - TODO check on mac/windows
                    warn!("Device qualifier request is not supported for full/low-speed devices");
                    usb.write(endpoint_number, [].into_iter());
                }
            }
            (DescriptorType::OtherSpeedConfiguration, _) => {
                if let Some(descriptor) = self.other_speed_configuration_descriptor {
                    usb.write_ref(endpoint_number, descriptor.iter().take(requested_length));
                } else {
                    // no other speed configuration, ack HostToDevice instead - TODO check check on mac/windows
                    warn!("Descriptors::write_descriptor() - no other speed configuration descriptor configured");
                    usb.write(endpoint_number, [].into_iter());
                }
            }
            (DescriptorType::String, 0) => {
                usb.write_ref(
                    endpoint_number,
                    self.string_descriptor_zero.iter().take(requested_length)
                );
            },
            (DescriptorType::String, number) => {
                let offset_index: usize = (number - 1).into();
                if offset_index > self.string_descriptors.len() {
                    warn!("Descriptors::write_descriptor() stall - unknown string descriptor {}", number);
                    return Some(setup_packet);
                }
                usb.write(
                    endpoint_number,
                    self.string_descriptors[offset_index]
                        .iter()
                        .take(requested_length),
                );
            }
            _ => {
                warn!(
                    "Descriptors::write_descriptor() stall - unhandled descriptor request {:?}, {}",
                    descriptor_type, descriptor_number
                );
                return Some(setup_packet);
            }
        }

        // consumed
        None
    }
}
