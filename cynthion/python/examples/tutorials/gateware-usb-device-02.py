#!/usr/bin/env python3
#
# This file is part of Cynthion.
#
# Copyright (c) 2024 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

from amaranth                                    import *
from luna.usb2                                   import USBDevice
from usb_protocol.emitters                       import DeviceDescriptorCollection

from luna.gateware.usb.request.windows           import (
    MicrosoftOS10DescriptorCollection,
    MicrosoftOS10RequestHandler,
)
from usb_protocol.emitters.descriptors.standard  import get_string_descriptor
from usb_protocol.types.descriptors.microsoft10  import RegistryTypes

VENDOR_ID  = 0x1209 # https://pid.codes/1209/
PRODUCT_ID = 0x0001

class GatewareUSBDevice(Elaboratable):
    """ A simple USB device that can also enumerate on Windows. """

    def create_standard_descriptors(self):
        """ Create the USB descriptors for the device. """

        descriptors = DeviceDescriptorCollection()

        # all USB devices have a single device descriptor
        with descriptors.DeviceDescriptor() as d:
            d.idVendor           = VENDOR_ID
            d.idProduct          = PRODUCT_ID
            d.iManufacturer      = "Cynthion Project"
            d.iProduct           = "Gateware USB Device"

            d.bNumConfigurations = 1

        # and at least one configuration descriptor
        with descriptors.ConfigurationDescriptor() as c:

            # with at least one interface descriptor
            with c.InterfaceDescriptor() as i:
                i.bInterfaceNumber = 0

                # interfaces also need endpoints to do anything useful
                # but we'll add those later!

        return descriptors


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
            # required for the Microsoft OS string descriptor at 0xEE
            avoid_blockram=True,
        )

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


if __name__ == "__main__":
    from luna import top_level_cli
    top_level_cli(GatewareUSBDevice)
