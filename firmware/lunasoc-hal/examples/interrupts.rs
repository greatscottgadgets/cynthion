#![no_std]
#![no_main]

use core::fmt::Write;

use panic_halt as _;
use riscv_rt::entry;

use lunasoc_hal as hal;
use moondancer_pac as pac;

lunasoc_hal::impl_serial! {
    Serial: pac::UART0,
}

lunasoc_hal::impl_timer! {
    Timer: pac::TIMER0,
}

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();
    let mut serial = Serial::new(peripherals.UART0);

    // configure and enable timer
    let one_second = pac::clock::sysclk();
    let mut timer = Timer::new(peripherals.TIMER0, one_second);
    timer.set_timeout_ticks(one_second / 2);
    timer.enable();

    // enable timer events
    timer.listen(hal::timer::Event::TimeOut);

    // enable interrupts
    unsafe {
        // set mstatus register: interrupt enable
        riscv::interrupt::enable();

        // set mie register: machine external interrupts enable
        riscv::register::mie::set_mext();

        // write csr: enable timer interrupt
        pac::csr::interrupt::enable(pac::Interrupt::TIMER0)
    }

    writeln!(serial, "Peripherals initialized, entering main loop.").unwrap();

    let mut uptime = 1;
    loop {
        writeln!(serial, "Uptime: {} seconds", uptime).unwrap();

        unsafe {
            riscv::asm::delay(pac::clock::sysclk());
        }
        uptime += 1;
    }
}

// interrupt handler
#[allow(non_snake_case)]
#[no_mangle]
fn MachineExternal() {
    static mut TOGGLE: bool = true;

    let mut serial = unsafe { Serial::summon() };

    if pac::csr::interrupt::is_pending(pac::Interrupt::TIMER0) {
        let timer = unsafe { Timer::summon() };
        timer.clear_pending();

        writeln!(serial, "MachineExternal - timer interrupt").unwrap();

        // blinkenlights
        let peripherals = unsafe { pac::Peripherals::steal() };
        let leds = &peripherals.LEDS;

        if unsafe { TOGGLE } {
            leds.output().write(|w| unsafe { w.bits(255) });
        } else {
            leds.output().write(|w| unsafe { w.bits(0) });
        }
        unsafe { TOGGLE = !TOGGLE };
    } else {
        writeln!(serial, "MachineExternal - unknown interrupt").unwrap();
    }
}
