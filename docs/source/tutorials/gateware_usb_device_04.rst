USB Gateware: Part 4 - Bulk Transfers
#####################################

This series of tutorial walks through the process of implementing a complete USB device with Cynthion and `LUNA <https://luna.readthedocs.io/>`__:

* :doc:`/tutorials/gateware_usb_device_01`
* :doc:`/tutorials/gateware_usb_device_02`
* :doc:`/tutorials/gateware_usb_device_03`
* :doc:`/tutorials/gateware_usb_device_04` *(This tutorial)*

The goal of this tutorial is to define Bulk Endpoints for the device we created in Part 3 that will allow us to efficiently perform larger data transfers than those allowed by Control Transfers.


Prerequisites
=============

 * Complete the :doc:`/tutorials/gateware_usb_device_01` tutorial.
 * Complete the :doc:`/tutorials/gateware_usb_device_02` tutorial. *(Optional, required for Windows support)*
 * Complete the :doc:`/tutorials/gateware_usb_device_03` tutorial.


Add Bulk Endpoints
==================

While Control transfers are well suited for command and status operations they are not the best way to exchange large quantities of data. Control transfers have high per-packet protocol overhead and can only transfer packets of 8 bytes on low speed (1.5Mbps) devices and 64 bytes on full (12Mbps) and high (512Mbps) speed devices.

On the other hand, Bulk transfers support a packet size of up to 512 bytes on high speed devices and do not require any protocol overhead.

In the first section we'll begin by updating our device's descriptors so it can inform the host that it has bulk endpoints available.


Update Device Descriptors
-------------------------

Open ``gateware-usb-device.py`` and add the highlighted lines:

.. code-block :: python
    :caption: gateware-usb-device.py
    :linenos:
    :emphasize-lines: 19-27, 32, 51-60

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
    from luna.gateware.usb.request.interface         import SetupPacket
    from luna.gateware.usb.usb2.request              import RequestHandlerInterface
    from luna.gateware.usb.usb2.transfer             import USBInStreamInterface
    from usb_protocol.types                          import USBRequestType

    from luna.gateware.stream                        import StreamInterface
    from luna.usb2                                   import (
        USBStreamInEndpoint,
        USBStreamOutEndpoint,
    )
    from usb_protocol.types                          import (
        USBDirection,
        USBTransferType,
    )

    VENDOR_ID  = 0x1209 # https://pid.codes/1209/
    PRODUCT_ID = 0x0001

    MAX_PACKET_SIZE = 512

    class VendorRequestHandler(ControlRequestHandler):
        ...

    class GatewareUSBDevice(Elaboratable):
        def create_standard_descriptors(self):
            descriptors = DeviceDescriptorCollection()

            with descriptors.DeviceDescriptor() as d:
                d.idVendor           = VENDOR_ID
                d.idProduct          = PRODUCT_ID
                d.iManufacturer      = "Cynthion Project"
                d.iProduct           = "Gateware USB Device"
                d.bNumConfigurations = 1

            with descriptors.ConfigurationDescriptor() as c:
                with c.InterfaceDescriptor() as i:
                    i.bInterfaceNumber = 0
                    # EP 0x01 OUT - receives bulk data from the host
                    with i.EndpointDescriptor() as e:
                        e.bEndpointAddress = USBDirection.OUT.to_endpoint_address(0x01)
                        e.bmAttributes     = USBTransferType.BULK
                        e.wMaxPacketSize   = MAX_PACKET_SIZE
                    # EP 0x82 IN  - transmits bulk data to the host
                    with i.EndpointDescriptor() as e:
                        e.bEndpointAddress = USBDirection.IN.to_endpoint_address(0x02)
                        e.bmAttributes     = USBTransferType.BULK
                        e.wMaxPacketSize   = MAX_PACKET_SIZE

            return descriptors

        def elaborate(self, platform):
            ...

This adds two endpoint descriptors to our default interface, each of type ``USBTransferType.BULK`` and with a ``MAX_PACKET_SIZE`` of 512. Where the endpoints differ is in their endpoint address. USB endpoint descriptors encode their direction in an 8 bit endpoint address. The first four bits encode the endpoint number, the next three bits are reserved and set to zero and the final bit encodes the direction; 0 for OUT and 1 for IN.

This means that an OUT endpoint number of 0x01 encodes to an endpoint address of 0x01 while an IN endpoint number of ``0x02`` encodes to the address ``0x82``. (``0b0000_0010 + 0b1000_0000 = 0b1000_0010 = 0x82``)


Add USB Stream Endpoints
------------------------

Once our endpoint descriptors have been added to our device configuration we will need some gateware that will be able to respond to USB requests from the host and allow us to receive and transmit data.

LUNA provides the ``USBStreamOutEndpoint`` and ``USBStreamInEndpoint`` modules which conform to the `Amaranth Data streams <https://amaranth-lang.org/docs/amaranth/latest/stdlib/stream.html>`__ interface. Simply put, streams provide a uniform mechanism for unidirectional exchange of arbitrary data between gateware modules.

.. code-block :: python
    :caption: gateware-usb-device.py
    :linenos:
    :emphasize-lines: 13-23

    ...

    class GatewareUSBDevice(Elaboratable):
        def create_standard_descriptors(self):
            ...

        def elaborate(self, platform):
            ...

            # add the vendor request handler
            control_endpoint.add_request_handler(VendorRequestHandler())

            # create and add stream endpoints for our device's Bulk IN & OUT endpoints
            ep_out = USBStreamOutEndpoint(
                endpoint_number=0x01,  # (EP 0x01)
                max_packet_size=MAX_PACKET_SIZE,
            )
            usb.add_endpoint(ep_out)
            ep_in = USBStreamInEndpoint(
                endpoint_number=0x02,  # (EP 0x82)
                max_packet_size=MAX_PACKET_SIZE
            )
            usb.add_endpoint(ep_in)

            # configure the device to connect by default when plugged into a host
            m.d.comb += usb.connect.eq(1)

            return m

We now have two streaming endpoints that are able to receive and transmit data between any other module that supports the Amaranth Data streams interface.

However, before we can stream any data across these endpoints we first need to come up with a *USB Function* for each of our endpoints. In other words, what does our device actually _do_?

This could be any data source and/or sink but for the purposes of this tutorial let's create a (very) simple storage device.


Define Endpoint Functions
-------------------------

Our device's endpoint functions will be a simple streaming memory store module that will allow us to read & write data over our bulk endpoints.

Using the OUT endpoint we can transmit a stream of data from the host to Cynthion and write into a local memory. Then, we'd like to be able to transmit a request from the host to the IN endpoint and retrieve the previously stored data.

This means we'll need some memory we can read and write to, so let's begin by creating an `Amaranth Memory component <https://amaranth-lang.org/docs/amaranth/latest/stdlib/memory.html>`__ which uses the FPGA's Block RAM for storage:

.. code-block :: python
    :caption: gateware-usb-device.py
    :linenos:
    :emphasize-lines: 6-33, 41-57

    ...

    class VendorRequestHandler(ControlRequestHandler):
        ...

    class StreamingMemoryStore(Elaboratable):
        def __init__(self, stream_out: StreamInterface, stream_in: StreamInterface):
            self.stream_out = stream_out
            self.stream_in  = stream_in

            # high when a memory write is in process
            self.write_active = Signal()

        def elaborate(self, platform):
            m = Module()

            # create a memory we can use as a data source/sink for our bulk endpoints
            m.submodules.ram = ram = Memory(
                width = 8,
                depth = MAX_PACKET_SIZE,
                init  = [0] * MAX_PACKET_SIZE
            )
            w_port = ram.write_port(domain="usb")
            r_port = ram.read_port(domain="usb")

            # set the write_active status to the write port's enable status
            m.d.comb += self.write_active.eq(w_port.en)

            # shortcuts
            stream_out  = self.stream_out
            stream_in   = self.stream_in

            return m

    class GatewareUSBDevice(Elaboratable):
        ...

        def elaborate(self, platform):
            ...

            # create and add stream endpoints for our device's Bulk IN & OUT endpoints
            ep_out = USBStreamOutEndpoint(
                endpoint_number=0x01,  # (EP 0x01)
                max_packet_size=MAX_PACKET_SIZE,
            )
            usb.add_endpoint(ep_out)
            ep_in = USBStreamInEndpoint(
                endpoint_number=0x02,  # (EP 0x82)
                max_packet_size=MAX_PACKET_SIZE
            )
            usb.add_endpoint(ep_in)

            # create a simple streaming memory storage module
            m.submodules.store = store = StreamingMemoryStore(ep_out.stream, ep_in.stream)

            # invalidate any data queued on ep_in when the memory performs a write operation
            m.d.comb += ep_in.discard.eq(store.write_active)

            # configure the device to connect by default when plugged into a host
            m.d.comb += usb.connect.eq(1)

            return m

Great, now we have a 8-bit wide memory that's large enough to store a full high-speed transfer packet! Let's implement the logic to handle read/write requests to our memory store module.


Bulk OUT Endpoint Gateware
--------------------------

To implement our OUT endpoint's stream we'll need to read the data stream coming from the host and write each bit into our memory component.

Let's implement this by adding the following lines:

.. code-block :: python
    :caption: gateware-usb-device.py
    :linenos:
    :emphasize-lines: 11-25

    ...

    class StreamingMemoryStore(Elaboratable):
        def elaborate(self, platform):
            ...

            # shortcuts
            stream_out  = self.stream_out
            stream_in   = self.stream_in

            # - EP 0x01 OUT logic  ------------------------------------------------

            # let the stream know we're always ready to start reading
            m.d.comb += stream_out.ready.eq(1)

            # wire the payload from the host up to our memory write port
            m.d.comb += w_port.data.eq(stream_out.payload)

            # read each byte coming in on the stream and write it to memory
            with m.If(stream_out.valid & stream_out.ready):
                m.d.comb += w_port.en.eq(1)
                m.d.usb += w_port.addr.eq(w_port.addr + 1);
            with m.Else():
                m.d.comb += w_port.en.eq(0)
                m.d.usb += w_port.addr.eq(0)

            return m

    ...

When the host makes a Bulk OUT request to the device we read each byte of the data packet from the endpoint stream as it is received and then write it to a consecutive address in our memory. We'll talk more about LUNA stream interfaces in a future tutorial but all you need to know for now is that they provide a uniform interface between LUNA endpoints and other components.

Finally, let's do the same for our IN endpoint's stream.

Bulk IN Endpoint Gateware
-------------------------

Add the following lines:

.. code-block :: python
    :caption: gateware-usb-device.py
    :linenos:
    :emphasize-lines: 11-26

    ...

    class StreamingMemoryStore(Elaboratable):
        def elaborate(self, platform):
            ...

            # - EP 0x01 OUT logic  ------------------------------------------------

            ...

            # - EP 0x82 IN logic  -------------------------------------------------

            # wire the payload to the host up to our memory read port
            m.d.comb += stream_in.payload.eq(r_port.data)

            # discard streamed data when memory write port is active
            m.d.comb += self.ep_in.discard.eq(w_port.en)

            # when the stream is ready and the write port is not active,
            # read each byte from memory and write it out to the stream
            with m.If(stream_in.ready & ~w_port.en):
                m.d.usb += stream_in.valid.eq(1)
                m.d.usb += r_port.addr.eq(r_port.addr + 1)
            with m.Else():
                m.d.usb += stream_in.valid.eq(0)
                m.d.usb += r_port.addr.eq(0)

            return m

    ...

This time, when the host makes a Bulk IN request, we write the content of each consecutive address of our memory to the stream. Let's try it out!


Test Bulk Endpoints
===================

Open up ``test-gateware-usb-device.py`` and add the following code to it:

.. literalinclude:: ../../../cynthion/python/examples/tutorials/test-gateware-usb-device-04.py
    :caption: test-gateware-usb-device.py
    :language: python
    :linenos:
    :emphasize-lines: 3, 11, 74-106, 124-125

Run the file with:

.. code-block :: sh

    python ./test-gateware-usb-device.py

Assuming everything is going to plan you should see two matching sets of random numbers:

.. code-block :: sh

    OUT endpoint transmitted 512 bytes: [252, 107, 106, 56] ... [109, 175, 112, 126]
    IN  endpoint received 512 bytes:    [252, 107, 106, 56] ... [109, 175, 112, 126]

Congratulations, if you made it this far then you've just finished building your first complete USB Gateware Device with custom vendor request control and bulk data transfer!


Exercises
=========

1. Add a vendor request to zero the memory.
2. Create a benchmark to test the speed of your device when doing Bulk IN and OUT transfers.
3. Use the contents of the memory to drive the pattern of the FPGA LEDs.
4. Move the device endpoint to ``aux_phy`` and attempt to capture the enumeration of a device plugged into a host via the ``target_phy`` port.


More information
================

* Beyond Logic's `USB in a NutShell <https://www.beyondlogic.org/usbnutshell/usb1.shtml>`__.
* `LUNA Documentation <https://luna.readthedocs.io/en/latest/>`__


Source Code
===========

.. literalinclude:: ../../../cynthion/python/examples/tutorials/gateware-usb-device-04.py
    :caption: gateware-usb-device-04.py
    :language: python
    :linenos:


.. literalinclude:: ../../../cynthion/python/examples/tutorials/test-gateware-usb-device-04.py
    :caption: test-gateware-usb-device-04.py
    :language: python
    :linenos:
