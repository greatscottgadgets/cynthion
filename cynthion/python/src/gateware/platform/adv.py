#
# This file is part of Cynthion.
#
# Copyright (c) 2020-2023 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

from collections import defaultdict

from apollo_fpga.gateware import ApolloAdvertiser
from luna.gateware.usb.usb2.control import USBControlEndpoint
from luna.gateware.usb.request.standard import StandardRequestHandler

from usb_protocol.types.descriptors.standard import StandardDescriptorNumbers, ConfigurationDescriptor, InterfaceDescriptor
from usb_protocol.emitters import DeviceDescriptorCollection


def control_phy_hook(usb_device, m):

    # Add ApolloAdvertiser submodule
    m.submodules.apollo_adv = adv = ApolloAdvertiser()

    # Find control endpoint or create one
    for endpoint in usb_device._endpoints:
        if isinstance(endpoint, USBControlEndpoint):
            control_ep = endpoint
            break
    else:
        control_ep = usb_device.add_control_endpoint()

    # Find standard request handler and modify the device descriptors
    for handler in control_ep._request_handlers:
        if isinstance(handler, StandardRequestHandler):
            # Inject additional Apollo interface
            intf_number = add_apollo_interface(handler.descriptors)
            break
    else:
        descriptors = DeviceDescriptorCollection()
        intf_number = add_apollo_interface(descriptors)
        control_ep.add_standard_request_handlers(descriptors)
    
    # Add request handler
    control_ep.add_request_handler(adv.default_request_handler(intf_number))


def add_apollo_interface(descriptors):
    ''' Inject an Apollo interface in the provided device descriptor '''
    #
    # Extract information from the previous configuration descriptor, if it exists
    #
    intf_descriptors = []
    intf_assigned    = defaultdict(bool)
    try:
        raw_bytes = descriptors.get_descriptor_bytes(StandardDescriptorNumbers.CONFIGURATION)
    except KeyError:
        conf_values = None
    else:
        # Parse configuration descriptor
        conf_values = ConfigurationDescriptor.parse(raw_bytes)
        raw_bytes = raw_bytes[conf_values.bLength:]

        # Store subordinate interface descriptors
        for i in range(conf_values.bNumInterfaces):
            
            # Parse descriptor information
            intf_values = InterfaceDescriptor.parse(raw_bytes)

            # Store the slice of bytes, including endpoints
            descr_len = raw_bytes[0]
            for i in range(intf_values.bNumEndpoints):
                descr_len += raw_bytes[descr_len]
            intf_descriptors.append(raw_bytes[:descr_len])
            raw_bytes = raw_bytes[descr_len:]
            
            # Flag interface number as assigned / occupied
            intf_assigned[intf_values.bInterfaceNumber] = True

    # Choose the lower, unassigned interface number
    intf_number = min(n for n in range(256) if not intf_assigned[n])

    #
    # Rebuild configuration descriptor with the new interface
    #
    with descriptors.ConfigurationDescriptor() as c:

        # Only set fields that won't be modified later
        if conf_values is not None:
            for field in ('bConfigurationValue', 'iConfiguration', 'bmAttributes', 'bMaxPower'):
                setattr(c, field, conf_values[field])

        # Add the previous interface descriptors along with our new Apollo interface
        for interface in intf_descriptors:
            c.add_subordinate_descriptor(interface)
        with c.InterfaceDescriptor() as i:
            i.bInterfaceNumber   = intf_number
            i.bInterfaceClass    = 0xff
            i.bInterfaceSubclass = 0x00  # Apollo interface

    return intf_number
