#!/usr/bin/env python
#
# This file is part of Cynthion.
#
# Copyright (c) 2026 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

""" Cynthion 'reset' command. """

import logging, time

import usb.core

from cynthion import shared


def cynthion_reset(device, args):
    logging.info("Resetting device...")

    # reset device
    device.soft_reset()

    # let the gateware take over in devices with a shared usb port
    device.allow_fpga_takeover_usb()

    # wait for the gateware to enumerate
    if _wait_for_gateware():
        logging.info("Operation complete!")
    else:
        logging.warning("Reset requested, but the Cynthion gateware did not enumerate.")
        logging.warning("The FPGA configuration flash may not contain a valid bitstream.")


def _wait_for_gateware(timeout=5.0):
    """Polls the USB bus until the Cynthion gateware enumerates or the timeout expires"""
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        if usb.core.find(idVendor=shared.usb.bVendorId.cynthion,
                         idProduct=shared.usb.bProductId.cynthion) is not None:
            return True
        time.sleep(0.25)
    return False
