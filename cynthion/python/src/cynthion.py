#
# This file is part of Cynthion
#

import logging

from .board import CynthionBoard

# Ensure that we have access to all Cynthion boards. Normally, we'd avoid
# importing an entire namespace, but in this case, this allows us to ensure
# that all board modules are loaded for autoidentification.
from .boards import *

active_connections = {}


def Cynthion(**board_identifiers):
    """
            Attempts to create a new instance of Cynthion board (sub)class
            most applicable to the given device. For example, if the attached
            board is a Cynthion, this will automatically create a
            Cynthion object.

            Accepts the same arguments as pyusb's usb.find() method, allowing narrowing
            to a more specific Cynthion by e.g. serial number. Like usb.find(), providing
            find_all will return a list of all found devices.

            Throws a DeviceNotFoundError if no device is avaiable and find_all is not set.
    """

    global active_connections

    # Grab serial number if it's in board_identifiers.
    if 'serial_number' in board_identifiers and board_identifiers['serial_number']:
        serial = board_identifiers['serial_number']
        del board_identifiers['serial_number']
    else:
        # TODO support multiple Cynthion's by serial number
        serial = "TODO"

    # If we already have an active connection with a Cynthion matching the serial number.
    if serial in active_connections:
        return active_connections[serial]

    # Otherwise, try to create a new Cynthion instance.
    if 'find_all' in board_identifiers and board_identifiers['find_all']:
        del board_identifiers['find_all']
        board = CynthionBoard.autodetect_all(**board_identifiers)[0]
    else:
        board = CynthionBoard.autodetect(**board_identifiers)

    if board is not None:
        active_connections[serial] = board

    return board
