#
# This file is part of Cynthion
#

import time

class CynthionInterface(object):
    """
    Generic base class for Cynthion peripherals.
    """


    def __init__(self, device):
        """ Default peripheral initializer -- just stores a reference to the relevant Cynthion. """

        self.device = device
