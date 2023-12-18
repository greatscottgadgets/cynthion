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

import cynthion

VENDOR_ID  = cynthion.shared.usb.bVendorId.cynthion
PRODUCT_ID = cynthion.shared.usb.bProductId.cynthion

EP_MAX_PACKET_SIZE = 512

class TestLibgreatProtocolRaw(unittest.TestCase):
    """Tests for libgreat protocol implementation."""

    def setUp(self):
        configure_default_logging(level=os.getenv("LOG_LEVEL", "DEBUG").upper())


    def test_raw_command(self):
        with usb1.USBContext() as context:
            device_handle = context.openByVendorIDAndProductID(VENDOR_ID, PRODUCT_ID)
            if device_handle is None:
                raise DeviceNotFoundError()
            device_handle.claimInterface(0)

            payload = [
                0x00, 0x00, 0x00, 0x00, # class_id:    0x0000 - core
                0x01, 0x00, 0x00, 0x00, # verb_number: 0x0001 - read_version_string
            ]
            response = device_handle.controlWrite(
                request_type=usb1.TYPE_VENDOR | usb1.RECIPIENT_ENDPOINT,
                request=backend.LIBGREAT_REQUEST_NUMBER,
                value=backend.LIBGREAT_VALUE_EXECUTE,
                index=0,
                data=payload,
                timeout=1000
            )
            logging.info(f"test_raw_command control write transfer received response bytes:{response}")

            response = device_handle.controlRead(
                request_type=usb1.TYPE_VENDOR | usb1.RECIPIENT_ENDPOINT,
                request=backend.LIBGREAT_REQUEST_NUMBER,
                value=backend.LIBGREAT_VALUE_EXECUTE,
                index=0,
                length=backend.LIBGREAT_MAX_COMMAND_SIZE,
                timeout=1000
            )
            logging.info(f"test_raw_command control read transfer received response bytes:{len(response)} {response}")
            self.assertEqual(response, b"v2023.0.1\0")


    def test_raw_command_large_output(self):
        with usb1.USBContext() as context:
            device_handle = context.openByVendorIDAndProductID(VENDOR_ID, PRODUCT_ID)
            if device_handle is None:
                raise DeviceNotFoundError()
            device_handle.claimInterface(0)

            payload_length = 718
            data = [b % 0xff for b in range(0, payload_length)]
            payload = [
                0x20, 0x01, 0x00, 0x00, # class_id:        0x0120 - moondancer
                0x2a, 0x00, 0x00, 0x00, # verb_number:     0x002a - test_write_endpoint
                0x01,                   # endpoint_number: 0x01
            ] + data                    # payload
            response = device_handle.controlWrite(
                request_type=usb1.TYPE_VENDOR | usb1.RECIPIENT_ENDPOINT,
                request=backend.LIBGREAT_REQUEST_NUMBER,
                value=backend.LIBGREAT_VALUE_EXECUTE,
                index=0,
                data=payload,
                timeout=1000
            )
            logging.info(f"test_raw_command_large_output control write transfer received response bytes:{response}")

            response = device_handle.controlRead(
                request_type=usb1.TYPE_VENDOR | usb1.RECIPIENT_ENDPOINT,
                request=backend.LIBGREAT_REQUEST_NUMBER,
                value=backend.LIBGREAT_VALUE_EXECUTE,
                index=0,
                length=backend.LIBGREAT_MAX_COMMAND_SIZE,
                timeout=1000
            )
            response = int.from_bytes(response, byteorder="little", signed=False)
            logging.info(f"test_raw_command_large_output control read transfer received response: {response}")
            self.assertEqual(response, payload_length)


    def test_raw_command_large_input(self):
        with usb1.USBContext() as context:
            device_handle = context.openByVendorIDAndProductID(VENDOR_ID, PRODUCT_ID)
            if device_handle is None:
                raise DeviceNotFoundError()
            device_handle.claimInterface(0)

            payload_length = 718
            payload = [
                0x20, 0x01, 0x00, 0x00, # class_id:       0x0120 - moondancer
                0x28, 0x00, 0x00, 0x00, # verb_number:    0x0028 - test_read_endpoint
            ] + list(int(payload_length).to_bytes(4, byteorder="little", signed=False)) # payload_length
            response = device_handle.controlWrite(
                request_type=usb1.TYPE_VENDOR | usb1.RECIPIENT_ENDPOINT,
                request=backend.LIBGREAT_REQUEST_NUMBER,
                value=backend.LIBGREAT_VALUE_EXECUTE,
                index=0,
                data=payload,
                timeout=1000
            )
            logging.info(f"test_raw_command_large_input control write transfer received response bytes:{response}")

            response = device_handle.controlRead(
                request_type=usb1.TYPE_VENDOR | usb1.RECIPIENT_ENDPOINT,
                request=backend.LIBGREAT_REQUEST_NUMBER,
                value=backend.LIBGREAT_VALUE_EXECUTE,
                index=0,
                length=backend.LIBGREAT_MAX_COMMAND_SIZE,
                timeout=1000
            )
            logging.info(f"test_raw_command_large_input control read transfer received response bytes:{len(response)} {response}")
            self.assertEqual(len(response), payload_length)


if __name__ == "__main__":
    unittest.main()
