#
# This file is part of Cynthion.
#
# Copyright (c) 2023 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

import logging
import os
import sys

from amaranth                 import Cat, DomainRenamer, Elaboratable, Module, ResetSignal, Signal, Record
from amaranth.build           import Attrs, Pins, PinsN, Resource, Subsignal
from amaranth.hdl.rec         import Record
from amaranth_stdio.serial    import AsyncSerial

from luna_soc.gateware.vendor.lambdasoc.periph         import Peripheral
from luna_soc.gateware.vendor.lambdasoc.periph.serial  import AsyncSerialPeripheral

from luna                            import configure_default_logging, top_level_cli
from luna.gateware.platform          import NullPin
from luna.gateware.usb.usb2.device   import USBDevice

from luna_soc.gateware.cpu.vexriscv  import VexRiscv
from luna_soc.gateware.lunasoc       import LunaSoC

from luna_soc.gateware.csr           import GpioPeripheral, LedPeripheral
from luna_soc.gateware.csr           import USBDeviceController
from luna_soc.gateware.csr           import SetupFIFOInterface, InFIFOInterface, OutFIFOInterface

from luna_soc.gateware.wishbone      import ECP5ConfigurationFlashInterface, WishboneSPIFlashReader

from .advertiser import ApolloAdvertiserPeripheral


# - MoondancerSoc ---------------------------------------------------------------

class MoondancerSoc(Elaboratable):
    ADDITIONAL_RESOURCES = [
        # PMOD B: UART
        Resource("uart", 1,
            Subsignal("rx",  Pins("1", conn=("pmod", 1), dir="i")),
            Subsignal("tx",  Pins("2", conn=("pmod", 1), dir="o")),
            Attrs(IO_TYPE="LVCMOS33")
        ),

        # PMOD B: JTAG
        Resource("jtag", 0,
            Subsignal("tms",  Pins("7",  conn=("pmod", 1), dir="i")),
            Subsignal("tdi",  Pins("8",  conn=("pmod", 1), dir="i")),
            Subsignal("tdo",  Pins("9",  conn=("pmod", 1), dir="o")),
            Subsignal("tck",  Pins("10", conn=("pmod", 1), dir="i")),
            Attrs(IO_TYPE="LVCMOS33")
        ),
    ]

    def __init__(self, clock_frequency, uart_baud_rate=115200):

        # TODO support an offset for flash address
        flash_offset = 0x00000000

        # Create our SoC...
        self.soc = LunaSoC(
            cpu=VexRiscv(
                variant="cynthion+jtag",
                reset_addr=0x10000000 + flash_offset, # spi flash address + offset
            ),
            clock_frequency=clock_frequency,
        )

        # ... add some stand-ins for our uart pins ...
        self.uart0_pins = Record([
            ('rx', [('i', 1)]),
            ('tx', [('o', 1)])
        ])
        self.uart1_pins = Record([
            ('rx', [('i', 1)]),
            ('tx', [('o', 1)])
        ])

        # ... add core peripherals: memory, timer, uart ...
        self.soc.add_core_peripherals(
            uart_pins=self.uart0_pins,
            uart_baud_rate=uart_baud_rate,
            internal_sram_size=65536,
            internal_sram_addr=0x40000000,
        )

        # ... add a qspi flash reader peripheral ...
        self.spi_bus = ECP5ConfigurationFlashInterface(bus=platform.request("qspi_flash"))
        self.spi_reader = WishboneSPIFlashReader(pads=self.spi_bus, size=0x400000, name="spiflash", domain="usb")
        self.soc.add_peripheral(self.spi_reader, addr=0x10000000)

        # ... add our LED peripheral, for simple output ...
        self.leds = LedPeripheral()
        self.soc.add_peripheral(self.leds, addr=0xf0001000)

        # ... add two gpio peripherals for our PMOD connectors ...
        self.gpioa = GpioPeripheral(width=8)
        self.gpiob = GpioPeripheral(width=8)
        self.soc.add_peripheral(self.gpioa, addr=0xf0002000)
        self.soc.add_peripheral(self.gpiob, addr=0xf0002100)

        # ... and the core USB controllers and eptri peripherals ...
        self.usb0 = USBDeviceController()
        self.usb0_ep_control = SetupFIFOInterface()
        self.usb0_ep_in = InFIFOInterface()
        self.usb0_ep_out = OutFIFOInterface()
        self.soc.add_peripheral(self.usb0, addr=0xf0003000)
        self.soc.add_peripheral(self.usb0_ep_control, as_submodule=False)
        self.soc.add_peripheral(self.usb0_ep_in, as_submodule=False)
        self.soc.add_peripheral(self.usb0_ep_out, as_submodule=False)

        self.usb1 = USBDeviceController()
        self.usb1_ep_control = SetupFIFOInterface()
        self.usb1_ep_in = InFIFOInterface()
        self.usb1_ep_out = OutFIFOInterface()
        self.soc.add_peripheral(self.usb1, addr=0xf0004000)
        self.soc.add_peripheral(self.usb1_ep_control, as_submodule=False)
        self.soc.add_peripheral(self.usb1_ep_in, as_submodule=False)
        self.soc.add_peripheral(self.usb1_ep_out, as_submodule=False)

        self.usb2 = USBDeviceController()
        self.usb2_ep_control = SetupFIFOInterface()
        self.usb2_ep_in = InFIFOInterface()
        self.usb2_ep_out = OutFIFOInterface()
        self.soc.add_peripheral(self.usb2, addr=0xf0005000)
        self.soc.add_peripheral(self.usb2_ep_control, as_submodule=False)
        self.soc.add_peripheral(self.usb2_ep_in, as_submodule=False)
        self.soc.add_peripheral(self.usb2_ep_out, as_submodule=False)

        # ... add a second uart peripheral ...
        self.uart1 = AsyncSerialPeripheral(core=AsyncSerial(
            data_bits = 8,
            divisor   = int(clock_frequency // uart_baud_rate),
            pins      = self.uart1_pins,
        ))
        self.soc.add_peripheral(self.uart1, addr=0xf0006000)

        # ... add an ApolloAdvertiser peripheral ...
        self.advertiser = ApolloAdvertiserPeripheral(clk_freq_hz=clock_frequency)
        self.soc.add_peripheral(self.advertiser, addr=0xf0007000)


    def elaborate(self, platform):
        m = Module()

        # add additional resource
        platform.add_resources(self.ADDITIONAL_RESOURCES)

        # generate our domain clocks/resets
        m.submodules.car = platform.clock_domain_generator()

        # add SoC to design and clock it off the 60 MHz "usb" domain
        # because VexriscV synthesis tops out at ~77 MHz
        m.submodules.soc = DomainRenamer({"sync": "usb"})(self.soc)

        # wire up the cpu external reset signal
        try:
            user1_io = platform.request("button_user")
            m.d.comb += self.soc.cpu.ext_reset.eq(user1_io.i)
        except:
            logging.warning("Platform does not support a user button for cpu reset")

        # add the spi bus
        m.submodules.spi_bus = self.spi_bus

        # connect GPIOA to Cynthion's PMOD A port
        pmoda_io = platform.request("user_pmod", 0)
        m.d.comb += [
            self.gpioa.pins.connect(pmoda_io),
        ]

        # connect UART0 to Cynthion's SAMD11 uart
        uart0_io = platform.request("uart", 0)
        m.d.comb += [
            uart0_io.tx.o.eq(self.uart0_pins.tx),
            self.uart0_pins.rx.eq(uart0_io.rx)
        ]
        if hasattr(uart0_io.tx, 'oe'):
            m.d.comb += uart0_io.tx.oe.eq(~self.soc.uart._phy.tx.rdy),

        # connect UART1 to Cynthion's PMOD B port
        uart1_io = platform.request("uart", 1)
        m.d.comb += [
            uart1_io.tx.o.eq(self.uart1_pins.tx),
            self.uart1_pins.rx.eq(uart1_io.rx)
        ]

        # connect JTAG0 to Cynthion's PMOD B port
        jtag0_io = platform.request("jtag", 0)
        m.d.comb += [
            self.soc.cpu.jtag_tms  .eq(jtag0_io.tms.i),
            self.soc.cpu.jtag_tdi  .eq(jtag0_io.tdi.i),
            jtag0_io.tdo.o         .eq(self.soc.cpu.jtag_tdo),
            self.soc.cpu.jtag_tck  .eq(jtag0_io.tck.i),
            self.soc.cpu.dbg_reset .eq(self.soc.cpu.ext_reset),
        ]

        # workaround: disable platform usb device hooks to take care
        # of the fact that USBDevice is under firmware control
        #
        # TODO 2024/04/17 remove this once the platform files are updated
        platform.usb_device_hooks = {}

        # create our USB devices, connect device controllers and add eptri endpoint handlers

        # target_phy
        ulpi0 = platform.request("target_phy")
        usb0_device = USBDevice(bus=ulpi0)
        usb0_device.add_endpoint(self.usb0_ep_control)
        usb0_device.add_endpoint(self.usb0_ep_in)
        usb0_device.add_endpoint(self.usb0_ep_out)
        m.d.comb += self.usb0.attach(usb0_device)
        m.submodules.usb0_device = usb0_device

        # aux_phy
        try:
            ulpi1 = platform.request("aux_phy")
        except:
            ulpi1 = platform.request("host_phy")
        usb1_device = USBDevice(bus=ulpi1)
        usb1_device.add_endpoint(self.usb1_ep_control)
        usb1_device.add_endpoint(self.usb1_ep_in)
        usb1_device.add_endpoint(self.usb1_ep_out)
        m.d.comb += self.usb1.attach(usb1_device)
        m.submodules.usb1_device = usb1_device

        # control_phy
        try:
            ulpi2 = platform.request("control_phy")
        except:
            ulpi2 = platform.request("sideband_phy")
        usb2_device = USBDevice(bus=ulpi2)
        usb2_device.add_endpoint(self.usb2_ep_control)
        usb2_device.add_endpoint(self.usb2_ep_in)
        usb2_device.add_endpoint(self.usb2_ep_out)
        m.d.comb += self.usb2.attach(usb2_device)
        m.submodules.usb2_device = usb2_device

        return m



# - main ----------------------------------------------------------------------

import luna

from luna.gateware.platform  import get_appropriate_platform
from luna_soc.generate       import Generate

if __name__ == "__main__":
    # Disable UnusedElaborable warnings
    from amaranth._unused import MustUse
    MustUse._MustUse__silence = True

    build_dir = os.path.join("build")

    # configure logging
    configure_default_logging()
    logging.getLogger().setLevel(logging.DEBUG)

    # select platform
    platform = get_appropriate_platform()
    if platform is None:
        logging.error("Failed to identify a supported platform")
        sys.exit(1)

    # configure clock frequency
    clock_frequency = int(platform.DEFAULT_CLOCK_FREQUENCIES_MHZ["usb"] * 1e6)

    logging.info(f"Building for {platform} with clock frequency: {clock_frequency}")

    # create design
    design = MoondancerSoc(clock_frequency=clock_frequency)

    # build soc
    logging.info("Building soc")
    overrides = {
        "debug_verilog": False,
        "verbose": False,
    }
    products = platform.build(design, do_program=False, build_dir=build_dir, **overrides)

    # log resources
    from luna_soc.generate import Introspect
    Introspect(design.soc).log_resources()

    # generate artifacts
    generate = Generate(design.soc)

    # generate: c-header and ld-script
    path = os.path.join(build_dir, "genc")
    if not os.path.exists(path):
        os.makedirs(path)

    logging.info("Generating c-header and ld-script: {}".format(path))
    with open(os.path.join(path, "resources.h"), "w") as f:
        generate.c_header(platform_name=platform.name, file=f)
    with open(os.path.join(path, "soc.ld"), "w") as f:
        generate.ld_script(file=f)

    # generate: svd file
    path = os.path.join(build_dir, "gensvd")
    if not os.path.exists(path):
        os.makedirs(path)

    logging.info("Generating svd file: {}".format(path))
    with open(os.path.join(path, "lunasoc.svd"), "w") as f:
        generate.svd(file=f)

    # generate: rust memory.x file
    path = os.path.join(build_dir, "genrust")
    if not os.path.exists(path):
        os.makedirs(path)

    logging.info("Generating memory.x file: {}".format(path))
    with open(os.path.join(path, "memory.x"), "w") as f:
        generate.memory_x(file=f)

    print("Build completed. Use 'cynthion configure build/top.bit' to upload bitstream to device.")

    # TODO
    #top_level_cli(design)
