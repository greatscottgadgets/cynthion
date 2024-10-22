#!/usr/bin/env python3
#
# This file is part of Cynthion.
#
# Copyright (c) 2024 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

from amaranth               import *
from luna.usb2              import USBDevice
from usb_protocol.emitters  import DeviceDescriptorCollection

VENDOR_ID  = 0x1209 # https://pid.codes/1209/
PRODUCT_ID = 0x0001

class GatewareUSBDevice(Elaboratable):
    """ A simple USB device that can only enumerate. """

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
        control_endpoint = usb.add_standard_control_endpoint(descriptors)

        # configure the device to connect by default when plugged into a host
        m.d.comb += usb.connect.eq(1)

        return m


if __name__ == "__main__":
    from luna import top_level_cli
    top_level_cli(GatewareUSBDevice)
