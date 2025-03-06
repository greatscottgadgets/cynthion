#!/usr/bin/env python3
#
# This file is part of Cynthion.
#
# Copyright (c) 2023-2025 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

import logging, os, sys, traceback, unittest

from luna           import configure_default_logging
from pygreat.comms  import CommandFailureError

import cynthion
from cynthion.interfaces.gpio import PinDirection, PinMode, InputConfiguration, OutputConfiguration

class TestGpio(unittest.TestCase):
    """Tests to verify that the gpio class is operational."""

    def setUp(self):
        configure_default_logging(level=os.getenv("LOG_LEVEL", "INFO").upper())
        self.board = cynthion.Cynthion()
        self.gpio  = self.board.gpio

    def test_api(self):
        result = self.board.board_name()
        self.assertEqual(result, "Facedancer (Cynthion Project)")

        result = self.board.supports_api("gpio")
        self.assertTrue(result)

        # test presence of low-level api
        self.assertNotEqual(self.board.apis.gpio, None)

        # test presence of interface api
        self.assertNotEqual(self.gpio, None)

    def test_configure_pin(self):
        api = self.board.apis.gpio

        # check that we can configure a pin
        api.configure_pin(0, 0, 1, 0b10)

        # check that we cannot configure a non-existent pin
        with self.assertRaises(Exception) as context:
            result = api.configure_pin(1, 0, True, PinMode.OpenDrain)
        self.assertEqual(CommandFailureError, type(context.exception))
        self.assertEqual('gpio.configure_pin:failed to execute [EINVAL]', str(context.exception))

    def test_configure_pin_interface(self):
        gpio = self.gpio

        # check that we can configure a pin via the Gpio object
        gpio.configure_pin((0, 0), OutputConfiguration(PinMode.OpenDrain), initial_value=True)

        # check that we can't configure a pin with the wrong direction
        with self.assertRaises(Exception) as context:
            result = gpio.configure_pin((0, 0), OutputConfiguration(PinMode.InputOnly), initial_value=True)
        self.assertEqual(ValueError, type(context.exception))
        self.assertEqual('Pin mode 0 is not an output.', str(context.exception))

        # check that we cannot configure a non-existent pin
        with self.assertRaises(Exception) as context:
            gpio.configure_pin((1, 0), OutputConfiguration(PinMode.OpenDrain), initial_value=True)
        self.assertEqual(CommandFailureError, type(context.exception))
        self.assertEqual('gpio.configure_pin:failed to execute [EINVAL]', str(context.exception))

        # check that we can configure a single pin
        a7 = gpio.get_pin("A7")
        a7.set_configuration(OutputConfiguration(PinMode.PushPull))

        # check that we cannot configure a pin with the wrong direction
        a7 = gpio.get_pin("A7")
        with self.assertRaises(Exception) as context:
            a7.set_configuration(InputConfiguration(PinMode.PushPull))
        self.assertEqual(ValueError, type(context.exception))
        self.assertEqual('Pin mode 1 is not an input.', str(context.exception))

    def test_get_pin_directions(self):
        api = self.board.apis.gpio

        # check that we can get pin directions
        pins = [ (0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7) ]
        result = api.get_pin_directions(*pins)
        self.assertEqual(len(result), len(pins))

        # check that we can only get 8 pin directions at a time
        pins += [(0, 0)]
        with self.assertRaises(Exception) as context:
            result = api.get_pin_directions(*pins)
        self.assertEqual(CommandFailureError, type(context.exception))
        self.assertEqual('gpio.get_pin_directions:failed to execute [EINVAL]', str(context.exception))

        # check that we cannot get direction for a non-existent pin
        pins = [ (0, 0), (1, 0) ]
        with self.assertRaises(Exception) as context:
            result = api.get_pin_directions(*pins)
        self.assertEqual(CommandFailureError, type(context.exception))
        self.assertEqual('gpio.get_pin_directions:failed to execute [EINVAL]', str(context.exception))

        # check that we can get a single pin direction
        pins = [ (0, 0) ]
        result = api.get_pin_directions(*pins)
        self.assertEqual(len(result), len(pins))

    def test_get_pin_configurations(self):
        api = self.board.apis.gpio

        # check that we can get pin configurations
        pins = [ (0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7) ]
        result = api.get_pin_configurations(*pins)
        self.assertEqual(len(result), len(pins))

        # check that we can only get 8 pin configurations at a time
        pins += [(0, 0)]
        with self.assertRaises(Exception) as context:
            result = api.get_pin_configurations(*pins)
        self.assertEqual(CommandFailureError, type(context.exception))
        self.assertEqual('gpio.get_pin_configurations:failed to execute [EINVAL]', str(context.exception))

        # check that we cannot get configuration for a non-existent pin
        pins = [ (0, 0), (1, 0) ]
        with self.assertRaises(Exception) as context:
            result = api.get_pin_configurations(*pins)
        self.assertEqual(CommandFailureError, type(context.exception))
        self.assertEqual('gpio.get_pin_configurations:failed to execute [EINVAL]', str(context.exception))

        # check that we can get a single pin configuration
        pins = [ (0, 0) ]
        result = api.get_pin_configurations(*pins)
        self.assertEqual(len(result), len(pins))

    def test_read_pins(self):
        api = self.board.apis.gpio

        # check that we can read pins
        pins = [ (0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7) ]
        result = api.read_pins(*pins)
        self.assertEqual(len(result), len(pins))

        # check that we can only read 8 pins at a time
        pins += [(0, 0)]
        with self.assertRaises(Exception) as context:
            result = api.read_pins(*pins)
        self.assertEqual(CommandFailureError, type(context.exception))
        self.assertEqual('gpio.read_pins:failed to execute [EINVAL]', str(context.exception))

        # check that we cannot read a non-existent pin
        pins = [ (0, 0), (1, 0) ]
        with self.assertRaises(Exception) as context:
            result = api.read_pins(*pins)
        self.assertEqual(CommandFailureError, type(context.exception))
        self.assertEqual('gpio.read_pins:failed to execute [EINVAL]', str(context.exception))

        # check that we can read a single pin configuration
        pins = [ (0, 0) ]
        result = api.read_pins(*pins)
        self.assertEqual(len(result), len(pins))

    def test_write_pins(self):
        api = self.board.apis.gpio

        # check that we can write pins
        pins = [ (0, 0, 1), (0, 1, 1), (0, 2, 1), (0, 3, 1), (0, 4, 1), (0, 5, 1), (0, 6, 1), (0, 7, 1) ]
        api.write_pins(*pins)

        # check that we can write more than 8 pins at a time
        pins += [(0, 0, 0)]
        api.write_pins(*pins)

        # check that we cannot write to a non-existent pin
        pins = [ (0, 0, 0), (1, 0, 0) ]
        with self.assertRaises(Exception) as context:
            api.write_pins(*pins)
        self.assertEqual(CommandFailureError, type(context.exception))
        self.assertEqual('gpio.write_pins:failed to execute [EINVAL]', str(context.exception))

        # check that we can write to a single pin
        pins = [ (0, 0, 0) ]
        api.write_pins(*pins)

    def test_gpio_operations(self):
        api = self.board.apis.gpio

        def test_pin(port, number):
            # Reset pin to a known state before starting tests
            api.configure_pin(port, number, 0, PinMode.PushPull)
            api.configure_pin(port, number, 0, PinMode.InputOnly)

            # Configure pin as InputOnly
            api.configure_pin(port, number, 1, PinMode.InputOnly)
            # check that pin mode and direction are set correctly
            pin_mode = api.get_pin_configurations((port, number))[0]
            pin_direction = api.get_pin_directions((port, number))[0]
            self.assertEqual(pin_mode, PinMode.InputOnly)
            self.assertEqual(pin_direction, PinDirection.Input)
            # make sure no initial value is set for an input pin
            value = api.read_pins((0, number))[0]
            self.assertEqual(value, 0)
            # make sure we cannot set the value for an input pin
            api.write_pins((0, number, 1))
            value = api.read_pins((0, number))[0]
            self.assertEqual(value, 0)

            # Configure pin as PushPull
            api.configure_pin(port, number, 1, PinMode.PushPull)
            # check that pin mode and direction are set correctly
            pin_mode = api.get_pin_configurations((port, number))[0]
            pin_direction = api.get_pin_directions((port, number))[0]
            self.assertEqual(pin_mode, PinMode.PushPull)
            self.assertEqual(pin_direction, PinDirection.Output)
            # make sure the initial value is set for an output pin
            value = api.read_pins((0, number))[0]
            self.assertEqual(value, 1)
            # make sure we can set the value for an output pin
            api.write_pins((0, number, 0))
            value = api.read_pins((0, number))[0]
            self.assertEqual(value, 0)

            # Configure pin as OpenDrain
            api.configure_pin(port, number, 1, PinMode.OpenDrain)
            # check that pin mode and direction are set correctly
            pin_mode = api.get_pin_configurations((port, number))[0]
            pin_direction = api.get_pin_directions((port, number))[0]
            self.assertEqual(pin_mode, PinMode.OpenDrain)
            self.assertEqual(pin_direction, PinDirection.Output)
            # in the absence of a pull-up resistor make sure our pin is low
            value = api.read_pins((0, number))[0]
            self.assertEqual(value, 0)
            # in the absence of a pull-up resistor make sure our pin is low
            api.write_pins((0, number, 1))
            value = api.read_pins((0, number))[0]
            self.assertEqual(value, 0)

            # Configure pin as Alternate
            api.configure_pin(port, number, 1, PinMode.PushPull) # set pin output field to a known state
            api.configure_pin(port, number, 0, PinMode.Alternate)
            # check that pin mode and direction are set correctly
            pin_mode = api.get_pin_configurations((port, number))[0]
            pin_direction = api.get_pin_directions((port, number))[0]
            self.assertEqual(pin_mode, PinMode.Alternate)
            self.assertEqual(pin_direction, PinDirection.Input)
            # make sure output field state is correct
            value = api.read_pins((0, number))[0]
            self.assertEqual(value, 1)
            # make sure output field state is correct
            api.write_pins((0, number, 0))
            value = api.read_pins((0, number))[0]
            self.assertEqual(value, 1)

            # Reset pin to a known state after running tests
            api.configure_pin(port, number, 0, PinMode.PushPull)
            api.configure_pin(port, number, 0, PinMode.InputOnly)

        for pin in range(0, 8):
            test_pin(0, pin)

    def test_user_button(self):
        if os.getenv("NONINTERACTIVE"):
            return

        api = self.board.apis.gpio

        print("Press the USER button to continue")
        while True:
            value = api.read_pins((2, 0))[0]
            if value != 0:
                self.assertEqual(value, 1)
                break

if __name__ == "__main__":
    unittest.main()
