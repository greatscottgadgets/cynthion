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

from importlib.resources       import files

from cynthion                  import shared
from fwup.dfu                  import DFUTarget
from tqdm                      import tqdm


SOC_FIRMWARE_FLASHADDR = 0x000b0000

_MSG_EDITABLE_INSTALL = """
If you have installed the 'cynthion' Python package from source please
run the following command in the package directory:

    make assets
"""
_MSG_UNSUPPORTED_FIRMWARE = """
There is no prebuilt Apollo firmware available for Cynthion r{}.{}.

Please see the developer documentation for instructions on how to build
and flash your own:

    https://cynthion.readthedocs.io/en/latest/hardware/bringup_guide.html
"""

_MSG_SOURCE_INSTALL = """
If you have installed the 'cynthion' Python package from source please
run the following commands in the package directory:

    make assets
    pip install --upgrade .
"""

def _find_assets_path():
    package_path = files(__package__.split('.')[0])
    if _is_editable_install():
        assets = os.path.join(package_path, "../assets")
    else:
        assets = os.path.join(package_path, "assets")

    return os.path.normpath(os.path.join(package_path, assets))


def _is_editable_install():
    package_name = __package__.split('.')[0]
    package_path = files(package_name)

    # were we installed with `pip -e .` ?
    if os.path.basename(package_path) == "src":
        return True


def _is_source_install():
    package_name = __package__.split('.')[0]
    package_path = files(package_name)

    # get .dist-info path
    dist_info_path = None
    site_packages_path = os.path.dirname(package_path)
    for item in os.listdir(site_packages_path):
        if item.endswith(".dist-info") and item.startswith(package_name.replace('-', '_')):
            dist_info_path = os.path.join(site_packages_path, item)
            break

    # pypi installs will always have a .dist-info/ directory
    if not dist_info_path:
        return True

    # .dist-info/direct_url.json only exists on non-pypi installs (PEP 610)
    return os.path.isfile(os.path.join(dist_info_path, "direct_url.json"))


def find_cynthion_asset(filename):
    """Returns the path to the requested asset filename"""

    asset_path = os.path.join(_find_assets_path(), filename)
    if os.path.isfile(asset_path):
        return asset_path
    else:
        logging.error(f"The Cynthion '{filename}' asset could not be located.")
        if _is_editable_install():
            logging.error(_MSG_EDITABLE_INSTALL)
        elif _is_source_install():
            logging.error(_MSG_SOURCE_INSTALL)
        sys.exit(1)


def find_cynthion_bitstream(device, filename):
    """Returns the path to the requested bitstream for the appropriate platform"""

    platform = _get_appropriate_platform_name(device)

    bitstream_path = os.path.join(_find_assets_path(), platform, filename)
    if os.path.isfile(bitstream_path):
        return bitstream_path
    else:
        logging.error(f"The Cynthion '{filename}' bitstream could not be located.")
        if _is_editable_install():
            logging.error(_MSG_EDITABLE_INSTALL)
        elif _is_source_install():
            logging.error(_MSG_SOURCE_INSTALL)
        sys.exit(1)


def flash_bitstream(device, filename):
    """Flashes the given filename to the FPGA configuration flash"""
    with open(filename, "rb") as f:
        bitstream = f.read()

    logging.info(f"Updating FPGA configuration flash with {len(bitstream)} bytes...")

    device.force_fpga_offline()
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

    # Check for unsupported hardware
    major, minor = device.detect_connected_version()
    if major == 0 and minor < 6:
        logging.error(_MSG_UNSUPPORTED_FIRMWARE.format(major, minor))
        sys.exit(1)

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

    device.force_fpga_offline()
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
