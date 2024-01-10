//! HAL implementation for LUNA EPTRI devices.
//!
//! Reference: https://github.com/hathach/tinyusb/compare/master...ktemkin:tinyusb:luna_riscv

mod error;
pub use error::ErrorKind;

use smolusb::device::Speed;
use smolusb::setup::*;
use smolusb::traits::{
    ReadControl, ReadEndpoint, UnsafeUsbDriverOperations, UsbDriver, UsbDriverOperations,
    WriteEndpoint,
};

use crate::pac;
use pac::interrupt::Interrupt;

use ladybug::{Bit, Channel};
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
        $USBX:ident: $USBX_CONTROLLER:ident, $USBX_EP_CONTROL:ident, $USBX_EP_IN:ident, $USBX_EP_OUT:ident, $LADYBUG_TRACE:expr,
    )+) => {
        $(
            pub struct $USBX {
                pub controller: pac::$USBX_CONTROLLER,
                pub ep_control: pac::$USBX_EP_CONTROL,
                pub ep_in: pac::$USBX_EP_IN,
                pub ep_out: pac::$USBX_EP_OUT,
                pub device_speed: Speed,
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
                        device_speed: Speed::Unknown,
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
                        device_speed: Speed::Unknown,
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
                            .ev_pending()
                            .modify(|r, w| w.pending().bit(r.pending().bit())),
                        Interrupt::$USBX_EP_CONTROL => self
                            .ep_control
                            .ev_pending()
                            .modify(|r, w| w.pending().bit(r.pending().bit())),
                        Interrupt::$USBX_EP_IN => self
                            .ep_in
                            .ev_pending()
                            .modify(|r, w| w.pending().bit(r.pending().bit())),
                        Interrupt::$USBX_EP_OUT => self
                            .ep_out
                            .ev_pending()
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
                            .ev_enable()
                            .write(|w| w.enable().bit(true)),
                        Interrupt::$USBX_EP_CONTROL => self
                            .ep_control
                            .ev_enable()
                            .write(|w| w.enable().bit(true)),
                        Interrupt::$USBX_EP_IN => self
                            .ep_in
                            .ev_enable()
                            .write(|w| w.enable().bit(true)),
                        Interrupt::$USBX_EP_OUT => self
                            .ep_out
                            .ev_enable()
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
                            .ev_enable()
                            .write(|w| w.enable().bit(false)),
                        Interrupt::$USBX_EP_CONTROL => self
                            .ep_control
                            .ev_enable()
                            .write(|w| w.enable().bit(false)),
                        Interrupt::$USBX_EP_IN => self
                            .ep_in
                            .ev_enable()
                            .write(|w| w.enable().bit(false)),
                        Interrupt::$USBX_EP_OUT => self
                            .ep_out
                            .ev_enable()
                            .write(|w| w.enable().bit(false)),
                        _ => {
                            warn!("Ignoring invalid interrupt enable: {:?}", interrupt);
                        }
                    }
                }

                pub fn ep_control_address(&self) -> u8 {
                    self.ep_control.address().read().address().bits()
                }
            }

            // - trait: UsbDriverOperations -----------------------------------

            impl UsbDriverOperations for $USBX {
                /// Set the interface up for new connections
                fn connect(&mut self, device_speed: Speed) {
                    // set the device speed
                    self.device_speed = device_speed;
                    match device_speed {
                        Speed::High => {
                            self.controller.full_speed_only().write(|w| w.full_speed_only().bit(false));
                            self.controller.low_speed_only().write(|w| w.low_speed_only().bit(false));
                        },
                        Speed::Full => {
                            self.controller.full_speed_only().write(|w| w.full_speed_only().bit(true));
                            self.controller.low_speed_only().write(|w| w.low_speed_only().bit(false));
                        },
                        /*Speed::Low => {
                            // FIXME still connects at full speed
                            self.controller.full_speed_only.write(|w| w.full_speed_only().bit(false));
                            self.controller.low_speed_only.write(|w| w.low_speed_only().bit(true));
                        }*/
                        _ => {
                            log::warn!("Requested unsupported device speed, ignoring: {:?}", device_speed);
                            self.device_speed = Speed::Unknown;
                        }
                    }

                    // disconnect device controller
                    self.controller.connect().write(|w| w.connect().bit(false));

                    // disable endpoint events
                    self.disable_interrupts();

                    // reset FIFOs
                    self.ep_control.reset().write(|w| w.reset().bit(true));
                    self.ep_in.reset().write(|w| w.reset().bit(true));
                    self.ep_out.reset().write(|w| w.reset().bit(true));

                    // connect device controller
                    self.controller.connect().write(|w| w.connect().bit(true));
                }

                fn disconnect(&mut self) {
                    // reset speed
                    self.controller.full_speed_only().write(|w| w.full_speed_only().bit(false));
                    self.controller.low_speed_only().write(|w| w.low_speed_only().bit(false));
                    self.device_speed = Speed::Unknown;

                    // disable endpoint events
                    self.disable_interrupts();

                    // reset device address to 0
                    self.set_address(0);

                    // disconnect device controller
                    self.controller.connect().write(|w| w.connect().bit(false));

                    // reset FIFOs
                    self.ep_control.reset().write(|w| w.reset().bit(true));
                    self.ep_in.reset().write(|w| w.reset().bit(true));
                    self.ep_out.reset().write(|w| w.reset().bit(true));
                }

                /// Perform a full reset of the device.
                fn reset(&self) {
                    // disable endpoint events
                    self.disable_interrupts();

                    // reset device address to 0
                    self.set_address(0);

                    // reset FIFOs
                    self.ep_control.reset().write(|w| w.reset().bit(true));
                    self.ep_in.reset().write(|w| w.reset().bit(true));
                    self.ep_out.reset().write(|w| w.reset().bit(true));

                    // re-enable endpoint events
                    self.enable_interrupts();

                    trace!("UsbInterface0::reset()");
                }

                /// Perform a bus reset of the device.
                ///
                /// This differs from `reset()` by not disabling
                /// USBx_CONTROLLER bus reset events.
                fn bus_reset(&self) {
                    // disable events
                    self.disable_interrupt(Interrupt::$USBX_CONTROLLER);
                    self.disable_interrupt(Interrupt::$USBX_EP_CONTROL);
                    self.disable_interrupt(Interrupt::$USBX_EP_IN);

                    // reset device address to 0
                    self.set_address(0);

                    // reset FIFOs
                    self.ep_control.reset().write(|w| w.reset().bit(true));
                    self.ep_in.reset().write(|w| w.reset().bit(true));
                    self.ep_out.reset().write(|w| w.reset().bit(true));

                    // re-enable events
                    self.enable_interrupt(Interrupt::$USBX_CONTROLLER);
                    self.enable_interrupt(Interrupt::$USBX_EP_CONTROL);
                    self.enable_interrupt(Interrupt::$USBX_EP_IN);

                    trace!("UsbInterface0::bus_reset()");
                }

                /// Acknowledge the status stage of an incoming control request.
                fn ack(&self, endpoint_number: u8, direction: Direction) {
                    $LADYBUG_TRACE(Channel::A, Bit::A_USB_ACK, || {
                        match direction {
                            // DeviceToHost - IN request, prime the endpoint so we can receive a zlp from the host
                            Direction::DeviceToHost => {
                                self.ep_out_prime_receive(endpoint_number);
                            }
                            // HostToDevice - OUT request, send a ZLP from the device to the host
                            Direction::HostToDevice => {
                                self.write(endpoint_number, [].into_iter());
                            }
                        }
                    })
                }

                fn set_address(&self, address: u8) {
                    self.ep_out
                        .address()
                        .write(|w| unsafe { w.address().bits(address & 0x7f) });
                    self.ep_control
                        .address()
                        .write(|w| unsafe { w.address().bits(address & 0x7f) });
                }

                /// Stall the given IN endpoint number
                fn stall_endpoint_in(&self, endpoint_number: u8) {
                    $LADYBUG_TRACE(Channel::A, Bit::A_USB_STALL_IN, || {
                        self.ep_in.stall().write(|w| w.stall().bit(true));
                        self.ep_in.epno().write(|w| unsafe { w.epno().bits(endpoint_number) });
                    });
                }

                /// Stall the given OUT endpoint number
                fn stall_endpoint_out(&self, endpoint_number: u8) {
                    $LADYBUG_TRACE(Channel::A, Bit::A_USB_STALL_OUT, || {
                        self.ep_out.epno().write(|w| unsafe { w.epno().bits(endpoint_number) });
                        self.ep_out.stall().write(|w| w.stall().bit(true));
                    });
                }

                /// Clear PID toggle bit for the given endpoint address.
                ///
                /// TODO this works most of the time, but not always ...
                /// TODO pass in endpoint number and direction separately
                ///
                /// Also see: https://github.com/greatscottgadgets/luna/issues/166
                fn clear_feature_endpoint_halt(&self, endpoint_address: u8) {
                    let endpoint_number = endpoint_address & 0xf;

                    if (endpoint_address & 0x80) == 0 {  // HostToDevice
                        self.ep_out.epno().write(|w| unsafe { w.epno().bits(endpoint_number) });
                        self.ep_out.pid().write(|w| w.pid().bit(false));

                    } else { // DeviceToHost
                        self.ep_in.epno().write(|w| unsafe { w.epno().bits(endpoint_number) });
                        self.ep_in.pid().write(|w| w.pid().bit(false));
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
                use smolusb::EP_MAX_ENDPOINTS;

                #[cfg(target_has_atomic)]
                const ATOMIC_FALSE: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);

                #[cfg(not(target_has_atomic))]
                pub static mut TX_ACK_ACTIVE: [bool; EP_MAX_ENDPOINTS] = [false; EP_MAX_ENDPOINTS];
                #[cfg(target_has_atomic)]
                pub static TX_ACK_ACTIVE: [core::sync::atomic::AtomicBool; EP_MAX_ENDPOINTS] =
                    [ATOMIC_FALSE; EP_MAX_ENDPOINTS];

            }

            impl UnsafeUsbDriverOperations for $USBX {
                #[inline(always)]
                unsafe fn set_tx_ack_active(&self, endpoint_number: u8) {
                    #[cfg(not(target_has_atomic))]
                    {
                        let endpoint_number = endpoint_number as usize;
                        riscv::interrupt::free(|| {
                            $USBX_CONTROLLER::TX_ACK_ACTIVE[endpoint_number] = true;
                        });
                    }
                    #[cfg(target_has_atomic)]
                    {
                        let endpoint_number = endpoint_number as usize;
                        use core::sync::atomic::Ordering;
                        $USBX_CONTROLLER::TX_ACK_ACTIVE[endpoint_number].store(true, Ordering::Relaxed);
                    }
                }
                #[inline(always)]
                unsafe fn clear_tx_ack_active(&self, endpoint_number: u8) {
                    #[cfg(not(target_has_atomic))]
                    {
                        let endpoint_number = endpoint_number as usize;
                        riscv::interrupt::free(|| {
                            $USBX_CONTROLLER::TX_ACK_ACTIVE[endpoint_number] = false;
                        });
                    }
                    #[cfg(target_has_atomic)]
                    {
                        let endpoint_number = endpoint_number as usize;
                        use core::sync::atomic::Ordering;
                        $USBX_CONTROLLER::TX_ACK_ACTIVE[endpoint_number].store(false, Ordering::Relaxed);
                    }
                }
                #[inline(always)]
                unsafe fn is_tx_ack_active(&self, endpoint_number: u8) -> bool {
                    #[cfg(not(target_has_atomic))]
                    {
                        let endpoint_number = endpoint_number as usize;
                        let active = riscv::interrupt::free(|| {
                            $USBX_CONTROLLER::TX_ACK_ACTIVE[endpoint_number]
                        });
                        active
                    }
                    #[cfg(target_has_atomic)]
                    {
                        let endpoint_number = endpoint_number as usize;
                        use core::sync::atomic::Ordering;
                        $USBX_CONTROLLER::TX_ACK_ACTIVE[endpoint_number].load(Ordering::Relaxed)
                    }
                }
            }

            // - trait: Read/Write traits -------------------------------------

            impl ReadControl for $USBX {
                fn read_control(&self, buffer: &mut [u8]) -> usize {
                    $LADYBUG_TRACE(Channel::B, Bit::B_USB_READ_CONTROL, || {
                        // drain fifo
                        let mut bytes_read = 0;
                        let mut overflow = 0;
                        while self.ep_control.have().read().have().bit() {
                            if bytes_read >= buffer.len() {
                                let _drain = self.ep_control.data().read().data().bits();
                                overflow += 1;
                            } else {
                                buffer[bytes_read] = self.ep_control.data().read().data().bits();
                                bytes_read += 1;
                            }
                        }

                        if bytes_read != buffer.len() {
                            warn!("  RX {} CONTROL {} bytes read - expected {}",
                                  stringify!($USBX),
                                  bytes_read, buffer.len());
                        }

                        if overflow == 0 {
                            trace!("  RX {} CONTROL {} bytes read", stringify!($USBX), bytes_read);
                        } else {
                            warn!("  RX {} CONTROL {} bytes read + {} bytes overflow",
                                  stringify!($USBX),
                                  bytes_read, overflow);
                        }

                        bytes_read + overflow
                    })
                }
            }

            impl ReadEndpoint for $USBX {
                /// Prepare OUT endpoint to receive a single packet.
                #[inline(always)]
                fn ep_out_prime_receive(&self, endpoint_number: u8) {
                    $LADYBUG_TRACE(Channel::A, Bit::A_USB_EP_OUT_PRIME, || {
                        // 0. clear receive buffer
                        self.ep_out.reset().write(|w| w.reset().bit(true));

                        // 1. select endpoint
                        self.ep_out
                            .epno()
                            .write(|w| unsafe { w.epno().bits(endpoint_number) });

                        // 2. prime endpoint
                        self.ep_out.prime().write(|w| w.prime().bit(true));

                        // 3. re-enable ep_out interface
                        self.ep_out.enable().write(|w| w.enable().bit(true));
                    });
                }

                #[inline(always)]
                fn read(&self, endpoint_number: u8, buffer: &mut [u8]) -> usize {
                    $LADYBUG_TRACE(Channel::A, Bit::A_USB_READ, || {
                        /*let mut bytes_read = 0;
                        let mut overflow = 0;
                        while self.ep_out.have().read().have().bit() {
                            if bytes_read >= buffer.len() {
                                // drain fifo
                                let _drain = self.ep_out.data().read().data().bits();
                                overflow += 1;
                            } else {
                                buffer[bytes_read] = self.ep_out.data().read().data().bits();
                                bytes_read += 1;
                            }
                        }*/

                        // getting a little better performance with an
                        // iterator, probably because it doesn't need to
                        // do a bounds check.
                        let mut bytes_read = 0;
                        let mut did_overflow = true;
                        for b in buffer.iter_mut() {
                            if self.ep_out.have().read().have().bit() {
                                *b = self.ep_out.data().read().data().bits();
                                bytes_read += 1;
                            } else {
                                did_overflow = false;
                                break;
                            }
                        }

                        // drain fifo if needed
                        let mut overflow = 0;
                        while did_overflow && self.ep_out.have().read().have().bit() {
                            let _drain = self.ep_out.data().read().data().bits();
                            overflow += 1;
                        }

                        if overflow == 0 {
                            trace!("  RX {} OUT {} {} bytes read", stringify!($USBX), endpoint_number, bytes_read);
                        } else {
                            warn!("  RX {} OUT {} {} bytes read + {} bytes overflow",
                                  stringify!($USBX),
                                  endpoint_number, bytes_read, overflow);
                        }

                        if bytes_read == 0 {
                            $LADYBUG_TRACE(Channel::A, Bit::A_USB_RX_ZLP, || {});
                        }

                        bytes_read + overflow
                    })
                }
            }

            impl WriteEndpoint for $USBX {
                fn write<'a, I>(&self, endpoint_number: u8, iter: I) -> usize
                where
                    I: Iterator<Item = u8>
                {
                    let max_packet_size = match (self.device_speed, endpoint_number) {
                        (_, 0) => 64,
                        (Speed::High, _) => smolusb::EP_MAX_PACKET_SIZE, // TODO const generic
                        (Speed::Full, _) => 64,
                        (_, _) => {
                            warn!("{}::write unsupported device speed: {:?}", stringify!($USBX), self.device_speed);
                            64
                        }
                    };
                    self.write_with_packet_size(endpoint_number, iter, max_packet_size)
                }

                fn write_with_packet_size<'a, I>(&self, endpoint_number: u8, iter: I, packet_size: usize) -> usize
                where
                    I: Iterator<Item = u8>
                {
                    $LADYBUG_TRACE(Channel::A, Bit::A_USB_WRITE, || {
                        unsafe { self.set_tx_ack_active(endpoint_number); }

                        // reset output fifo if needed
                        // FIXME rather return an error
                        if self.ep_in.have().read().have().bit() {
                            warn!("  {} clear tx", stringify!($USBX));
                            self.ep_in.reset().write(|w| w.reset().bit(true));
                        }

                        let mut bytes_written: usize = 0;
                        for byte in iter {
                            self.ep_in.data().write(|w| unsafe { w.data().bits(byte) });
                            bytes_written += 1;

                            // check if we've written a packet yet and need to send it
                            if bytes_written % packet_size == 0 {
                                // prime the IN endpoint to send it
                                $LADYBUG_TRACE(Channel::B, Bit::B_USB_EP_IN_EPNO, || {
                                    self.ep_in
                                        .epno()
                                        .write(|w| unsafe { w.epno().bits(endpoint_number) });
                                });
                                // wait for transmission to complete
                                let mut timeout = 0;
                                while self.ep_in.have().read().have().bit() {
                                    timeout += 1;
                                    if timeout > 5_000_000 {
                                        log::error!(
                                            "{}::write timed out after {} bytes",
                                            stringify!($USBX),
                                            bytes_written
                                        );
                                        // TODO return an error
                                        return bytes_written;
                                    }
                                }
                            }
                        }

                        // finally, prime IN endpoint to either send
                        // remaining queued data or a ZLP if the fifo
                        // is empty and transmission is complete
                        $LADYBUG_TRACE(Channel::B, Bit::B_USB_EP_IN_EPNO, || {
                            self.ep_in
                                .epno()
                                .write(|w| unsafe { w.epno().bits(endpoint_number) });
                        });

                        if bytes_written == 0 {
                            $LADYBUG_TRACE(Channel::A, Bit::A_USB_TX_ZLP, || {});
                        }

                        bytes_written
                    })
                }

            }

            // mark implementation as complete
            impl UsbDriver for $USBX {}
        )+
    }
}

#[inline(always)]
fn no_trace<R>(_channel: Channel, _bit_number: u8, f: impl FnOnce() -> R) -> R {
    f()
}

impl_usb! {
    Usb0: USB0, USB0_EP_CONTROL, USB0_EP_IN, USB0_EP_OUT, ladybug::trace,
    Usb1: USB1, USB1_EP_CONTROL, USB1_EP_IN, USB1_EP_OUT, no_trace,
    Usb2: USB2, USB2_EP_CONTROL, USB2_EP_IN, USB2_EP_OUT, no_trace,
}
