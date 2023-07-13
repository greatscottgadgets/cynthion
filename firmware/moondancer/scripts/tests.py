#!/usr/bin/env python3
#
# This file is part of Cynthion.
#
# Copyright (c) 2023 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

import logging, os, sys, traceback, unittest

from luna import configure_default_logging

import cynthion


class TestGcpProtocol(unittest.TestCase):
    """Tests for GCP protocol implementation."""

    def setUp(self):
        configure_default_logging(level=os.getenv("LOG_LEVEL", "INFO").upper())
        self.board = cynthion.Cynthion()

    def test_connectivity(self):
        result = self.board.board_name()
        logging.debug(f"test_connectivity: {result}")
        self.assertEqual(result, "Cynthion in Moondancer mode")

    def test_class_firmware(self):
        result = self.board.supports_api("firmware")
        self.assertTrue(result)

        api = self.board.apis.firmware
        result = api.initialize()
        logging.debug(f"test_class_firmware: {result}")

        self.assertEqual(result, (256, 2097152))

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


if __name__ == "__main__":
    unittest.main()
