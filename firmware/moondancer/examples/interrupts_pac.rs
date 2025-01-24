#![allow(dead_code)]
#![no_std]
#![no_main]

use riscv_rt::entry;

use log::{error, info};

use moondancer::pac;

// - interrupt handler --------------------------------------------------------

#[allow(non_snake_case)]
#[no_mangle]
fn MachineExternal() {
    let leds = unsafe { pac::LEDS::steal() };
    let timer0 = unsafe { pac::TIMER0::steal() };
    let timer1 = unsafe { pac::TIMER1::steal() };

    if pac::csr::interrupt::is_pending(pac::Interrupt::TIMER0) {
        timer0
            .ev_pending()
            .modify(|r, w| w.mask().bit(r.mask().bit()));
        leds.output().write(|w| unsafe { w.bits(0b11_1000) });
    } else if pac::csr::interrupt::is_pending(pac::Interrupt::TIMER1) {
        timer1
            .ev_pending()
            .modify(|r, w| w.mask().bit(r.mask().bit()));
        leds.output().write(|w| unsafe { w.bits(0b00_0111) });
    } else {
        error!("MachineExternal - unknown interrupt");
    }
}

// - riscv_rt::main -----------------------------------------------------------

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();
    let leds = &peripherals.LEDS;
    let timer0 = &peripherals.TIMER0;
    let timer1 = &peripherals.TIMER1;

    // initialize logging
    moondancer::log::init();

    // configure leds
    //
    //   INPUT      = 0b00
    //   PUSH_PULL  = 0b01
    //   OPEN_DRAIN = 0b10
    let mode: u16 = 0b01_01_01_01_01_01_01_01;
    leds.mode().write(|w| unsafe { w.bits(mode) });

    // configure timers
    let t = pac::clock::sysclk() as f32 / 3.2;
    timer0.mode().write(|w| w.periodic().bit(true));
    timer0
        .reload()
        .write(|w| unsafe { w.value().bits(t as u32) });
    timer0.enable().write(|w| w.enable().bit(true));
    timer1.mode().write(|w| w.periodic().bit(true));
    timer1
        .reload()
        .write(|w| unsafe { w.value().bits(pac::clock::sysclk()) });
    timer1.enable().write(|w| w.enable().bit(true));

    // enable timer events
    timer0.ev_enable().write(|w| w.mask().bit(true));
    timer1.ev_enable().write(|w| w.mask().bit(true));

    // enable interrupts
    unsafe {
        // set mstatus register: interrupt enable
        riscv::interrupt::enable();

        // set mie register: machine external interrupts enable
        riscv::register::mie::set_mext();

        // write csr: enable timer interrupts
        pac::csr::interrupt::enable(pac::Interrupt::TIMER0);
        pac::csr::interrupt::enable(pac::Interrupt::TIMER1);
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
