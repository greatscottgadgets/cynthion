#
# This file is part of Cynthion
#

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

    if 'find_all' in board_identifiers and board_identifiers['find_all']:
        del board_identifiers['find_all']
        return CynthionBoard.autodetect_all(**board_identifiers)
    else:
        return CynthionBoard.autodetect(**board_identifiers)


def CynthionSingleton(serial=None):
    """ Returns a Cynthion object, re-using an existing object if we already have a connection to the given Cynthion. """

    # If we already have a Cynthion with the given serial,
    if serial in active_connections:
        device = active_connections[serial]
        if device.comms.still_connected():
            return device

    # Otherwise, try to create a new Cynthion instance.
    Cynthion = Cynthion(serial_number=serial)
    active_connections[serial] = Cynthion


    return Cynthion
