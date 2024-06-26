#!/usr/bin/env python3
#
# This file is part of Cynthion
#
# Copyright (c) 2024 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

""" Utility to update the MCU firmware (Apollo). """

import os
import logging

from fwup.dfu           import DFUTarget
from cynthion.shared    import usb
from tqdm               import tqdm

# Default device identifiers.
APOLLO_VENDOR_ID  = usb.bVendorId.apollo
APOLLO_PRODUCT_ID = usb.bProductId.apollo

def find_cynthion_asset(filename):
    module_path = os.path.dirname(__file__)
    asset_path = os.path.join(module_path, '../../assets', filename)
    if os.path.isfile(asset_path):
        return asset_path
    else:
        return None

def main(custom_file=None):

    if custom_file is None:
        # Grab Cynthion firmware binary that should have shipped with the tool.
        firmware_path = find_cynthion_asset("apollo.bin")
        if firmware_path is None:
            logging.error("Firmware asset not found.")
            exit(1)
    else:
        if os.path.isfile(custom_file):
            firmware_path = custom_file
        else:
            logging.error("Provided file does not exist.")
            exit(1)

    # Read firmware binary.
    with open(firmware_path, 'rb') as f:
        program_data = f.read()

    # Create a DFU programmer instance.
    board = DFUTarget(idVendor=APOLLO_VENDOR_ID, idProduct=APOLLO_PRODUCT_ID)

    # Program firmware binary.
    size_to_program = board.size_to_program(program_data)
    logging.info(f"Programming {len(program_data)} bytes...")
    with tqdm(total=size_to_program, ncols=80, unit='B', leave=False) as progress:
        board.program(program_data, status_callback=lambda written, _: progress.update(written))
    logging.info("Programming complete!")
    board.run_user_program()


if __name__ == "__main__":
    main()
