#!/usr/bin/env python3
#
# This file is part of Cynthion.
#
# Copyright (c) 2023 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

import logging, os, sys, traceback, unittest

from luna import configure_default_logging

from facedancer.backends.moondancer import InterruptEvent

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

        # interrupt queue should be empty
        result = api.get_interrupt_events()
        self.assertEqual(len(result), 0)

        # test interrupt queue should have four items
        result = api.test_get_interrupt_events()
        self.assertEqual(len(result), 4)

        # test known values for each item
        result = list(map(InterruptEvent.parse, result))
        self.assertEqual(result[0], InterruptEvent.USB_BUS_RESET)
        self.assertEqual(result[0].interface, 0)
        self.assertEqual(result[0].endpoint_number, 0)
        self.assertEqual(result[1], InterruptEvent.USB_RECEIVE_SETUP_PACKET)
        self.assertEqual(result[1].interface, 1)
        self.assertEqual(result[1].endpoint_number, 1)
        self.assertEqual(result[2], InterruptEvent.USB_RECEIVE_PACKET)
        self.assertEqual(result[2].interface, 2)
        self.assertEqual(result[2].endpoint_number, 2)
        self.assertEqual(result[3], InterruptEvent.USB_SEND_COMPLETE)
        self.assertEqual(result[3].interface, 0)
        self.assertEqual(result[3].endpoint_number, 3)
        for message in result:
            logging.debug(f"get_interrupt_events() -> {message} -> {message.interface} -> {message.endpoint_number}")

        # test read_endpoint
        result = api.test_read_endpoint(63);
        self.assertEqual(len(result), 63)
        logging.debug(f"read_endpoint() -> {len(result)} -> {result}")

        # TODO test long read (currently dies at 63)

if __name__ == "__main__":
    unittest.main()
