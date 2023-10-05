#!/usr/bin/env python3
#
# This file is part of Cynthion.
#
# Copyright (c) 2023 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

import logging, os, sys, traceback, unittest

import cynthion

class TestSharedValues(unittest.TestCase):
    """Tests for values shared via `cynthion.git/shared/`"""

    def test_usb(self):
        self.assertEqual(cynthion.shared.usb.bVendorId.cynthion, 0x1d50)
        self.assertEqual(cynthion.shared.usb.bProductId.cynthion, 0x615b)

if __name__ == "__main__":
    unittest.main()
