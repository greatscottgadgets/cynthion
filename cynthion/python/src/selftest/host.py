#!/usr/bin/env python3
#
# This file is part of Cynthion.
#
# Copyright (c) 2020-2024 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

from apollo_fpga.support.selftest import ApolloSelfTestCase, named_test
from .registers import *


ALLOWED_HYPERRAM_IDS = (0x0c81, 0x0c86)


class StandaloneTester(ApolloSelfTestCase):

    def __init__(self, dut):
        self.dut = dut


    def assertPhyRegister(self, phy_register_base: int, register: int, expected_value: int):
        """ Asserts that a PHY register contains a given value.

        Parameters:
            phy_register_base -- The base address of the PHY window in the debug SPI
                                 address range.
            register          -- The PHY register to check.
            value             -- The expected value of the relevant PHY register.
        """

        # Set the address of the ULPI register we're going to read from.
        self.dut.registers.register_write(phy_register_base, register)
        self.dut.registers.register_write(phy_register_base, register)

        # ... and read back its value.
        actual_value = self.dut.registers.register_read(phy_register_base + 1)

        # Finally, validate it.
        if actual_value != expected_value:
            raise AssertionError(f"PHY register {register} was {actual_value}, not expected {expected_value}")


    def assertPhyReadBack(self, phy_register_base: int, value: int):
        """ Writes a value to the PHY scratch register and asserts that the read-back matches.

        Parameters:
            phy_register_base -- The base address of the PHY window in the debug SPI
                                 address range.
            value             -- The value written to the scratch register.
        """

        # Set the address of the ULPI register we're going to read from.
        self.dut.registers.register_write(phy_register_base, 0x16)

        # Write the value to it.
        self.dut.registers.register_write(phy_register_base + 1, value)

        # Set the address again to perform the read.
        self.dut.registers.register_write(phy_register_base, 0x16)

        # ... and read back the value.
        actual_value = self.dut.registers.register_read(phy_register_base + 1)

        # Finally, validate it.
        if actual_value != value:
            raise AssertionError(f"PHY scratch register read-back was {actual_value}, not expected {value}")


    def assertPhyPresence(self, register_base: int):
        """ Assertion that fails iff the given PHY isn't detected. """

        # Check the value of our four ID registers, which should
        # read 2404:0900 (vendor: microchip; product: USB3343).
        self.assertPhyRegister(register_base, 0, 0x24)
        self.assertPhyRegister(register_base, 1, 0x04)
        self.assertPhyRegister(register_base, 2, 0x09)
        self.assertPhyRegister(register_base, 3, 0x00)

        # Write some patterns to the scratch register & read them back
        # to exercise all the DATA# lines.
        self.assertPhyReadBack(register_base, 0x00)
        self.assertPhyReadBack(register_base, 0xff)
        for i in range(8):
            self.assertPhyReadBack(register_base, (1 << i))


    def assertHyperRAMRegister(self, address: int, expected_values: int):
        """ Assertion that fails iff a RAM register doesn't hold the expected value. """

        self.dut.registers.register_write(REGISTER_RAM_REG_ADDR, address)
        actual_value =  self.dut.registers.register_read(REGISTER_RAM_VALUE)

        if actual_value not in expected_values:
            raise AssertionError(f"RAM register {address} was {actual_value}, not one of expected {expected_values}")


    @named_test("Debug module")
    def test_debug_connection(self, dut):
        self.assertRegisterValue(1, 0x54455354)


    @named_test("AUX PHY")
    def test_host_phy(self, dut):
        self.assertPhyPresence(REGISTER_AUX_ADDR)


    @named_test("TARGET PHY")
    def test_target_phy(self, dut):
        self.assertPhyPresence(REGISTER_TARGET_ADDR)


    @named_test("CONTROL PHY")
    def test_sideband_phy(self, dut):
        self.assertPhyPresence(REGISTER_CONTROL_ADDR)


    @named_test("HyperRAM")
    def test_hyperram(self, dut):
        self.assertHyperRAMRegister(0, ALLOWED_HYPERRAM_IDS)
