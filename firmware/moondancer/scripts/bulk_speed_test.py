#!/usr/bin/env python3
# pylint: disable=no-member
#
# This file is part of Cynthion.
#
# Copyright (c) 2023 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

import os
import sys
import logging
import time

from enum import IntEnum

import usb1

from luna import configure_default_logging

import cynthion

VENDOR_ID  = cynthion.shared.usb.bVendorId.example
PRODUCT_ID = cynthion.shared.usb.bProductId.example

BULK_ENDPOINT_NUMBER = 1
COMMAND_ENDPOINT_NUMBER = 2

# Set the total amount of data to be used in our speed test.
TEST_DATA_SIZE = 2 * 1024 * 1024 # 2MB

# Set the size each transfer will receive or transmit
TEST_TRANSFER_SIZE = 16 * 1024

# Size of the host-size "transfer queue" -- this is effectively the number of async transfers we'll
# have scheduled at a given time.
#
# Typical rates are:
#
# IN
#  1: 4.173176849909332MB/s.
#  2: 5.0328733340888006MB/s.
#  4: 5.0091525616969435MB/s.
#  8: 5.3542598818498535MB/s.
# 16: 5.392326674816055MB/s.
#
# OUT heapless::Queue
#  1: 1.4349925228227383MB/s.
#  2: 1.4666992693303362MB/s.
#  4: 1.457957841985751MB/s.
#  8: 1.4654306572531672MB/s.
# 16: 1.4512499419971148MB/s.
#
# OUT bbqueue
#  1: 1.6461090465711896MB/s.
#  2: 1.6093034339079635MB/s.
#  4: 1.6206375879382768MB/s.
#  8: 1.7831124476261475MB/s.
# 16: 1.6684233822170424MB/s.
#
# OUT drop packets instead of sending to main loop
#  1: 2.1133309407538894MB/s.
#  2: 2.2171208320139666MB/s.
#  4: 2.2115455978314476MB/s.
#  8: 2.2631431327794385MB/s.
# 16: 2.205459558884749MB/s.
#
# OUT reset endpoint
#  1: 11.370056274755495MB/s.
#  2: 17.527108715416382MB/s.
#  4: 24.430036727672277MB/s.
#  8: 21.272581091551572MB/s.
# 16: LIBUSB_ERROR_NOT_FOUND
#
# OUT records
#  8: 5.78624228905323MB/s.
#
# IN  records
#  8: 1.8312864324208076MB/s.
#
TRANSFER_QUEUE_DEPTH = 8


# Test commands
class TestCommand(IntEnum):
    Stop = 0x01,
    In   = 0x23,
    Out  = 0x42,

# Error messages
_messages = {
    1: "error'd out",
    2: "timed out",
    3: "was prematurely cancelled",
    4: "was stalled",
    5: "lost the device it was connected to",
    6: "sent more data than expected."
}


# - Bulk Speed Test -----------------------------------------------------------

def run_speed_test(direction=usb1.ENDPOINT_IN):
    """ Runs a simple IN speed test, and reports throughput. """

    test_data = bytearray([x % 256 for x in range(512)])
    total_data_exchanged = 0
    failed_out = False

    def _should_terminate():
        """ Returns true iff our test should terminate. """
        return (total_data_exchanged > TEST_DATA_SIZE) or failed_out


    def _transfer_completed(transfer: usb1.USBTransfer):
        """ Callback executed when an async transfer completes. """
        nonlocal total_data_exchanged, failed_out

        status = transfer.getStatus()

        # If the transfer completed.
        if status in (usb1.TRANSFER_COMPLETED,):

            # Count the data exchanged in this packet...
            total_data_exchanged += transfer.getActualLength()
            logging.debug(f"usb1.TRANSFER_COMPLETED: {total_data_exchanged} bytes")

            if direction == usb1.ENDPOINT_IN:
                buffer = transfer.getBuffer()
                logging.debug(f"  buffer length: {len(buffer)}")
                logging.debug(f"  {list(buffer[:8])} -> {list(buffer[-8:])}")

            # ... and if we should terminate, abort.
            if _should_terminate():
                logging.debug("usb1.TRANSFER_COMPLETED terminating")
                return

            # Otherwise, re-submit the transfer.
            transfer.submit()

        else:
            failed_out = status

    with usb1.USBContext() as context:

        # Grab a reference to our device...
        device = context.openByVendorIDAndProductID(VENDOR_ID, PRODUCT_ID)

        # ... and claim its bulk interface.
        device.claimInterface(0)

        # Submit a set of transfers to perform async comms with.
        active_transfers = []
        for _ in range(TRANSFER_QUEUE_DEPTH):

            # Allocate the transfer...
            transfer = device.getTransfer()
            endpoint = direction | BULK_ENDPOINT_NUMBER
            if direction == usb1.ENDPOINT_IN:
                transfer.setBulk(endpoint, TEST_TRANSFER_SIZE, callback=_transfer_completed, timeout=1000)
            else:
                transfer.setBulk(endpoint, test_data, callback=_transfer_completed, timeout=1000)

            # ... and store it.
            active_transfers.append(transfer)

        # Start our benchmark timer.
        start_time = time.time()

        # Submit our transfers all at once.
        for transfer in active_transfers:
            transfer.submit()

        # Tell Cynthion to start transmitting/receiving
        if direction == usb1.ENDPOINT_IN:
            device.bulkWrite(COMMAND_ENDPOINT_NUMBER, [TestCommand.In])
        else:
            device.bulkWrite(COMMAND_ENDPOINT_NUMBER, [TestCommand.Out])

        # Run our transfers until we get enough data.
        while not _should_terminate():
            context.handleEvents()

        # Figure out how long this took us.
        end_time = time.time()
        elapsed = end_time - start_time

        # Tell Cynthion to stop transmitting/receiving
        device.bulkWrite(COMMAND_ENDPOINT_NUMBER, [TestCommand.Stop])

        # Cancel all of our active transfers.
        for transfer in active_transfers:
            if transfer.isSubmitted():
                transfer.cancel()

        # If we failed out; tell Cynthion to stop transmitting and indicate it.
        if (failed_out):
            device.bulkWrite(COMMAND_ENDPOINT_NUMBER, [TestCommand.Stop])
            logging.error(f"Test failed because a transfer {_messages[failed_out]}.")
            sys.exit(failed_out)

        bytes_per_second = total_data_exchanged / elapsed
        logging.info(f"Exchanged {total_data_exchanged / 1000000}MB total at {bytes_per_second / 1000000}MB/s.")




# - main entry point ----------------------------------------------------------

if __name__ == "__main__":
    configure_default_logging(level=os.getenv("LOG_LEVEL", "INFO").upper())

    logging.info(f"Total data transmitted for each test: {TEST_DATA_SIZE} bytes")
    logging.info(f"Individual transfer size: {TEST_TRANSFER_SIZE} bytes")
    logging.info(f"Transfer queue depth: {TRANSFER_QUEUE_DEPTH}")

    try:
        logging.info("Running IN speed test...")
        run_speed_test(direction=usb1.ENDPOINT_IN)
        time.sleep(1)

        logging.info("Running OUT speed test...")
        run_speed_test(direction=usb1.ENDPOINT_OUT)

    except Exception as e:
        logging.error(f"USB Bulk speed test failed: {e}")
