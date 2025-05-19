#!/usr/bin/env python3
# pylint: disable=maybe-no-member
#
# This file is part of Cynthion.
#
# Copyright (c) 2020-2023 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

""" Generic USB analyzer backend generator for LUNA. """

import time
import errno


import usb
from datetime import datetime
from enum import IntEnum, IntFlag

from amaranth                            import Signal, Elaboratable, Module, DomainRenamer, ResetInserter
from amaranth.build.res                  import ResourceError
from usb_protocol.emitters               import DeviceDescriptorCollection
from usb_protocol.types                  import USBRequestType, USBRequestRecipient

from luna.usb2                           import USBDevice, USBStreamInEndpoint
from luna                                import top_level_cli

from luna.gateware.usb.request.control   import ControlRequestHandler
from luna.gateware.usb.stream            import USBInStreamInterface
from luna.gateware.stream.generator      import StreamSerializer
from luna.gateware.architecture.car      import LunaECP5DomainGenerator
from luna.gateware.architecture.flash_sn import ECP5FlashUIDStringDescriptor
from luna.gateware.interface.ulpi        import UTMITranslator
from luna.gateware.usb.usb2.control      import USBControlEndpoint
from luna.gateware.usb.request.standard  import StandardRequestHandler
from luna.gateware.usb.request.windows   import MicrosoftOS10DescriptorCollection, MicrosoftOS10RequestHandler

from apollo_fpga.gateware.advertiser     import ApolloAdvertiser, ApolloAdvertiserRequestHandler

from usb_protocol.emitters.descriptors.standard import get_string_descriptor
from usb_protocol.types.descriptors.microsoft10 import RegistryTypes

from .analyzer                           import USBAnalyzer
from .fifo                               import Stream16to8, StreamFIFO, AsyncFIFOReadReset, HyperRAMPacketFIFO

import cynthion


USB_SPEED_HIGH       = 0b00
USB_SPEED_FULL       = 0b01
USB_SPEED_LOW        = 0b10

USB_VENDOR_ID        = cynthion.shared.usb.bVendorId.cynthion
USB_PRODUCT_ID       = cynthion.shared.usb.bProductId.cynthion

BULK_ENDPOINT_NUMBER  = 1
BULK_ENDPOINT_ADDRESS = 0x80 | BULK_ENDPOINT_NUMBER
MAX_BULK_PACKET_SIZE  = 512


class USBAnalyzerRegister(Elaboratable):

    def __init__(self, reset=0x00):
        self.current = Signal(8, reset=reset)
        self.next = Signal(8)
        self.write = Signal()

    def elaborate(self, platform):
        m = Module()
        with m.If(self.write):
            m.d.sync += self.current.eq(self.next)
        return m


class USBAnalyzerVendorRequests(IntEnum):
    GET_STATE = 0
    SET_STATE = 1
    GET_SPEEDS = 2
    SET_TEST_CONFIG = 3


class USBAnalyzerSupportedSpeeds(IntFlag):
    USB_SPEED_AUTO = 0b0001
    USB_SPEED_LOW  = 0b0010
    USB_SPEED_FULL = 0b0100
    USB_SPEED_HIGH = 0b1000


class USBAnalyzerVendorRequestHandler(ControlRequestHandler):

    def __init__(self, state, test_config):
        self.state = state
        self.test_config = test_config
        super().__init__()

    def elaborate(self, platform):
        m = Module()
        interface = self.interface

        # Create convenience aliases for our interface components.
        setup               = interface.setup
        handshake_generator = interface.handshakes_out

        # Transmitter for small-constant-response requests
        m.submodules.transmitter = transmitter = \
            StreamSerializer(data_length=1, domain="usb", stream_type=USBInStreamInterface, max_length_width=1)

        # Handle vendor requests to our interface.
        with m.If(
                (setup.type == USBRequestType.VENDOR) &
                (setup.recipient == USBRequestRecipient.INTERFACE) &
                (setup.index == 0)):

            m.d.comb += interface.claim.eq(
                (setup.request == USBAnalyzerVendorRequests.GET_STATE) |
                (setup.request == USBAnalyzerVendorRequests.SET_STATE) |
                (setup.request == USBAnalyzerVendorRequests.GET_SPEEDS)|
                (setup.request == USBAnalyzerVendorRequests.SET_TEST_CONFIG))

            with m.FSM(domain="usb"):

                # IDLE -- not handling any active request
                with m.State('IDLE'):

                    # If we've received a new setup packet, handle it.
                    with m.If(setup.received):

                        # Select which vendor we're going to handle.
                        with m.Switch(setup.request):

                            with m.Case(USBAnalyzerVendorRequests.GET_STATE):
                                m.next = 'GET_STATE'
                            with m.Case(USBAnalyzerVendorRequests.SET_STATE):
                                m.next = 'SET_STATE'
                            with m.Case(USBAnalyzerVendorRequests.GET_SPEEDS):
                                m.next = 'GET_SPEEDS'
                            with m.Case(USBAnalyzerVendorRequests.SET_TEST_CONFIG):
                                m.next = 'SET_TEST_CONFIG'

                # GET_STATE -- Fetch the device's state
                with m.State('GET_STATE'):
                    self.handle_simple_data_request(m, transmitter, self.state.current, length=1)

                # SET_STATE -- The host is trying to set our state
                with m.State('SET_STATE'):
                    self.handle_register_write_request(m, self.state.next, self.state.write)

                # GET_SPEEDS -- Fetch the device's supported USB speeds
                with m.State('GET_SPEEDS'):
                    supported_speeds = \
                        USBAnalyzerSupportedSpeeds.USB_SPEED_LOW | \
                        USBAnalyzerSupportedSpeeds.USB_SPEED_FULL | \
                        USBAnalyzerSupportedSpeeds.USB_SPEED_HIGH
                    self.handle_simple_data_request(m, transmitter, supported_speeds, length=1)

                # SET_TEST_CONFIG -- The host is trying to configure our test device
                with m.State('SET_TEST_CONFIG'):
                    self.handle_register_write_request(m, self.test_config.next, self.test_config.write)

        return m


class USBAnalyzerApplet(Elaboratable):
    """ Gateware that serves as a generic USB analyzer backend.

    WARNING: This is _incomplete_! It's missing:
        - DRAM backing for analysis
    """

    def create_descriptors(self, platform, sharing):
        """ Create the descriptors we want to use for our device. """

        major, minor = platform.version
        descriptors = DeviceDescriptorCollection()

        #
        # We'll add the major components of the descriptors we we want.
        # The collection we build here will be necessary to create a standard endpoint.
        #

        # We'll need a device descriptor...
        with descriptors.DeviceDescriptor() as d:
            d.idVendor           = USB_VENDOR_ID
            d.idProduct          = USB_PRODUCT_ID

            d.iManufacturer      = "Cynthion Project"
            d.iProduct           = "USB Analyzer"
            d.iSerialNumber      = ECP5FlashUIDStringDescriptor
            d.bcdDevice          = major + (minor * 0.01)

            d.bNumConfigurations = 1


        # ... and a description of the USB configuration we'll provide.
        with descriptors.ConfigurationDescriptor() as c:

            with c.InterfaceDescriptor() as i:
                i.bInterfaceNumber = 0
                i.bInterfaceClass = 0xFF
                i.bInterfaceSubclass = cynthion.shared.usb.bInterfaceSubClass.analyzer
                i.bInterfaceProtocol = cynthion.shared.usb.bInterfaceProtocol.analyzer

                with i.EndpointDescriptor() as e:
                    e.bEndpointAddress = BULK_ENDPOINT_ADDRESS
                    e.wMaxPacketSize   = MAX_BULK_PACKET_SIZE

            # Include Apollo stub interface, if using a shared port.
            if sharing is not None:
                with c.InterfaceDescriptor() as i:
                    i.bInterfaceNumber = 1
                    i.bInterfaceClass = 0xFF
                    i.bInterfaceSubclass = cynthion.shared.usb.bInterfaceSubClass.apollo
                    i.bInterfaceProtocol = ApolloAdvertiserRequestHandler.PROTOCOL_VERSION

        return descriptors


    def elaborate(self, platform):
        m = Module()

        # State register
        m.submodules.state = state = USBAnalyzerRegister()

        # Test config register
        m.submodules.test_config = test_config = USBAnalyzerRegister(reset=0x01)

        # Generate our clock domains.
        clocking = LunaECP5DomainGenerator()
        m.submodules.clocking = clocking

        # Create our UTMI translator.
        ulpi = platform.request("target_phy")
        m.submodules.utmi = utmi = UTMITranslator(ulpi=ulpi)

        # Strap our power controls to be in VBUS passthrough by default,
        # on the target port.
        if platform.version >= (0, 6):
            # On Cynthion r1.4, Target-C to Target-A VBUS passthrough is
            # off by default and must be enabled by the gateware.
            m.d.comb += [
                platform.request("target_c_vbus_en").o  .eq(1),
            ]
            # On Cynthion r0.6 - r1.3 this passthrough is enabled by
            # default, even with the hardware unpowered, but it does no
            # harm to explicitly set it here.
        else:
            # On Cynthion r0.1 - r0.5, there is no `target_c_vbus_en`
            # signal. The following two signals are needed to have
            # the same effect:
            m.d.comb += [
                platform.request("power_a_port").o      .eq(0),
                platform.request("pass_through_vbus").o .eq(1),
            ]

        # Set up our parameters.
        m.d.comb += [

            # Set PHY mode to non-driving as we want to passively observe.
            #
            # `dp_pulldown`, `dm_pulldown` and `term_select` do not need to be
            # configured as these values are "don't cares" for this specific
            # `op_mode` (see ULPI Specification rev. 1.1 Table 41).
            utmi.op_mode     .eq(0b01),
            utmi.xcvr_select .eq(state.current[1:3]),
        ]

        # Select the appropriate PHY according to platform version.
        if platform.version >= (0, 6):
            phy_name = "control_phy"

            # Also set up a test device on the AUX PHY.
            m.submodules += AnalyzerTestDevice(test_config)
        else:
            phy_name = "host_phy"

        # Check how the port is shared with Apollo.
        sharing = platform.port_sharing(phy_name)

        # Create our USB uplink interface...
        uplink_ulpi = platform.request(phy_name)
        m.submodules.usb = usb = USBDevice(bus=uplink_ulpi)

        # Create descriptors.
        descriptors = self.create_descriptors(platform, sharing)

        # Add Microsoft OS 1.0 descriptors for Windows compatibility.
        descriptors.add_descriptor(get_string_descriptor("MSFT100\xee"), index=0xee)
        msft_descriptors = MicrosoftOS10DescriptorCollection()
        with msft_descriptors.ExtendedCompatIDDescriptor() as c:
            with c.Function() as f:
                f.bFirstInterfaceNumber = 0
                f.compatibleID          = 'WINUSB'
            if sharing is not None:
                with c.Function() as f:
                    f.bFirstInterfaceNumber = 1
                    f.compatibleID          = 'WINUSB'
        with msft_descriptors.ExtendedPropertiesDescriptor() as d:
            with d.Property() as p:
                p.dwPropertyDataType = RegistryTypes.REG_SZ
                p.PropertyName       = "DeviceInterfaceGUID"
                p.PropertyData       = "{88bae032-5a81-49f0-bc3d-a4ff138216d6}"

        # Add our standard control endpoint to the device.
        control_endpoint = usb.add_standard_control_endpoint(descriptors, avoid_blockram=True)

        # Add handler for Microsoft descriptors.
        msft_handler = MicrosoftOS10RequestHandler(msft_descriptors, request_code=0xee)
        control_endpoint.add_request_handler(msft_handler)

        # Add our vendor request handler to the control endpoint.
        vendor_request_handler = USBAnalyzerVendorRequestHandler(state, test_config)
        control_endpoint.add_request_handler(vendor_request_handler)

        # If needed, create an advertiser and add its request handler.
        if sharing == "advertising":
            adv = m.submodules.adv = ApolloAdvertiser()
            control_endpoint.add_request_handler(adv.default_request_handler(1))

        # Add a stream endpoint to our device.
        stream_ep = USBStreamInEndpoint(
            endpoint_number=BULK_ENDPOINT_NUMBER,
            max_packet_size=MAX_BULK_PACKET_SIZE
        )
        usb.add_endpoint(stream_ep)

        # Create a USB analyzer.
        m.submodules.analyzer = analyzer = USBAnalyzer(utmi_interface=utmi)

        # Follow this with a HyperRAM FIFO for additional buffering.
        reset_on_start = ResetInserter(analyzer.discarding)
        m.submodules.psram_fifo = psram_fifo = reset_on_start(
            HyperRAMPacketFIFO(out_fifo_depth=128))

        # Convert the 16-bit stream into an 8-bit one for output.
        m.submodules.s16to8 = s16to8 = reset_on_start(Stream16to8())

        # Add a special stream clock converter for 'sync' to 'usb' crossing.
        m.submodules.clk_conv = clk_conv = StreamFIFO(
            AsyncFIFOReadReset(width=8, depth=4, r_domain="usb", w_domain="sync"))

        m.d.comb += [
            # Connect enable signal to host-controlled state register.
            analyzer.capture_enable     .eq(state.current[0]),

            # Flush endpoint when analyzer is idle with capture disabled.
            stream_ep.flush             .eq(analyzer.idle & ~analyzer.capture_enable),

            # Discard data buffered by endpoint when the analyzer discards its data.
            stream_ep.discard           .eq(analyzer.discarding),

            # USB stream pipeline.
            psram_fifo.input            .stream_eq(analyzer.stream),
            s16to8.input                .stream_eq(psram_fifo.output),
            clk_conv.input              .stream_eq(s16to8.output),
            clk_conv.fifo.ext_rst       .eq(analyzer.discarding),
            stream_ep.stream            .stream_eq(clk_conv.output),

            usb.connect                 .eq(1),

            # LED indicators.
            platform.request("led", 0).o  .eq(analyzer.capturing),
            platform.request("led", 1).o  .eq(stream_ep.stream.valid),
            platform.request("led", 2).o  .eq(analyzer.overrun),

            platform.request("led", 3).o  .eq(utmi.session_valid),
            platform.request("led", 4).o  .eq(utmi.rx_active),
            platform.request("led", 5).o  .eq(utmi.rx_error),
        ]

        # Return our elaborated module.
        return m


class AnalyzerTestDevice(Elaboratable):
    """ Built-in example device that can be used to test the analyzer. """

    SPEEDS = (USB_SPEED_HIGH, USB_SPEED_FULL, USB_SPEED_LOW)

    EP0_MAX_SIZE = {
        USB_SPEED_HIGH: 64,
        USB_SPEED_FULL: 64,
        USB_SPEED_LOW: 8,
    }

    INT_EP_MAX_SIZE = {
        USB_SPEED_HIGH: 512,
        USB_SPEED_FULL: 64,
        USB_SPEED_LOW: 8,
    }

    INT_EP_NUM = {
        USB_SPEED_HIGH: 1,
        USB_SPEED_FULL: 2,
        USB_SPEED_LOW: 3,
    }

    def __init__(self, config):
        self.config = config

    def create_descriptors(self, speed):
        descriptors = DeviceDescriptorCollection()

        with descriptors.DeviceDescriptor() as d:
            d.idVendor           = cynthion.shared.usb.bVendorId.example
            d.idProduct          = cynthion.shared.usb.bProductId.analyzer_test
            d.iManufacturer      = "Cynthion Project"
            d.iProduct           = "USB Analyzer Test Device"
            d.bcdDevice          = 0.01
            d.bNumConfigurations = 1
            d.bMaxPacketSize0    = self.EP0_MAX_SIZE[speed]

        with descriptors.ConfigurationDescriptor() as c:
            with c.InterfaceDescriptor() as i:
                i.bInterfaceNumber = 0
                with i.EndpointDescriptor() as e:
                    e.bEndpointAddress = 0x80 | self.INT_EP_NUM[speed]
                    e.bmAttributes     = 0x03 # Interrupt endpoint
                    e.wMaxPacketSize   = self.INT_EP_MAX_SIZE[speed]
                    e.bInterval        = 0x05 # 5ms interval

        descriptors.add_descriptor(
                get_string_descriptor("MSFT100\xee"), index=0xee)

        return descriptors

    def elaborate(self, platform):
        m = Module()

        # Create a USB device and connect it as required.
        m.submodules.usb = usb = USBDevice(bus=platform.request("aux_phy"))
        current_speed = self.config.current[1:3]
        m.d.comb += [
            usb.connect.eq(self.config.current[0]),
            usb.low_speed_only.eq(current_speed == USB_SPEED_LOW),
            usb.full_speed_only.eq(current_speed == USB_SPEED_FULL),
        ]

        # Create control endpoint.
        control_ep = USBControlEndpoint(utmi=usb.utmi)

        # Add standard request handlers for each speed.
        for speed in self.SPEEDS:
            handler = StandardRequestHandler(
                self.create_descriptors(speed),
                self.EP0_MAX_SIZE[speed],
                avoid_blockram=True,
                blacklist=[lambda setup,speed=speed: current_speed != speed])
            control_ep.add_request_handler(handler)

        # Add Microsoft descriptors for Windows compatibility.
        msft_descriptors = MicrosoftOS10DescriptorCollection()
        with msft_descriptors.ExtendedCompatIDDescriptor() as c:
            with c.Function() as f:
                f.bFirstInterfaceNumber = 0
                f.compatibleID          = 'WINUSB'

        # Add handler for Microsoft descriptors.
        msft_handler = MicrosoftOS10RequestHandler(
                msft_descriptors, request_code=0xee)
        control_ep.add_request_handler(msft_handler)

        # Add control endpoint.
        usb.add_endpoint(control_ep)

        # Add IN endpoints for each speed.
        for speed in self.SPEEDS:
            in_ep = USBStreamInEndpoint(
                endpoint_number=self.INT_EP_NUM[speed],
                max_packet_size=self.INT_EP_MAX_SIZE[speed])
            usb.add_endpoint(in_ep)

            # Output a counter to the endpoint.
            counter = Signal(8)
            m.d.comb += [
                in_ep.stream.valid.eq(1),
                in_ep.stream.payload.eq(counter),
            ]
            with m.If(in_ep.stream.ready):
                m.d.usb += counter.eq(counter + 1)

        return m


if __name__ == "__main__":
    top_level_cli(USBAnalyzerApplet)
