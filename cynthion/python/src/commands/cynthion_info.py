#!/usr/bin/env python
#
# This file is part of Cynthion.
#
# Copyright (c) 2023-2024 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

""" Cynthion 'info' command. """

import logging, sys

from apollo_fpga import ApolloDebugger

from .. import __version__, shared


def cynthion_info(args):
    logging.info(f"Cynthion version: {__version__}")
    if ApolloDebugger.print_info(force_offline=args.force_offline, out=logging.info,
                ids=[(shared.usb.bVendorId.apollo, shared.usb.bProductId.apollo)],
                stub_ids=[(shared.usb.bVendorId.cynthion, shared.usb.bProductId.cynthion)]):
        if not args.force_offline:
            logging.info("For additional device information use the --force-offline option.")
    else:
        logging.info("Could not find a Cynthion device.")
        sys.exit(1)
