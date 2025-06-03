#!/usr/bin/env python3
#
# This file is part of Cynthion.
#
# Copyright (c) 2025 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

""" Speed constants used in USB analyzer. """

from enum import IntEnum, IntFlag
from luna.gateware.usb.usb2 import USBSpeed

class USBAnalyzerSpeed(IntEnum):
    """ Enumeration for analyzer speed settings. """
    HIGH = USBSpeed.HIGH
    FULL = USBSpeed.FULL
    LOW  = USBSpeed.LOW
    AUTO = 0b11
