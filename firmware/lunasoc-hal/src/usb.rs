//! Simple USB implementation

mod error;
pub use error::ErrorKind;

use smolusb::control::*;
use smolusb::traits::{
    ControlRead, EndpointRead, EndpointWrite, EndpointWriteRef, UnsafeUsbDriverOperations,
    UsbDriver, UsbDriverOperations,
};

use crate::pac;
use pac::interrupt::Interrupt;

use log::{trace, warn};

/// Macro to generate hal wrappers for pac::USBx peripherals
///
/// For example:
///
///     impl_usb! {
///         Usb0: USB0, USB0_EP_CONTROL, USB0_EP_IN, USB0_EP_OUT,
///         Usb1: USB1, USB1_EP_CONTROL, USB1_EP_IN, USB1_EP_OUT,
///     }
///
macro_rules! impl_usb {
    ($(
        $USBX:ident: $USBX_CONTROLLER:ident, $USBX_EP_CONTROL:ident, $USBX_EP_IN:ident, $USBX_EP_OUT:ident,
    )+) => {
        $(
            pub struct $USBX {
                pub controller: pac::$USBX_CONTROLLER,
                pub ep_control: pac::$USBX_EP_CONTROL,
                pub ep_in: pac::$USBX_EP_IN,
                pub ep_out: pac::$USBX_EP_OUT,
            }

            impl $USBX {
                /// Create a new `Usb` from the [`USB`](pac::USB) peripheral.
                pub fn new(
                    controller: pac::$USBX_CONTROLLER,
                    ep_control: pac::$USBX_EP_CONTROL,
                    ep_in: pac::$USBX_EP_IN,
                    ep_out: pac::$USBX_EP_OUT,
                ) -> Self {
                    Self {
                        controller,
                        ep_control,
                        ep_in,
                        ep_out,
                    }
                }

                /// Release the [`USB`](pac::USB) peripheral and consume self.
                pub fn free(
                    self,
                ) -> (
                    pac::$USBX_CONTROLLER,
                    pac::$USBX_EP_CONTROL,
                    pac::$USBX_EP_IN,
                    pac::$USBX_EP_OUT,
                ) {
                    (self.controller, self.ep_control, self.ep_in, self.ep_out)
                }

                /// Obtain a static `Usb` instance for use in e.g. interrupt handlers
                ///
                /// # Safety
                ///
                /// 'Tis thine responsibility, that which thou doth summon.
                #[inline(always)]
                pub unsafe fn summon() -> Self {
                    Self {
                        controller: pac::Peripherals::steal().$USBX_CONTROLLER,
                        ep_control: pac::Peripherals::steal().$USBX_EP_CONTROL,
                        ep_in: pac::Peripherals::steal().$USBX_EP_IN,
                        ep_out: pac::Peripherals::steal().$USBX_EP_OUT,
                    }
                }
            }

            impl $USBX {
                pub fn enable_interrupts(&self) {
                    // clear all event handlers
                    self.clear_pending(Interrupt::$USBX_CONTROLLER);
                    self.clear_pending(Interrupt::$USBX_EP_CONTROL);
                    self.clear_pending(Interrupt::$USBX_EP_IN);
                    self.clear_pending(Interrupt::$USBX_EP_OUT);

                    // enable all device controller events
                    self.enable_interrupt(Interrupt::$USBX_CONTROLLER);
                    self.enable_interrupt(Interrupt::$USBX_EP_CONTROL);
                    self.enable_interrupt(Interrupt::$USBX_EP_IN);
                    self.enable_interrupt(Interrupt::$USBX_EP_OUT);
                }

                pub fn disable_interrupts(&self) {
                    // clear all event handlers
                    self.clear_pending(Interrupt::$USBX_CONTROLLER);
                    self.clear_pending(Interrupt::$USBX_EP_CONTROL);
                    self.clear_pending(Interrupt::$USBX_EP_IN);
                    self.clear_pending(Interrupt::$USBX_EP_OUT);

                    // disable all device controller events
                    self.disable_interrupt(Interrupt::$USBX_CONTROLLER);
                    self.disable_interrupt(Interrupt::$USBX_EP_CONTROL);
                    self.disable_interrupt(Interrupt::$USBX_EP_IN);
                    self.disable_interrupt(Interrupt::$USBX_EP_OUT);
                }

                #[inline(always)]
                pub fn is_pending(&self, interrupt: Interrupt) -> bool {
                    pac::csr::interrupt::pending(interrupt)
                }

                #[inline(always)]
                pub fn clear_pending(&self, interrupt: Interrupt) {
                    match interrupt {
                        Interrupt::$USBX_CONTROLLER => self
                            .controller
                            .ev_pending
                            .modify(|r, w| w.pending().bit(r.pending().bit())),
                        Interrupt::$USBX_EP_CONTROL => self
                            .ep_control
                            .ev_pending
                            .modify(|r, w| w.pending().bit(r.pending().bit())),
                        Interrupt::$USBX_EP_IN => self
                            .ep_in
                            .ev_pending
                            .modify(|r, w| w.pending().bit(r.pending().bit())),
                        Interrupt::$USBX_EP_OUT => self
                            .ep_out
                            .ev_pending
                            .modify(|r, w| w.pending().bit(r.pending().bit())),
                        _ => {
                            warn!("Ignoring invalid interrupt clear pending: {:?}", interrupt);
                        }
                    }
                }

                pub fn enable_interrupt(&self, interrupt: Interrupt) {
                    match interrupt {
                        Interrupt::$USBX_CONTROLLER => self
                            .controller
                            .ev_enable
                            .write(|w| w.enable().bit(true)),
                        Interrupt::$USBX_EP_CONTROL => self
                            .ep_control
                            .ev_enable
                            .write(|w| w.enable().bit(true)),
                        Interrupt::$USBX_EP_IN => self
                            .ep_in
                            .ev_enable
                            .write(|w| w.enable().bit(true)),
                        Interrupt::$USBX_EP_OUT => self
                            .ep_out
                            .ev_enable
                            .write(|w| w.enable().bit(true)),
                        _ => {
                            warn!("Ignoring invalid interrupt enable: {:?}", interrupt);
                        }
                    }
                }

                pub fn disable_interrupt(&self, interrupt: Interrupt) {
                    match interrupt {
                        Interrupt::$USBX_CONTROLLER => self
                            .controller
                            .ev_enable
                            .write(|w| w.enable().bit(false)),
                        Interrupt::$USBX_EP_CONTROL => self
                            .ep_control
                            .ev_enable
                            .write(|w| w.enable().bit(false)),
                        Interrupt::$USBX_EP_IN => self
                            .ep_in
                            .ev_enable
                            .write(|w| w.enable().bit(false)),
                        Interrupt::$USBX_EP_OUT => self
                            .ep_out
                            .ev_enable
                            .write(|w| w.enable().bit(false)),
                        _ => {
                            warn!("Ignoring invalid interrupt enable: {:?}", interrupt);
                        }
                    }
                }

                /// Prepare endpoint to receive a single OUT packet.
                #[inline(always)]
                pub fn ep_out_prime_receive(&self, endpoint: u8) {
                    // clear receive buffer
                    self.ep_out.reset.write(|w| w.reset().bit(true));

                    // select endpoint
                    self.ep_out
                        .epno
                        .write(|w| unsafe { w.epno().bits(endpoint) });

                    // prime endpoint
                    self.ep_out.prime.write(|w| w.prime().bit(true));

                    // enable it
                    self.ep_out.enable.write(|w| w.enable().bit(true));
                }

                pub fn ep_control_address(&self) -> u8 {
                    self.ep_control.address.read().address().bits()
                }
            }

            // - trait: UsbDriverOperations -----------------------------------

            impl UsbDriverOperations for $USBX {
                /// Set the interface up for new connections
                fn connect(&self) -> u8 {
                    // disconnect device controller
                    self.controller.connect.write(|w| w.connect().bit(false));

                    // disable endpoint events
                    self.disable_interrupt(Interrupt::$USBX_CONTROLLER);
                    self.disable_interrupt(Interrupt::$USBX_EP_CONTROL);
                    self.disable_interrupt(Interrupt::$USBX_EP_IN);
                    self.disable_interrupt(Interrupt::$USBX_EP_OUT);

                    // reset FIFOs
                    self.ep_control.reset.write(|w| w.reset().bit(true));
                    self.ep_in.reset.write(|w| w.reset().bit(true));
                    self.ep_out.reset.write(|w| w.reset().bit(true));

                    // connect device controller
                    self.controller.connect.write(|w| w.connect().bit(true));

                    // 0: High, 1: Full, 2: Low, 3:SuperSpeed (incl SuperSpeed+)
                    self.controller.speed.read().speed().bits()
                }

                fn disconnect(&self) {
                    // disconnect device controller
                    self.controller.connect.write(|w| w.connect().bit(false));

                    // disable endpoint events
                    self.disable_interrupt(Interrupt::$USBX_CONTROLLER);
                    self.disable_interrupt(Interrupt::$USBX_EP_CONTROL);
                    self.disable_interrupt(Interrupt::$USBX_EP_IN);
                    self.disable_interrupt(Interrupt::$USBX_EP_OUT);

                    // reset FIFOs
                    self.ep_control.reset.write(|w| w.reset().bit(true));
                    self.ep_in.reset.write(|w| w.reset().bit(true));
                    self.ep_out.reset.write(|w| w.reset().bit(true));
                }

                fn reset(&self) -> u8 {
                    // disable endpoint events
                    self.disable_interrupt(Interrupt::$USBX_EP_CONTROL);
                    self.disable_interrupt(Interrupt::$USBX_EP_IN);
                    self.disable_interrupt(Interrupt::$USBX_EP_OUT);

                    // reset device address to 0
                    self.ep_control
                        .address
                        .write(|w| unsafe { w.address().bits(0) });

                    // reset FIFOs
                    self.ep_control.reset.write(|w| w.reset().bit(true));
                    self.ep_in.reset.write(|w| w.reset().bit(true));
                    self.ep_out.reset.write(|w| w.reset().bit(true));

                    self.enable_interrupts();

                    // 0: High, 1: Full, 2: Low, 3:SuperSpeed (incl SuperSpeed+)
                    let speed = self.controller.speed.read().speed().bits();
                    trace!("UsbInterface0::reset() -> {}", speed);
                    speed
                }

                fn bus_reset(&self) -> u8 {
                    // disable events
                    self.ep_control.ev_enable.write(|w| w.enable().bit(false));
                    self.ep_in.ev_enable.write(|w| w.enable().bit(false));
                    self.ep_out.ev_enable.write(|w| w.enable().bit(false));

                    // reset device address to 0
                    self.ep_control.address.write(|w| unsafe { w.address().bits(0) });
                    self.ep_out.address.write(|w| unsafe { w.address().bits(0) });

                    // reset fifo handlers
                    self.ep_control.reset.write(|w| w.reset().bit(true));
                    self.ep_in.reset.write(|w| w.reset().bit(true));
                    self.ep_out.reset.write(|w| w.reset().bit(true));

                    // clear any pending interrupts
                    self.controller.ev_pending.write(|w| w.pending().bit(true));
                    self.ep_control.ev_pending.write(|w| w.pending().bit(true));
                    self.ep_in.ev_pending.write(|w| w.pending().bit(true));
                    self.ep_out.ev_pending.write(|w| w.pending().bit(true));

                    // re-enable events
                    self.ep_control.ev_enable.write(|w| w.enable().bit(true));
                    self.ep_in.ev_enable.write(|w| w.enable().bit(true));
                    self.ep_out.ev_enable.write(|w| w.enable().bit(true));
                    self.controller.ev_enable.write(|w| w.enable().bit(true));

                    // 0: High, 1: Full, 2: Low, 3:SuperSpeed (incl SuperSpeed+)
                    let speed = self.controller.speed.read().speed().bits();
                    trace!("UsbInterface0::reset() -> {}", speed);
                    speed
                }

                /// Acknowledge the status stage of an incoming control request.
                fn ack_status_stage(&self, packet: &SetupPacket) {
                    match Direction::from(packet.request_type) {
                        // If this is an IN request, read a zero-length packet (ZLP) from the host..
                        Direction::DeviceToHost => self.ep_out_prime_receive(0),
                        // ... otherwise, send a ZLP.
                        Direction::HostToDevice => self.write(0, [].into_iter()),
                    }
                }

                fn ack(&self, endpoint: u8, direction: Direction) {
                    match direction {
                        // If this is an IN request, read a zero-length packet (ZLP) from the host..
                        Direction::DeviceToHost => self.ep_out_prime_receive(endpoint),
                        // ... otherwise, send a ZLP.
                        Direction::HostToDevice => self.write(endpoint, [].into_iter()),
                    }
                }

                fn set_address(&self, address: u8) {
                    self.ep_control
                        .address
                        .write(|w| unsafe { w.address().bits(address & 0x7f) });
                    self.ep_out
                        .address
                        .write(|w| unsafe { w.address().bits(address & 0x7f) });
                }

                /// Stalls the current control request.
                fn stall_request(&self) {
                    self.ep_in.epno.write(|w| unsafe { w.epno().bits(0) });
                    self.ep_in.stall.write(|w| w.stall().bit(true));
                    self.ep_out.epno.write(|w| unsafe { w.epno().bits(0) });
                    self.ep_out.stall.write(|w| w.stall().bit(true));
                }

                /// Set the stall state for the given endpoint address
                ///
                /// TODO endpoint_address is a USB address i.e. masked with 0x80
                /// for direction. It may be more consistent to actually pass
                /// in the direction and peripheral address separately
                fn stall_endpoint_address(&self, endpoint_address: u8, state: bool) {
                    match Direction::from(endpoint_address) {
                        Direction::HostToDevice => {
                            self.ep_out
                                .epno
                                .write(|w| unsafe { w.epno().bits(endpoint_address) });
                            self.ep_out.stall.write(|w| w.stall().bit(state));
                            log::debug!("  usb::stall_endpoint EP_OUT: {} -> {}", endpoint_address, state);
                        }
                        Direction::DeviceToHost => {
                            self.ep_in
                                .epno
                                .write(|w| unsafe { w.epno().bits(endpoint_address & 0xf) });
                            self.ep_in.stall.write(|w| w.stall().bit(state));
                            log::debug!("  usb::stall_endpoint EP_IN: {} -> {}", endpoint_address & 0xf, state);
                        }
                    }
                }

                /// Set stall for the given IN endpoint address
                fn stall_endpoint_in(&self, endpoint: u8) {
                    self.ep_in.epno.write(|w| unsafe { w.epno().bits(endpoint) });
                    self.ep_in.stall.write(|w| w.stall().bit(true));

                    // wait a moment, then clear stall
                    // TODO is this the correct behaviour?
                    unsafe { riscv::asm::delay(2000); }
                    self.ep_in.reset.write(|w| w.reset().bit(true));
                }

                /// Set stall for the given OUT endpoint address
                /// TODO test this!
                fn stall_endpoint_out(&self, endpoint: u8) {
                    self.ep_out.epno.write(|w| unsafe { w.epno().bits(endpoint) });
                    self.ep_out.stall.write(|w| w.stall().bit(true));

                    // wait a moment, then clear stall
                    // TODO does ep_out clear stall as with ep_in ?
                    // TODO is this the correct behaviour?
                    unsafe { riscv::asm::delay(2000); }
                    self.ep_out.reset.write(|w| w.reset().bit(true));
                }

                /// Clear PID toggle bit for the given endpoint address.
                ///
                /// TODO this works most of the time, but not always ...
                ///
                /// Also see: https://github.com/greatscottgadgets/luna/issues/166
                fn clear_feature_endpoint_halt(&self, endpoint_address: u8) {
                    let endpoint_number = endpoint_address & 0xf;

                    if (endpoint_address & 0x80) == 0 {  // HostToDevice
                        self.ep_out.epno.write(|w| unsafe { w.epno().bits(endpoint_number) });
                        self.ep_out.pid.write(|w| w.pid().bit(false));

                    } else { // DeviceToHost
                        self.ep_in.epno.write(|w| unsafe { w.epno().bits(endpoint_number) });
                        self.ep_in.pid.write(|w| w.pid().bit(false));
                    }

                    // TODO figure out why throughput is higher if we emit log messages
                    // this smacks of a deeper problem ...
                    log::debug!("  usb::clear_feature_endpoint_halt: 0x{:x}", endpoint_address);
                }
            }

            // - trait: UnsafeUsbDriverOperations -----------------------------

            // These are being used to work around the behaviour where we can only
            // set the device address after we have transmitted our STATUS ACK
            // response.
            //
            // This is not a particularly safe approach.
            #[allow(non_snake_case)]
            mod $USBX_CONTROLLER {
                #[cfg(not(target_has_atomic))]
                pub static mut TX_ACK_ACTIVE: bool = false;
                #[cfg(target_has_atomic)]
                pub static TX_ACK_ACTIVE: core::sync::atomic::AtomicBool =
                    core::sync::atomic::AtomicBool::new(false);
            }

            impl UnsafeUsbDriverOperations for $USBX {
                #[inline(always)]
                unsafe fn set_tx_ack_active(&self) {
                    #[cfg(not(target_has_atomic))]
                    {
                        riscv::interrupt::free(|| {
                            $USBX_CONTROLLER::TX_ACK_ACTIVE = true;
                        });
                    }
                    #[cfg(target_has_atomic)]
                    {
                        use core::sync::atomic::Ordering;
                        $USBX_CONTROLLER::TX_ACK_ACTIVE.store(true, Ordering::Relaxed);
                    }
                }
                #[inline(always)]
                unsafe fn clear_tx_ack_active(&self) {
                    #[cfg(not(target_has_atomic))]
                    {
                        riscv::interrupt::free(|| {
                            $USBX_CONTROLLER::TX_ACK_ACTIVE = false;
                        });
                    }
                    #[cfg(target_has_atomic)]
                    {
                        use core::sync::atomic::Ordering;
                        $USBX_CONTROLLER::TX_ACK_ACTIVE.store(false, Ordering::Relaxed);
                    }
                }
                #[inline(always)]
                unsafe fn is_tx_ack_active(&self) -> bool {
                    #[cfg(not(target_has_atomic))]
                    {
                        let active = riscv::interrupt::free(|| {
                            $USBX_CONTROLLER::TX_ACK_ACTIVE
                        });
                        active
                    }
                    #[cfg(target_has_atomic)]
                    {
                        use core::sync::atomic::Ordering;
                        $USBX_CONTROLLER::TX_ACK_ACTIVE.load(Ordering::Relaxed)
                    }
                }
            }

            // - trait: Read/Write traits -------------------------------------

            impl ControlRead for $USBX {
                fn read_control(&self, buffer: &mut [u8]) -> usize {
                    // drain fifo
                    let mut bytes_read = 0;
                    let mut overflow = 0;
                    while self.ep_control.have.read().have().bit() {
                        if bytes_read >= buffer.len() {
                            let _drain = self.ep_control.data.read().data().bits();
                            overflow += 1;
                        } else {
                            buffer[bytes_read] = self.ep_control.data.read().data().bits();
                            bytes_read += 1;
                        }
                    }

                    trace!("  RX CONTROL {} bytes + {} overflow: {:?}", bytes_read, overflow, &buffer[0..bytes_read]);

                    bytes_read
                }
            }

            impl EndpointRead for $USBX {
                #[inline(always)]
                fn read(&self, endpoint: u8, buffer: &mut [u8]) -> usize {
                    /*let mut bytes_read = 0;
                    let mut overflow = 0;
                    while self.ep_out.have.read().have().bit() {
                        if bytes_read >= buffer.len() {
                            // drain fifo
                            let _drain = self.ep_out.data.read().data().bits();
                            overflow += 1;
                        } else {
                            buffer[bytes_read] = self.ep_out.data.read().data().bits();
                            bytes_read += 1;
                        }
                    }*/

                    // getting a little better performance with an
                    // iterator, probably because it doesn't need to
                    // do a bounds check.
                    let mut bytes_read = 0;
                    for b in buffer.iter_mut() {
                        if self.ep_out.have.read().have().bit() {
                            *b = self.ep_out.data.read().data().bits();
                            bytes_read += 1;
                        } else {
                            break;
                        }
                    }

                    // drain fifo if needed
                    let mut overflow = 0;
                    while self.ep_out.have.read().have().bit() {
                        let _drain = self.ep_out.data.read().data().bits();
                        overflow += 1;
                    }

                    trace!("  RX OUT{} {} bytes read + {} bytes overflow", endpoint, bytes_read, overflow);

                    bytes_read
                }
            }

            impl EndpointWrite for $USBX {
                #[inline(always)]
                fn write<I>(&self, endpoint: u8, iter: I)
                where
                    I: Iterator<Item = u8>,
                {
                    // reset output fifo if needed
                    // TODO rather return an error
                    if self.ep_in.have.read().have().bit() {
                        trace!("  clear tx");
                        self.ep_in.reset.write(|w| w.reset().bit(true));
                    }

                    // write data
                    let mut bytes_written: usize = 0;
                    for byte in iter {
                        self.ep_in.data.write(|w| unsafe { w.data().bits(byte) });
                        bytes_written += 1;
                    }

                    // finally, prime IN endpoint
                    self.ep_in
                        .epno
                        .write(|w| unsafe { w.epno().bits(endpoint & 0xf) });

                    trace!("  TX {} bytes", bytes_written);
                }
            }

            impl EndpointWriteRef for $USBX {
                #[inline(always)]
                fn write_ref<'a, I>(&self, endpoint: u8, iter: I)
                where
                    I: Iterator<Item = &'a u8>,
                {
                    // reset output fifo if needed
                    // TODO rather return an error
                    if self.ep_in.have.read().have().bit() {
                        trace!("  clear tx");
                        self.ep_in.reset.write(|w| w.reset().bit(true));
                    }

                    // write data
                    let mut bytes_written: usize = 0;
                    for byte in iter {
                        self.ep_in.data.write(|w| unsafe { w.data().bits(*byte) });
                        bytes_written += 1;
                    }

                    // finally, prime IN endpoint
                    self.ep_in
                        .epno
                        .write(|w| unsafe { w.epno().bits(endpoint & 0xf) });

                    trace!("  TX {} bytes", bytes_written);
                }
            }

            // mark implementation as complete
            impl UsbDriver for $USBX {}
        )+
    }
}

impl_usb! {
    Usb0: USB0, USB0_EP_CONTROL, USB0_EP_IN, USB0_EP_OUT,
    Usb1: USB1, USB1_EP_CONTROL, USB1_EP_IN, USB1_EP_OUT,
    Usb2: USB2, USB2_EP_CONTROL, USB2_EP_IN, USB2_EP_OUT,
}
