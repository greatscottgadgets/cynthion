use crate::device::Speed;
use crate::setup::{Direction, SetupPacket};

use zerocopy::AsBytes;

use core::slice;

// - UsbDriverOperations ------------------------------------------------------

// convenience alias
pub trait UsbDriver:
    ReadControl
    + ReadEndpoint
    + WriteEndpoint
    + WriteRefEndpoint
    + UsbDriverOperations
    + UnsafeUsbDriverOperations
{
}

pub trait UsbDriverOperations {
    /// Speed
    fn set_speed(&self, device_speed: Speed);
    /// Connect
    fn connect(&self);
    /// Disconnect
    fn disconnect(&self);
    /// Reset
    fn reset(&self);
    /// Bus Reset
    fn bus_reset(&self);
    /// Acknowledge the status stage of an incoming control request.
    fn ack_status_stage(&self, packet: &SetupPacket);
    fn ack(&self, endpoint_number: u8, direction: Direction);
    fn set_address(&self, address: u8);
    /// Stall the current control request.
    fn stall_control_request(&self);
    /// Stall the given IN endpoint
    fn stall_endpoint_in(&self, endpoint_number: u8);
    /// Stall the given OUT endpoint
    fn stall_endpoint_out(&self, endpoint_number: u8);
    /// Unstall the given IN endpoint
    fn unstall_endpoint_in(&self, endpoint_number: u8);
    /// Unstall the given OUT endpoint
    fn unstall_endpoint_out(&self, endpoint_number: u8);

    /// Clear any halt condition on the target endpoint, and clear the data toggle bit.
    fn clear_feature_endpoint_halt(&self, endpoint_address: u8);
}

pub trait UnsafeUsbDriverOperations {
    unsafe fn set_tx_ack_active(&self);
    unsafe fn clear_tx_ack_active(&self);
    unsafe fn is_tx_ack_active(&self) -> bool;
}

// - UsbRead/UsbWrite ---------------------------------------------------------

pub trait ReadControl {
    /// Read a setup packet from the control endpoint
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

// These two should be one trait

pub trait WriteEndpoint {
    /// Write iterator to a single packet
    ///
    /// Returns the number of bytes written to the endpoint.
    fn write<'a, I>(&self, endpoint_number: u8, iter: I) -> usize
    where
        I: Iterator<Item = u8>;

    /// Write iterator to multiple packets
    ///
    /// Returns the number of bytes written to the endpoint.
    fn write_packets<'a, I>(&self, endpoint_number: u8, iter: I, packet_size: usize) -> usize
    where
        I: Iterator<Item = u8>;
}

pub trait WriteRefEndpoint {
    /// Write iterator to a single packet
    ///
    /// Returns the number of bytes written to the endpoint.
    fn write_ref<'a, I>(&self, endpoint_number: u8, iter: I) -> usize
    where
        I: Iterator<Item = &'a u8>;
}

// - AsIterator ---------------------------------------------------------------

pub trait AsByteSliceIterator: AsBytes {
    fn as_iter(&self) -> slice::Iter<u8> {
        self.as_bytes().iter()
    }
}

trait AsByteIterator<'a> {
    type AsIter: Iterator<Item = &'a u8>;
    fn as_iter(&'a self) -> Self::AsIter;
}

trait AsIterator<'a> {
    type Item;
    type AsIter: Iterator<Item = Self::Item>;
    fn as_iter(&'a self) -> Self::AsIter;
}
