#
# This file is part of Cynthion
#

"""
Module containing the core definitions for a Cynthion board.
"""

import string
from weakref import WeakSet

from pygreat.board import GreatBoard

from .interfaces import led, gpio

from .shared import usb

# Default device identifiers.
CYNTHION_VENDOR_ID  = usb.bVendorId.cynthion
CYNTHION_PRODUCT_ID = usb.bProductId.cynthion

# libgreat backend interface subclass
LIBGREAT_INTERFACE_SUBCLASS = usb.bInterfaceSubClass.moondancer

# libgreat backend interface protocol version
LIBGREAT_INTERFACE_PROTOCOL = usb.bInterfaceProtocol.moondancer

# Quirk constant that helps us identify libusb's pipe errors, which bubble
# up as generic USBErrors with errno 32 on affected platforms.
LIBUSB_PIPE_ERROR = 32

# Total seconds we should wait after a reset before reconnecting.
RECONNECT_DELAY = 3


class CynthionBoard(GreatBoard):
    """
    Class describing Cynthion devices.
    """

    # Default device identifiers.
    BOARD_VENDOR_ID = CYNTHION_VENDOR_ID
    BOARD_PRODUCT_ID = CYNTHION_PRODUCT_ID

    """
    The mappings from GPIO names to port numbers. Paths in names can be delineated
    with underscores to group gpios. For example, if Jumper 7, Pin 3 is Port 5, Pin 11,
    you could add an entry that reads "J7_P3": (5, 11).
    """
    GPIO_MAPPINGS = {}

    # FIXME: should these interfaces be in libgreat?

    #
    # Quick way to add simple Python wrappers around comms classes.
    # Create an entry for the relevant comms class, and provide a tuple of
    #   (attr_name, peripheral_class).
    #
    SIMPLE_CLASS_MAPPINGS = {
        # TODO 'firmware': ('onboard_flash', DeviceFirmwareManager),
        'gpio': ('gpio', gpio.GPIO),
    }

    def __init__(self, *args, **device_identifiers):
        """ Initialize a new CynthionBoard instance with our additional properties. """

        # Create a new list of interfaces and programmers.
        self._interfaces = []
        self._instantiated_programmers = WeakSet()

        # Add libgreat usb interface protocol to device identifiers.
        device_identifiers['interface_subclass'] = LIBGREAT_INTERFACE_SUBCLASS
        device_identifiers['interface_protocol'] = LIBGREAT_INTERFACE_PROTOCOL

        super(CynthionBoard, self).__init__(*args, **device_identifiers)


    def available_interfaces(self):
        """ Returns a list of peripheral properties that exist on this board. """
        return self._interfaces[:]


    def _populate_leds(self, led_count):
        """Adds the standard set of LEDs to the board object.

        Args:
            led_count -- The number of LEDS present on the board.
        """

        self._add_interface('leds', {})

        for i in range(0, led_count):
            self.leds[i] = led.LED(self, i)


    def _populate_gpio(self):
        """ Adds GPIO pin definitions to the board's main GPIO object. """

        # Handle each GPIO mapping.
        for name, pin in self.GPIO_MAPPINGS.items():
            self.gpio.register_gpio(name, pin)


    def _add_interface(self, name, instance):
        """
        Adds a peripheral to the Cynthion object. Prefer this over adding attributes directly,
        as it adds peripherals to a list that can be queried by the user.

        Arguments:
            name -- The name of the attribute to add to this board. "i2c" would create a
                .i2c property on this board.
            instance -- The object to add as that property.
        """

        self._interfaces.append(name)
        setattr(self, name, instance)


    def _add_simple_interface(self, name, cls, *args, **kwargs):
        """ Adds a given interface to this board.

        Arguments:
            name -- The attribute name to be added to the board.
            cls -- The class to be instantiated to create the given object.
        """

        # Create an instance of the relevant peripheral class...
        instance = cls(self, *args, **kwargs)

        # ... and add it to this board.
        self._add_interface(name, instance)



    def _populate_simple_interfaces(self):
        """ Adds simple interfaces to the board object by parsing the SIMPLE_CLASS_MAPPINGS dictionary. """

        for comms_class, interface in self.SIMPLE_CLASS_MAPPINGS.items():

            # If the relevant API is supported, add the relevant peripheral.
            if self.supports_api(comms_class):
                name, python_class = interface
                self._add_simple_interface(name, python_class)

    def available_programmers(self, as_dictionary=False):
        """ Returns the list of available programmers. """

        from types import ModuleType

        programmers = {}


        for module in ProgrammerModules.__dict__.values():
            if isinstance(module, ModuleType) and hasattr(module, 'create_programmer'):
                module_name = module.__name__.split('.')[-1]
                programmers[module_name] = module

        if as_dictionary:
            return programmers
        else:
            return list(programmers.values())


    def create_programmer(self, name, *args, **kwargs):
        """ Creates a new instance of the programmer with the given name. """

        try:
            programmer_module = self.available_programmers(True)[name]
            programmer = programmer_module.create_programmer(self, *args, **kwargs)

            # Keep a weak reference to the relevant programmer.
            # This is useful for re-attaching programmers after a disconnect.
            self._instantiated_programmers.add(programmer)

            # Finally, return the created programmer.
            return programmer

        except KeyError:
            raise KeyError("no available programmer named {}".format(name))


    def __dir__(self):
        """ Generate a cleaned-up dir listing for the relevant board. """

        items = super(CynthionBoard, self).__dir__()
        return [item for item in items if item[0] in string.ascii_lowercase]
