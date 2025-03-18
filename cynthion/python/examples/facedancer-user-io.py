import asyncio
import logging

from facedancer                   import main, errors
from facedancer.devices.keyboard  import USBKeyboardDevice

from cynthion                     import Cynthion

# Subclass USBKeyboardDevice
class MyKeyboardDevice(USBKeyboardDevice):
    def __post_init__(self):
        super().__post_init__()

        # Get a Cynthion instance.
        cynthion = Cynthion()

        # Get USER Leds
        self.leds = cynthion.leds

        # Make sure all USER Leds are off
        [led.off() for led in self.leds.values()]

        # Get USER Button
        self.user_button = cynthion.gpio.get_pin("USER")

    def handle_bus_reset(self):
        # Strobe USER Led0 every time we see a bus reset
        self.leds[0].strobe(duration=0.1)
        super().handle_bus_reset()

    def handle_request(self, request):
        # Strobe USER Led1 every time the host makes a control request
        self.leds[1].strobe(duration=0.1)
        super().handle_request(request)

    def control_send(self, endpoint_number, in_request, data, *, blocking = False):
        # Strobe USER Led2 every time the device responds to a control request
        self.leds[2].strobe(duration=0.1)
        super().control_send(endpoint_number, in_request, data, blocking=blocking)

    def handle_data_requested(self, endpoint):
        report = self._generate_hid_report()
        endpoint.send(report)

        # Strobe USER Led3 every time the host requested a HID report descriptor from the host
        if report[2] == 0:
            self.leds[3].strobe(duration=0.1)
        # Strobe USER Led4 if the report descriptor contained a scancode for the host
        else:
            self.leds[4].strobe(duration=0.1)

# Rubber-ducky control script
async def type_letters():
    # Wait for device to connect
    await asyncio.sleep(2)

    logging.info("Press the USER button to proceed.")

    # Wait until Cynthion's USER button is pressed
    while device.user_button.read() == False:
        await asyncio.sleep(0.01)

    logging.info("Typing string into target device.")

    # Type a string with the device
    await device.type_string("echo hello, facedancer\n")

    logging.info("Finished. Press the USER button again to quit.")

    # Done
    while device.user_button.read() == False:
        await asyncio.sleep(0.01)

    raise errors.EndEmulation("User quit the emulation.")

# Start emulation
device = MyKeyboardDevice()
main(device, type_letters())
