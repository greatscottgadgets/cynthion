pub use lunasoc_hal::*;
pub use crate::pac;

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

use self::smolusb::device::Speed;
use self::smolusb::setup::Direction;
use self::smolusb::traits::{
    ReadControl, ReadEndpoint, UnsafeUsbDriverOperations, UsbDriver, UsbDriverOperations,
    WriteEndpoint,
};
use self::usb::DEFAULT_TIMEOUT;
self::impl_usb! {
    Usb0: usb0, pac::USB0, pac::USB0_EP_CONTROL, pac::USB0_EP_IN, pac::USB0_EP_OUT,
    Usb1: usb1, pac::USB1, pac::USB1_EP_CONTROL, pac::USB1_EP_IN, pac::USB1_EP_OUT,
    Usb2: usb2, pac::USB2, pac::USB2_EP_CONTROL, pac::USB2_EP_IN, pac::USB2_EP_OUT,
}
