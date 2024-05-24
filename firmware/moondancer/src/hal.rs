pub use lunasoc_hal::*;

self::impl_gpio! {
    Gpio0: pac::GPIOA,
}

self::impl_serial! {
    Serial0: pac::UART,
    Serial1: pac::UART1,
}

self::impl_timer! {
    Timer0: pac::TIMER,
}

use pac::Interrupt;
use self::smolusb::device::Speed;
use self::smolusb::setup::Direction;
use self::smolusb::traits::{UsbDriver, UsbDriverOperations, UnsafeUsbDriverOperations, ReadControl, ReadEndpoint, WriteEndpoint};
use self::usb::DEFAULT_TIMEOUT;
self::impl_usb! {
    Usb0: USB0, USB0_EP_CONTROL, USB0_EP_IN, USB0_EP_OUT,
    Usb1: USB1, USB1_EP_CONTROL, USB1_EP_IN, USB1_EP_OUT,
    Usb2: USB2, USB2_EP_CONTROL, USB2_EP_IN, USB2_EP_OUT,
    //Usb0: pac::USB0, pac::USB0_EP_CONTROL, pac::USB0_EP_IN, pac::USB0_EP_OUT,
    //Usb1: pac::USB1, pac::USB1_EP_CONTROL, pac::USB1_EP_IN, pac::USB1_EP_OUT,
    //Usb2: pac::USB2, pac::USB2_EP_CONTROL, pac::USB2_EP_IN, pac::USB2_EP_OUT,
}
