#
# This file is part of Cynthion.
#
# Copyright (c) 2024 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

from amaranth              import *
from amaranth.lib          import wiring
from amaranth.lib.wiring   import In, flipped, connect

from amaranth_soc          import csr


class Peripheral(wiring.Component):
    """ Peripheral for retrieving Cynthion hardware information at runtime.

    Registers
    ---------
    version_major : read
        Cynthion hardware major revision number.
    version_minor : read
        Cynthion hardware minor revision number.
    """

    class Version(csr.Register, access="r"):
        """Version register

            major : Contains the Cynthion hardware major revision number.
            minor : Contains the Cynthion hardware minor revision number.
        """
        major : csr.Field(csr.action.R, unsigned(8))
        minor : csr.Field(csr.action.R, unsigned(8))

    def __init__(self):
        # registers
        regs = csr.Builder(addr_width=4, data_width=8)
        self._version = regs.add("version", self.Version())

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

        connect(m, self.bus, self._bridge.bus)

        # Cynthion hardware revision numbers
        major, minor = platform.version
        m.d.comb += [
            self._version.f.major.r_data.eq(major),
            self._version.f.minor.r_data.eq(minor),
        ]

        return m
