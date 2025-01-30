#
# This file is part of Cynthion.
#
# Copyright (c) 2024 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

from amaranth                      import *
from amaranth.lib                  import wiring
from amaranth.lib.wiring           import In, flipped, connect

from amaranth_soc  import csr

from apollo_fpga.gateware          import ApolloAdvertiser


class Peripheral(wiring.Component):
    """ Controller peripheral for ApolloAdvertiser"""

    class Control(csr.Register, access="w"):
        """Control register

            enable : Set this bit to '1' to start ApolloAdvertiser and disconnect the
                     Cynthion USB control port from Apollo.
        """
        enable : csr.Field(csr.action.W, unsigned(1))


    def __init__(self, pad=None, clk_freq_hz=None):
        # advertiser
        self.advertiser = ApolloAdvertiser(pad=pad, clk_freq_hz=clk_freq_hz)

        # registers
        regs = csr.Builder(addr_width=1, data_width=8)
        self._control = regs.add("control", self.Control())

        # bridge
        self._bridge = csr.Bridge(regs.as_memory_map())

        # bus
        super().__init__({
            "bus" : In(self._bridge.bus.signature),
        })
        self.bus.memory_map = self._bridge.bus.memory_map


    def elaborate(self, platform):
        m = Module()
        m.submodules += self._bridge

        # bus
        connect(m, self.bus, self._bridge.bus)

        # advertiser stop signal is enabled by default
        stop = Signal(init=1)

        # update advertiser stop signal on register write
        with m.If(self._control.f.enable.w_stb):
            m.d.sync += stop.eq(~self._control.f.enable.w_data)

        # advertiser
        m.submodules.advertiser = self.advertiser
        m.d.comb += self.advertiser.stop.eq(stop)

        return m
