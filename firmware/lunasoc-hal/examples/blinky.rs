#![no_std]
#![no_main]

use panic_halt as _;
use riscv_rt::entry;

use lunasoc_hal as hal;
use moondancer_pac as pac;

use hal::hal::delay::DelayUs;

lunasoc_hal::impl_timer! {
    Timer: pac::TIMER,
}

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();
    let leds = &peripherals.LEDS;
    let mut timer = Timer::new(peripherals.TIMER, pac::clock::sysclk());

    let mut direction = true;
    let mut led_state = 0b110000;

    loop {
        timer.delay_ms(100_u32).unwrap();

        if direction {
            led_state >>= 1;
            if led_state == 0b000011 {
                direction = false;
            }
        } else {
            led_state <<= 1;
            if led_state == 0b110000 {
                direction = true;
            }
        }

        leds.output()
            .write(|w| unsafe { w.output().bits(led_state) });
    }
}
