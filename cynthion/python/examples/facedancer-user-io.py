import asyncio
import logging
import sys


from facedancer                   import main, errors
from facedancer.devices.keyboard  import USBKeyboardDevice

from cynthion                     import Cynthion
from cynthion.interfaces.gpio     import PinDirection

# Subclass USBKeyboardDevice
class MyKeyboardDevice(USBKeyboardDevice):
    def __post_init__(self):
        super().__post_init__()

        # Get a Cynthion instance.
        cynthion = Cynthion()

        # Get USER button
        self.user = cynthion.gpio.get_pin("USER")

        # Get USER leds
        self.leds = cynthion.leds
        [led.off() for led in self.leds.values()]

        # Get some USER pmod outputs
        self.a1  = cynthion.gpio.get_pin("A1")
        self.a2  = cynthion.gpio.get_pin("A2")
        self.a3  = cynthion.gpio.get_pin("A3")
        self.a4  = cynthion.gpio.get_pin("A4")
        self.a1.set_direction(PinDirection.Output)
        self.a2.set_direction(PinDirection.Output)
        self.a3.set_direction(PinDirection.Output)
        self.a4.set_direction(PinDirection.Output)

        # Get some USER pmod inputs
        self.a7  = cynthion.gpio.get_pin("A7")
        self.a8  = cynthion.gpio.get_pin("A8")
        self.a9  = cynthion.gpio.get_pin("A9")
        self.a10 = cynthion.gpio.get_pin("A10")
        self.a7.set_direction(PinDirection.Input)
        self.a8.set_direction(PinDirection.Input)
        self.a9.set_direction(PinDirection.Input)
        self.a10.set_direction(PinDirection.Input)

    # Strobe USER led0 and USER pmod A1 every time we see a bus reset.
    def handle_bus_reset(self):
        self.leds[0].strobe(duration=0.1)
        self.a1.strobe()
        super().handle_bus_reset()

    # Strobe USER led1 and USER pmod A2 every time the host makes a control request.
    def handle_request(self, request):
        self.leds[1].strobe(duration=0.1)
        self.a2.strobe()
        super().handle_request(request)

    # Strobe USER led2 and USER pmod A3 every time the device responds to a control request.
    def control_send(self, endpoint_number, in_request, data, *, blocking = False):
        self.leds[2].strobe(duration=0.1)
        self.a3.strobe()
        super().control_send(endpoint_number, in_request, data, blocking=blocking)

    # Strobe USER led3 and USER pmod A4 every time the host requests data from the host.
    def handle_data_requested(self, endpoint):
        self.a4.strobe()
        report = self._generate_hid_report()
        if report[2] == 0:
            self.leds[3].strobe(duration=0.1)
        else:
            self.leds[4].strobe(duration=0.1)
        endpoint.send(report)

# rubber ducky
async def type_letters():
    # Wait for device to connect
    await asyncio.sleep(2)

    logging.info("Press the USER button to proceed.")

    # Wait until Cynthion's USER button is pressed
    await device.user.on_state(True)

    logging.info("Typing string into target device.")

    # Type a string with the device
    await device.type_string("echo hello, facedancer\n")

    logging.info("Finished. Press the USER button again to quit.")

    # Done
    await device.user.on_state(True)
    raise errors.EndEmulation("User quit the emulation.")

device = MyKeyboardDevice()
main(device, type_letters())
