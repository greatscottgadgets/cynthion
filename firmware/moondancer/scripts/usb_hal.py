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


VENDOR_ID  = 0x1209 # https://pid.codes/1209/
PRODUCT_ID = 0x0001 # pid.codes Test PID

#import cynthion
#VENDOR_ID  = cynthion.shared.usb.bVendorId.cynthion
#PRODUCT_ID = cynthion.shared.usb.bProductId.cynthion

EP_MAX_PACKET_SIZE = 512

class TestTransfers(unittest.TestCase):
    """Tests for usb transfers."""

    def setUp(self):
        configure_default_logging(level=os.getenv("LOG_LEVEL", "DEBUG").upper())
        logging.info("OH HAI!")

    def test_control_transfer(self):
        logging.info("test_control_transfer")

        with usb1.USBContext() as context:
            device_handle = context.openByVendorIDAndProductID(VENDOR_ID, PRODUCT_ID)
            if device_handle is None:
                raise DeviceNotFoundError()

            device_handle.claimInterface(0)

            payload_length = 128
            payload = bytes([b % 0xff for b in range(payload_length)])
            flags = 0

            response = device_handle.controlWrite(
                request_type=usb1.TYPE_VENDOR | usb1.RECIPIENT_ENDPOINT,
                request=backend.LIBGREAT_REQUEST_NUMBER,
                value=backend.LIBGREAT_VALUE_EXECUTE,
                index=flags,
                data=payload,
                timeout=1000
            )
            logging.info(f"control write transfer sent {response} bytes.")

            # response = device_handle.controlRead(
            #     request_type=usb1.TYPE_VENDOR | usb1.RECIPIENT_ENDPOINT,
            #     request=backend.LIBGREAT_REQUEST_NUMBER,
            #     value=backend.LIBGREAT_VALUE_EXECUTE,
            #     index=flags,
            #     length=payload_length,
            #     timeout=1000
            # )
            # response = bytes(response)
            #
            # logging.info(f"control read transfer received {len(response)} bytes: {response}")


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
