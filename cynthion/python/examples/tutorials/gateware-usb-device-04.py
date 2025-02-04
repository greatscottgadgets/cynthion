#!/usr/bin/env python3
#
# This file is part of Cynthion.
#
# Copyright (c) 2024 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

from amaranth                                    import *
from amaranth.lib.fifo                           import SyncFIFO
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

from luna.usb2                                   import USBStreamInEndpoint, USBStreamOutEndpoint
from usb_protocol.types                          import USBDirection, USBTransferType

VENDOR_ID  = 0x1209 # https://pid.codes/1209/
PRODUCT_ID = 0x0001

MAX_PACKET_SIZE = 512

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


class GatewareUSBDevice(Elaboratable):
    """ A simple USB device that can communicate with the host via vendor and bulk requests. """

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

                # an endpoint for receiving bulk data from the host
                with i.EndpointDescriptor() as e:
                    e.bEndpointAddress = USBDirection.OUT.to_endpoint_address(0x01) # EP 0x01 OUT
                    e.bmAttributes     = USBTransferType.BULK
                    e.wMaxPacketSize   = MAX_PACKET_SIZE

                # and an endpoint for transmitting bulk data to the host
                with i.EndpointDescriptor() as e:
                    e.bEndpointAddress = USBDirection.IN.to_endpoint_address(0x02)  # EP 0x82 IN
                    e.bmAttributes     = USBTransferType.BULK
                    e.wMaxPacketSize   = MAX_PACKET_SIZE

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


if __name__ == "__main__":
    from luna import top_level_cli
    top_level_cli(GatewareUSBDevice)
