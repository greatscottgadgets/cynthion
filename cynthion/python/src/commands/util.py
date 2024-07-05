#!/usr/bin/env python
#
# This file is part of Cynthion.
#
# Copyright (c) 2023 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

""" Support functions for Cynthion CLI. """

import argparse
import os
import logging
import re
import sys
import textwrap
import usb

from apollo_fpga.commands.cli  import ensure_unconfigured
from cynthion                  import shared
from fwup.dfu                  import DFUTarget
from tqdm                      import tqdm


SOC_FIRMWARE_FLASHADDR = 0x000b0000


def get_bitstream_information():
    d = usb.core.find(
        idVendor=shared.usb.bVendorId.cynthion,
        idProduct=shared.usb.bProductId.cynthion
    )
    if d is None:
        return None

    minor = d.bcdDevice & 0xFF
    major = d.bcdDevice >> 8

    return {
        "product": d.product,
        "manufacturer": d.manufacturer,
        "hardware": f"r{major}.{minor}",
        "flash_uid": d.serial_number,
    }


def find_cynthion_asset(filename):
    """Returns the path to the requested asset filename"""
    module_path = os.path.dirname(__file__)
    asset_path = os.path.join(module_path, '../../assets', filename)
    if os.path.isfile(asset_path):
        return asset_path
    else:
        return None


def find_cynthion_bitstream(filename):
    """Returns the path to the requested bitstream for the appropriate platform"""
    from luna.gateware.platform  import get_appropriate_platform
    platform = get_appropriate_platform()

    module_path = os.path.dirname(__file__)
    bitstream_path = os.path.join(module_path, '../../assets/', type(platform).__name__, filename)
    if os.path.isfile(bitstream_path):
        return bitstream_path
    else:
        return None

    return bitstream_path


def flash_bitstream(device, filename):
    """Flashes the given filename to the FPGA configuration flash"""
    with open(filename, "rb") as f:
        bitstream = f.read()

    logging.info(f"Updating FPGA configuration flash with {len(bitstream)} bytes...")

    ensure_unconfigured(device)
    with device.jtag as jtag:
        programmer = device.create_jtag_programmer(jtag)
        programmer.flash(bitstream, offset=0)
    logging.info("Operation complete!")

    # reset device
    device.soft_reset()

    # let the gateware take over in devices with a shared usb port
    device.allow_fpga_takeover_usb()


def flash_mcu_firmware(device, filename):
    """Flashes the given filename to the device's Microcontroller"""
    with open(filename, 'rb') as f:
        firmware = f.read()

    # create a DFU programmer instance
    board = DFUTarget(idVendor=shared.usb.bVendorId.apollo, idProduct=shared.usb.bProductId.apollo)

    logging.info(f"Updating device firmware with {len(firmware)} bytes...")

    # program firmware binary
    size_to_program = board.size_to_program(firmware)
    with tqdm(total=size_to_program, ncols=80, unit='B', leave=False) as progress:
        board.program(firmware, status_callback=lambda written, _: progress.update(written))

    logging.info("Operation complete!")

    # restart firmware
    board.run_user_program()


def flash_soc_firmware(device, filename):
    """Flashes the given filename to the section of configuration flash reserved for the SoC"""
    with open(filename, "rb") as f:
        firmware = f.read()

    logging.info(f"Updating SoC firmware flash with {len(firmware)} bytes...")

    ensure_unconfigured(device)
    with device.jtag as jtag:
        programmer = device.create_jtag_programmer(jtag)
        programmer.flash(firmware, offset=SOC_FIRMWARE_FLASHADDR)
    logging.info("Operation complete!")


def run_bitstream(device, filename):
    with open(filename, "rb") as f:
        bitstream = f.read()

    logging.info(f"Uploading target bitstream to FPGA with {len(bitstream)} bytes...")

    ensure_unconfigured(device)
    with device.jtag as jtag:
        programmer = device.create_jtag_programmer(jtag)
        programmer.configure(bitstream)
    logging.info("Operation complete!")

    # let the gateware take over in devices with a shared usb port
    device.allow_fpga_takeover_usb()


class HelpFormatter(argparse.HelpFormatter):
    def __init__(self, prog):
        if "COLUMNS" in os.environ:
            columns = int(os.environ["COLUMNS"])
        else:
            try:
                columns, _ = os.get_terminal_size(sys.stderr.fileno())
            except OSError:
                columns = 80
        super().__init__(prog, width=columns, max_help_position=28)

    def _fill_text(self, text, width, indent):
        def filler(match):
            text = match[0]
            if text.startswith("::"):
                return text[2:]

            list_match = re.match(r"(\s*)(\*.+)", text, flags=re.S)
            if list_match:
                text = re.sub(r"(\S)\s+(\S)", r"\1 \2", list_match[2])
                text = textwrap.fill(text, width,
                                     initial_indent=indent + "  ",
                                     subsequent_indent=indent + "    ")
            else:
                text = textwrap.fill(text, width,
                                     initial_indent=indent,
                                     subsequent_indent=indent)

            text = text + (match[2] or "")
            text = re.sub(r"(\w-) (\w)", r"\1\2", text)
            return text

        text = textwrap.dedent(text).strip()
        return re.sub(r"((?!\n\n)(?!\n\s+(?:\*|\$|\d+\.)).)+(\n*)?", filler, text, flags=re.S)