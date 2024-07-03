#
# This file is part of Cynthion.
#
# Copyright (c) 2024 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

from amaranth               import Cat, Elaboratable, Module, Signal, Record
from luna.gateware.platform import NullPin
from luna_soc.gateware.csr  import Peripheral

from apollo_fpga.gateware   import ApolloAdvertiser

class ApolloAdvertiserPeripheral(Peripheral, Elaboratable):
    """ Controller peripheral for ApolloAdvertiser

    CSR registers
    -------------
    enable : read/write
        Set this bit to '1' to start ApolloAdvertiser and disconnect the Cynthion USB control port from Apollo.
    """

    def __init__(self, clk_freq_hz=None):
        super().__init__()

        self._clk_freq = clk_freq_hz

        #
        # Registers
        #

        regs         = self.csr_bank()
        self._enable = regs.csr(1, "rw", desc="""
            Set this bit to '1' to start ApolloAdvertiser and disconnect the Cynthion USB control port from Apollo.
        """)

        # wishbone connection
        self._bridge = self.bridge(data_width=32, granularity=8, alignment=2)
        self.bus     = self._bridge.bus


    def elaborate(self, platform):
        m = Module()
        m.submodules.bridge = self._bridge

        # advertiser stop signal is enabled by default
        stop = Signal(reset=1)

        # update advertiser stop signal on register write
        with m.If(self._enable.w_stb):
            m.d.sync += stop.eq(~self._enable.w_data)

        # create advertiser
        m.submodules.advertiser = advertiser = ApolloAdvertiser(clk_freq_hz=self._clk_freq)
        m.d.comb += advertiser.stop.eq(stop)

        return m
