=======================================
Using Cynthion USER I/O with Facedancer
=======================================

In addition to the four USB ports Cynthion also includes the following user i/o ports:

* :ref:`USER Button <hardware/device_overview:Left View>`
* :ref:`USER PMOD A & B <hardware/device_overview:Front View>`
* :ref:`USER LEDs <hardware/device_overview:Top View>`

Apart from PMOD B, which is used for the Facedancer SoC UART and JTAG interface, all of these can be accessed from within Python Facedancer devices.

Before proceeding, please ensure you have completed all steps in the :doc:`getting_started` and :doc:`getting_started_facedancer` sections.


Requirements
------------

* A Cynthion running the Facedancer bitstream.
* Two USB Cables.


Using Cynthion APIs
-------------------

To access Cynthion USER i/o using Python you first need an instance of the ``Cynthion`` object, which can be created as follows:

.. code-block :: python

    from cynthion import Cynthion

    c = Cynthion()

Once you have a ``Cynthion`` instance you can easily access USER i/o using the ``leds`` and ``gpio`` APIs, for example:

.. code-block :: python

    from cynthion import Cynthion

    # Get Cynthion instance
    c = Cynthion()

    # Turn on USER Led5
    c.leds[5].on()

    # Turn off USER Led5
    c.leds[5].off()

    # Toggle USER Led4
    c.leds[4].toggle()

    # Get USER Button
    user_button = c.gpio.get_pin("USER")

    # Wait for the USER Button to be pressed
    while user_button.read() == False:
        pass


Using USER Button and Leds with Facedancer
------------------------------------------

Lets modify the Facedancer rubber-ducky example to give us a bit more information and control using Cynthion USER i/o. We'll subclass a Facedancer device and add some calls to the USER i/o APIs in response to host requests and device responses.

Create a new Python file called ``facedancer-user-io.py`` and add the following content:

.. code-block :: python
    :caption: facedancer-user-io.py
    :linenos:

    import logging

    from facedancer                   import main
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
        await device.user.on_state(True)

        logging.info("Typing string into target device.")

        # Type a string with the device
        await device.type_string("echo hello, facedancer\n")

        logging.info("Finished. Press the USER button again to quit.")

        # Done
        await device.user.on_state(True)
        raise errors.EndEmulation("User quit the emulation.")

    # Start emulation
    device = MyKeyboardDevice()
    main(device, type_letters())


Open a terminal and run:

.. code-block :: sh

    python ./facedancer-user-io.py

If everything went well you should see prompts at various points to press the USER button to continue execution as well as the USER Leds flashiing in response to device events.


Using USER Pmod inputs and outputs
----------------------------------

In addition to the USER Button and Leds, Facedancer can also make use of Cynthion USER Pmod A (USER Pmod B is used for JTAG and UART duties) to trigger or respond to external hardware.

They use the same ``gpio`` APIs as the USER Button but individual pins can also be configured as inputs or outputs:

.. code-block :: python

    from cynthion import           Cynthion
    from cynthion.interfaces.gpio  import PinDirection

    # Get Cynthion instance
    c = Cynthion()

    # Get USER Pmod Pin A1 and configure it as an output
    a1 = c.gpio.get_pin("A1")
    a1.set_direction(PinDirection.Output)

    # Set Pin A1 state to high
    a1.write(True)

    # Set Pin A1 state to low
    a1.write(False)

    # Strobe Pin A1
    a1.strobe()

    # Get USER Pmod Pin A7 and configure it as an input
    a7 = c.gpio.get_pin("A7")
    a7.set_direction(PinDirection.Input)

    # Read the current value of Pin A7


* (Optional) A logic analyzer such as the Great Scott Gadgets `GreatFET One <https://greatscottgadgets.com/greatfet/one/>`__

.. code-block :: python
    :caption: facedancer-user-io.py
    :linenos:



PMOD Inputs
-----------

TODO


Source Code
-----------

.. literalinclude:: ../../cynthion/python/examples/facedancer-user-io.py
    :caption: facedancer-user-io.py
    :language: python
    :linenos:
