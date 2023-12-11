#!/usr/bin/env python3
#
# This file is part of FaceDancer.
#


import logging

from facedancer import main
# TODO from facedancer.devices.mass_storage import USBMassStorageDevice
from mass_storage import USBMassStorageDevice

device = USBMassStorageDevice()

async def hello():
    """ Waits for the host to connect, and then says hello. """

    logging.info("Waiting for the host to connect.")
    await device.wait_for_host()
    logging.info("Host connected!")

main(device, hello())
