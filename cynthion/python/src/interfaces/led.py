# This file is part of Cynthion
#
# Copyright (c) 2020-2025 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

import time

from ..interface import CynthionInterface


class LED(CynthionInterface):
    """ Simple periheral that allows control of an LED through the Cynthion HAL."""

    def __init__(self, board, led_number):
        """Create a new object representing a Cynthion LED.

        board -- The Cynthion board object that owns the given LED.
        led_number -- The one-indexed LED number. On Cynthion boards, this
                matches the number printed on the silkscreen.
        """

        # Store a reference to the parent board.
        self.board = board

        # Store which of the LEDs we refer to.
        self.led_number = led_number

    # Function that toggles the relevant LED value. """
    def toggle(self):
        self.board.apis.leds.toggle(self.led_number)

    # Function that turns on the relevant LED value. """
    def on(self):
        self.board.apis.leds.on(self.led_number)

    # Function that turns off the relevant LED value. """
    def off(self):
        self.board.apis.leds.off(self.led_number)

    # Function that strobes the relevant LED value on and off. """
    def strobe(self, duration=None):
        self.board.apis.leds.on(self.led_number)
        if duration:
            time.sleep(duration)
        self.board.apis.leds.off(self.led_number)
