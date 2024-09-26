#![no_std]
#![no_main]

use moondancer::pac;

use log::{error, info};

use riscv_rt::entry;

#[cfg(feature = "vexriscv")]
#[riscv_rt::pre_init]
unsafe fn pre_main() {
    pac::cpu::vexriscv::flush_icache();
    #[cfg(feature = "vexriscv_dcache")]
    pac::cpu::vexriscv::flush_dcache();
}

const FLASH_ADDR: usize = 0x1000b000;
const READ_LENGTH: usize = 32;

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();
    let spi0 = &peripherals.SPI0;

    // initialize logging
    moondancer::log::init();

    info!("Peripherals initialized, entering main loop.");

    loop {
        unsafe {
            riscv::asm::delay(60_000_000);
        }

        // read flash memory
        let mut buffer = [0_u8; READ_LENGTH];
        for offset in 0..READ_LENGTH {
            let addr = FLASH_ADDR + offset;
            let byte = unsafe { core::ptr::read_volatile(addr as *mut u8) };
            buffer[offset] = byte;
        }
        info!("Read flash memory: {:02x?}", buffer);

        // read flash uuid
        read_flash_uuid(&spi0);
    }
}

fn read_flash_uuid(spi0: &pac::SPI0) {
    // configure spi0 phy
    spi0.phy().write(|w| unsafe { w.length().bits(8) });
    spi0.phy().write(|w| unsafe { w.width().bits(1) });
    spi0.phy().write(|w| unsafe { w.mask().bits(1) });
    spi0.cs().write(|w| w.select().bit(false));

    // check if we can write to spi0
    if !spi_ready(&|| spi0.tx().read().ready().bit()) {
        error!("spi write timeout");
        return;
    }

    // write flash id command to spi0
    let command: [u8; 13] = [0x4b, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for byte in command {
        spi0.tx().write(|w| unsafe { w.data().bits(byte as u32) });
    }

    // check if we can read from spi0
    if !spi_ready(&|| spi0.rx().read().ready().bit()) {
        error!("spi read timeout");
        return;
    }

    // read response
    let mut response = [0_u8; 32];
    let mut n = 0;
    while spi0.rx().read().ready().bit() {
        response[n] = spi0.rx().read().data().bits() as u8;
        n = n + 1;
        if n >= response.len() {
            error!("read overflow");
            return;
        }
    }

    if n != 13 {
        error!("invalid response length: {} - {:02x?}", n, &response[..n]);
        return;
    }

    info!("flash uuid: {:02x?}", &response[5..n]);
}

fn spi_ready(f: &dyn Fn() -> bool) -> bool {
    let mut timeout = 0;

    while !f() {
        timeout += 1;
        if timeout > 1000 {
            return false;
        }
    }

    return true;
}
