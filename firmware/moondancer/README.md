# moondancer

MoonDancer firmware for the Great Scott Gadgets Cynthion.




## debug support

The moondancer SoC exposes UART and JTAG ports on Cynthion's PMOD B port:

    Pin 1:  UART1 rx
    Pin 2:  UART1 rx

    Pin 7:  JTAG tms
    Pin 8:  JTAG tdi
    Pin 9:  JTAG tdo
    Pin 10: JTAG tck


### UART

The Cynthion SoC provides two UART ports.

UART0 is connected to the Cynthion SAMD11 microcontroller and can only be accessed if the SoC firmware does not make use of the Cynthion's Control port (USB0).

UART1 is connected to the Cynthion's PMOD B port and be accessed via a serial adapter.

    picocom --imap lfcrlf -b 115200 /dev/cu.usbserial-1301


### JTAG

The Cynthion SoC provides a JTAG port connected to the Vexriscv processor.

It is connected to the Cynthion's PMOD B port and can be used to load firmware and debug the CPU.

#### Load firmware

    openocd -f openocd-jtag+serial.cfg -f flash.cfg

#### Debug firmware

Terminal 2:

    openocd -f openocd-jtag+serial.cfg

Terminal 3:

    cargo run --release


## Cynthion USB2 Control Port

By default the Cynthion USB2 Control port is managed by Apollo.

If you would like to take over control from apollo and use USB2 in your own firmware you can disable the ApolloAdvertiser peripheral using something like:

    let peripherals = pac::Peripherals::take().unwrap();

    let advertiser = peripherals.ADVERTISER;
    advertiser.enable().write(|w| w.enable().bit(true));

Note that you will no longer be able to receive UART0 data via Apollo in this case and will need to use UART1 instead.
