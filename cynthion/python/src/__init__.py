from __future__ import print_function
# Alias objects to make them easier to import.

from .cynthion import Cynthion
from .cynthion import CynthionSingleton
from .cynthion import CynthionBoard

from . import gateware
from . import shared


Cynthion = Cynthion  # pyflakes


class _CynthionSingletonWrapper(object):

    """
    Convenience function that acts like CynthionSingleton, but also allows Magic:
    accessing a property on this object will act as though that property had been
    accessed on a result of a CynthionSingleton() call.

    That's heckin' unreadable, so in short-- accessing a property on a relevant object
    will attempt to 1) call the property on the sanest existing Cynthion object; or
    2) create a new Cynthion object, if necessary.
    """

    def __init__(self, serial=None):
        self.serial = serial

    def __getitem__(self, serial):
        return _CynthionSingletonWrapper(serial)

    def __getattr__(self, name):
        return getattr(CynthionSingleton(self.serial), name)

    def __call__(self, serial=None):
        return CynthionSingleton(serial)

    def __dir__(self):
        return dir(CynthionSingleton(self.serial))

CynthionSingleton = _CynthionSingletonWrapper()


# TODO deprecate in favor of:
#
#   from importlib.resources import files
#   assets_directory = files("cynthion").joinpath("assets")
#
def cynthion_assets_directory():
    """ Provide a quick function that helps us get at our assets directory. """
    import os

    # Find the path to the module, and then find its assets folder.
    module_path = os.path.dirname(__file__)
    return os.path.join(module_path, 'assets')


def find_cynthion_asset(filename):
    """ Returns the path to a given Cynthion asset, if it exists, or None if the Cynthion asset isn't provided."""
    import os

    asset_path = os.path.join(cynthion_assets_directory(), filename)

    if os.path.isfile(asset_path):
        return asset_path
    else:
        return None
