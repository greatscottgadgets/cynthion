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

ENDPOINT_BULK_OUT = 0x01
ENDPOINT_BULK_IN  = 0x81

VENDOR_REQUEST = 0x65
VENDOR_VALUE_CONTROL_OUT = 0x0001
VENDOR_VALUE_CONTROL_IN = 0x0002
VENDOR_VALUE_BULK_OUT = 0x0003
VENDOR_VALUE_BULK_IN = 0x0004

PAYLOAD_LENGTH = 70

# usb_hal.rs MAX_TRANSFER_SIZE is 2048
MAX_TRANSFER_SIZE = 2048


class TestTransfers(unittest.TestCase):
    """Tests for usb transfers."""

    def setUp(self):
        configure_default_logging(level=os.getenv("LOG_LEVEL", "DEBUG").upper())

        #import random
        #self.payload_length = random.randint(1, MAX_TRANSFER_SIZE - 1)
        self.payload_length = PAYLOAD_LENGTH
        self.test_data = bytes([b % 0xff for b in range(0, self.payload_length)])

    # TODO unreliable at High
    def test_control_transfer_out(self):
        logging.info("test_control_transfer_out")

        with usb1.USBContext() as context:
            device_handle = context.openByVendorIDAndProductID(VENDOR_ID, PRODUCT_ID)
            if device_handle is None:
                raise DeviceNotFoundError()
            device_handle.claimInterface(0)

            payload = self.test_data
            response = device_handle.controlWrite(
                request_type=usb1.TYPE_VENDOR | usb1.RECIPIENT_ENDPOINT,
                request=VENDOR_REQUEST,
                value=VENDOR_VALUE_CONTROL_OUT,
                index=len(payload),
                data=payload,
                timeout=1000
            )
            logging.info(f"Control write transfer sent {response} bytes.")


    # TODO unreliable at High
    def off_test_control_transfer_in(self):
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
                index=self.payload_length,
                length=self.payload_length,
                timeout=1000
            )
            response = bytes(response)
            logging.info(f"Control read transfer received {len(response)} bytes.")


    # TODO unreliable at High
    def off_test_bulk_transfer_out(self):
        logging.info("test_bulk_transfer_out")

        with usb1.USBContext() as context:
            device_handle = context.openByVendorIDAndProductID(VENDOR_ID, PRODUCT_ID)
            if device_handle is None:
                raise DeviceNotFoundError()
            device_handle.claimInterface(0)

            payload = self.test_data
            response = device_handle.controlWrite(
                request_type=usb1.TYPE_VENDOR | usb1.RECIPIENT_ENDPOINT,
                request=VENDOR_REQUEST,
                value=VENDOR_VALUE_BULK_OUT,
                index=len(payload),
                data=[],
                timeout=1000
            )

            response = device_handle.bulkWrite(
                endpoint=ENDPOINT_BULK_OUT,
                data=payload,
                timeout=1000
            )
            logging.info(f"bulk write transfer sent {response} bytes.")


    # TODO unreliable at High
    def off_test_bulk_transfer_in(self):
        logging.info("test_bulk_transfer_in")

        with usb1.USBContext() as context:
            device_handle = context.openByVendorIDAndProductID(VENDOR_ID, PRODUCT_ID)
            if device_handle is None:
                raise DeviceNotFoundError()
            device_handle.claimInterface(0)

            response = device_handle.controlWrite(
                request_type=usb1.TYPE_VENDOR | usb1.RECIPIENT_ENDPOINT,
                request=VENDOR_REQUEST,
                value=VENDOR_VALUE_BULK_IN,
                index=self.payload_length,
                data=[],
                timeout=1000
            )

            response = device_handle.bulkRead(
                endpoint=ENDPOINT_BULK_IN & 0x7f,
                length=self.payload_length,
                timeout=1000
            )
            response = bytes(response)
            logging.info(f"bulk read transfer received {len(response)} bytes.")


if __name__ == "__main__":
    unittest.main()
