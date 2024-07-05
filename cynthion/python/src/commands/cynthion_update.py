#!/usr/bin/env python
#
# This file is part of Cynthion.
#
# Copyright (c) 2023 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

""" Cynthion 'update' command. """

from .util import find_cynthion_asset, find_cynthion_bitstream
from .util import flash_bitstream, flash_soc_firmware, flash_mcu_firmware


def cynthion_update(device, args):
    if args.bitstream:
        flash_bitstream(device, find_cynthion_bitstream(device, "analyzer.bit"))
    elif args.mcu_firmware:
        flash_mcu_firmware(device, find_cynthion_asset("apollo.bin"))
    else:
        flash_bitstream(device, find_cynthion_bitstream(device, "analyzer.bit"))
        flash_mcu_firmware(device, find_cynthion_asset("apollo.bin"))
