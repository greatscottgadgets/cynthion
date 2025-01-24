#![no_std]
#![no_main]

use moondancer::pac;

use hal::hal::delay::DelayUs;
use moondancer::hal;

use log::{error, info};

use riscv_rt::entry;

// - main entry point ---------------------------------------------------------

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();
    let leds = &peripherals.LEDS;
    leds.output().write(|w| unsafe { w.bits(0b000000) });

    // initialize logging
    moondancer::log::init();

    // configure gpio0 pins: 7-4:output, 3-0:input
    // 0b00=input_only, 0b01=push_pull, 0b10=open_drain
    let gpio0 = &peripherals.GPIO0;
    gpio0.mode().write(|w| unsafe {
        w.pin_0()
            .bits(0b00)
            .pin_1()
            .bits(0b00)
            .pin_2()
            .bits(0b00)
            .pin_3()
            .bits(0b00)
            .pin_4()
            .bits(0b01)
            .pin_5()
            .bits(0b01)
            .pin_6()
            .bits(0b01)
            .pin_7()
            .bits(0b01)
    });

    info!("Peripherals initialized, entering main loop.");

    let mut counter = 0;

    loop {
        // send gpio0 ping 0-3 to leds 0-3
        let inputs = gpio0.input().read().bits();
        leds.output()
            .write(|w| unsafe { w.bits(inputs & 0b0000_1111) });

        // set gpio0 pins 4-7 to counter value
        gpio0
            .output()
            .write(|w| unsafe { w.bits(counter & 0b1111_0000) });

        counter += 1;
    }
}
