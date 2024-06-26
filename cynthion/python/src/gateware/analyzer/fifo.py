#
# This file is part of Cynthion.
#
# Copyright (c) 2024 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

from amaranth import Elaboratable, Module, Signal, Cat

from luna.gateware.stream import StreamInterface


class Stream16to8(Elaboratable):
    def __init__(self, msb_first=True):
        self.msb_first = msb_first
        self.input     = StreamInterface(payload_width=16)
        self.output    = StreamInterface(payload_width=8)

    def elaborate(self, platform):
        m = Module()

        input_data = self.input.payload
        if self.msb_first:
            input_data = Cat(input_data[8:16], input_data[0:8])

        odd_byte   = Signal()
        data_shift = Signal.like(self.input.payload)  # shift register
        m.d.comb  += self.output.payload.eq(data_shift[0:8])

        # When the output stream is not stalled...
        with m.If(self.output.ready | ~self.output.valid):

            # If odd_byte is asserted, send the buffered second byte
            with m.If(odd_byte):
                m.d.sync += [
                    data_shift          .eq(data_shift[8:]),
                    self.output.valid   .eq(1),
                    odd_byte            .eq(0),
                ]

            # Otherwise, consume a new word from the input stream
            with m.Else():
                m.d.comb += self.input.ready .eq(1)
                m.d.sync += self.output.valid.eq(self.input.valid)
                with m.If(self.input.valid):
                    m.d.sync += [
                        data_shift .eq(input_data),
                        odd_byte   .eq(1),
                    ]

        return m
