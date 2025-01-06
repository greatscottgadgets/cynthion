#![no_std]
#![no_main]

use panic_halt as _;
use riscv_rt::entry;

use moondancer_pac as pac;

const SYSTEM_CLOCK_FREQUENCY: u32 = pac::clock::sysclk();

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();
    let leds = &peripherals.LEDS;
    let timer = &peripherals.TIMER0;

    let mut direction = true;
    let mut led_state = 0b11000000;

    loop {
        delay_ms(timer, SYSTEM_CLOCK_FREQUENCY, 100);

        if direction {
            led_state >>= 1;
            if led_state == 0b00000011 {
                direction = false;
            }
        } else {
            led_state <<= 1;
            if led_state == 0b11000000 {
                direction = true;
            }
        }

        leds.output()
            .write(|w| unsafe { w.bits(led_state) });
    }
}

fn delay_ms(timer: &pac::TIMER0, sys_clk: u32, ms: u32) {
    let ticks: u32 = sys_clk / 1_000 * ms;

    timer.reload().write(|w| unsafe { w.value().bits(ticks) });
    timer.enable().write(|w| w.enable().bit(true));

    while timer.counter().read().value().bits() > 0 {
        unsafe {
            riscv::asm::nop();
        }
    }

    timer.enable().write(|w| w.enable().bit(false));
}
