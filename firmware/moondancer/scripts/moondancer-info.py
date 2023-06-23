#!/usr/bin/env python

# Test GreatFET connection

import sys, traceback
import greatfet

def main():
    gf = greatfet.GreatFET()

    print("Found a {}!".format(gf.board_name()))
    print("  Board ID: {}".format(gf.board_id()))
    print("  Firmware version: {}".format(gf.firmware_version()))
    print("  Part ID: {}".format(gf.part_id()))
    print("  Serial number: {}".format(gf.serial_number()))

if __name__ == "__main__":
    try:
        main()
    except Exception as e:
        print(f"\nException: {e}")
        print(traceback.format_exc())
