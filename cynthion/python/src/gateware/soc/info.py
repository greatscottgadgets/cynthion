#
# This file is part of Cynthion.
#
# Copyright (c) 2024 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

from amaranth               import Cat, Elaboratable, Module, Signal, Record
from luna.gateware.platform import NullPin
from luna_soc.gateware.csr  import Peripheral

from apollo_fpga.gateware   import ApolloAdvertiser

class CynthionInformationPeripheral(Peripheral, Elaboratable):
    """ Peripheral for retrieving Cynthion hardware information at runtime.

    CSR registers
    -------------
    version_major : read
        Cynthion hardware major revision number.
    version_minor : read
        Cynthion hardware minor revision number.
    """

    def __init__(self):
        super().__init__()

        #
        # Registers
        #

        regs         = self.csr_bank()
        self._version_major = regs.csr(8, "r", desc="""
            Contains the Cynthion hardware major revision number.
        """)
        self._version_minor = regs.csr(8, "r", desc="""
            Contains the Cynthion hardware minor revision number.
        """)

        # wishbone connection
        self._bridge = self.bridge(data_width=32, granularity=8, alignment=2)
        self.bus     = self._bridge.bus


    def elaborate(self, platform):
        m = Module()
        m.submodules.bridge = self._bridge

        # Cynthion hardware revision numbers
        major, minor = platform.version
        m.d.comb += [
            self._version_major.r_data.eq(major),
            self._version_minor.r_data.eq(minor),
        ]

        return m
