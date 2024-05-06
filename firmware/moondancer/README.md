# moondancer

Moondancer firmware for the Great Scott Gadgets Cynthion.


## Building and running

### Build SoC bitstream

Before you can run any firmware you will first need to build the Moondancer SoC bitstream:

    cd cynthion.git/cynthion/python

    make soc

### Execute Firmware

Once the SoC bitstream has been built you can execute your firmware with:

    cargo run --release

The behaviour of `cargo run` if governed by the `runner` parameter in the `.cargo/config.toml` file.

By default, it uses the `.cargo/apollo.sh` script to perform the following steps, in order:

1. Converts the ELF executable produced by the Rust compiler into a firmware binary image.
2. Uses `apollo` to flash the firmware binary image to Cynthion's SPI flash memory.
3. Uses `apollo` to configure Cynthion's FPGA with the SoC bitstream.
4. Starts a serial terminal for viewing firmware log output.


## Using Cynthion's Control Port (USB2)

By default Cynthion's Control port is used by Apollo.

If you would like to take over control from Apollo and use it in your own firmware you can disable the ApolloAdvertiser peripheral with:

    let peripherals = pac::Peripherals::take().unwrap();

    let advertiser = peripherals.ADVERTISER;
    advertiser.enable().write(|w| w.enable().bit(true));

Note that you will no longer be able to access the SoC's UART0 peripheral via Apollo in this case and will need to use UART1 instead.


## Firmware Debugging

The Moondancer SoC exposes a second UART and a JTAG connecter on Cynthion's PMOD B connector:

    PMOD B
    +-----+-----+----+----+----+----+
    | 3v3 | gnd |  4 |  3 |  2 |  1 |
    +-----+-----+----+----+----+----+
    | 3v3 | gnd | 10 |  9 |  8 |  7 |
    +-----+-----+----+----+----+----+

    Pin 1:  UART rx
    Pin 2:  UART tx

    Pin 7:  JTAG tms
    Pin 8:  JTAG tdi
    Pin 9:  JTAG tdo
    Pin 10: JTAG tck

### UART

The Cynthion SoC provides two UART ports.

UART0 is connected to Cynthion's SAMD11 microcontroller and can only be accessed if the SoC firmware does not make use of the Cynthion's Control port (USB2).

UART1 is connected to pins 1 and 2 of the Cynthion's PMOD B port and be accessed via a serial adapter.

    picocom --imap lfcrlf -b 115200 /dev/cu.usbserial-1301


### JTAG

The JTAG port is connected directly to the Vexriscv processor and can be used to load firmware and debug the CPU.

The exact details of using this capability will depend on the particular JTAG probe and debug software you are using, but as an example, you could use [`openocd`](https://openocd.org/) and [`gdb`](https://www.sourceware.org/gdb/) as follows:


#### 1. configure openocd

To configure openocd for a FTDI adapter you'll need something like this:

    #
    # .cargo/openocd.cfg
    #

    # select adapter driver
    adapter driver ftdi

    # configure adapter driver
    ftdi vid_pid 0x0403 0x6011
    ftdi channel 0
    ftdi layout_init 0xfff8 0xfffb
    ftdi tdo_sample_edge falling

    # configure transport
    transport select jtag
    adapter speed 25000

    # configure jtag tap
    set _CHIPNAME riscv
    set _TARGETNAME $_CHIPNAME.cpu

    # create jtag tap
    jtag newtap $_CHIPNAME cpu -irlen 5 -expected-id 0x10002FFF
    target create $_TARGETNAME.0 riscv -chain-position $_TARGETNAME

You can then run `openocd` as follows:

    openocd -f .cargo/openocd.cfg

Which, if everything is working, should give you something like:

    Open On-Chip Debugger 0.12.0
    Licensed under GNU GPL v2
    For bug reports, read
            http://openocd.org/doc/doxygen/bugs.html
    riscv.cpu
    Info : Listening on port 6666 for tcl connections
    Info : Listening on port 4444 for telnet connections
    Info : clock speed 25000 kHz
    Info : JTAG tap: riscv.cpu tap/device found: 0x10002fff (mfg: 0x7ff (<invalid>), part: 0x0002, ver: 0x1)
    Info : datacount=1 progbufsize=2
    Info : Disabling abstract command reads from CSRs.
    Info : Examined RISC-V core; found 1 harts
    Info :  hart 0: XLEN=32, misa=0x40000042
    Info : starting gdb server for riscv.cpu.0 on 3333
    Info : Listening on port 3333 for gdb connections

#### 2. configure gdb

You'll need a basic configuration file for gdb which looks something like this:

    #
    # .cargo/openocd.gdb
    #

    # connect to openocd
    target extended-remote :3333

    # print demangled symbols
    set print asm-demangle on

    # detect unhandled exceptions, hard faults and panics
    break DefaultHandler
    break HardFault
    break rust_begin_unwind

    # load firmware into memory
    load

#### 3. configure cargo to use gdb

By default the firmware uses `apollo` to flash your firmware and bitstream to Cynthion.

You will therefore need to modify the `.cargo/config.toml` configuration file to instead use `gdb` as the runner for your firmware:

    #
    # .cargo/config.toml
    #

    [target.riscv32imac-unknown-none-elf]
    runner = "apollo.sh"                     # <==
    rustflags = [
      "-C", "link-arg=-Tmemory.x",
      "-C", "link-arg=-Tlink.x",
    ]

    [build]
    target = "riscv32imac-unknown-none-elf"

Locate the `runner` parameter and modify it as follows:

    runner = "riscv64-unknown-elf-gdb -q -x .cargo/openocd.gdb"

#### 4. configure memory.x

Normally your firmware boots from Cynthion's SPI flash memory but, when using JTAG, you'll be loading it directly into SoC main memory.

However, to do this you'll first need to let the Rust linker know of your plans!

In the top-level `cynthion.git/firmware/` directory you should see a file called `memory.x` which contains the memory layout used by the Rust linker:

    #
    # memory.x
    #

    MEMORY {
        ...
    }

    REGION_ALIAS("REGION_TEXT", spiflash);    # <==
    REGION_ALIAS("REGION_RODATA", spiflash);  # <==
    REGION_ALIAS("REGION_DATA", mainram);
    REGION_ALIAS("REGION_BSS", mainram);
    REGION_ALIAS("REGION_HEAP", mainram);
    REGION_ALIAS("REGION_STACK", mainram);

You'll want to modify this file and change the two `REGION_ALIAS` directives pointing at `spiflash` to point to `mainram` instead:

    REGION_ALIAS("REGION_TEXT", mainram);
    REGION_ALIAS("REGION_RODATA", mainram);

If you don't do this `gdb` will attempt to load your firmware to the `spiflash` address which, given that the SoC's spi controller peripheral is read-only, will result in mild hilarity.


#### 5. configure Cynthion bitstream

You're almost there! The last step you'll need to perform before being able to debug your firmware requires you to build the SoC and configure Cynthion with the bitstream:

    cd cynthion.git/cynthion/python

    make soc
    make load

#### 6. start gdb

If you have successfully completed all the steps above and have a functioning connection to the SoC via `openocd` you should now able to execute your firmware with:

    cargo run --release --bin hello

If everything went well, you'll be dropped into a familiar `gdb` shell:

    Reading symbols from target/riscv32imac-unknown-none-elf/release/hello
    _start () at asm.S:27
    Breakpoint 1 at 0x4000109a: file src/lib.rs, line 498.
    Function "HardFault" not defined.
    Make breakpoint pending on future shared library load? (y or [n]) [answered N; input not from terminal]
    Breakpoint 2 at 0x40001054: file moondancer/src/panic_log.rs, line 15.
    Loading section .text, size 0x1172 lma 0x40000000
    Loading section .rodata, size 0x2f4 lma 0x40001174
    Loading section .data, size 0x10 lma 0x40001468
    Start address 0x40000000, load size 5238
    Transfer rate: 196 KB/sec, 1746 bytes/write.
    (gdb)

To begin execution simply type `continue` and, if you've got a serial terminal open to one of Cynthion's UART ports, you'll see something like:

    INFO    Peripherals initialized, entering main loop.
    INFO    left: 3
    INFO    right: 7
    INFO    left: 11
    ...

For more information about debugging Embedded Rust check out [The Embedded Rust Book - Hardware](https://docs.rust-embedded.org/book/start/hardware.html).
