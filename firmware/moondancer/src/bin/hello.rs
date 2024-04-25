#![no_std]
#![no_main]

use moondancer::pac;

use hal::hal::delay::DelayUs;
use moondancer::hal;

use log::{debug, info};

use riscv_rt::entry;

#[cfg(feature = "vexriscv")]
#[riscv_rt::pre_init]
unsafe fn pre_main() {
    pac::cpu::vexriscv::flush_icache();
    #[cfg(feature = "vexriscv_dcache")]
    pac::cpu::vexriscv::flush_dcache();
}

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();
    let leds = &peripherals.LEDS;

    // initialize logging
    moondancer::log::init();

    let mut timer = hal::Timer0::new(peripherals.TIMER, pac::clock::sysclk());
    let mut counter = 0;
    let mut direction = true;
    let mut led_state = 0b11_0000;

    info!("Peripherals initialized, entering main loop.");

    loop {
        timer.delay_ms(100).unwrap();

        if direction {
            led_state >>= 1;
            if led_state == 0b00_0011 {
                direction = false;
                info!("left: {}", counter);
            }
        } else {
            led_state <<= 1;
            if led_state == 0b11_0000 {
                direction = true;
                debug!("right: {}", counter);
            }
        }

        leds.output()
            .write(|w| unsafe { w.output().bits(led_state) });
        counter += 1;
    }
}
