#!/usr/bin/env python3
#
# This file is part of Cynthion.
#
# Copyright (c) 2023 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

import logging, os, platform, sys, traceback, unittest

import usb1

from luna import configure_default_logging

from pygreat.comms_backends.usb1 import USB1CommsBackend as backend

import cynthion

VENDOR_ID  = cynthion.shared.usb.bVendorId.cynthion
PRODUCT_ID = cynthion.shared.usb.bProductId.cynthion

EP_MAX_PACKET_SIZE = 512


class TestLibgreatProtocol(unittest.TestCase):
    """Tests for libgreat protocol implementation."""

    def setUp(self):
        configure_default_logging(level=os.getenv("LOG_LEVEL", "INFO").upper())
        self.board = cynthion.Cynthion()

    def test_error_no_function(self):
        api = self.board.apis.firmware
        result = api.supports_verb("test_error_no_function")
        self.assertFalse(result)

        with self.assertRaises(Exception) as context:
            result = api.test_error_no_function()
        self.assertTrue("object has no attribute 'test_error_no_function'" in str(context.exception))

    def test_error_return_code(self):
        from pygreat.errors import LIBGREAT_ERROR_NAMES
        def get_error_code(name):
            return [n for n in LIBGREAT_ERROR_NAMES if LIBGREAT_ERROR_NAMES[n] == name][0]

        api = self.board.apis.selftest

        result = api.supports_verb("test_error_return_code")
        self.assertTrue(result)

        result = api.test_error_return_code(0)
        self.assertEqual(result, "ok")
        logging.debug(f"test_error_return_code: {result}")

        code = get_error_code("EBUSY")
        with self.assertRaises(Exception) as context:
            result = api.test_error_return_code(code)
        logging.debug(f"test_error_return code got: {str(context.exception)}")
        self.assertTrue("EBUSY" in str(context.exception))

        code = get_error_code("ECONNRESET")
        with self.assertRaises(Exception) as context:
            result = api.test_error_return_code(code)
        logging.debug(f"test_error_return code got: {str(context.exception)}")
        self.assertTrue("ECONNRESET" in str(context.exception))


class TestLibgreatEndpoints(unittest.TestCase):
    """Tests to verify that libgreat endpoints behave as expected."""

    def setUp(self):
        configure_default_logging(level=os.getenv("LOG_LEVEL", "INFO").upper())
        self.board = cynthion.Cynthion()

    def test_device(self):
        # query device a little
        with usb1.USBContext() as context:

            device_handle = context.openByVendorIDAndProductID(VENDOR_ID, PRODUCT_ID)
            device = device_handle.getDevice()
            logging.debug(f"device: {device}")
            if platform.system() != "Windows":
                logging.debug(f"  manufacturer: {device.getManufacturer()}")
                logging.debug(f"  product: {device.getProduct()}")

            for configuration in device.iterConfigurations():
                logging.debug(f"configuration: {configuration}")
                for interface in configuration:
                    logging.debug(f"  interface: {interface}")
                    for setting in interface:
                        logging.debug(f"    protocol: {setting.getProtocol()}")
                        for endpoint in setting:
                            logging.debug(f"    endpoint: 0x{endpoint.getAddress():x}")

    def test_api_command(self):
        api = self.board.apis.core

        response = api.read_version_string()
        self.assertEqual(response, "r1.0")

        logging.debug(f"received api response: {len(response)} -> '{response}'")


    def test_large_api_commands(self):
        api = self.board.apis.moondancer

        response = api.test_read_endpoint(711)
        self.assertEqual(len(response), 711)
        logging.debug(f"received api response: {len(response)} -> '{response}'")

        payload = [b % 0xff for b in range(0, 711)]
        response = api.test_write_endpoint(1, bytes(payload))
        self.assertEqual(response, len(payload))
        logging.debug(f"received api response: {response}")


if __name__ == "__main__":
    unittest.main()
