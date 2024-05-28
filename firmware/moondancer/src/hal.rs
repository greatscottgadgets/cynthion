pub use crate::pac;
pub use lunasoc_hal::*;

lunasoc_hal::impl_gpio! {
    Gpio0: pac::GPIOA,
}

lunasoc_hal::impl_serial! {
    Serial0: pac::UART,
    Serial1: pac::UART1,
}

lunasoc_hal::impl_timer! {
    Timer0: pac::TIMER,
}

pub use lunasoc_hal::smolusb;
use lunasoc_hal::smolusb::device::Speed;
use lunasoc_hal::smolusb::setup::Direction;
use lunasoc_hal::smolusb::traits::{
    ReadControl, ReadEndpoint, UnsafeUsbDriverOperations, UsbDriver, UsbDriverOperations,
    WriteEndpoint,
};
use lunasoc_hal::usb::DEFAULT_TIMEOUT;
lunasoc_hal::impl_usb! {
    Usb0: usb0, pac::USB0, pac::USB0_EP_CONTROL, pac::USB0_EP_IN, pac::USB0_EP_OUT,
    Usb1: usb1, pac::USB1, pac::USB1_EP_CONTROL, pac::USB1_EP_IN, pac::USB1_EP_OUT,
    Usb2: usb2, pac::USB2, pac::USB2_EP_CONTROL, pac::USB2_EP_IN, pac::USB2_EP_OUT,
}
