#!/usr/bin/env python3
#
# This file is part of Cynthion
#
# Copyright (c) 2023 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

""" Utility for gathering information about connected Cynthions. """

import argparse
import errno
import inspect
import logging
import os
import sys
import textwrap

from cynthion import Cynthion

from pygreat.comms import GeneratedCommsClass

def print_host_info():
    """ Prints information about the installed host tools and software environment. """
    logging.info("Host tools info:")

    command_path = os.path.dirname(__file__)
    module_path  = os.path.dirname(command_path)

    try:
        import pkg_resources

        cynthion_version = pkg_resources.require("cynthion")[0].version
        pygreat_version = pkg_resources.require("pygreat")[0].version

        logging.info('\thost module version: {}'.format(cynthion_version))
        logging.info('\tpygreat module version: {}'.format(pygreat_version))

    except ImportError:
        logging.info("\tCan't get module version info -- setuptools is missing.")

    logging.info("\tpython version: {}".format(sys.version.split('\n')[0]))
    logging.info("\tmodule path: {}".format(module_path))
    logging.info("\tcommand path: {}\n".format(command_path))


def print_apollo_device_info(device, args):
    """ Command that prints information about devices connected to the scan chain to the console. """

    logging.info(f"Found a {device.get_compatibility_string()} device!")
    logging.info(f"\tHardware: {device.get_hardware_name()}")
    logging.info(f"\tSerial number: {device.serial_number}\n")


def print_moondancer_device_info(device):
    """ Prints the core information for a moondancer device. """

    logging.info("Found a {}!".format(device.board_name()))
    logging.info("\tlibgreat Board ID: {}".format(device.board_id()))
    logging.info("\tFirmware version: {}".format(device.firmware_version()))
    logging.info("\tPart ID: {}".format(device.part_id()))
    logging.info("\tSerial number: {}".format(device.serial_number()))

    # If this board has any version warnings to display, dipslay them.
    warnings = device.version_warnings()
    if warnings:
        wrapped_warnings = textwrap.wrap(warnings)
        wrapped_warnings = "\n".join(["    {}".format(line) for line in wrapped_warnings])
        logging.warn("\n  !!! WARNING !!!\n{}\n".format(wrapped_warnings))


def print_moondancer_apis(device):
    """ Prints a human-readable summary of the moondancer device's provided APIs. """

    logging.info("\tAPIs supported:")

    # Print each of the supported APIs.
    for api_name in device.comms.apis:
        printed = False

        # Get a shortcut to the provided RPC API.
        api = device.comms.apis[api_name]
        logging.info("\t  {}:".format(api.CLASS_NAME))

        # Print all methods on the given API.
        methods = inspect.getmembers(api, inspect.ismethod)


        # Otherwise, print all of the methods.
        for method_name, method in methods:

            # Don't print private-API methods.
            if method_name.startswith('_'):
                continue

            # Don't print inherited methods.
            if hasattr(GeneratedCommsClass, method_name):
                continue

            printed = True

            # Extract a summary for our view, and print it.
            # TODO: base the amount printed on the terminal size
            method_docs = inspect.getdoc(method)
            method_first_line = method_docs.split("\n")
            method_summary = method_first_line[0][0:60]
            logging.info("\t    {} -- {}".format(method_name, method_summary))

        # If we had nothing to print for the class,
        if not printed:
            logging.info("\t    <no introspectable methods>")



def main():
    # Set up python's logging to act as a simple print, for now.
    logging.basicConfig(level=logging.INFO, format="%(message)-s")

    # Set up a simple argument parser.
    parser = argparse.ArgumentParser(
        description="Utility for gathering information about connected Cynthions.")
    parser.add_argument('-A', '--api', dest='print_apis', action='store_true',
                        help="Print information about each device's supported APIs.")
    parser.add_argument('-a', '--all', dest='print_all', action='store_true',
                        help="Print all available information about the device & host.")
    parser.add_argument('-H', '--host', dest='print_host', action='store_true')
    parser.add_argument('-q', '--quiet', dest='quiet', action='store_true',
                        help="Prints only the serial numbers of detected Cynthions")
    args = parser.parse_args()


    # If requested, print information about the Cynthion install environment.
    if args.print_host or args.print_all:
        print_host_info()
        if not args.print_all:
            sys.exit(0)

    # Try to find all apollo devices
    from apollo_fpga import ApolloDebugger
    try:
        device = ApolloDebugger()
    except:
        device = None
    if not device:
        logging.error('No Cynthion devices found!')
        sys.exit(errno.ENODEV)

    # If we're in quiet mode, print only the serial number and abort.
    if args.quiet:
        logging.info(device.serial_number)
        sys.exit(0)

    # Print apollo device information
    print_apollo_device_info(device, args)

    # Try to find all moondancer devices
    devices = Cynthion(find_all=True)
    if not devices:
        logging.error('No Cynthion Moondancer devices found!')
        sys.exit(errno.ENODEV)

    # Print moondancer devices information
    for device in devices:

        # Otherwise, print the core information.
        print_moondancer_device_info(device)

        # If desired, print all APIs.
        if args.print_apis or args.print_all:
            print_moondancer_apis(device)

        logging.info(" ")


if __name__ == '__main__':
    main()
