USB Gateware: Part 3 - Control Transfers
########################################

This series of tutorial walks through the process of implementing a complete USB device with Cynthion and `LUNA <https://luna.readthedocs.io/>`__:

* :doc:`/tutorials/gateware_usb_device_01`
* :doc:`/tutorials/gateware_usb_device_02`
* :doc:`/tutorials/gateware_usb_device_03` *(This tutorial)*
* :doc:`/tutorials/gateware_usb_device_04`

The goal of this tutorial is to define a control interface for the device we created in Part 1 that will allow it to receive and respond to control requests from a host.


Prerequisites
=============

 * Complete the :doc:`/tutorials/gateware_usb_device_01` tutorial.
 * Complete the :doc:`/tutorials/gateware_usb_device_02` tutorial. *(Optional, required for Windows support)*


Data Transfer between a Host and Device
=======================================

USB is a *host-centric bus*, what this means is that all transfers are initiated by the host irrespective of the direction of data transfer.

For data transfers to the device, the host issues an OUT token to notify the device of an incoming data transfer. When data has to be transferred from the device, the host issues an IN token to notify the device that it should send some data to the host.

The USB 2.0 specification defines four endpoint or transfer types:

* **Control Transfers:** Typically used for command and status operations, control transfers are the only transfer type with a defined USB format.
* **Bulk Transfers:** Bulk transfers are best suited for large amounts of data delivered in bursts such as file transfers to/from a storage device or the captured packet data from Cynthion to the control host.
* **Interrupt Transfers:** Interrupt transfers are a bit of a misnomer as the host needs to continuously poll the device to check if an interrupt has occurred but the principle is the same. Commonly used for peripherals that generate input events such as a keyboard or mouse.
* **Isochronous Transfers:** Finally, isochronous transfers occur continuously with a fixed periodicity. Suited for time-sensitive information such as video or audio streams they do not offer any guarantees on delivery. If a packet or frame is dropped it's is up to the host driver to decide on how to best handle it.

By default all LUNA devices have a default implementation for two endpoints: An OUT Control Endpoint and an IN Control endpoint. These endpoints are used by the host to enumerate the device but they can also be extended to support various other class or custom vendor requests.

We'll start by extending our control endpoints to support two vendor requests: One to set the state of the Cynthion **FPGA LEDs** and another to get the state of the Cynthion **USER BUTTON**.


Extend Default Control Endpoints
--------------------------------

To implement vendor requests, begin by adding a ``VendorRequestHandler`` to our device's control endpoint:

.. code-block :: python
    :caption: gateware-usb-device.py
    :linenos:
    :emphasize-lines: 12-14, 19-43, 83-84

    from amaranth                                    import *
    from luna.usb2                                   import USBDevice
    from usb_protocol.emitters                       import DeviceDescriptorCollection

    from luna.gateware.usb.request.windows           import (
        MicrosoftOS10DescriptorCollection,
        MicrosoftOS10RequestHandler,
    )
    from usb_protocol.emitters.descriptors.standard  import get_string_descriptor
    from usb_protocol.types.descriptors.microsoft10  import RegistryTypes

    from luna.gateware.stream.generator              import StreamSerializer
    from luna.gateware.usb.request.control           import ControlRequestHandler
    from luna.gateware.usb.usb2.transfer             import USBInStreamInterface

    VENDOR_ID  = 0x1209 # https://pid.codes/1209/
    PRODUCT_ID = 0x0001

    class VendorRequestHandler(ControlRequestHandler):
        VENDOR_SET_FPGA_LEDS   = 0x01
        VENDOR_GET_USER_BUTTON = 0x02

        def elaborate(self, platform):
            m = Module()

            # shortcuts
            interface: RequestHandlerInterface = self.interface
            setup: SetupPacket = self.interface.setup

            # get a reference to the FPGA LEDs and USER button
            fpga_leds   = Cat(platform.request("led", i).o for i in range(6))
            user_button = platform.request("button_user").i

            # create a streamserializer for transmitting IN data back to the host
            serializer = StreamSerializer(
                domain           = "usb",
                stream_type      = USBInStreamInterface,
                data_length      = 1,
                max_length_width = 1,
            )
            m.submodules += serializer

            return m

    class GatewareUSBDevice(Elaboratable):

        ...

        def elaborate(self, platform):
            m = Module()

            # configure cynthion's clocks and reset signals
            m.submodules.car = platform.clock_domain_generator()

            # request the physical interface for cynthion's TARGET C port
            ulpi = platform.request("target_phy")

            # create the USB device
            m.submodules.usb = usb = USBDevice(bus=ulpi)

            # create our standard descriptors and add them to the device's control endpoint
            descriptors = self.create_standard_descriptors()
            control_endpoint = usb.add_standard_control_endpoint(
                descriptors,
                avoid_blockram=True,
            )

            # add microsoft os 1.0 descriptors and request handler
            descriptors.add_descriptor(get_string_descriptor("MSFT100\xee"), index=0xee)
            msft_descriptors = MicrosoftOS10DescriptorCollection()
            with msft_descriptors.ExtendedCompatIDDescriptor() as c:
                with c.Function() as f:
                    f.bFirstInterfaceNumber = 0
                    f.compatibleID          = 'WINUSB'
            with msft_descriptors.ExtendedPropertiesDescriptor() as d:
                with d.Property() as p:
                    p.dwPropertyDataType = RegistryTypes.REG_SZ
                    p.PropertyName       = "DeviceInterfaceGUID"
                    p.PropertyData       = "{88bae032-5a81-49f0-bc3d-a4ff138216d6}"
            msft_handler = MicrosoftOS10RequestHandler(msft_descriptors, request_code=0xee)
            control_endpoint.add_request_handler(msft_handler)

            # add our vendor request handler
            control_endpoint.add_request_handler(VendorRequestHandler())

            # configure the device to connect by default when plugged into a host
            m.d.comb += usb.connect.eq(1)

            return m


Vendor requests are unique to a device and are identified by the 8-bit ``bRequest`` field of the control transfer setup packet. Here we've defined two id's corresponding to setting the led states and getting the button state.

So far our ``VendorRequestHandler`` contains references to Cynthion's **FPGA LEDs** and **USER BUTTON**, as well as a **StreamSerializer** we'll be using to send data back to the host when it asks for the **USER BUTTON** status.


Implement Vendor Request Handlers
---------------------------------

Let's implement that functionality below:

.. code-block :: python
    :caption: gateware-usb-device.py
    :linenos:
    :emphasize-lines: 25-65

    class VendorRequestHandler(ControlRequestHandler):
        VENDOR_SET_FPGA_LEDS   = 0x01
        VENDOR_GET_USER_BUTTON = 0x02

        def elaborate(self, platform):
            m = Module()

            # Shortcuts.
            interface: RequestHandlerInterface = self.interface
            setup: SetupPacket = self.interface.setup

            # Grab a reference to the FPGA LEDs and USER button.
            fpga_leds   = Cat(platform.request("led", i).o for i in range(6))
            user_button = platform.request("button_user").i

            # Create a StreamSerializer for sending IN data back to the host
            serializer = StreamSerializer(
                domain           = "usb",
                stream_type      = USBInStreamInterface,
                data_length      = 1,
                max_length_width = 1,
            )
            m.submodules += serializer

            # we've received a setup packet containing a vendor request.
            with m.If(setup.type == USBRequestType.VENDOR):
                # use a state machine to sequence our request handling
                with m.FSM(domain="usb"):
                    with m.State("IDLE"):
                        with m.If(setup.received):
                            with m.Switch(setup.request):
                                with m.Case(self.VENDOR_SET_FPGA_LEDS):
                                    m.next = "HANDLE_SET_FPGA_LEDS"
                                with m.Case(self.VENDOR_GET_USER_BUTTON):
                                    m.next = "HANDLE_GET_USER_BUTTON"

                    with m.State("HANDLE_SET_FPGA_LEDS"):
                        # take ownership of the interface
                        m.d.comb += interface.claim.eq(1)

                        # if we have an active data byte, set the FPGA LEDs to the payload
                        with m.If(interface.rx.valid & interface.rx.next):
                            m.d.usb += fpga_leds.eq(interface.rx.payload[0:6])

                        # once the receive is complete, respond with an ACK
                        with m.If(interface.rx_ready_for_response):
                            m.d.comb += interface.handshakes_out.ack.eq(1)

                        # finally, once we reach the status stage, send a ZLP
                        with m.If(interface.status_requested):
                            m.d.comb += self.send_zlp()
                            m.next = "IDLE"

                    with m.State("HANDLE_GET_USER_BUTTON"):
                        # take ownership of the interface
                        m.d.comb += interface.claim.eq(1)

                        # write the state of the user button into a local data register
                        data = Signal(8)
                        m.d.comb += data[0].eq(user_button)

                        # transmit our data using a built-in handler function that
                        # automatically advances the FSM back to the 'IDLE' state on
                        # completion
                        self.handle_simple_data_request(m, serializer, data)

            return m



When handling a control request in LUNA the first thing we look at is the ``setup.type`` field of the setup packet interface. We could check for other types such as ``USBRequestType.CLASS`` or ``USBRequestType.DEVICE`` if we wanted to implement handlers for them but, in this case, we're only interested in vendor requests.

Next, we take ownership of the interface, in order to avoid conflicting with the standard, or other registered request handlers. Then we sequence the actual request handling with an `Amaranth Finite State Machine <https://amaranth-lang.org/docs/amaranth/latest/guide.html#lang-fsm>`__, starting in the  ``IDLE`` state.

While in ``IDLE`` we wait for the ``setup.received`` signal to go high and signal the arrival of a new control request. We then parse the ``setup.request`` field to identify the next state to advance our FSM to. (We could also use the other setup packet fields such as ``wValue`` and ``wIndex`` for dispatch or as arguments but for now we're just intered in ``bRequest``.)

We then implement two handlers, the first is ``HANDLE_SET_FPGA_LEDS``, which needs to read the data sent with our OUT control request in order to set the fpga leds state.

Then the second, in ``HANDLE_GET_USER_BUTTON`` we will use one of the built-in LUNA helper function to respond to our IN control request with the data containing the state of the user button.


Test Control Endpoints
======================

First, remember to build and upload the device gateware to your Cynthion with:

.. code-block :: sh

    python ./gateware-usb-device.py

Then, open your ``test-gateware-usb-device.py`` script from the previous tutorials and add the following code to it:

.. literalinclude:: ../../../cynthion/python/examples/tutorials/test-gateware-usb-device-03.py
    :caption: test-gateware-usb-device.py
    :language: python
    :linenos:
    :emphasize-lines: 2, 7-8,  22-68, 78-87


Run the file with:

.. code-block :: sh

    python ./test-gateware-usb-device.py

And, if all goes well you should see the **FPGA LEDs** on Cynthion counting in binary. If you press and release the **USER** button you should see the count reset back to zero and the following text in the terminal.

.. code-block :: sh

    USER button is: ON
    USER button is: OFF

Job done!

In the next part of the tutorial we'll finish up by adding IN and OUT Bulk endpoints to our device.


Exercises
=========

1. Add a vendor request to retrieve the current state of the FPGA LEDs.
2. Add a vendor request that will disconnect and then re-connect your device to the USB bus.


More information
================

* Beyond Logic's `USB in a NutShell <https://www.beyondlogic.org/usbnutshell/usb1.shtml>`__.
* `LUNA Documentation <https://luna.readthedocs.io/en/latest/>`__


Source Code
===========

.. literalinclude:: ../../../cynthion/python/examples/tutorials/gateware-usb-device-03.py
    :caption: gateware-usb-device-03.py
    :language: python
    :linenos:


.. literalinclude:: ../../../cynthion/python/examples/tutorials/test-gateware-usb-device-03.py
    :caption: test-gateware-usb-device-03.py
    :language: python
    :linenos:
