#!/usr/bin/env python
#
# This file is part of Cynthion.
#
# Copyright (c) 2023 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

""" Cynthion 'run' command. """

import logging, os, platform, subprocess, sys, tempfile

from .util  import find_cynthion_asset, find_cynthion_bitstream
from .util  import flash_soc_firmware, run_bitstream


def cynthion_run(device, args):
    if args.bitstream is not None:
        run_bitstream(device, args.bitstream)

    elif args.target == "analyzer":
        run_bitstream(device, find_cynthion_bitstream(device, f"analyzer.bit"))

    elif args.target == "facedancer":
        if platform.system() == "Windows":
            logging.error("\nFacedancer and USBProxy are not currently supported on Windows.")
            logging.error("Attempting to use Facedancer or USBProxy on Windows may cause")
            logging.error("USB analysis to stop working.\n")
            logging.error("For more information please see the tracking issue:\n")
            logging.error("  https://github.com/greatscottgadgets/cynthion/issues/170\n")
            logging.error("Command aborted.")
            sys.exit(1)
        flash_soc_firmware(device, find_cynthion_asset("moondancer.bin"))
        run_bitstream(device, find_cynthion_bitstream(device, f"facedancer.bit"))

    elif args.target == "selftest":
        _run_selftest(device, args)

    else:
        logging.error(f"unknown target: {args.target}")
        return


def _run_selftest(device, args):
    from cynthion.selftest.gateware import SelftestDevice
    from cynthion.selftest.host import StandaloneTester

    run_bitstream(device, find_cynthion_bitstream(device, f"selftest.bit"))
    device.close()

    selftest_device = SelftestDevice()

    tester = StandaloneTester(selftest_device)
    tester.run_tests()
    print()
