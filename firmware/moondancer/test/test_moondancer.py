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
    """Tests for libgreat class: Moondancer"""

    def setUp(self):
        configure_default_logging(level=os.getenv("LOG_LEVEL", "INFO").upper())

        self.board = cynthion.Cynthion()

    def test_connectivity(self):
        response = self.board.board_name()
        logging.debug(f"test_connectivity: {response}")
        self.assertEqual(response, "Facedancer (Cynthion Project)")

    def test_class_moondancer(self):
        response = self.board.supports_api("moondancer")
        self.assertTrue(response)

    def test_get_interrupt_events(self):
        api = self.board.apis.moondancer

        # interrupt queue should be empty
        response = api.get_interrupt_events()
        self.assertEqual(len(response), 0)

        # test interrupt queue should have four items
        response = api.test_get_interrupt_events()
        self.assertEqual(len(response), 4)

        # test known values for each item
        response = list(map(InterruptEvent, response))
        self.assertEqual(response[0], InterruptEvent.USB_BUS_RESET)
        self.assertEqual(response[0].endpoint_number, 0)
        self.assertEqual(response[1], InterruptEvent.USB_RECEIVE_CONTROL)
        self.assertEqual(response[1].endpoint_number, 1)
        self.assertEqual(response[2], InterruptEvent.USB_RECEIVE_PACKET)
        self.assertEqual(response[2].endpoint_number, 2)
        self.assertEqual(response[3], InterruptEvent.USB_SEND_COMPLETE)
        self.assertEqual(response[3].endpoint_number, 3)
        for message in response:
            logging.debug(f"test_get_interrupt_events() -> {message} -> {message.endpoint_number}")


    def test_read_endpoint(self):
        api = self.board.apis.moondancer

        payload_lengths = [ 63, 64, 65, 127, 128, 129, 511, 512, 513, 767, 768, 769 ]

        for payload_length in payload_lengths:
            logging.debug(f"test_read_endpoint: read {payload_length} bytes")
            response = api.test_read_endpoint(payload_length);
            self.assertEqual(len(response), payload_length)
            logging.debug(f"test_read_endpoint() -> {len(response)}") # -> {response}")


    def test_write_endpoint(self):
        api = self.board.apis.moondancer

        payload_lengths = [ 63, 64, 65, 127, 128, 129, 511, 512, 513, 767, 768, 769 ]

        for payload_length in payload_lengths:
            logging.debug(f"test_write_endpoint: write {payload_length} bytes")
            payload = [b % 0xff for b in range(0, payload_length)]
            response = api.test_write_endpoint(1, bytes(payload))
            self.assertEqual(response, len(payload))
            logging.debug(f"test_write_endpoint() -> {len(payload)} -> {response}")



if __name__ == "__main__":
    unittest.main()
