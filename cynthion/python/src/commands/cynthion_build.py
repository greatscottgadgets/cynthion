#!/usr/bin/env python
#
# This file is part of Cynthion.
#
# Copyright (c) 2023 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

""" Cynthion 'build' command. """

import logging, os, subprocess, sys, tempfile

from . import util


def cynthion_build(device, args):
    if args.luna_platform:
        platform_name = util._get_appropriate_platform_name(device)
        sys.stdout.write(f"cynthion.gateware.platform:{platform_name}\n")
        sys.exit(0)

    elif args.target == "analyzer":
        _build_analyzer(device, args)
    elif args.target == "facedancer":
        _build_facedancer(device, args)
    elif args.target == "selftest":
        _build_selftest(device, args)
    else:
        logging.error(f"unknown target: {args.target}")


def _build_analyzer(device, args):
    from luna import top_level_cli
    from cynthion.gateware.analyzer.top import USBAnalyzerApplet
    sys.argv = [sys.argv[0]]
    top_level_cli(USBAnalyzerApplet)


def _build_facedancer(device, args):
    from luna_soc import top_level_cli
    from cynthion.gateware.facedancer.top import MoondancerSoc

    with tempfile.TemporaryDirectory() as tmp:
        filename = os.path.join(tmp, "moondancer.bin")
        proc = subprocess.Popen(
            args=["cargo", "objcopy", "--release", "--", "-Obinary", filename],
            cwd="../../firmware/moondancer/"
        )
        proc.wait()
        flash_soc_firmware(device, filename)

    sys.argv = [sys.argv[0]]
    top_level_cli(MoondancerSoc())


def _build_selftest(device, args):
    from cynthion.selftest.gateware import SelftestDevice
    from cynthion.selftest.host import StandaloneTester

    from luna import top_level_cli
    sys.argv = [sys.argv[0]]
    device = top_level_cli(SelftestDevice)

    # run test
    tester = StandaloneTester(device)
    tester.run_tests()
    print()
