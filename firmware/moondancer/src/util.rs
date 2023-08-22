#![allow(dead_code, unused_imports, unused_variables)] // TODO

use smolusb::event::UsbEvent;
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

    // USB0 UsbBusReset
    if usb0.is_pending(pac::Interrupt::USB0) {
        usb0.clear_pending(pac::Interrupt::USB0);
        InterruptEvent::Usb(Target, UsbEvent::BusReset)

    // USB0_EP_CONTROL UsbReceiveSetupPacket
    } else if usb0.is_pending(pac::Interrupt::USB0_EP_CONTROL) {
        let endpoint = usb0.ep_control.epno.read().bits() as u8;
        usb0.clear_pending(pac::Interrupt::USB0_EP_CONTROL);
        InterruptEvent::Usb(Target, UsbEvent::ReceiveSetupPacket(endpoint))

    // USB0_EP_OUT UsbReceiveData
    } else if usb0.is_pending(pac::Interrupt::USB0_EP_OUT) {
        let endpoint = usb0.ep_out.data_ep.read().bits() as u8;
        usb0.clear_pending(pac::Interrupt::USB0_EP_OUT);
        InterruptEvent::Usb(Target, UsbEvent::ReceivePacket(endpoint))

    // USB0_EP_IN UsbTransferComplete
    } else if usb0.is_pending(pac::Interrupt::USB0_EP_IN) {
        let endpoint = usb0.ep_in.epno.read().bits() as u8;
        usb0.clear_pending(pac::Interrupt::USB0_EP_IN);

        // TODO something a little bit safer would be nice
        unsafe {
            usb0.clear_tx_ack_active();
        }

        InterruptEvent::Usb(Target, UsbEvent::SendComplete(endpoint))

    // - usb1 interrupts - "aux_phy" (host on r0.4) --

    // USB1 UsbBusReset
    } else if usb1.is_pending(pac::Interrupt::USB1) {
        usb1.clear_pending(pac::Interrupt::USB1);
        InterruptEvent::Usb(Aux, UsbEvent::BusReset)

    // USB1_EP_CONTROL UsbReceiveSetupPacket
    } else if usb1.is_pending(pac::Interrupt::USB1_EP_CONTROL) {
        let endpoint = usb1.ep_control.epno.read().bits() as u8;
        usb1.clear_pending(pac::Interrupt::USB1_EP_CONTROL);
        InterruptEvent::Usb(Aux, UsbEvent::ReceiveSetupPacket(endpoint))

    // USB1_EP_OUT UsbReceiveData
    } else if usb1.is_pending(pac::Interrupt::USB1_EP_OUT) {
        let endpoint = usb1.ep_out.data_ep.read().bits() as u8;
        usb1.clear_pending(pac::Interrupt::USB1_EP_OUT);
        InterruptEvent::Usb(Aux, UsbEvent::ReceivePacket(endpoint))

    // USB1_EP_IN UsbTransferComplete
    } else if usb1.is_pending(pac::Interrupt::USB1_EP_IN) {
        let endpoint = usb1.ep_in.epno.read().bits() as u8;
        usb1.clear_pending(pac::Interrupt::USB1_EP_IN);

        // TODO something a little safer would be nice
        unsafe {
            usb1.clear_tx_ack_active();
        }

        InterruptEvent::Usb(Aux, UsbEvent::SendComplete(endpoint))

    // - usb2 interrupts - "control_phy" (sideband on r0.4) --

    // USB2 UsbBusReset
    } else if usb2.is_pending(pac::Interrupt::USB2) {
        usb2.clear_pending(pac::Interrupt::USB2);
        InterruptEvent::Usb(Control, UsbEvent::BusReset)

    // USB2_EP_CONTROL UsbReceiveSetupPacket
    } else if usb2.is_pending(pac::Interrupt::USB2_EP_CONTROL) {
        let endpoint = usb2.ep_control.epno.read().bits() as u8;
        usb2.clear_pending(pac::Interrupt::USB2_EP_CONTROL);
        InterruptEvent::Usb(Control, UsbEvent::ReceiveSetupPacket(endpoint))

    // USB2_EP_OUT UsbReceiveData
    } else if usb2.is_pending(pac::Interrupt::USB2_EP_OUT) {
        let endpoint = usb2.ep_out.data_ep.read().bits() as u8;
        usb2.clear_pending(pac::Interrupt::USB2_EP_OUT);
        InterruptEvent::Usb(Control, UsbEvent::ReceivePacket(endpoint))

    // USB2_EP_IN UsbTransferComplete
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
