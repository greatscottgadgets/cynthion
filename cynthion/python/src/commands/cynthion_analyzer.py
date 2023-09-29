#!/usr/bin/env python3
#
# This file is part of Cynthion
#
# Copyright (c) 2023 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

""" Utility for accessing Cynthion's USB analyzer. """

import argparse
import errno
import inspect
import logging
import os
import sys
import textwrap

from cynthion import Cynthion


def main():
    # Set up python's logging to act as a simple print, for now.
    logging.basicConfig(level=logging.INFO, format="%(message)-s")

    # Set up a simple argument parser.
    parser = argparse.ArgumentParser(
        description="Utility for accessing Cynthion's USB analyzer.")
    parser.add_argument('-v', '--version', dest='version', action='store_true',
                        help="Prints version information for Cynthion's USB analyzer.")
    args = parser.parse_args()


if __name__ == '__main__':
    main()
