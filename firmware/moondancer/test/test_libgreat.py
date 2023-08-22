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

VENDOR_ID  = cynthion.board.CYNTHION_VENDOR_ID
PRODUCT_ID = cynthion.board.CYNTHION_PRODUCT_ID

EP_MAX_PACKET_SIZE = 512


class TestLibgreatProtocol(unittest.TestCase):
    """Tests for libgreat protocol implementation."""

    def setUp(self):
        configure_default_logging(level=os.getenv("LOG_LEVEL", "DEBUG").upper())
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
        self.assertTrue("EBUSY" in str(context.exception))

        code = get_error_code("ECONNRESET")
        with self.assertRaises(Exception) as context:
            result = api.test_error_return_code(code)
        self.assertTrue("ECONNRESET" in str(context.exception))


class TestLibgreatEndpoints(unittest.TestCase):
    """Tests to verify that libgreat endpoints behave as expected."""

    def setUp(self):
        configure_default_logging(level=os.getenv("LOG_LEVEL", "DEBUG").upper())
        self.board = cynthion.Cynthion()

    def test_device(self):
        # query device a little
        with usb1.USBContext() as context:

            device_handle = context.openByVendorIDAndProductID(VENDOR_ID, PRODUCT_ID)
            device = device_handle.getDevice()
            print(f"device: {device}")
            print(f"  manufacturer: {device.getManufacturer()}")
            print(f"  product: {device.getProduct()}")

            for configuration in device.iterConfigurations():
                print(f"configuration: {configuration}")
                for interface in configuration:
                    print(f"  interface: {interface}")
                    for setting in interface:
                        print(f"    protocol: {setting.getProtocol()}")
                        for endpoint in setting:
                            print(f"    endpoint: 0x{endpoint.getAddress():x}")

    def test_api_command(self):
        api = self.board.apis.core

        response = api.read_version_string()
        self.assertEqual(response, "v2023.0.1")

        logging.debug(f"received api response: {len(response)} -> '{response}'")


    def test_large_api_commands(self):
        api = self.board.apis.moondancer

        # TODO caps out at 511
        response = api.test_read_endpoint(511)
        self.assertEqual(len(response), 511)
        logging.debug(f"received api response: {len(response)} -> '{response}'")

        # TODO maxes out at 503 because we're encoding function arguments too
        payload = [b % 0xff for b in range(0, 503)]
        response = api.test_write_endpoint(1, bytes(payload))
        self.assertEqual(response, len(payload))
        logging.debug(f"received api response: {response}")


if __name__ == "__main__":
    unittest.main()
