#![no_std]
#![no_main]

use core::fmt::Write;

use panic_halt as _;
use riscv_rt::entry;

use lunasoc_hal as hal;
use moondancer_pac as pac;

use hal::hal::delay::DelayUs;

lunasoc_hal::impl_serial! {
    Serial: pac::UART0,
}

lunasoc_hal::impl_timer! {
    Timer: pac::TIMER0,
}

const SYSTEM_CLOCK_FREQUENCY: u32 = pac::clock::sysclk();

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();

    let leds = &peripherals.LEDS;
    let mut serial = Serial::new(peripherals.UART0);
    let mut timer = Timer::new(peripherals.TIMER0, SYSTEM_CLOCK_FREQUENCY);

    writeln!(serial, "Peripherals initialized, entering main loop.").unwrap();

    let mut direction = true;
    let mut led_state = 0b11000000;
    let mut uptime = 0;

    loop {
        timer.delay_ms(100_u32).unwrap();

        if uptime % 10 == 0 {
            writeln!(serial, "Uptime: {} seconds", uptime / 10).unwrap();
        }
        uptime += 1;

        if direction {
            led_state >>= 1;
            if led_state == 0b00000011 {
                direction = false;
                writeln!(serial, "left").unwrap();
            }
        } else {
            led_state <<= 1;
            if led_state == 0b11000000 {
                direction = true;
                writeln!(serial, "right").unwrap();
            }
        }

        leds.output()
            .write(|w| unsafe { w.bits(led_state) });
    }
}
