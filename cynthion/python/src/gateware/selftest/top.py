#!/usr/bin/env python3
#
# This file is part of Cynthion
#
# Copyright (c) 2023 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

""" Utility to run a hardware self-test on a connected Cynthion. """

from cynthion.selftest.gateware import SelftestDevice
from cynthion.selftest.host import StandaloneTester
from luna import top_level_cli


def main():
    device = top_level_cli(SelftestDevice)

if __name__ == '__main__':
    main()
