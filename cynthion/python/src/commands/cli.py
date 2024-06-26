#!/usr/bin/env python
#
# This file is part of Cynthion.
#
# Copyright (c) 2023 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

""" Root command that delegates to all Cynthion subcommands. """

import argparse
import logging
import sys

from apollo_fpga import ApolloDebugger
from apollo_fpga.commands.cli import Command, COMMANDS as APOLLO_COMMANDS


def cynthion_info(device, args):
    # just call the Apollo implementation for now
    command = next((c for c in APOLLO_COMMANDS if c.name == "info"), None)
    command.handler(device, args)

def cynthion_selftest(device, args):
    from .cynthion_selftest import main as main_selftest
    sys.argv = [sys.argv[0]]
    main_selftest()

def cynthion_mcu_firmware(device, args):
    from .cynthion_mcu_firmware import main as main_mcu_firmware
    sys.argv = [sys.argv[0]]
    main_mcu_firmware(args.file)

CYNTHION_COMMANDS = [
    # Apollo commands can be intercepted by simply supplying a new Command with the same name and (optionally) options
    Command("info",     handler=cynthion_info, args=[(("--force-offline",), dict(action='store_true'))],
            help="Print device info.", ),
    Command("selftest", handler=cynthion_selftest,
            help="Run a hardware self-test on a connected Cynthion."),
    Command("mcu-firmware", handler=cynthion_mcu_firmware, args=[(("file",), dict(nargs="?"))],
            help="Update Apollo firmware version.")
]


def main():
    # combine apollo and cynthion commands, overwriting apollo implementations
    # with cynthion implementations, where defined.
    apollo_commands   = dict(zip([c.name for c in APOLLO_COMMANDS],   APOLLO_COMMANDS))
    cynthion_commands = dict(zip([c.name for c in CYNTHION_COMMANDS], CYNTHION_COMMANDS))
    apollo_commands.update(cynthion_commands)
    commands = apollo_commands.values()

    # Set up a simple argument parser.
    parser = argparse.ArgumentParser(description="Apollo FPGA Configuration / Debug tool",
            formatter_class=argparse.RawTextHelpFormatter)
    sub_parsers = parser.add_subparsers(dest="command", metavar="command")
    for command in commands:
        cmd_parser = sub_parsers.add_parser(command.name, aliases=command.alias, help=command.help)
        cmd_parser.set_defaults(func=command.handler)
        for arg in command.args:
            if isinstance(arg, tuple):
                cmd_parser.add_argument(*arg[0], **arg[1])
            else:
                cmd_parser.add_argument(arg)

    args = parser.parse_args()
    if not args.command:
        parser.print_help()
        return

    # Force the FPGA offline by default in most commands to force Apollo mode if needed.
    force_offline = args.force_offline if "force_offline" in args else True
    device = ApolloDebugger(force_offline=force_offline)

    # Set up python's logging to act as a simple print, for now.
    logging.basicConfig(level=logging.INFO, format="%(message)-s")

    # Execute the relevant command.
    args.func(device, args)


if __name__ == '__main__':
    main()
