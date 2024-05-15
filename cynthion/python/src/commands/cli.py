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


def cynthion_selftest(device, args):
    from .cynthion_selftest import main as main_selftest
    sys.argv = [sys.argv[0]]
    main_selftest()


COMMANDS = [
    Command("selftest", handler=cynthion_selftest,
            help="Run a hardware self-test on a connected Cynthion.", ),
]


def main():
    # combine apollo and cynthion commands
    commands = APOLLO_COMMANDS + COMMANDS

    # Set up a simple argument parser.
    parser = argparse.ArgumentParser(description="Apollo FPGA Configuration / Debug tool",
            formatter_class=argparse.RawTextHelpFormatter)
    sub_parsers = parser.add_subparsers(dest="command", metavar="command")
    for command in commands:
        cmd_parser = sub_parsers.add_parser(command.name, aliases=command.alias, help=command.help)
        cmd_parser.set_defaults(func=command.handler)
        for arg in command.args:
            cmd_parser.add_argument(arg)

    args = parser.parse_args()
    if not args.command:
        parser.print_help()
        return

    device = ApolloDebugger()

    # Set up python's logging to act as a simple print, for now.
    logging.basicConfig(level=logging.INFO, format="%(message)-s")

    # Execute the relevant command.
    args.func(device, args)


if __name__ == '__main__':
    main()
