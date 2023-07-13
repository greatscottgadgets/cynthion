#!/usr/bin/env python3

# Test Cynthion Moondancer connection

import cynthion
import sys, traceback

def main():
    board = cynthion.Cynthion()

    print("Found a {}!".format(board.board_name()))
    print("  Board ID: {}".format(board.board_id()))
    print("  Firmware version: {}".format(board.firmware_version()))
    print("  Part ID: {}".format(board.part_id()))
    print("  Serial number: {}".format(board.serial_number()))

if __name__ == "__main__":
    try:
        main()
    except Exception as e:
        print(f"\nException: {e}")
        print(traceback.format_exc())
