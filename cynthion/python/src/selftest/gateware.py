#!/usr/bin/env python3
#
# This file is part of Cynthion.
#
# Copyright (c) 2020-2024 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

from amaranth import Signal, Elaboratable, Module, Cat, ClockDomain, ClockSignal, ResetSignal
from amaranth.lib.cdc import FFSynchronizer

from luna.gateware.architecture.car   import LunaECP5DomainGenerator
from luna.gateware.interface.jtag     import JTAGRegisterInterface
from luna.gateware.interface.ulpi     import ULPIRegisterWindow
from luna.gateware.interface.psram    import HyperRAMPHY, HyperRAMInterface

from .registers import *


CLOCK_FREQUENCIES = {
    "fast": 60,
    "sync": 60,
    "usb":  60
}


class SelftestDevice(Elaboratable):

    def elaborate(self, platform):
        m = Module()

        # Generate our clock domains.
        clocking = LunaECP5DomainGenerator(clock_frequencies=CLOCK_FREQUENCIES)
        m.submodules.clocking = clocking

        registers = JTAGRegisterInterface(default_read_value=0xDEADBEEF)
        m.submodules.registers = registers

        # Simple applet ID register.
        registers.add_read_only_register(REGISTER_ID, read=0x54455354)

        # LED test register.
        led_reg = registers.add_register(REGISTER_LEDS, size=6, name="leds", init=0b111111)
        led_out   = Cat([platform.request("led", i, dir="o").o for i in range(0, 6)])
        m.d.comb += led_out.eq(led_reg)

        #
        # ULPI PHY windows
        #
        self.add_ulpi_registers(m, platform,
            ulpi_bus="target_phy",
            register_base=REGISTER_TARGET_ADDR
        )
        self.add_ulpi_registers(m, platform,
            ulpi_bus="aux_phy" if platform.version >= (0, 6) else "host_phy",
            register_base=REGISTER_AUX_ADDR
        )
        self.add_ulpi_registers(m, platform,
            ulpi_bus="control_phy" if platform.version >= (0, 6) else "sideband_phy",
            register_base=REGISTER_CONTROL_ADDR
        )


        #
        # HyperRAM test connections.
        #
        ram_bus = platform.request('ram')
        psram_phy = HyperRAMPHY(bus=ram_bus)
        psram = HyperRAMInterface(phy=psram_phy.phy)
        m.submodules += [psram_phy, psram]

        psram_address_changed = Signal()
        psram_address = registers.add_register(REGISTER_RAM_REG_ADDR, write_strobe=psram_address_changed)

        # Store last read word from HyperRAM.
        psram_read_data = Signal.like(psram.read_data)
        with m.If(psram.read_ready):
            m.d.sync += psram_read_data.eq(psram.read_data)
        registers.add_sfr(REGISTER_RAM_VALUE, read=psram_read_data)

        # Hook up our PSRAM.
        m.d.comb += [
            ram_bus.reset.o        .eq(0),
            psram.single_page      .eq(0),
            psram.perform_write    .eq(0),
            psram.register_space   .eq(1),
            psram.final_word       .eq(1),
            psram.start_transfer   .eq(psram_address_changed),
            psram.address          .eq(psram_address),
        ]

        return m


    def add_ulpi_registers(self, m, platform, *, ulpi_bus, register_base):
        """ Adds a set of ULPI registers to the active design. """

        target_ulpi      = platform.request(ulpi_bus)

        ulpi_reg_window  = ULPIRegisterWindow()
        m.submodules  += ulpi_reg_window

        m.d.comb += [
            ulpi_reg_window.ulpi_data_in  .eq(target_ulpi.data.i),
            ulpi_reg_window.ulpi_dir      .eq(target_ulpi.dir.i),
            ulpi_reg_window.ulpi_next     .eq(target_ulpi.nxt.i),

            target_ulpi.clk.o    .eq(ClockSignal("usb")),
            target_ulpi.rst.o    .eq(ResetSignal("usb")),
            target_ulpi.stp.o    .eq(ulpi_reg_window.ulpi_stop),
            target_ulpi.data.o   .eq(ulpi_reg_window.ulpi_data_out),
            target_ulpi.data.oe  .eq(~target_ulpi.dir.i)
        ]

        register_address_change  = Signal()
        register_value_change    = Signal()

        # ULPI register address.
        registers = m.submodules.registers
        registers.add_register(register_base + 0,
            write_strobe=register_address_change,
            value_signal=ulpi_reg_window.address,
            size=6
        )
        m.submodules.clocking.stretch_sync_strobe_to_usb(m,
            strobe=register_address_change,
            output=ulpi_reg_window.read_request,
        )

        # ULPI register value.
        registers.add_sfr(register_base + 1,
            read=ulpi_reg_window.read_data,
            write_signal=ulpi_reg_window.write_data,
            write_strobe=register_value_change
        )
        m.submodules.clocking.stretch_sync_strobe_to_usb(m,
            strobe=register_value_change,
            output=ulpi_reg_window.write_request
        )
