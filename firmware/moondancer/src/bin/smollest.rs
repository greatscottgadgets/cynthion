#![allow(dead_code, unused_imports, unused_mut, unused_variables)]
#![no_std]
#![no_main]

use core::fmt::Write;
use moondancer::pac;

// - panic handler ------------------------------------------------------------

// disable panic handler in lib.rs to use this
/*#[no_mangle]
#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::ptr::write_volatile(IO_LEDS as *mut u32, 0b11_1100) };
    loop { }
}*/

#[export_name = "ExceptionHandler"]
fn custom_exception_handler(panic_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::ptr::write_volatile(IO_LEDS as *mut u32, 0b11_1110) };
    loop {}
}

// - riscv_rt::main -----------------------------------------------------------

#[riscv_rt::entry]
fn main() -> ! {
    const MSG: &'static str = "Entering main loop.";
    uart_tx(MSG);

    let mut counter = 0;
    loop {
        unsafe { riscv::asm::delay(1_000_000) };
        unsafe { core::ptr::write_volatile(IO_LEDS as *mut u32, counter & 0b11_1111) };
        counter += 1;
    }
}

// - peripherals --------------------------------------------------------------

const IO_BASE: usize = 0x8000_0000;
const IO_LEDS: usize = IO_BASE + 0x0080;
const IO_UART_TX_DATA: usize = IO_BASE + 0x0010;
const IO_UART_TX_RDY: usize = IO_BASE + 0x0014;

fn uart_tx(s: &str) {
    for b in s.bytes() {
        while unsafe { core::ptr::read_volatile(IO_UART_TX_RDY as *mut u32) } == 0 {}
        unsafe { core::ptr::write_volatile(IO_UART_TX_DATA as *mut u32, b as u32 & 0b1111_1111) };
    }
}
