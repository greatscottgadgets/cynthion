#!/usr/bin/env python3
#
# facedancer-usbproxy.py

import logging

from facedancer import FacedancerUSBApp
from facedancer.USBConfiguration import USBConfiguration
from facedancer.USBInterface import USBInterface
from facedancer.USBEndpoint import USBEndpoint
from facedancer.USBProxy import USBProxyDevice, USBProxyFilter
from facedancer.filters.standard import USBProxySetupFilters
from facedancer.filters.logging import USBProxyPrettyPrintFilter


# - MidiChordFilter -----------------------------------------------------------

def button_to_row(button):
    return int(button % 3 == 0)

def button_to_note(button):
    if button_to_row(button) == 0:
        return int(button/3)
    else:
        return int((button-1) / 3)

# C:0, D:2, E:4, F:5, G:7, A:9, B:11, C:12
IONIAN = {
    0: [ 0,  4,  7, 11], # CEGB
    1: [ 2,  5,  9, 12], # DFAC
    2: [ 4,  7, 11,  2], # EGBD
    3: [ 5,  9, 12,  4], # FACE
    4: [ 7, 11,  2,  5], # GBDF
    5: [ 9, 12,  4,  7], # ACEG
    6: [11,  2,  5,  9], # BDFA
    7: [12,  4,  7, 11], # CEGB
}

# C:0, D:2, Eb:3, F:5, G:7, Ab:8, Bb:10, C:12
AEOLIAN = {
    0: [ 0,  3,  7, 10],
    1: [ 2,  5,  8, 12],
    2: [ 3,  7, 10,  2],
    3: [ 5,  8, 12,  3],
    4: [ 7, 10,  2,  5],
    5: [ 8, 12,  3,  7],
    6: [10,  2,  5,  8],
    7: [12,  3,  7, 10],
}

# 60 == middle-c
def note_to_chord(note, scale=IONIAN, transpose=60):
    return list(map(lambda x: x + transpose, scale[note]))

class MidiChordFilter(USBProxyFilter):
    """
    Sample filter that generates MIDI chords from notes.
    Demonstrates how dead simple this is. :)
    """

    def __init__(self):
        self.mode_7th = False
        self.scale = IONIAN

    # data[0]:4-7 -- Cable Number
    # data[0]:0-3 -- Code Index Number (e.g. 0x09/0x08 = note on/off)
    # data[1]     -- MIDI_0            (e.g. 0x90/0x80 = note on/off)
    # data[2]     -- MIDI_1            (e.g. pitch)
    # data[3]     -- MIDI_2            (e.g. velocity)
    def filter_in(self, ep_num, data):
        print(f"filter input length: {len(data)} => ep_num:{ep_num}  type+chan:{data[0]:x} onoff:{data[1]:x} note:{data[2]:x} vel:{data[3]:x}")
        button = data[2]
        output = []

        # set modes
        if button == 0x19:   # bank left
            self.scale = IONIAN
            data[3] = 0
            print(f"ionian: {self.scale}")
            return ep_num, data
        elif button == 0x1a: # bank right
            self.scale = AEOLIAN
            data[3] = 0
            print(f"aeolian: {self.scale}")
            return ep_num, data
        elif button == 0x1b and data[1] == 0x90: # solo
            self.mode_7th = not self.mode_7th
            data[3] = 0
            print(f"mode_7th: {self.mode_7th}")
            return ep_num, data

        # make notes
        try:
            # map button to note and chord
            note = button_to_note(button)
            chord = note_to_chord(note, self.scale)

            # for both rows, add the root
            data[2] = chord[0]
            output += data

            # for 1st row, construct the diatonic chord
            if button_to_row(button) == 0:
                data[3] = 64 # less velocity
                # add 3rd
                data[2] = chord[1]
                output += data
                # add 5th
                data[2] = chord[2]
                output += data
                # optionally, add 7th
                if self.mode_7th:
                    data[2] = chord[3]
                    output += data
            else:
                # shift 2nd row up an octave
                output[2] += 12

        except Exception as e:
            print(f"Oops: {e}")
            return ep_num, data

        print(f"filter output length: {len(output)} => {output}")

        return ep_num, output

def main():
    # Create a new proxy/MITM connection to device
    u = FacedancerUSBApp(verbose=2)
    d = USBProxyDevice(u, idVendor=0x09e8, idProduct=0x0031, verbose=2) # Akai MIDIMix

    d.add_filter(USBProxySetupFilters(d, verbose=2))
    d.add_filter(MidiChordFilter())
    d.add_filter(USBProxyPrettyPrintFilter(verbose=5))
    d.connect()

    try:
        d.run()
    # SIGINT raises KeyboardInterrupt
    except KeyboardInterrupt:
        d.disconnect()

if __name__ == "__main__":
    log_format = "%(levelname)-8s[%(module)-12s]  >%(message)s"
    logging.basicConfig(level=logging.INFO, format=log_format)
    main()
