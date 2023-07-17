#!/usr/bin/env python3
#
# This file is part of Cynthion.
#
# Copyright (c) 2023 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

import logging, os, sys, traceback, unittest

from luna import configure_default_logging

import cynthion


class TestMoondancer(unittest.TestCase):
    """Tests for GCP class: Moondancer"""

    def setUp(self):
        configure_default_logging(level=os.getenv("LOG_LEVEL", "DEBUG").upper())

        self.board = cynthion.Cynthion()

    def test_connectivity(self):
        result = self.board.board_name()
        logging.debug(f"test_connectivity: {result}")
        self.assertEqual(result, "Cynthion in Moondancer mode")

    def test_class_moondancer(self):
        result = self.board.supports_api("moondancer")
        self.assertTrue(result)

    def test_get_event(self):
        api = self.board.apis.moondancer

        result = api.get_event()
        logging.debug(f"get_event() -> {result}")

        result = api.get_events()
        logging.debug(f"get_events() -> {result}")


if __name__ == "__main__":
    unittest.main()
