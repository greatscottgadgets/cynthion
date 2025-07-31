#
# This file is part of Cynthion.
#
# Copyright (c) 2020-2023 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

import os

from amaranth import Fragment, Module

from amaranth.build import *
try:
    from amaranth.vendor.lattice_ecp5 import LatticeECP5Platform
except:
    from amaranth.vendor import LatticeECP5Platform
from amaranth_boards.resources import *

from luna.gateware.platform.core import LUNAApolloPlatform
from luna.gateware.architecture.car import LunaECP5DomainGenerator

__all__ = ["CynthionPlatform"]

class CynthionPlatform(LUNAApolloPlatform, LatticeECP5Platform):
    """ Board description for Cynthion """

    default_clk = "clk_60MHz"

    # Provide the type that'll be used to create our clock domains.
    clock_domain_generator = LunaECP5DomainGenerator

    #
    # Default clock frequencies for each of our clock domains.
    #
    # Different revisions have different FPGA speed grades, and thus the
    # default frequencies will vary.
    #
    DEFAULT_CLOCK_FREQUENCIES_MHZ = {
        "fast": 240,
        "sync": 120,
        "usb":  60
    }

    #
    # Preferred DRAM bus I/O (de)-skewing constants.
    #
    ram_timings = dict(
        # Set max skew to meet IO setup times
        # TODO: remove this & use the PLL to produce a 90degree clock signal instead.
        clock_skew = 127
    )

    # Provides any platform-specific ULPI registers necessary.
    # This is the spot to put any platform-specific vendor registers that need
    # to be written.
    ulpi_extra_registers = {
        0x39: 0b000110 # USB3343: swap D+ and D- to match the hardware design
    }

    def toolchain_prepare(self, fragment, name, **kwargs):
        overrides = {
            'ecppack_opts': '--compress --freq 38.8'
        }

        return super().toolchain_prepare(fragment, name, **overrides, **kwargs)

    def toolchain_program(self, products, name):
        """ Programs the relevant LUNA board via its sideband connection. """

        from apollo_fpga import ApolloDebugger
        from apollo_fpga.ecp5 import ECP5_JTAGProgrammer

        # Create our connection to the debug module.
        debugger = ApolloDebugger()

        # Grab our generated bitstream, and upload it to the FPGA.
        bitstream =  products.get("{}.bit".format(name))
        with debugger.jtag as jtag:
            programmer = ECP5_JTAGProgrammer(jtag)
            programmer.configure(bitstream)

        # Let the LUNA gateware take over in devices with shared USB port
        try:
            debugger.allow_fpga_takeover_usb()
        except AttributeError:
            pass


    def toolchain_flash(self, products, name="top"):
        """ Programs the LUNA board's flash via its sideband connection. """

        from apollo_fpga import ApolloDebugger
        from apollo_fpga.ecp5 import ECP5_JTAGProgrammer

        # Create our connection to the debug module.
        debugger = ApolloDebugger()
        self._ensure_unconfigured(debugger)

        # Grab our generated bitstream, and upload it to the .
        bitstream =  products.get("{}.bit".format(name))
        with debugger.jtag as jtag:
            programmer = ECP5_JTAGProgrammer(jtag)
            programmer.flash(bitstream)

        debugger.soft_reset()

        # Let the LUNA gateware take over in devices with shared USB port
        try:
            debugger.allow_fpga_takeover_usb()
        except AttributeError:
            pass

    def pseudo_power_supply_fragment(self):
        """ Fragment to assign fixed values to the pseudo power supply pins """
        m = Module()
        if ("pseudo_vccio", 0) in self.resources:
            m.d.comb += self.request("pseudo_vccio").o.eq(-1)
        if ("pseudo_gnd", 0) in self.resources:
            m.d.comb += self.request("pseudo_gnd").o.eq(0)
        return m

    # This list can be safely overriden in child classes.
    # Fragments defined in this class are always added.
    append_fragments = [ pseudo_power_supply_fragment ]

    def prepare(self, elaboratable, name="top", **kwargs):
        fragment = Fragment.get(elaboratable, self)

        # Merge base CynthionPlatform fragments with board-specific ones
        append_fragments = set(CynthionPlatform.append_fragments) | set(self.append_fragments)

        for subfragment in append_fragments:
            fragment.add_subfragment(Fragment.get(subfragment(self), self))

        return super().prepare(fragment, name, **kwargs)
