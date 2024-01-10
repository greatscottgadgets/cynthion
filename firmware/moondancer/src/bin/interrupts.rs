#![no_std]
#![no_main]

use moondancer::{hal, pac};

use log::{error, info};
use riscv_rt::entry;

// - interrupt handler --------------------------------------------------------

#[allow(non_snake_case)]
#[no_mangle]
fn MachineExternal() {
    static mut TOGGLE: bool = true;

    if pac::csr::interrupt::pending(pac::Interrupt::TIMER) {
        let timer = unsafe { hal::Timer::summon() };
        timer.clear_pending();

        //writeln!(serial, "MachineExternal - timer interrupt").unwrap();

        // blinkenlights
        let peripherals = unsafe { pac::Peripherals::steal() };
        let leds = &peripherals.LEDS;

        if unsafe { TOGGLE } {
            leds.output().write(|w| unsafe { w.output().bits(0b1) });
        } else {
            leds.output().write(|w| unsafe { w.output().bits(0b0) });
        }
        unsafe { TOGGLE = !TOGGLE };
    } else {
        error!("MachineExternal - unknown interrupt");
    }
}

// - main entry point ---------------------------------------------------------

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();
    let serial = hal::Serial::new(peripherals.UART);
    moondancer::log::init(serial);

    // configure and enable timer
    let one_second = pac::clock::sysclk();
    let mut timer = hal::Timer::new(peripherals.TIMER, one_second);
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
        pac::csr::interrupt::enable(pac::Interrupt::TIMER)
    }

    info!("Peripherals initialized, entering main loop.");

    let mut uptime = 1;
    loop {
        info!("Uptime: {} seconds", uptime);

        unsafe {
            riscv::asm::delay(pac::clock::sysclk());
        }
        uptime += 1;
    }
}
