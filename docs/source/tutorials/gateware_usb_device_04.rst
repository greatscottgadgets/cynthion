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
    :emphasize-lines: 19-26, 31, 50-59

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

LUNA provides the ``USBStreamOutEndpoint`` and ``USBStreamInEndpoint`` components which conform to the `Amaranth Data streams <https://amaranth-lang.org/docs/amaranth/latest/stdlib/stream.html>`__ interface. Simply put, streams provide a uniform mechanism for unidirectional exchange of arbitrary data between gateware components.

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

This could be any data source and/or sink but for the purposes of this tutorial let's create a simple loopback function that will accept a bulk OUT request from the host and then return the request payload when the host makes a bulk IN request.


Define Endpoint Functions
-------------------------

A simple implementation for our device's endpoint functions could be a simple FIFO (First In First Out) queue with enough space to hold the 512 bytes of a bulk transfer.

Using the OUT endpoint we could then transmit a stream of data from the host to Cynthion and write it into the FIFO. Then, when we transmit a request from the host to the IN endpoint we can stream the previously queued data back to the host.

We're only working in a single clock-domain so we can use a `SyncFIFO <https://amaranth-lang.org/docs/amaranth/latest/stdlib/fifo.html#amaranth.lib.fifo.SyncFIFO>`__ from the Amaranth standard library for our queue:


.. code-block :: python
    :caption: gateware-usb-device.py
    :linenos:
    :emphasize-lines: 2, 28-44

    from amaranth                                    import *
    from amaranth.lib.fifo                           import SyncFIFO
    from luna.usb2                                   import USBDevice
    from usb_protocol.emitters                       import DeviceDescriptorCollection
    ...

    class VendorRequestHandler(ControlRequestHandler):
        ...

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

            # create a FIFO queue we'll connect to the stream interfaces of our
            # IN & OUT endpoints
            m.submodules.fifo = fifo = DomainRenamer("usb")(
                SyncFIFO(width=8, depth=MAX_PACKET_SIZE)
            )

            # connect our Bulk OUT endpoint's stream interface to the FIFO's write port
            stream_out = ep_out.stream
            m.d.comb += fifo.w_data.eq(stream_out.payload)
            m.d.comb += fifo.w_en.eq(stream_out.valid)
            m.d.comb += stream_out.ready.eq(fifo.w_rdy)

            # connect our Bulk IN endpoint's stream interface to the FIFO's read port
            stream_in  = ep_in.stream
            m.d.comb += stream_in.payload.eq(fifo.r_data)
            m.d.comb += stream_in.valid.eq(fifo.r_rdy)
            m.d.comb += fifo.r_en.eq(stream_in.ready)

            # configure the device to connect by default when plugged into a host
            m.d.comb += usb.connect.eq(1)

            return m

.. note::

    Something to take note off is the use of an Amaranth `DomainRenamer <https://amaranth-lang.org/docs/amaranth/latest/guide.html#renaming-domains>`__ component to wrap ``SyncFIFO`` in the following lines:

    .. code-block :: python

        m.submodules.fifo = fifo = DomainRenamer("usb")(
            fifo.SyncFIFO(width=8, depth=MAX_PACKET_SIZE)
        )

    Any moderately complex FPGA hardware & gateware design will usually consist of multiple clock-domains running at different frequencies. Cynthion, for example, has three clock domains:

    * ``sync`` - the default clock domain, running at 120 MHz.
    * ``usb``  - the clock domain for USB components and gateware, running at 60 MHz.
    * ``fast`` - a fast clock domain used for the HyperRAM, running at 240 MHz.

    Because our designs so far have all been interfacing with Cynthion's USB components we've only needed to use the ``usb`` clock domain. However, reusable Amaranth components such as ``SyncFIFO`` are usually implemented using the default ``sync`` domain. We therefore need to be able to rename its clock domain to match the domain used in our design. This is what ``DomainRenamer`` does.

And that's it, we've defined our endpoint functions! Let's try it out.


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

1. Create a benchmark to test the speed of your device when doing Bulk IN and OUT transfers.
2. Move the device endpoint to ``aux_phy`` and attempt to capture the packets exchanged between a device plugged into a host via the ``target_phy`` port.


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
