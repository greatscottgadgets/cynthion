#![allow(dead_code, unused_imports, unused_variables)] // TODO

///! USB control interface
use log::{debug, error, trace};

use crate::error::{SmolError, SmolResult};
use crate::event::UsbEvent;
use crate::setup::{Direction, SetupPacket};
use crate::traits::UsbDriver;

/// Represents USB control transfer state.
#[derive(Debug)]
pub enum State {
    Idle,

    /// Device has received bus reset
    Reset,

    /// Device has received PID SETUP
    SetupStage,

    /// Device has received PID OUT
    OutDataStage(SetupPacket),

    /// Device has received PID IN
    InDataStage,

    /// Device has received PID STALL
    Stalled,

    /// Error(endpoint_number: u8)
    Error(u8),
}

/// Performs USB control transfers.
pub struct Control<'a, D, const MAX_RECEIVE_SIZE: usize> {
    state: State,
    rx_buffer: [u8; MAX_RECEIVE_SIZE],
    rx_buffer_position: usize,

    //driver: &'a D,
    _marker: core::marker::PhantomData<&'a D>,
}

impl<'a, D, const MAX_RECEIVE_SIZE: usize> Control<'a, D, MAX_RECEIVE_SIZE>
where
    D: UsbDriver,
{
    pub fn new() -> Self {
        Self {
            //driver: driver,
            state: State::Idle,
            _marker: core::marker::PhantomData,

            rx_buffer: [0; MAX_RECEIVE_SIZE],
            rx_buffer_position: 0,
        }
    }
}

// - event dispatch -----------------------------------------------------------

pub struct ControlEvent<'a, const MAX_RECEIVE_SIZE: usize> {
    pub endpoint_number: u8,
    pub setup_packet: SetupPacket,
    pub data: [u8; MAX_RECEIVE_SIZE],
    pub bytes_read: usize,
    pub _marker: core::marker::PhantomData<&'a ()>,
}

impl<'a, const MAX_RECEIVE_SIZE: usize> core::fmt::Debug for ControlEvent<'a, MAX_RECEIVE_SIZE> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "ControlResponse {{ endpoint_number: {}, setup_packet: {:?}, data: {:?} }}",
            self.endpoint_number,
            self.setup_packet,
            &self.data[..self.bytes_read]
        )
    }
}

impl<'a, D, const MAX_RECEIVE_SIZE: usize> Control<'a, D, MAX_RECEIVE_SIZE>
where
    D: UsbDriver,
{
    pub fn dispatch(
        &mut self,
        driver: &D,
        event: UsbEvent,
    ) -> SmolResult<Option<ControlEvent<'a, MAX_RECEIVE_SIZE>>> {
        trace!("CONTROL dispatch({:?})", event);

        match event {
            UsbEvent::BusReset => {
                self.handle_usb_bus_reset(driver)?;
                Ok(None)
            }
            UsbEvent::ReceiveControl(endpoint_number) => {
                match self.handle_receive_setup_packet(driver, endpoint_number)? {
                    Some(setup_packet) => Ok(Some(ControlEvent {
                        endpoint_number,
                        setup_packet,
                        data: self.rx_buffer,
                        bytes_read: 0,
                        _marker: core::marker::PhantomData,
                    })),
                    None => Ok(None),
                }
            }
            UsbEvent::ReceivePacket(endpoint_number) => {
                match self.handle_receive_packet(driver, endpoint_number)? {
                    Some((setup_packet, data)) => {
                        let bytes_read = data.len();
                        Ok(Some(ControlEvent {
                            endpoint_number,
                            setup_packet,
                            data: self.rx_buffer,
                            bytes_read,
                            _marker: core::marker::PhantomData,
                        }))
                    }
                    None => Ok(None),
                }
            }
            UsbEvent::SendComplete(endpoint_number) => {
                self.handle_send_complete(driver, endpoint_number)?;
                Ok(None)
            }
            event => { // TODO handle ReceiveSetupPacket
                log::warn!("CONTROL dispatch() unhandled event: {:?}", event);
                Ok(None)
            }
        }
    }

    pub fn foo(&'a mut self) -> &'a [u8] {
        &self.rx_buffer
    }

    // USBx
    pub fn handle_usb_bus_reset(&mut self, driver: &D) -> SmolResult<()> {
        driver.bus_reset();
        self.state = State::Reset;
        log::info!("CONTROL handle_usb_bus_reset");
        Ok(())
    }

    // USBx_EP_CONTROL n
    pub fn handle_receive_setup_packet(
        &mut self,
        driver: &D,
        endpoint_number: u8,
    ) -> SmolResult<Option<SetupPacket>> {
        let mut buffer = [0_u8; 8];
        let _bytes_read = driver.read_control(&mut buffer);
        let setup_packet = SetupPacket::from(buffer);
        let direction = setup_packet.direction();
        let length: usize = setup_packet.length as usize;

        self.state = State::SetupStage;

        trace!("CONTROL handle_receive_setup_packet(endpoint_number: {}) state:{:?} direction:{:?} length:{}",
               endpoint_number, self.state, direction, length);

        // TODO make sure endpoint is not stalled ?
        // driver.unstall_endpoint_out(endpoint_number);

        // OUT transfer
        if direction == Direction::HostToDevice {
            trace!("  OUT {} bytes", length);

            if length > MAX_RECEIVE_SIZE {
                // has data stage, but too big too receive
                error!("  data stage too big: {}", length);
                self.set_error(driver, endpoint_number);
                return Ok(None); // TODO return error
            } else if length > 0 {
                // has data stage
                self.state = State::OutDataStage(setup_packet);
                driver.ack(0, Direction::HostToDevice);
                return Ok(None); // handle_receive_packet will return it
            } else {
                // no data stage, we're done
                self.state = State::Idle;
                return Ok(Some(setup_packet));
            }

        // IN transfer - ack ?
        } else {
            trace!("  IN {} bytes", length);

            if length > 0 {
                // has data stage
                self.state = State::InDataStage;
            } else {
                // no data stage, we're done
                self.state = State::Idle;
            }

            return Ok(Some(setup_packet));
        }
    }

    // USBx_EP_OUT n
    pub fn handle_receive_packet(
        &mut self,
        driver: &D,
        endpoint_number: u8,
    ) -> SmolResult<Option<(SetupPacket, &[u8])>> {
        trace!(
            "CONTROL handle_receive_packet(endpoint_number: {}) state:{:?}",
            endpoint_number,
            self.state
        );

        let offset = self.rx_buffer_position;
        let bytes_read = driver.read(endpoint_number, &mut self.rx_buffer[offset..]);
        driver.ep_out_prime_receive(endpoint_number);

        trace!(
            "  read {} bytes, buffer position: {}",
            bytes_read,
            offset + bytes_read
        );
        trace!("  {:?}", &self.rx_buffer[offset..offset + bytes_read]);

        match self.state {
            State::OutDataStage(setup_packet) => {
                if bytes_read == 0 {
                    trace!("  ACK TODO early abort bytes_read:{}", bytes_read);
                }

                let length = setup_packet.length as usize;

                self.rx_buffer_position += bytes_read;
                if self.rx_buffer_position >= length {
                    self.rx_buffer_position = 0;
                    self.state = State::Idle;
                    return Ok(Some((setup_packet, &self.rx_buffer[..length])));
                } else {
                    // more data awaits
                    return Ok(None);
                }
            }

            // it's an ack
            _ => {
                trace!("  ACK bytes_read:{}", bytes_read);
            }
        }

        Ok(None)
    }

    // USBx_EP_IN n
    pub fn handle_send_complete(&mut self, driver: &D, endpoint_number: u8) -> SmolResult<()> {
        trace!(
            "CONTROL handle_send_complete(endpoint_number: {}) state:{:?}",
            endpoint_number,
            self.state
        );

        Ok(())
    }
}

// - helpers ------------------------------------------------------------------

impl<'a, D, const MAX_RECEIVE_SIZE: usize> Control<'a, D, MAX_RECEIVE_SIZE>
where
    D: UsbDriver,
{
    fn set_error(&mut self, driver: &D, endpoint_number: u8) {
        self.state = State::Error(endpoint_number);
        driver.stall_endpoint_out(endpoint_number);
        driver.stall_endpoint_in(endpoint_number);
    }
}
