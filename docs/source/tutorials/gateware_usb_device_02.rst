USB Gateware: Part 2 - WCID Descriptors
#######################################

This series of tutorial walks through the process of implementing a complete USB device with Cynthion and `LUNA <https://luna.readthedocs.io/>`__:

* :doc:`/tutorials/gateware_usb_device_01`
* :doc:`/tutorials/gateware_usb_device_02` *(This tutorial)*
* :doc:`/tutorials/gateware_usb_device_03`
* :doc:`/tutorials/gateware_usb_device_04`

The goal of this tutorial is to define the descriptors that will tell Microsoft Windows to use the built-in generic WinUSB driver to communicate with our device.

This tutorial is optional and only required if you would like to use your device on Windows.


Prerequisites
=============

 * Complete the :doc:`/tutorials/gateware_usb_device_01` tutorial.


WCID Devices
============

WCID devices or "Windows Compatible ID devices", are USB devices that provide extra information to Windows in order to facilitate automatic driver installation or, more frequently, allow programs to obtain direct access to the device.

Historically, Windows required manual installation of drivers for non-class devices with custom vendor interfaces. Contrasted with Linux or macOS which will automatically assign a generic USB driver that allows for direct interaction with the device's endpoints via a cross-platform library such as `libusb <https://libusb.info/>`__ or operating system API's.

Microsoft eventually relented and now provide a Windows-specific mechanism for a device to advertise that it requires a generic WinUSB driver.

The full details are documented in the `Microsoft OS 1.0 <https://learn.microsoft.com/en-us/windows-hardware/drivers/usbcon/microsoft-os-1-0-descriptors-specification>`__ and `Microsoft OS 2.0  <https://learn.microsoft.com/en-us/windows-hardware/drivers/usbcon/microsoft-os-2-0-descriptors-specification>`__ specifications but the basic mechanism consists of a set of Windows-specific descriptor requests made by the host whenever a new device is plugged in.

For Microsoft OS 1.0, this boils down to three descriptor requests we need to be able to handle:

1. Microsoft OS String Descriptor
2. Microsoft Compatible ID Feature Descriptor
3. Microsoft Extended Properties Feature Descriptor


Microsoft OS String Descriptor
------------------------------

To start with, edit your ``gateware-usb-device.py`` file from the previous tutorial and add/modify the highlighted lines:

.. code-block :: python
    :caption: gateware-usb-device.py
    :linenos:
    :emphasize-lines: 5, 24-35

    from amaranth                                    import *
    from luna.usb2                                   import USBDevice
    from usb_protocol.emitters                       import DeviceDescriptorCollection

    from usb_protocol.emitters.descriptors.standard  import get_string_descriptor

    ...

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
                # the blockram descriptor handler lacks support for
                # non-contiguous string descriptor indices, which is
                # required for the Microsoft OS string descriptor at 0xEE.
                avoid_blockram=True,
            )

            # add the microsoft os string descriptor
            descriptors.add_descriptor(get_string_descriptor("MSFT100\xee"), index=0xee)

            # configure the device to connect by default when plugged into a host
            m.d.comb += usb.connect.eq(1)

            return m

The Microsoft OS String Descriptor responds to a standard String Descriptor request with an index of ``0xee``. It encodes two values:

.. code-block :: python

    0x12,         # Descriptor Length: 18 bytes
    0x03,         # Descriptor Type: 3 = String
    0x4d, 0x00,   # M
    0x53, 0x00,   # S
    0x46, 0x00,   # F
    0x54, 0x00,   # T
    0x31, 0x00,   # 1
    0x30, 0x00,   # 0
    0x30, 0x00,   # 0
    0xee, 0x00,   # Vendor Code: 0xee

The first 14 bytes correspond to the little-endian encoded Unicode string ``MSFT100``, with the remaining two bytes corresponding to the Vendor Code Windows should use when requesting the other descriptors. This is often set to the same value as the Microsoft OS String Descriptor index of ``0xee``, but you can use another value if it conflicts with an existing Vendor Code used by your device.


Microsoft Compatible ID Feature Descriptor
------------------------------------------

Next, add the Microsoft Compatible ID Feature Descriptor:

.. code-block :: python
    :caption: gateware-usb-device.py
    :linenos:
    :emphasize-lines: 5-7, 23-30

    from amaranth                                    import *
    from luna.usb2                                   import USBDevice
    from usb_protocol.emitters                       import DeviceDescriptorCollection

    from luna.gateware.usb.request.windows           import (
        MicrosoftOS10DescriptorCollection,
    )
    from usb_protocol.emitters.descriptors.standard  import get_string_descriptor


    VENDOR_ID  = 0x1209 # https://pid.codes/1209/
    PRODUCT_ID = 0x0001

    class GatewareUSBDevice(Elaboratable):
        ...

        def elaborate(self, platform):
            ...

            # add the microsoft os string descriptor
            descriptors.add_descriptor(get_string_descriptor("MSFT100\xee"), index=0xee)

            # add a microsoft descriptor collection for our other two microsoft descriptors
            msft_descriptors = MicrosoftOS10DescriptorCollection()

            # add the microsoft compatible id feature descriptor
            with msft_descriptors.ExtendedCompatIDDescriptor() as c:
                with c.Function() as f:
                    f.bFirstInterfaceNumber = 0
                    f.compatibleID          = 'WINUSB'

            # configure the device to connect by default when plugged into a host
            m.d.comb += usb.connect.eq(1)

            return m

Our remaining descriptors are not returned via Standard Requests, instead they are implemented as Vendor Requests with Microsoft-defined Vendor Indices and the Vendor Code supplied in the Microsoft OS String Descriptor. We will implement the actual vendor request handler in the final step of the tutorial but for now we are just defining the Microsoft OS 1.0 Descriptor Collection that will contain these descriptors.

Our example is defining the simplest possible Compatible ID Feature descriptor, specifying a Function with a device interface number of ``0`` and a compatible ID of ``WINUSB``. This is how we tell Windows to use the generic WinUSB driver for the interface.

If our device had multiple interfaces we could simply extended this by adding additional functions for each interface like so:

.. code-block :: python

    with msft_descriptors.ExtendedCompatIDDescriptor() as c:
        with c.Function() as f:
            f.bFirstInterfaceNumber = 0
            f.compatibleID          = 'WINUSB'
        with c.Function() as f:
            f.bFirstInterfaceNumber = 1
            f.compatibleID          = 'WINUSB'
        ...



Microsoft Extended Properties Feature Descriptor
------------------------------------------------

We now come to our third descriptor, the Microsoft Extended Properties Feature Descriptor:

.. code-block :: python
    :caption: gateware-usb-device.py
    :linenos:
    :emphasize-lines: 9, 31-36

    from amaranth                                    import *
    from luna.usb2                                   import USBDevice
    from usb_protocol.emitters                       import DeviceDescriptorCollection

    from luna.gateware.usb.request.windows           import (
        MicrosoftOS10DescriptorCollection,
    )
    from usb_protocol.emitters.descriptors.standard  import get_string_descriptor
    from usb_protocol.types.descriptors.microsoft10  import RegistryTypes

    ..

    class GatewareUSBDevice(Elaboratable):
        ...

        def elaborate(self, platform):
            ...

            # add the microsoft os string descriptor
            descriptors.add_descriptor(get_string_descriptor("MSFT100\xee"), index=0xee)

            # add a microsoft descriptor collection for our other two microsoft descriptors
            msft_descriptors = MicrosoftOS10DescriptorCollection()

            # add the microsoft compatible id feature descriptor
            with msft_descriptors.ExtendedCompatIDDescriptor() as c:
                with c.Function() as f:
                    f.bFirstInterfaceNumber = 0
                    f.compatibleID          = 'WINUSB'

            # add microsoft extended properties feature descriptor
            with msft_descriptors.ExtendedPropertiesDescriptor() as d:
                with d.Property() as p:
                    p.dwPropertyDataType = RegistryTypes.REG_SZ
                    p.PropertyName       = "DeviceInterfaceGUID"
                    p.PropertyData       = "{88bae032-5a81-49f0-bc3d-a4ff138216d6}"

            # configure the device to connect by default when plugged into a host
            m.d.comb += usb.connect.eq(1)

            return m

The Extended Properties Feature Descriptor can be used to define additional device registry settings but, in our example, we only define the Device Interface GUID we'd like our device to be accessed with.

In this case it's the Microsoft-defined GUID of ``{88bae032-5a81-49f0-bc3d-a4ff138216d6}`` which is `defined as <https://learn.microsoft.com/en-us/windows-hardware/drivers/install/system-defined-device-setup-classes-available-to-vendors>`__ *"all USB devices that don't belong to another class"*. If, for example, our device were a Keyboard or Mouse we'd need to use the appropriate value here.


Microsoft Descriptor Request Handler
------------------------------------

Finally, now that all our descriptors are defined we need to add the actual Vendor Request Handler that will be responsible for responding to descriptor requests from a Windows Host:

.. code-block :: python
    :caption: gateware-usb-device.py
    :linenos:
    :emphasize-lines: 7, 39-41

    from amaranth                                    import *
    from luna.usb2                                   import USBDevice
    from usb_protocol.emitters                       import DeviceDescriptorCollection

    from luna.gateware.usb.request.windows           import (
        MicrosoftOS10DescriptorCollection,
        MicrosoftOS10RequestHandler,
    )
    from usb_protocol.emitters.descriptors.standard  import get_string_descriptor
    from usb_protocol.types.descriptors.microsoft10  import RegistryTypes

    ..

    class GatewareUSBDevice(Elaboratable):
        ...

        def elaborate(self, platform):
            ...

            # add the microsoft os string descriptor
            descriptors.add_descriptor(get_string_descriptor("MSFT100\xee"), index=0xee)

            # add a microsoft descriptor collection for our other two microsoft descriptors
            msft_descriptors = MicrosoftOS10DescriptorCollection()

            # add the microsoft compatible id feature descriptor
            with msft_descriptors.ExtendedCompatIDDescriptor() as c:
                with c.Function() as f:
                    f.bFirstInterfaceNumber = 0
                    f.compatibleID          = 'WINUSB'

            # add microsoft extended properties feature descriptor
            with msft_descriptors.ExtendedPropertiesDescriptor() as d:
                with d.Property() as p:
                    p.dwPropertyDataType = RegistryTypes.REG_SZ
                    p.PropertyName       = "DeviceInterfaceGUID"
                    p.PropertyData       = "{88bae032-5a81-49f0-bc3d-a4ff138216d6}"

            # add the request handler for Microsoft descriptors
            msft_handler = MicrosoftOS10RequestHandler(msft_descriptors, request_code=0xee)
            control_endpoint.add_request_handler(msft_handler)

            # configure the device to connect by default when plugged into a host
            m.d.comb += usb.connect.eq(1)

            return m

LUNA provides a pre-defined implementation for handling Microsoft OS10 Descriptor Requests and only requires the descriptor collection and the vendor request code we defined in the Microsoft OS10 String Descriptor.


Testing the Device
==================

Connect
-------

* For this tutorial you will need to connect the Cynthion **TARGET C** port to a Windows computer for testing.
* Plug  the **CONTROL** port into the computer you've been using to control Cynthion. If this is the same machine as the Windows computer you're using to test, plug it in there.

Build
-----

Build the device gateware and upload it to your Cynthion by typing the following into your terminal shell:

.. code-block :: sh

    python ./gateware-usb-device.py

If everything went well we should now be able to check if Windows can recognize the device.


Test
----

.. tab:: Windows

    To test whether the WCID descriptors have been recognized, open the Windows Device Manager and look for the device under the *"Universal Serial Bus devices"* section:

        .. image:: ../../images/tutorial_gateware_usb_device/with_wcid.png
          :alt: Gateware USB Device on Windows without WCID Descriptors.

.. tab:: Python

    You should find that the Python test program from :doc:`/tutorials/gateware_usb_device_01` now works as expected:

    .. code-block :: python
        :caption: test-gateware-usb-device.py
        :linenos:

        import usb1

        def list_devices(context):
            for device in context.getDeviceList():
                try:
                    manufacturer = device.getManufacturer()
                    product = device.getProduct()
                    print(f"{device}:  {manufacturer} - {product}")
                except Exception as e:
                    print(f"{device}: {e}")

        if __name__ == "__main__":
            with usb1.USBContext() as context:
                list_devices(context)

    Run the file with:

    .. code-block :: sh

        python ./test-gateware-usb-device.py

    And, if the device is recognized, you should see a line like:

    .. code-block :: sh
        :emphasize-lines: 4

        Bus 000 Device 001: ID 1d5c:5010:  Fresco Logic, Inc. - USB2.0 Hub
        Bus 000 Device 002: ID 1d5c:5000:  Fresco Logic, Inc. - USB3.0 Hub
        Bus 000 Device 003: ID 1d50:615c:  Great Scott Gadgets - Cynthion Apollo Debugger
        Bus 000 Device 007: ID 1209:0001:  Cynthion Project - Gateware USB Device


Conclusion
==========

Our device can now be enumerated by Microsoft Windows but it can't actually do anything yet. In the next part
we'll learn how to add Vendor Request Handlers to our device that allow it to receive and respond to control requests from the host: :doc:`/tutorials/gateware_usb_device_03`


Exercises
=========

* Modify the example to use a different request code, does it still work?
* Could you use the information you learnt in this tutorial modify the LUNA `ACM Serial example <https://github.com/greatscottgadgets/luna/blob/main/luna/gateware/usb/devices/acm.py>`__ example to support Windows?
* Modify the ``PropertyData`` field of the extended properties descriptor to one of the `Microsoft-provided USB device class drivers <https://learn.microsoft.com/en-us/windows-hardware/drivers/usbcon/supported-usb-classes>`__. What happens?


More information
================

* Pete Batard's excellent introduction to `WCID Devices <https://github.com/pbatard/libwdi/wiki/WCID-Devices>`__.
* `Microsoft OS 1.0 Descriptors Specification <https://learn.microsoft.com/en-us/windows-hardware/drivers/usbcon/microsoft-os-1-0-descriptors-specification>`__.
* `Microsoft OS 2.0 Descriptors Specification <https://learn.microsoft.com/en-us/windows-hardware/drivers/usbcon/microsoft-os-2-0-descriptors-specification>`__.
* Microsoft `USB device class drivers included in Windows <https://learn.microsoft.com/en-us/windows-hardware/drivers/usbcon/supported-usb-classes>`__.
* Microsoft `System-defined device setup classes available to vendors <https://learn.microsoft.com/en-us/windows-hardware/drivers/install/system-defined-device-setup-classes-available-to-vendors>`__.


Source Code
===========

.. literalinclude:: ../../../cynthion/python/examples/tutorials/gateware-usb-device-02.py
    :caption: gateware-usb-device-02.py
    :language: python
    :linenos:


.. literalinclude:: ../../../cynthion/python/examples/tutorials/test-gateware-usb-device-02.py
    :caption: test-gateware-usb-device-02.py
    :language: python
    :linenos:
