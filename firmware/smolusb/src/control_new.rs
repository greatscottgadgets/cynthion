#![allow(dead_code, unused_imports, unused_mut, unused_variables)]

use core::marker::PhantomData;

use log::{debug, error, info, warn};

use crate::device::Speed;
use crate::event::UsbEvent;
use crate::descriptor::*;
use crate::setup::{Direction, Request, RequestType, SetupPacket};
use crate::traits::{AsByteSliceIterator, UsbDriver};

use ladybug::Channel;

// - Control ------------------------------------------------------------------

#[derive(Debug)]
pub enum Callback {
    SetAddress(u8),
    EndpointOutPrimeReceive(u8),
    EndpointInSendZLP(u8),
    Ack(u8, Direction),
}

impl Callback {
    pub fn call<D>(&self, usb: &D, control_state: State) -> State
    where
        D: UsbDriver
    {
        use Callback::*;
        //info!("  callback {:?}", self);
        ladybug::trace(Channel::B, 5, || {
            match *self {
                SetAddress(address) => {
                    usb.set_address(address);
                    State::Idle
                }
                Ack(endpoint_number, Direction::DeviceToHost) |
                EndpointOutPrimeReceive(endpoint_number) => {
                    // DeviceToHost - IN request,  prime the endpoint because the host will send a zlp to the device
                    usb.ack(endpoint_number, Direction::DeviceToHost);
                    control_state
                }
                Ack(endpoint_number, Direction::HostToDevice) |
                EndpointInSendZLP(endpoint_number) => {
                    // HostToDevice - OUT request, send a ZLP from the device to the host
                    usb.ack(endpoint_number, Direction::HostToDevice);
                    control_state
                }
            }
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum State {
    Idle,
    Data(Direction, SetupPacket),
    Status(Direction),
    Stalled,
}

impl core::fmt::Display for State {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            State::Data(direction, _) => {
                write!(f, "Data({:?})", direction)
            }
            state => {
                core::fmt::Debug::fmt(self, f)
            }
        }
    }
}

pub struct Control<'a, D, const RX_BUFFER_SIZE: usize> {
    endpoint_number: u8,
    descriptors: Descriptors<'a>,

    state: State,
    cb_send_complete: Option<Callback>,
    cb_receive_packet: Option<Callback>,
    configuration: Option<u8>,

    rx_buffer: [u8; RX_BUFFER_SIZE],
    rx_buffer_position: usize,

    _marker: PhantomData<&'a D>,
}


impl<'a, D, const RX_BUFFER_SIZE: usize> Control<'a, D, RX_BUFFER_SIZE>
where
    D: UsbDriver
{
    pub fn new(endpoint_number: u8, descriptors: Descriptors<'a>) -> Self {
        Self {
            endpoint_number,
            descriptors: descriptors.set_total_lengths(), // TODO figure out a better solution

            state: State::Idle,
            cb_send_complete: None,
            cb_receive_packet: None,
            configuration: None,

            rx_buffer: [0; RX_BUFFER_SIZE],
            rx_buffer_position: 0,

            _marker: PhantomData,
        }
    }

    fn set_state(& mut self, state: State) {
        self.state = state;
    }

    pub fn handle_event(&mut self, usb: &D, event: UsbEvent) -> Option<(SetupPacket, &[u8])> {
        if matches!(self.state, State::Stalled) {
            // unstall endpoint?
            error!("Control::handle_event() is in stalled state. Dropping event: {:?}", event);
            return None;
        }

        use UsbEvent::*;
        match event {
            BusReset => {
                self.bus_reset();
                None
            }
            ReceiveControl(endpoint_number) => {
                if endpoint_number != self.endpoint_number {
                    error!("event endpoint does not match control endpoint");
                }
                let mut buffer = [0_u8; 8];
                let bytes_read = usb.read_control(&mut buffer);
                if bytes_read != 8 {
                    error!("Received {} bytes for Setup packet. Dropping control event.", bytes_read);
                    return None;
                }
                let setup_packet = SetupPacket::from(buffer);
                self.receive_control(usb, setup_packet).map(|setup_packet| {
                    (setup_packet, &[] as &[u8])
                })
            }
            #[cfg(feature="chonky_events")]
            ReceiveSetupPacket(endpoint_number, setup_packet) => {
                if endpoint_number != self.endpoint_number {
                    error!("event endpoint does not match control endpoint");
                }
                self.receive_control(usb, setup_packet).map(|setup_packet| {
                    (setup_packet, &[] as &[u8])
                })
            }
            ReceivePacket(endpoint_number) => {
                if endpoint_number != self.endpoint_number {
                    error!("event endpoint does not match control endpoint");
                }
                let mut packet_buffer: [u8; 512] = [0; 512];
                let bytes_read = usb.read(self.endpoint_number, &mut packet_buffer);

                // pulse zlp reads
                if bytes_read == 0 {
                    ladybug::trace(Channel::B, 7, || {
                    });
                }

                let result = self.receive_packet(usb, &packet_buffer[..bytes_read]);

                result
            }
            #[cfg(feature="chonky_events")]
            ReceiveBuffer(endpoint_number, bytes_read, packet_buffer) => {
                if endpoint_number != self.endpoint_number {
                    error!("event endpoint does not match control endpoint");
                }
                self.receive_packet(usb, &packet_buffer[..bytes_read])
            }
            SendComplete(endpoint_number) => {
                if endpoint_number != self.endpoint_number {
                    error!("event endpoint does not match control endpoint");
                }
                self.send_complete(usb);
                None
            }
        }
    }


    // - bus reset ------------------------------------------------------------

    fn bus_reset(&mut self) {
        // TODO use Default so this doesn't need to be maintained
        self.state = State::Idle;
        self.cb_send_complete = None;
        self.cb_receive_packet = None;
        self.configuration = None;
        self.rx_buffer = [0; RX_BUFFER_SIZE];
        self.rx_buffer_position = 0;
    }

    // - receive control ------------------------------------------------------

    fn receive_control(&mut self, usb: &D, setup_packet: SetupPacket) -> Option<SetupPacket> {
        if !matches!(self.state, State::Idle) {
            warn!("Control::receive_control() not idle");
        }

        let direction = setup_packet.direction();
        let length: usize = setup_packet.length as usize;

        // check for data stage
        if length > 0 {
            self.set_state(State::Data(direction, setup_packet));
        } else {
            self.set_state(State::Status(direction));
        }

        // try to handle setup packet
        let request_type = setup_packet.request_type();
        let request = setup_packet.request();

        debug!(
            "Starting {} {} bytes, {:?} {:?} {:?} ",
            self.state, length, request_type, request, setup_packet.value.to_le_bytes()
        );

        match (direction, &request_type, &request) {
            (Direction::DeviceToHost, RequestType::Standard, Request::GetDescriptor) => {
                // register callback for successful transmission to host -> Prime ep_out for host zlp
                self.cb_send_complete = Some(Callback::EndpointOutPrimeReceive(self.endpoint_number));

                // write descriptor
                match self.descriptors.write(usb, self.endpoint_number, setup_packet) {
                    None => {
                        // request handled, consumed
                    }
                    Some(setup_packet) => {
                        // unknown so we need to pass it back to the caller for handling
                        return Some(setup_packet);
                    }
                }
            }

            (Direction::HostToDevice, RequestType::Standard, Request::SetAddress) => {
                // register callback for successful zlp to host -> Set device address
                let address: u8 = (setup_packet.value & 0x7f) as u8;
                self.cb_send_complete = Some(Callback::SetAddress(address));

                // send ZLP to host to end status stage
                usb.ack(self.endpoint_number, Direction::HostToDevice);
            }

            (Direction::HostToDevice, RequestType::Standard, Request::SetConfiguration) => {
                let configuration: u8 = setup_packet.value as u8;
                if configuration > 1 {
                    warn!(
                        "Request::SetConfiguration stall - unknown configuration {}",
                        configuration
                    );
                    self.configuration = None;
                    usb.stall_control_request();
                    self.set_state(State::Stalled);
                    return None;
                } else {
                    self.configuration = Some(configuration);
                }

                // send ZLP to host to end status stage
                usb.ack(self.endpoint_number, Direction::HostToDevice);
            }

            (Direction::DeviceToHost, RequestType::Standard, Request::GetConfiguration) => {
                if let Some(configuration) = self.configuration {
                    usb.write_ref(self.endpoint_number, [configuration].iter());
                } else {
                    usb.write_ref(self.endpoint_number, [0].iter());
                }

                // prepare to receive ZLP from host to end status stage
                usb.ack(self.endpoint_number, Direction::DeviceToHost);
            }

            (_, RequestType::Standard, Request::ClearFeature) => { // TODO Direction ?
                info!("  Request::ClearFeature {:?}", direction);
                // TODO
            }

            (_, RequestType::Standard, Request::SetFeature) => { // TODO Direction ?
                info!("  Request::SetFeature {:?}", direction);
                // TODO
            }

            (_, RequestType::Standard, Request::GetStatus) => { // TODO Direction ?
                info!("  Request::GetStatus {:?}", direction);
                // TODO
            }

            // unknown requests
            (Direction::HostToDevice, _, _) => {
                if length == 0 {
                    // no incoming data from host so we can pass it back to the caller for handling
                    warn!("  TODO should this ever happen?");
                    return Some(setup_packet);
                } else {
                    // has incoming data from host so we should hold on to it for now
                    // ... and prime for reception
                    self.rx_buffer_position = 0;
                    usb.ep_out_prime_receive(self.endpoint_number);
                }
            }
            (Direction::DeviceToHost, _, _) => {
                // no incoming data from host so we can pass it back to the caller for handling
                return Some(setup_packet);
            }
        }

        // consumed
        None
    }

    // - receive packet -------------------------------------------------------

    fn receive_packet(&mut self, usb: &D, packet_buffer: &[u8]) -> Option<(SetupPacket, &[u8])> {
        // execute any receive packet callback we may have registered
        if let Some(callback) = self.cb_receive_packet.take() {
            let new_state = callback.call(usb, self.state);
            self.set_state(new_state);
            if matches!(self.state, State::Idle) {
                // we are done here
                return None;
            }
        }

        let bytes_read = packet_buffer.len();

        debug!("  receive_packet() {} ({} bytes)", self.state, bytes_read);

        // handle the packet
        match self.state {
            State::Data(Direction::DeviceToHost, setup_packet) => {
                // this should NOT be firing after GetDescriptor
                warn!("  receive_packet() TODO Data(DeviceToHost) {} bytes", bytes_read);
                /*if bytes_read != 0 {
                    warn!("  TODO this should not have happened");
                }
                self.set_state(State::Idle);*/
            }
            State::Status(Direction::DeviceToHost) => {
                // we received a zlp from the host acknowledging the successful
                // completion of an IN data stage. TODO check bytes_read == 0?
                // e.g. after receipt of GetDescriptor data
                if bytes_read != 0 {
                    warn!("  TODO this should also not have happened");
                }
                self.set_state(State::Idle);
            }

            State::Data(Direction::HostToDevice, setup_packet) => {
                if bytes_read == 0 { // && rx_buffer_position > 0
                    info!("  zlp from host in {} - TODO early abort?", self.state);
                    /*self.set_state(State::Idle);
                    let offset = self.rx_buffer_position;
                    let rx_buffer = &self.rx_buffer[..offset];
                    self.rx_buffer_position = 0;
                    return Some((setup_packet, rx_buffer))*/
                }

                // we received a data packet from the host as part of an OUT control transfer.
                let endpoint_number = self.endpoint_number;
                let result = self.append_packet(setup_packet, packet_buffer);
                match result {
                    Some(rx_buffer) => {
                        // transfer is complete
                        // send ZLP to host to end data/status??? stage
                        usb.ack(endpoint_number, Direction::HostToDevice);
                        debug!("  OUT control transfer done, sent zlp");
                        return Some((setup_packet, rx_buffer))
                    }
                    None => {
                        // still expecting more data, prime for reception
                        usb.ep_out_prime_receive(endpoint_number);
                    }
                }
            }
            State::Status(Direction::HostToDevice) => {
                warn!("  receive_packet() TODO Status(HostToDevice)");
            }

            State::Stalled => {
                warn!("  receive_packet() TODO Stalled state");
            }
            State::Idle => {
                warn!("  receive_packet() should not be in Idle state");
            }
        }

        None
    }

    fn append_packet(
        &mut self,
        setup_packet: SetupPacket,
        packet_buffer: &[u8]
    ) -> Option<&[u8]> {
        let bytes_read = packet_buffer.len();
        let bytes_expected = setup_packet.length as usize;

        debug!("  append_packet() {} bytes ({}/{})", bytes_read, bytes_read + self.rx_buffer_position, bytes_expected);

        // guards
        if bytes_read == 0 { // && rx_buffer_position > 0
            warn!("  TODO host has ended data stage early");
        } else if self.rx_buffer_position + bytes_read > RX_BUFFER_SIZE {
            error!("  TODO receive buffer overflow");
        }

        // append packet to Control rx_buffer
        let mut offset = self.rx_buffer_position;
        self.rx_buffer[offset..offset + bytes_read].copy_from_slice(&packet_buffer[..bytes_read]);
        offset += bytes_read;

        // are we done yet?
        if offset >= bytes_expected {
            self.rx_buffer_position = 0;
            // should be set from send_complete for the zlp we're sending
            //self.set_state(State::Idle);
            Some(&self.rx_buffer[..bytes_expected])
        } else {
            // still waiting for more data...
            self.rx_buffer_position = offset;
            None
        }
    }

    // - send complete --------------------------------------------------------

    fn send_complete(&mut self, usb: &D) {
        debug!("  send_complete()  {}", self.state);

        // execute any send complete callback we may have registered
        if let Some(callback) = self.cb_send_complete.take() {
            let new_state = callback.call(usb, self.state);
            self.set_state(new_state);
            if matches!(self.state, State::Idle) {
                // we are done here
                return;
            }
        }

        match self.state {
            State::Data(Direction::DeviceToHost, setup_packet) => {
                // we sent a packet of IN data
                // e.g. after sending GetDescriptor data
                self.set_state(State::Status(Direction::DeviceToHost))
            }
            State::Status(Direction::DeviceToHost) => {
                warn!("  send_complete() TODO Status(DeviceToHost)");
            }

            State::Data(Direction::HostToDevice, setup_packet) => {
                // we sent a zlp to the host ackowledging the completiong of a
                // transaction with an OUT data stage
                // e.g. after receiving an OUT control transfer
                self.set_state(State::Idle);
            }
            State::Status(Direction::HostToDevice) => {
                // we sent a zlp to the host acknowledging the completion of an OUT status stage
                // e.g. after SetAddress, SetConfiguration
                self.set_state(State::Idle);
            }

            State::Stalled => {
                warn!("  send_complete() TODO Stalled state");
            }
            State::Idle => {
                warn!("  send_complete() should not be in Idle state");
            }
        }

        /*
        match self.state {
            // we sent a ZLP to end the status stage
            State::Status(direction) => {
                self.set_state(State::Idle);
            }

            // we sent some data and now we can enter the Status stage
            State::Data(direction, setup_packet) => {
                self.set_state(State::InStatus);
            }

            _ => {
                // TODO
                info!("send_complete() unhandled state: {:?}", self.state);
            }
        }*/

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

        // if the host is requesting less than the maximum amount of data,
        // only respond with the amount requested
        let requested_length = setup_packet.length as usize;

        let bytes_written = match (&descriptor_type, descriptor_number) {
            (DescriptorType::Device, 0) => {
                usb.write_ref(
                    endpoint_number,
                    self.device_descriptor.as_iter().take(requested_length)
                )
            },
            (DescriptorType::Configuration, _) => {
                usb.write_ref(
                    endpoint_number,
                    self.configuration_descriptor.iter().take(requested_length),
                )
            },
            (DescriptorType::DeviceQualifier, _) => {
                if self.device_speed == Speed::High {
                    if let Some(descriptor) = &self.device_qualifier_descriptor {
                        usb.write_ref(
                            endpoint_number,
                            descriptor.as_iter().take(requested_length)
                        )
                    } else {
                        // no device qualifier configured, ack HostToDevice instead - TODO check check on mac/windows
                        debug!("  No device qualifier configured for high-speed device");
                        usb.write(endpoint_number, [].into_iter())
                    }
                } else {
                    // for full/low speed devices, ack HostToDevice instead - TODO check on mac/windows
                    debug!("  Device qualifier request is not supported for full/low-speed devices");
                    usb.write(endpoint_number, [].into_iter())
                }
            }
            (DescriptorType::OtherSpeedConfiguration, _) => {
                if let Some(descriptor) = self.other_speed_configuration_descriptor {
                    usb.write_ref(endpoint_number, descriptor.iter().take(requested_length))
                } else {
                    // no other speed configuration, ack HostToDevice instead - TODO check check on mac/windows
                    debug!("  Descriptors::write_descriptor() - no other speed configuration descriptor configured");
                    usb.write(endpoint_number, [].into_iter())
                }
            }
            (DescriptorType::String, 0) => {
                usb.write_ref(
                    endpoint_number,
                    self.string_descriptor_zero.iter().take(requested_length)
                )
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
