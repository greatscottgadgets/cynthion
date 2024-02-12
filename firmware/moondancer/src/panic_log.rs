use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

use log::error;

// - panic handler ------------------------------------------------------------

#[no_mangle]
#[inline(never)]
#[panic_handler]
extern "Rust" fn panic(_panic_info: &PanicInfo) -> ! {
    // panic stations
    let peripherals = unsafe { crate::pac::Peripherals::steal() };
    let leds = &peripherals.LEDS;
    leds.output()
        .write(|w| unsafe { w.output().bits(0b10_1010) });

    #[cfg(feature = "nightly")]
    if let Some(message) = _panic_info.message() {
        error!("Panic: {}", message);
    } else {
        error!("Panic: Unknown");
    }

    // TODO This takes up about 4Kb of the firmware size!
    /*if let Some(location) = _panic_info.location() {
        error!("Panicked at '{}:{}'", location.file(), location.line(),);
    }*/
    error!("Firmware Panicked");

    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}
