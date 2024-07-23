#!/usr/bin/env python
#
# This file is part of Cynthion.
#
# Copyright (c) 2023-2024 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

""" Support functions for Cynthion CLI. """

import argparse
import os
import logging
import platform
import re
import sys
import textwrap
import usb

from apollo_fpga.commands.cli  import ensure_unconfigured
from cynthion                  import shared
from fwup.dfu                  import DFUTarget
from tqdm                      import tqdm


SOC_FIRMWARE_FLASHADDR = 0x000b0000


def _find_assets_path():
    try:
        # <= 3.8
        from importlib_resources import files
    except:
        # >= 3.9
        from importlib.resources import files

    pkg_path = files("cynthion")
    if os.path.basename(pkg_path) == "src":
        assets = os.path.join(pkg_path, "../assets")
    else:
        assets = os.path.join(pkg_path, "assets")

    return os.path.normpath(os.path.join(pkg_path, assets))


def find_cynthion_asset(filename):
    """Returns the path to the requested asset filename"""

    asset_path = os.path.join(_find_assets_path(), filename)
    if os.path.isfile(asset_path):
        return asset_path
    else:
        return None


def find_cynthion_bitstream(device, filename):
    """Returns the path to the requested bitstream for the appropriate platform"""

    platform = _get_appropriate_platform_name(device)

    bitstream_path = os.path.join(_find_assets_path(), platform, filename)
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

    # Release Apollo debugger
    device.close()

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

    with device.jtag as jtag:
        programmer = device.create_jtag_programmer(jtag)
        programmer.configure(bitstream)
    logging.info("Operation complete!")

    # let the gateware take over in devices with a shared usb port
    device.allow_fpga_takeover_usb()


def _get_appropriate_platform_name(device):
    """Returns the LUNA platform name for the connected Cynthion"""
    # Retrieve the version of the attached device.
    major, minor = device.detect_connected_version()

    return f"CynthionPlatformRev{major}D{minor}"


class HelpFormatter(argparse.HelpFormatter):
    def __init__(self, prog):
        if "COLUMNS" in os.environ:
            columns = int(os.environ["COLUMNS"])
        else:
            try:
                columns, _ = os.get_terminal_size(sys.stderr.fileno())
            except OSError:
                columns = 80
        super().__init__(prog, width=columns, max_help_position=30)

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
