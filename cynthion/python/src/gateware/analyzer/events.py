#
# This file is part of Cynthion.
#
# Copyright (c) 2024 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

from enum import IntEnum


class USBAnalyzerEvent(IntEnum):
    NONE                = 0

    CAPTURE_STOP_NORMAL = 1
    CAPTURE_STOP_FULL   = 2
    CAPTURE_STOP_ERROR  = 3

    CAPTURE_START_BASE  = 4

    CAPTURE_START_HIGH  = 4
    CAPTURE_START_FULL  = 5
    CAPTURE_START_LOW   = 6
    CAPTURE_START_AUTO  = 7

    SPEED_DETECT_BASE   = 8

    SPEED_DETECT_HIGH   = 8
    SPEED_DETECT_FULL   = 9
    SPEED_DETECT_LOW    = 10
    SPEED_DETECT_AUTO   = 11

    VBUS_DISCONNECTED   = 12
    VBUS_CONNECTED      = 13

    SUSPEND_ENDED       = 14
    SUSPEND_STARTED     = 15

    BUS_RESET           = 16
    DEVICE_CHIRP_SEEN   = 17
    HOST_CHIRP_SEEN     = 18
