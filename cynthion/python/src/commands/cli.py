#!/usr/bin/env python
#
# This file is part of Cynthion.
#
# Copyright (c) 2023-2024 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

""" Root command that delegates to all Cynthion subcommands. """

import argparse
import logging
import sys

from sys                       import platform

from apollo_fpga               import ApolloDebugger

from .util                     import HelpFormatter

from . import cynthion_info, cynthion_flash, cynthion_build, cynthion_run, cynthion_setup, cynthion_update


def main():
    # Set up python's logging to act as a simple print, for now.
    logging.basicConfig(level=logging.INFO, format="%(message)-s")

    # Set up an argument parser.
    parser = argparse.ArgumentParser(description="Cynthion command line interface",
                                     formatter_class=HelpFormatter)
    command_parsers = parser.add_subparsers(dest="command", metavar="command")

    # cynthion run
    run_parser = command_parsers.add_parser("run", help="run a bitstream on the FPGA", formatter_class=HelpFormatter)
    run_parser.set_defaults(func=cynthion_run)
    run_parser.add_argument("--bitstream", help="(advanced) run the bitstream at <filename>", metavar="<filename>", required=False)
    run_subparsers  = run_parser.add_subparsers(dest="target", metavar="BITSTREAM")
    run_analyzer_parser = run_subparsers.add_parser("analyzer", help="run the USB Analyzer bitstream", formatter_class=HelpFormatter)
    run_facedancer_parser = run_subparsers.add_parser("facedancer", help="run the Facedancer bitstream", formatter_class=HelpFormatter)
    run_selftest_parser = run_subparsers.add_parser("selftest", help="run the hardware self-test bitstream", formatter_class=HelpFormatter)

    # cynthion flash
    flash_parser = command_parsers.add_parser("flash", help="overwrite the FPGA's configuration flash with the target bitstream", formatter_class=HelpFormatter)
    flash_parser.set_defaults(func=cynthion_flash)
    flash_group = flash_parser.add_mutually_exclusive_group()
    flash_group.add_argument("--bitstream", help="(advanced) flash the bitstream at <filename>", metavar="<filename>", required=False)
    flash_group.add_argument("--soc-firmware", help="(advanced) flash the soc firmware at <filename>", metavar="<filename>", required=False)
    flash_group.add_argument("--mcu-firmware", help="(advanced) flash the mcu firmware at <filename>", metavar="<filename>", required=False)
    flash_subparsers  = flash_parser.add_subparsers(dest="target", metavar="BITSTREAM")
    flash_analyzer_parser = flash_subparsers.add_parser("analyzer", help="flash the USB Analyzer bitstream", formatter_class=HelpFormatter)
    flash_facedancer_parser = flash_subparsers.add_parser("facedancer", help="flash the Facedancer bitstream", formatter_class=HelpFormatter)

    # cynthion build
    build_parser = command_parsers.add_parser("build")
    build_parser.set_defaults(func=cynthion_build)
    build_parser.add_argument("--luna-platform", action='store_true', help="display the LUNA_PLATFORM variable for the connected device")
    build_subparsers  = build_parser.add_subparsers(dest="target", metavar="TARGET")
    build_analyzer_parser = build_subparsers.add_parser("analyzer")
    build_facedancer_parser = build_subparsers.add_parser("facedancer")
    build_selftest_parser = build_subparsers.add_parser("selftest")

    # cynthion update
    update_parser = command_parsers.add_parser("update", help="update MCU firmware and FPGA configuration flash to the latest installed versions", formatter_class=HelpFormatter)
    update_parser.set_defaults(func=cynthion_update)
    update_parser.add_argument("--mcu-firmware", action='store_true',   help="only update the MCU firmware")
    update_parser.add_argument("--bitstream", action='store_true', help="only update the FPGA bitstream")

    # cynthion info
    info_parser = command_parsers.add_parser("info", help="print device information", formatter_class=HelpFormatter)
    info_parser.add_argument("--force-offline", action='store_true', help="force the FPGA to release the CONTROL port")
    info_parser.set_defaults(func=cynthion_info)

    # cynthion setup (Linux-only for now)
    if platform == "linux" or platform == "linux2":
        setup_parser = command_parsers.add_parser("setup", help="install Cynthion support files required for operation", formatter_class=HelpFormatter)
        setup_parser.set_defaults(func=cynthion_setup)
        setup_group = setup_parser.add_mutually_exclusive_group()
        setup_group.add_argument("--check", action='store_true', help="check Cynthion support files")
        setup_group.add_argument("--uninstall", action='store_true', help="remove Cynthion support files")
        setup_parser.add_argument("--udev", action='store_true', help="Linux udev access rules")

    # Parse arguments.
    args = parser.parse_args()
    if not args.command:
        parser.print_help()
        return

    # Force the FPGA offline by default in most commands to force Apollo mode if needed.
    force_offline = args.force_offline if "force_offline" in args else True

    # Execute the relevant command.
    if args.func in (cynthion_info, cynthion_setup):
        args.func(args)
    else:
        device = ApolloDebugger(force_offline=force_offline)
        args.func(device, args)
