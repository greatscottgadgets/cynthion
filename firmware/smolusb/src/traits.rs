use crate::device::Speed;
use crate::setup::Direction;

use zerocopy::AsBytes;

use core::slice;

// - UsbDriverOperations ------------------------------------------------------

// convenience alias
pub trait UsbDriver: ReadControl + ReadEndpoint + WriteEndpoint + UsbDriverOperations {}

pub trait UsbDriverOperations {
    /// Connect the device.
    fn connect(&mut self, device_speed: Speed);
    /// Disconnect the device.
    fn disconnect(&mut self);
    /// Perform a reset of the device.
    fn bus_reset(&self);
    /// Acknowledge the status stage of an incoming control request.
    fn ack(&self, endpoint_number: u8, direction: Direction);
    /// Set the device address.
    fn set_address(&self, address: u8);
    /// Stall the given IN endpoint number.
    fn stall_endpoint_in(&self, endpoint_number: u8);
    /// Stall the given OUT endpoint number.
    fn stall_endpoint_out(&self, endpoint_number: u8);

    /// Clear any halt condition on the target endpoint address, and clear the data toggle bit.
    fn clear_feature_endpoint_halt(&self, endpoint_address: u8);
}

/// These are used to deal with the situation where we need to block
/// on receipt of the host ACK following a usb write inside an ongoing
/// operation and are unable to process
/// [`UsbEvent::SendComplete`](crate::event::UsbEvent::SendComplete)
/// interrupt events.
///
/// Not having to do this is a powerful argument for implementing
/// async support.
///
/// This is not a particularly safe approach.
pub trait UnsafeUsbDriverOperations {
    /// Sets an atomic flag for the given endpoint number in order to
    /// be able to block on an event in an interrupt handler.
    ///
    /// # Safety
    ///
    /// Remember that the flag will stay set if your interrupt event
    /// never happens!
    unsafe fn set_tx_ack_active(&self, endpoint_number: u8);
    /// Clears an atomic flag for the given endpoint number in order to
    /// be able to block on an event in an interrupt handler.
    ///
    /// # Safety
    ///
    /// Remember that the flag will stay set if your interrupt event
    /// never happens!
    unsafe fn clear_tx_ack_active(&self, endpoint_number: u8);
    /// Tests an atomic flag for the given endpoint number in order to
    /// be able to block on an event in an interrupt handler.
    ///
    /// # Safety
    ///
    /// Remember that the flag will stay set if your interrupt event
    /// never happens!
    unsafe fn is_tx_ack_active(&self, endpoint_number: u8) -> bool;
}

// - UsbRead/UsbWrite ---------------------------------------------------------

pub trait ReadControl {
    /// Read a setup packet from the control endpoint.
    ///
    /// Returns the number of bytes read from the control endpoint.
    fn read_control(&self, buffer: &mut [u8]) -> usize;
}

pub trait ReadEndpoint {
    /// Prepare the given OUT endpoint to receive a single packet.
    fn ep_out_prime_receive(&self, endpoint_number: u8);

    /// Read a packet from the given endpoint.
    ///
    /// Returns the number of bytes read from the endpoint.
    fn read(&self, endpoint_number: u8, buffer: &mut [u8]) -> usize;
}

pub trait WriteEndpoint {
    /// Write iterator to endpoint
    ///
    /// Returns the number of bytes written to the endpoint.
    fn write<I>(&self, endpoint_number: u8, iter: I) -> usize
    where
        I: Iterator<Item = u8>;

    /// Write iterator to endpoint using the given packet size
    ///
    /// Returns the number of bytes written to the endpoint.
    fn write_with_packet_size<I>(&self, endpoint_number: u8, iter: I, packet_size: usize) -> usize
    where
        I: Iterator<Item = u8>;
}

// - AsIterator ---------------------------------------------------------------

pub trait AsByteSliceIterator: AsBytes {
    fn as_iter(&self) -> slice::Iter<u8> {
        self.as_bytes().iter()
    }
}
