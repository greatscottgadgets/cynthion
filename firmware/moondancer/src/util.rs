#![allow(dead_code, unused_imports, unused_variables)] // TODO

use smolusb::event::UsbEvent;
use smolusb::setup::SetupPacket;
use smolusb::traits::{
    ReadControl, ReadEndpoint, UnsafeUsbDriverOperations, UsbDriverOperations, WriteEndpoint,
    WriteRefEndpoint,
};

use crate::event::InterruptEvent;
use crate::{hal, pac};

use pac::csr::interrupt;

pub fn get_usb_interrupt_event() -> InterruptEvent {
    use crate::UsbInterface::{Aux, Control, Target};

    let peripherals = unsafe { pac::Peripherals::steal() };
    let usb0 = unsafe { hal::Usb0::summon() }; // target
    let usb1 = unsafe { hal::Usb1::summon() }; // aux
    let usb2 = unsafe { hal::Usb2::summon() }; // control

    let pending = interrupt::reg_pending();

    // - usb0 interrupts - "target_phy" --

    // USB0 BusReset
    if usb0.is_pending(pac::Interrupt::USB0) {
        usb0.clear_pending(pac::Interrupt::USB0);

        // handle bus reset in interrupt handler for lowest latency
        usb0.bus_reset();

        InterruptEvent::Usb(Target, UsbEvent::BusReset)

    // USB0_EP_CONTROL ReceiveControl
    } else if usb0.is_pending(pac::Interrupt::USB0_EP_CONTROL) {
        let endpoint = usb0.ep_control.epno.read().bits() as u8;
        usb0.clear_pending(pac::Interrupt::USB0_EP_CONTROL);
        InterruptEvent::Usb(Target, UsbEvent::ReceiveControl(endpoint))

        // read setup packet in interrupt handler for lowest latency
        /*let mut setup_packet_buffer = [0_u8; 8];
        let bytes_read = usb0.read_control(&mut setup_packet_buffer);
        if bytes_read == 0 {
            return InterruptEvent::ErrorMessage("ERROR Received 0 bytes for setup packet!!!");
        }
        let setup_packet = SetupPacket::try_from(setup_packet_buffer).unwrap();
        InterruptEvent::Usb(Target, UsbEvent::ReceiveSetupPacket(endpoint, setup_packet))*/

    // USB0_EP_OUT ReceivePacket
    } else if usb0.is_pending(pac::Interrupt::USB0_EP_OUT) {
        let endpoint = usb0.ep_out.data_ep.read().bits() as u8;
        usb0.clear_pending(pac::Interrupt::USB0_EP_OUT);
        InterruptEvent::Usb(Target, UsbEvent::ReceivePacket(endpoint))

    // USB0_EP_IN SendComplete
    } else if usb0.is_pending(pac::Interrupt::USB0_EP_IN) {
        let endpoint = usb0.ep_in.epno.read().bits() as u8;
        usb0.clear_pending(pac::Interrupt::USB0_EP_IN);

        // TODO something a little bit safer would be nice
        unsafe {
            usb0.clear_tx_ack_active();
        }

        InterruptEvent::Usb(Target, UsbEvent::SendComplete(endpoint))

    // - usb1 interrupts - "aux_phy" (host on r0.4) --

    // USB1 BusReset
    } else if usb1.is_pending(pac::Interrupt::USB1) {
        usb1.clear_pending(pac::Interrupt::USB1);
        InterruptEvent::Usb(Aux, UsbEvent::BusReset)

    // USB1_EP_CONTROL ReceiveControl
    } else if usb1.is_pending(pac::Interrupt::USB1_EP_CONTROL) {
        let endpoint = usb1.ep_control.epno.read().bits() as u8;
        usb1.clear_pending(pac::Interrupt::USB1_EP_CONTROL);
        InterruptEvent::Usb(Aux, UsbEvent::ReceiveControl(endpoint))

    // USB1_EP_OUT ReceivePacket
    } else if usb1.is_pending(pac::Interrupt::USB1_EP_OUT) {
        let endpoint = usb1.ep_out.data_ep.read().bits() as u8;
        usb1.clear_pending(pac::Interrupt::USB1_EP_OUT);
        InterruptEvent::Usb(Aux, UsbEvent::ReceivePacket(endpoint))

    // USB1_EP_IN SendComplete
    } else if usb1.is_pending(pac::Interrupt::USB1_EP_IN) {
        let endpoint = usb1.ep_in.epno.read().bits() as u8;
        usb1.clear_pending(pac::Interrupt::USB1_EP_IN);

        // TODO something a little safer would be nice
        unsafe {
            usb1.clear_tx_ack_active();
        }

        InterruptEvent::Usb(Aux, UsbEvent::SendComplete(endpoint))

    // - usb2 interrupts - "control_phy" (sideband on r0.4) --

    // USB2 BusReset
    } else if usb2.is_pending(pac::Interrupt::USB2) {
        usb2.clear_pending(pac::Interrupt::USB2);
        InterruptEvent::Usb(Control, UsbEvent::BusReset)

    // USB2_EP_CONTROL ReceiveControl
    } else if usb2.is_pending(pac::Interrupt::USB2_EP_CONTROL) {
        let endpoint = usb2.ep_control.epno.read().bits() as u8;
        usb2.clear_pending(pac::Interrupt::USB2_EP_CONTROL);
        InterruptEvent::Usb(Control, UsbEvent::ReceiveControl(endpoint))

    // USB2_EP_OUT ReceivePacket
    } else if usb2.is_pending(pac::Interrupt::USB2_EP_OUT) {
        let endpoint = usb2.ep_out.data_ep.read().bits() as u8;
        usb2.clear_pending(pac::Interrupt::USB2_EP_OUT);
        InterruptEvent::Usb(Control, UsbEvent::ReceivePacket(endpoint))

    // USB2_EP_IN SendComplete
    } else if usb2.is_pending(pac::Interrupt::USB2_EP_IN) {
        let endpoint = usb2.ep_in.epno.read().bits() as u8;
        usb2.clear_pending(pac::Interrupt::USB2_EP_IN);

        // TODO something a little safer would be nice
        unsafe {
            usb2.clear_tx_ack_active();
        }

        InterruptEvent::Usb(Control, UsbEvent::SendComplete(endpoint))

    // - unhandled interrupt --
    } else {
        InterruptEvent::UnhandledInterrupt(pending)
    }
}
