#!/usr/bin/env python
#
# This file is part of Cynthion.
#
# Copyright (c) 2023 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

""" Cynthion 'info' command. """

import logging

from apollo_fpga.commands.cli  import COMMANDS as APOLLO_COMMANDS


def cynthion_info(device, args, info=None):
    if device:
        # just wrap the Apollo implementation for now
        command = next((c for c in APOLLO_COMMANDS if c.name == "info"), None)
        command.handler(device, args)
    else:
        logging.info("Detected a Cynthion device!")

    # print some information about the bitstream
    if info:
        logging.info(f"\tBitstream: {info['product']} ({info['manufacturer']})")
        if device is None:
            logging.info(f"\tHardware: Cynthion {info['hardware']}")
        logging.info(f"\tFlash UID: {info['flash_uid']}")

    # tell user how to get all display info if we weren't in Apollo mode
    if device is None:
        logging.info("\nApollo stub interface found. Press the PROGRAM button to switch the device")
        logging.info("to Apollo mode or use 'cynthion info --force-offline' to show all device info.")
