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

    LINESTATE_BASE      = 12

    LINESTATE_SE0       = 12
    LINESTATE_CHIRP_J   = 13
    LINESTATE_CHIRP_K   = 14
    LINESTATE_CHIRP_SE1 = 15
    LINESTATE_LS_J      = 16
    LINESTATE_LS_K      = 17
    LINESTATE_FS_J      = 18
    LINESTATE_FS_K      = 19
    LINESTATE_SE1       = 20

    VBUS_INVALID        = 21
    VBUS_VALID          = 22
    LS_ATTACH           = 23
    FS_ATTACH           = 24
    BUS_RESET           = 25
    DEVICE_CHIRP_VALID  = 26
    HOST_CHIRP_VALID    = 27
    SUSPEND             = 28
    RESUME              = 29
    LS_KEEPALIVE        = 30
