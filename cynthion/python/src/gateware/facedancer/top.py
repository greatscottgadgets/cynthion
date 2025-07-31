#
# This file is part of Cynthion.
#
# Copyright (c) 2020-2025 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

import logging, os, sys

from amaranth                        import *
from amaranth.build                  import Attrs, Pins, PinsN, Platform, Resource, Subsignal
from amaranth.lib                    import wiring
from amaranth.lib.wiring             import Component, In, Out, flipped

from luna.gateware.usb.usb2.device   import USBDevice

from amaranth_soc                    import csr, gpio, wishbone
from amaranth_soc.csr.wishbone       import WishboneCSRBridge

from luna_soc.gateware.core          import blockram, spiflash, timer, uart, usb2
from luna_soc.gateware.core.spiflash import ECP5ConfigurationFlashInterface, SPIPHYController
from luna_soc.gateware.cpu           import InterruptController, VexRiscv
from luna_soc.gateware.provider      import cynthion as provider

from . import advertiser, info


# - component: Soc ------------------------------------------------------------

class Soc(Component):
    def __init__(self, clock_frequency_hz, domain="sync"):
        super().__init__({})

        self.clock_frequency_hz = clock_frequency_hz
        self.domain             = domain
        self.firmware_start     = 0x000b0000

        # configuration
        self.blockram_base        = 0x00000000
        self.blockram_size        = 0x00010000  # 65536 bytes
        self.spiflash_base        = 0x10000000
        self.spiflash_size        = 0x00400000  # 4 MiB
        self.hyperram_base        = 0x20000000  # Winbond W956A8MBYA6I
        self.hyperram_size        = 0x08000000  # 8 * 1024 * 1024

        self.csr_base             = 0xf0000000
        self.leds_base            = 0x00000000
        self.gpio0_base           = 0x00000100
        self.gpio1_base           = 0x00000200
        self.uart0_base           = 0x00000300
        self.uart1_base           = 0x00000400
        self.timer0_base          = 0x00000500
        self.timer0_irq           = 0
        self.timer1_base          = 0x00000600
        self.timer1_irq           = 1
        self.spi0_base            = 0x00000700
        self.usb0_base            = 0x00000800
        self.usb0_irq             = 2
        self.usb0_ep_control_base = 0x00000900
        self.usb0_ep_control_irq  = 3
        self.usb0_ep_in_base      = 0x00000a00
        self.usb0_ep_in_irq       = 4
        self.usb0_ep_out_base     = 0x00000b00
        self.usb0_ep_out_irq      = 5
        self.usb1_base            = 0x00000c00
        self.usb1_irq             = 6
        self.usb1_ep_control_base = 0x00000d00
        self.usb1_ep_control_irq  = 7
        self.usb1_ep_in_base      = 0x00000e00
        self.usb1_ep_in_irq       = 8
        self.usb1_ep_out_base     = 0x00000f00
        self.usb1_ep_out_irq      = 9
        self.usb2_base            = 0x00001000
        self.usb2_irq             = 10
        self.usb2_ep_control_base = 0x00001100
        self.usb2_ep_control_irq  = 11
        self.usb2_ep_in_base      = 0x00001200
        self.usb2_ep_in_irq       = 12
        self.usb2_ep_out_base     = 0x00001300
        self.usb2_ep_out_irq      = 13
        self.advertiser_base      = 0x00001400
        self.info_base            = 0x00001500
        self.user0_base           = 0x00001600

        # cpu
        self.cpu = VexRiscv(
            variant="cynthion+jtag",
            reset_addr=self.spiflash_base + self.firmware_start
        )

        # interrupt controller
        self.interrupt_controller = InterruptController(width=len(self.cpu.irq_external))

        # bus
        self.wb_arbiter  = wishbone.Arbiter(
            addr_width=30,
            data_width=32,
            granularity=8,
            features={"cti", "bte", "err"}
        )
        self.wb_decoder  = wishbone.Decoder(
            addr_width=30,
            data_width=32,
            granularity=8,
            features={"cti", "bte", "err"}
        )

        # blockram
        self.blockram = blockram.Peripheral(size=self.blockram_size)
        self.wb_decoder.add(self.blockram.bus, addr=self.blockram_base, name="blockram")

        # spiflash
        self.spiflash_provider = provider.QSPIFlashProvider("qspi_flash")
        self.spiflash_bus = ECP5ConfigurationFlashInterface(bus=self.spiflash_provider.pins)
        self.spiflash_phy = SPIPHYController(pads=self.spiflash_bus, domain=self.domain, divisor=0)
        self.spiflash = spiflash.Peripheral(
            self.spiflash_phy,
            with_controller = True,
            controller_name = "spi0",
            with_mmap       = True,
            mmap_size       = self.spiflash_size,
            mmap_name       = "spiflash",
            domain          = self.domain,
        )
        self.wb_decoder.add(self.spiflash.bus, addr=self.spiflash_base, name="spiflash")

        # csr decoder
        self.csr_decoder = csr.Decoder(addr_width=28, data_width=8)

        # spi0
        self.csr_decoder.add(self.spiflash.csr, addr=self.spi0_base, name="spi0")

        # leds
        self.led_count = 6
        self.leds = gpio.Peripheral(pin_count=self.led_count, addr_width=3, data_width=8)
        self.csr_decoder.add(self.leds.bus, addr=self.leds_base, name="leds")

        # gpio0
        self.gpio0 = gpio.Peripheral(pin_count=8, addr_width=3, data_width=8)
        self.csr_decoder.add(self.gpio0.bus, addr=self.gpio0_base, name="gpio0")

        # gpio1
        self.gpio1 = gpio.Peripheral(pin_count=8, addr_width=3, data_width=8)
        self.csr_decoder.add(self.gpio1.bus, addr=self.gpio1_base, name="gpio1")

        # uart0
        uart_baud_rate = 115200
        divisor = int(clock_frequency_hz // uart_baud_rate)
        self.uart0 = uart.Peripheral(divisor=divisor)
        self.csr_decoder.add(self.uart0.bus, addr=self.uart0_base, name="uart0")

        # uart1
        uart_baud_rate = 115200
        divisor = int(clock_frequency_hz // uart_baud_rate)
        self.uart1 = uart.Peripheral(divisor=divisor)
        self.csr_decoder.add(self.uart1.bus, addr=self.uart1_base, name="uart1")

        # timer0
        self.timer0 = timer.Peripheral(width=32)
        self.csr_decoder.add(self.timer0.bus, addr=self.timer0_base, name="timer0")
        self.interrupt_controller.add(self.timer0, number=self.timer0_irq, name="timer0")

        # timer1
        self.timer1 = timer.Peripheral(width=32)
        self.csr_decoder.add(self.timer1.bus, addr=self.timer1_base, name="timer1")
        self.interrupt_controller.add(self.timer1, name="timer1", number=self.timer1_irq)

        # usb0 - target_phy
        self.usb0            = usb2.device.Peripheral()
        self.usb0_ep_control = usb2.ep_control.Peripheral()
        self.usb0_ep_in      = usb2.ep_in.Peripheral()
        self.usb0_ep_out     = usb2.ep_out.Peripheral()
        self.csr_decoder.add(self.usb0.bus,            addr=self.usb0_base,            name="usb0")
        self.csr_decoder.add(self.usb0_ep_control.bus, addr=self.usb0_ep_control_base, name="usb0_ep_control")
        self.csr_decoder.add(self.usb0_ep_in.bus,      addr=self.usb0_ep_in_base,      name="usb0_ep_in")
        self.csr_decoder.add(self.usb0_ep_out.bus,     addr=self.usb0_ep_out_base,     name="usb0_ep_out")
        self.interrupt_controller.add(self.usb0,            name="usb0",            number=self.usb0_irq)
        self.interrupt_controller.add(self.usb0_ep_control, name="usb0_ep_control", number=self.usb0_ep_control_irq)
        self.interrupt_controller.add(self.usb0_ep_in,      name="usb0_ep_in",      number=self.usb0_ep_in_irq)
        self.interrupt_controller.add(self.usb0_ep_out,     name="usb0_ep_out",     number=self.usb0_ep_out_irq)

        # usb1 - aux_phy
        self.usb1            = usb2.device.Peripheral()
        self.usb1_ep_control = usb2.ep_control.Peripheral()
        self.usb1_ep_in      = usb2.ep_in.Peripheral()
        self.usb1_ep_out     = usb2.ep_out.Peripheral()
        self.csr_decoder.add(self.usb1.bus,            addr=self.usb1_base,            name="usb1")
        self.csr_decoder.add(self.usb1_ep_control.bus, addr=self.usb1_ep_control_base, name="usb1_ep_control")
        self.csr_decoder.add(self.usb1_ep_in.bus,      addr=self.usb1_ep_in_base,      name="usb1_ep_in")
        self.csr_decoder.add(self.usb1_ep_out.bus,     addr=self.usb1_ep_out_base,     name="usb1_ep_out")
        self.interrupt_controller.add(self.usb1,            name="usb1",            number=self.usb1_irq)
        self.interrupt_controller.add(self.usb1_ep_control, name="usb1_ep_control", number=self.usb1_ep_control_irq)
        self.interrupt_controller.add(self.usb1_ep_in,      name="usb1_ep_in",      number=self.usb1_ep_in_irq)
        self.interrupt_controller.add(self.usb1_ep_out,     name="usb1_ep_out",     number=self.usb1_ep_out_irq)

        # usb2 - control_phy
        self.usb2            = usb2.device.Peripheral()
        self.usb2_ep_control = usb2.ep_control.Peripheral()
        self.usb2_ep_in      = usb2.ep_in.Peripheral()
        self.usb2_ep_out     = usb2.ep_out.Peripheral()
        self.csr_decoder.add(self.usb2.bus,            addr=self.usb2_base,            name="usb2")
        self.csr_decoder.add(self.usb2_ep_control.bus, addr=self.usb2_ep_control_base, name="usb2_ep_control")
        self.csr_decoder.add(self.usb2_ep_in.bus,      addr=self.usb2_ep_in_base,      name="usb2_ep_in")
        self.csr_decoder.add(self.usb2_ep_out.bus,     addr=self.usb2_ep_out_base,     name="usb2_ep_out")
        self.interrupt_controller.add(self.usb2,            name="usb2",            number=self.usb2_irq)
        self.interrupt_controller.add(self.usb2_ep_control, name="usb2_ep_control", number=self.usb2_ep_control_irq)
        self.interrupt_controller.add(self.usb2_ep_in,      name="usb2_ep_in",      number=self.usb2_ep_in_irq)
        self.interrupt_controller.add(self.usb2_ep_out,     name="usb2_ep_out",     number=self.usb2_ep_out_irq)

        # apollo advertiser
        self.advertiser_provider = provider.ApolloAdvertiserProvider("int")
        self.advertiser = advertiser.Peripheral(pad=self.advertiser_provider.pins, clk_freq_hz=clock_frequency_hz)
        self.csr_decoder.add(self.advertiser.bus, addr=self.advertiser_base, name="advertiser")

        # soc info
        self.info = info.Peripheral()
        self.csr_decoder.add(self.info.bus, addr=self.info_base, name="info")

        # user0
        self.user0 = gpio.Peripheral(pin_count=1, addr_width=3, data_width=8)
        self.csr_decoder.add(self.user0.bus, addr=self.user0_base, name="user0")

        # wishbone csr bridge
        self.wb_to_csr = WishboneCSRBridge(self.csr_decoder.bus, data_width=32)
        self.wb_decoder.add(self.wb_to_csr.wb_bus, addr=self.csr_base, sparse=False, name="wb_to_csr")

    def elaborate(self, platform):
        m = Module()

        # bus
        m.submodules += [self.wb_arbiter, self.wb_decoder]
        wiring.connect(m, self.wb_arbiter.bus, self.wb_decoder.bus)

        # cpu
        m.submodules += self.cpu
        self.wb_arbiter.add(self.cpu.ibus)
        self.wb_arbiter.add(self.cpu.dbus)

        # interrupt controller
        m.submodules += self.interrupt_controller
        m.d.comb += self.cpu.irq_external.eq(self.interrupt_controller.pending)

        # blockram
        m.submodules += self.blockram

        # spiflash
        m.submodules += [self.spiflash_provider, self.spiflash, self.spiflash_bus, self.spiflash_phy]

        # csr decoder
        m.submodules += self.csr_decoder

        # leds
        led_provider = provider.LEDProvider("led", pin_count=self.led_count)
        m.submodules += [led_provider, self.leds]
        for n in range(self.led_count):
            wiring.connect(m, self.leds.pins[n], led_provider.pins[n])

        # gpio0
        gpio0_provider = provider.GPIOProvider("user_pmod", 0)
        m.submodules += [gpio0_provider, self.gpio0]
        for n in range(8):
            wiring.connect(m, self.gpio0.pins[n], gpio0_provider.pins[n])

        # gpio1
        # gpio1_provider = provider.GPIOProvider("user_pmod", 1)
        # m.submodules += [gpio1_provider, self.gpio1]
        # for n in range(8):
        #     wiring.connect(m, self.gpio1.pins[n], gpio1_provider.pins[n])

        # uart0
        uart0_provider = provider.UARTProvider("uart", 0)
        m.submodules += [uart0_provider, self.uart0]
        wiring.connect(m, self.uart0.pins, uart0_provider.pins)

        # uart1
        uart1_provider = provider.UARTProvider("uart", 1)
        m.submodules += [uart1_provider, self.uart1]
        wiring.connect(m, self.uart1.pins, uart1_provider.pins)

        # timer0
        m.submodules += self.timer0

        # timer1
        m.submodules += self.timer1

        # usb0 - target_phy
        ulpi0_provider = provider.ULPIProvider("target_phy")
        usb0_device = USBDevice(bus=ulpi0_provider.bus)
        usb0_device.add_endpoint(self.usb0_ep_control)
        usb0_device.add_endpoint(self.usb0_ep_in)
        usb0_device.add_endpoint(self.usb0_ep_out)
        m.d.comb += self.usb0.attach(usb0_device)
        m.submodules += [ulpi0_provider, self.usb0, usb0_device]

        # usb1 - aux_phy
        ulpi1_provider = provider.ULPIProvider(["aux_phy", "host_phy"])
        usb1_device = USBDevice(bus=ulpi1_provider.bus)
        usb1_device.add_endpoint(self.usb1_ep_control)
        usb1_device.add_endpoint(self.usb1_ep_in)
        usb1_device.add_endpoint(self.usb1_ep_out)
        m.d.comb += self.usb1.attach(usb1_device)
        m.submodules += [ulpi1_provider, self.usb1, usb1_device]

        # usb2 - control_phy
        ulpi2_provider = provider.ULPIProvider(["control_phy", "sideband_phy"])
        usb2_device = USBDevice(bus=ulpi2_provider.bus)
        usb2_device.add_endpoint(self.usb2_ep_control)
        usb2_device.add_endpoint(self.usb2_ep_in)
        usb2_device.add_endpoint(self.usb2_ep_out)
        m.d.comb += self.usb2.attach(usb2_device)
        m.submodules += [ulpi2_provider, self.usb2, usb2_device]

        # advertiser
        m.submodules += [self.advertiser, self.advertiser_provider]

        # info
        m.submodules += self.info

        # user0
        user0_provider = provider.ButtonProvider("button_user", 0)
        m.submodules += [user0_provider, self.user0]
        wiring.connect(m, self.user0.pins[0], user0_provider.pins[0])

        # wishbone csr bridge
        m.submodules += self.wb_to_csr

        # wire up the cpu external reset signal
        delay = Signal(18)
        with m.If(~delay.all()):
            m.d.sync += delay.eq(delay + 1)
            m.d.comb += self.cpu.ext_reset.eq(1)

        # wire up the cpu jtag signals
        try:
            jtag0_io = platform.request("jtag", 0)
            m.d.comb += [
                self.cpu.jtag_tms     .eq(jtag0_io.tms.i),
                self.cpu.jtag_tdi     .eq(jtag0_io.tdi.i),
                jtag0_io.tdo.o        .eq(self.cpu.jtag_tdo),
                self.cpu.jtag_tck     .eq(jtag0_io.tck.i),
            ]
        except:
            logging.warning("Platform does not support jtag")

        return DomainRenamer({
            "sync": self.domain,
        })(m)


# - module: Top ---------------------------------------------------------------

class Top(Elaboratable):
    ADDITIONAL_RESOURCES = [
        # PMOD B: UART
        Resource("uart", 1,
            Subsignal("rx",  Pins("1", conn=("pmod", 1), dir="i")),
            Subsignal("tx",  Pins("2", conn=("pmod", 1), dir="o")),
            Attrs(IO_TYPE="LVCMOS33")
        ),

        # PMOD B: DEBUG
        Resource("debug", 0,
            Subsignal("a",  Pins("3", conn=("pmod", 1), dir="io")),
            Subsignal("b",  Pins("4", conn=("pmod", 1), dir="io")),
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

    def __init__(self, clock_frequency_hz, domain="sync"):
        self.clock_frequency_hz = clock_frequency_hz
        self.domain = domain

        self.soc = Soc(clock_frequency_hz=self.clock_frequency_hz, domain=self.domain)

    def elaborate(self, platform):
        # add additional resources (only supported on platforms > r0.4)
        if platform.version not in [(0, 1), (0, 2), (0, 3), (0, 4)]:
            platform.add_resources(self.ADDITIONAL_RESOURCES)

        m = Module()

        # generate our domain clocks/resets
        m.submodules.car = platform.clock_domain_generator()

        # add soc to design
        m.submodules += self.soc

        return m


# - build ---------------------------------------------------------------------

if __name__ == "__main__":
    from luna                    import configure_default_logging
    from luna.gateware.platform  import get_appropriate_platform
    from luna_soc                import top_level_cli

    # configure logging
    configure_default_logging()
    logging.getLogger().setLevel(logging.DEBUG)

    # select platform
    platform = get_appropriate_platform()
    if platform is None:
        logging.error("Failed to identify a supported platform")
        sys.exit(1)

    # configure domain
    domain = "usb"

    # configure clock frequency
    clock_frequency_hz = int(platform.DEFAULT_CLOCK_FREQUENCIES_MHZ[domain] * 1e6)
    logging.info(f"Building for {platform} with domain {domain} and clock frequency: {clock_frequency_hz}")

    # create design
    design = Top(clock_frequency_hz=clock_frequency_hz, domain=domain)

    # invoke cli
    _overrides = {
        "debug_verilog": False,
        "verbose": False,
    }
    top_level_cli(design, **_overrides)
