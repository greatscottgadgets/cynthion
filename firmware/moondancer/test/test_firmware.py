#!/usr/bin/env python3
#
# This file is part of Cynthion.
#
# Copyright (c) 2023 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

import logging, os, sys, traceback, unittest

from luna import configure_default_logging

import cynthion


class TestFirmware(unittest.TestCase):
    """Tests to verify that the board firmware is operational."""

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



if __name__ == "__main__":
    unittest.main()
