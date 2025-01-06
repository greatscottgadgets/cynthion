#![no_std]
#![no_main]

use panic_halt as _;
use riscv_rt::entry;

use moondancer_pac as pac;
use pac::csr;

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();
    let timer = &peripherals.TIMER0;

    // configure and enable timer
    timer
        .reload()
        .write(|w| unsafe { w.value().bits(pac::clock::sysclk() / 2) });
    timer.enable().write(|w| w.enable().bit(true));

    // enable timer events
    timer.ev_enable().write(|w| unsafe { w.mask().bits(1) });

    // enable interrupts
    unsafe {
        // set mstatus register: interrupt enable
        riscv::interrupt::enable();

        // set mie register: machine external interrupts enable
        riscv::register::mie::set_mext();

        // write csr: enable timer interrupt
        csr::interrupt::enable(pac::Interrupt::TIMER0)
    }

    loop {
        unsafe {
            riscv::asm::delay(pac::clock::sysclk());
        }
        uart_tx("Ping\n");
    }
}

// - interrupt handler --------------------------------------------------------

#[allow(non_snake_case)]
#[no_mangle]
unsafe fn MachineExternal() {
    static mut TOGGLE: bool = true;

    let peripherals = unsafe { pac::Peripherals::steal() };
    let leds = &peripherals.LEDS;
    let timer = &peripherals.TIMER0;

    if csr::interrupt::is_pending(pac::Interrupt::TIMER0) {
        // clear interrupt
        timer.ev_pending().modify(|r, w| w.mask().bits(r.mask().bits()));

        // blinkenlights
        if TOGGLE {
            leds.output().write(|w| unsafe { w.bits(255) });
        } else {
            leds.output().write(|w| unsafe { w.bits(0) });
        }
        TOGGLE = !TOGGLE;
    } else {
        uart_tx("MachineExternal - unknown interrupt\n");
    }
}

// - exception handler --------------------------------------------------------

#[allow(non_snake_case)]
#[no_mangle]
unsafe fn ExceptionHandler(_trap_frame: &riscv_rt::TrapFrame) -> ! {
    uart_tx("ExceptionHandler\n");
    loop {}
}

// - helpers ------------------------------------------------------------------

fn uart_tx(string: &str) {
    let peripherals = unsafe { pac::Peripherals::steal() };
    let uart = &peripherals.UART0;

    for c in string.chars() {
        while uart.tx_ready().read().txe().bit() == false {
            unsafe {
                riscv::asm::nop();
            }
        }
        uart.tx_data()
            .write(|w| unsafe { w.data().bits(c as u8) })
    }
}
