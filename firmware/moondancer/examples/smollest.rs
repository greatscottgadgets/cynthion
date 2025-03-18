#![no_std]
#![no_main]

// - panic handler ------------------------------------------------------------

#[no_mangle]
#[panic_handler]
fn panic(_panic_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::ptr::write_volatile(IO_LEDS as *mut u32, 0b10_0000) };
    loop {}
}

#[export_name = "ExceptionHandler"]
fn custom_exception_handler(_panic_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::ptr::write_volatile(IO_LEDS as *mut u32, 0b01_0000) };
    loop {}
}

// - riscv_rt::main -----------------------------------------------------------

#[riscv_rt::entry]
fn main() -> ! {
    const MSG: &'static str = "Entering main loop.\n";
    uart_tx(MSG);

    let mut counter = 0;
    loop {
        unsafe { riscv::asm::delay(1_000_000) };
        unsafe { core::ptr::write_volatile(IO_LEDS as *mut u8, counter & 0b11_1111) };
        counter += 1;
    }
}

// - peripherals --------------------------------------------------------------

// see: moondancer-pac/svd/moondancer.svd
const IO_BASE: usize = 0xf000_0000;
const IO_LEDS: usize = IO_BASE + 0x0003;
const IO_UART1_TX_DATA: usize = IO_BASE + 0x0400;
const IO_UART1_TX_RDY: usize = IO_BASE + 0x0408;

fn uart_tx(s: &str) {
    for b in s.bytes() {
        while unsafe { core::ptr::read_volatile(IO_UART1_TX_RDY as *mut u8) } == 0 {}
        unsafe { core::ptr::write_volatile(IO_UART1_TX_DATA as *mut u8, b) };
    }
}
