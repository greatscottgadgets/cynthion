import asyncio
import logging

from facedancer import main
from facedancer.devices.keyboard     import USBKeyboardDevice
from facedancer.classes.hid.keyboard import KeyboardModifiers

device = USBKeyboardDevice()

async def type_letters():
    # Wait for device to connect
    await asyncio.sleep(2)

    # Type a string with the device
    await device.type_string("echo hello, facedancer\n")

main(device, type_letters())
