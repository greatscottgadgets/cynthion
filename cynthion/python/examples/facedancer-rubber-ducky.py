import asyncio

from facedancer                   import main
from facedancer.devices.keyboard  import USBKeyboardDevice

device = USBKeyboardDevice()

async def type_letters():
    # Wait for device to connect
    await asyncio.sleep(2)

    # Type a string with the device
    await device.type_string("echo hello, facedancer\n")

main(device, type_letters())
