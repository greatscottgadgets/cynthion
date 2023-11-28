#!/usr/bin/env python3
#
# This file is part of Cynthion.
#
# Copyright (c) 2023 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

import logging, os, sys, traceback, unittest

import usb1

from luna import configure_default_logging

from pygreat.comms_backends.usb1 import USB1CommsBackend as backend
from pygreat.errors import DeviceNotFoundError

import cynthion

VENDOR_ID  = cynthion.shared.usb.bVendorId.example
PRODUCT_ID = cynthion.shared.usb.bProductId.example

EP_MAX_PACKET_SIZE = 512

BULK_ENDPOINT_OUT = 0x01
BULK_ENDPOINT_IN  = 0x81

VENDOR_REQUEST = 0x65
VENDOR_VALUE_CONTROL_OUT = 0x0001
VENDOR_VALUE_CONTROL_IN = 0x0002
VENDOR_VALUE_BULK_OUT = 0x0003
VENDOR_VALUE_BULK_IN = 0x0004

# usb_hal.rs MAX_TRANSFER_SIZE is 2048
PAYLOAD_LENGTH = 400


class TestTransfers(unittest.TestCase):
    """Tests for usb transfers."""

    def setUp(self):
        configure_default_logging(level=os.getenv("LOG_LEVEL", "DEBUG").upper())
        logging.info("OH HAI!")


    def off_test_control_transfer_out(self):
        logging.info("test_control_transfer_out")

        with usb1.USBContext() as context:
            device_handle = context.openByVendorIDAndProductID(VENDOR_ID, PRODUCT_ID)
            if device_handle is None:
                raise DeviceNotFoundError()
            device_handle.claimInterface(0)

            payload = bytes([b % 0xff for b in range(PAYLOAD_LENGTH)])

            response = device_handle.controlWrite(
                request_type=usb1.TYPE_VENDOR | usb1.RECIPIENT_ENDPOINT,
                request=VENDOR_REQUEST,
                value=VENDOR_VALUE_CONTROL_OUT,
                index=len(payload),
                data=payload,
                timeout=1000
            )
            logging.info(f"control write transfer sent {response} bytes.")


    def test_control_transfer_in(self):
        logging.info("test_control_transfer_in")

        with usb1.USBContext() as context:
            device_handle = context.openByVendorIDAndProductID(VENDOR_ID, PRODUCT_ID)
            if device_handle is None:
                raise DeviceNotFoundError()
            device_handle.claimInterface(0)

            response = device_handle.controlRead(
                request_type=usb1.TYPE_VENDOR | usb1.RECIPIENT_ENDPOINT,
                request=VENDOR_REQUEST,
                value=VENDOR_VALUE_CONTROL_IN,
                index=PAYLOAD_LENGTH,
                length=PAYLOAD_LENGTH,
                timeout=1000
            )
            response = bytes(response)
            logging.info(f"control read transfer received {len(response)} bytes: {response}")


    def off_test_bulk_transfer(self):
        with usb1.USBContext() as context:
            device_handle = context.openByVendorIDAndProductID(VENDOR_ID, PRODUCT_ID)
            if device_handle is None:
                raise DeviceNotFoundError()

            device_handle.claimInterface(0)

            payload = bytes([b % 0xff for b in range(0, 1024)])
            response = device_handle.bulkWrite(backend.LIBGREAT_BULK_OUT_ENDPOINT_NUMBER, payload)
            logging.info(f"bulk write transfer received response: {response}")


if __name__ == "__main__":
    unittest.main()
