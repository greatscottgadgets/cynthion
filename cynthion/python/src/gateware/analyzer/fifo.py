#
# This file is part of Cynthion.
#
# Copyright (c) 2024 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

from amaranth import Elaboratable, Module, Signal, Cat
from amaranth.lib.fifo import SyncFIFO

from luna.gateware.stream import StreamInterface
from luna.gateware.interface.psram import HyperRAMInterface, HyperRAMPHY


class StreamFIFO(Elaboratable):
    def __init__(self, fifo):
        self.fifo   = fifo
        self.input  = StreamInterface(payload_width=fifo.width)
        self.output = StreamInterface(payload_width=fifo.width)

    def elaborate(self, platform):
        m = Module()

        m.submodules.fifo = self.fifo

        m.d.comb += [
            self.fifo.w_data    .eq(self.input.payload),
            self.fifo.w_en      .eq(self.input.valid),
            self.input.ready    .eq(self.fifo.w_rdy),

            self.output.payload .eq(self.fifo.r_data),
            self.output.valid   .eq(self.fifo.r_rdy),
            self.fifo.r_en      .eq(self.output.ready),
        ]

        return m


class HyperRAMPacketFIFO(Elaboratable):
    def __init__(self):
        self.input  = StreamInterface(payload_width=16)
        self.output = StreamInterface(payload_width=16)

    def elaborate(self, platform):
        m = Module()

        # HyperRAM submodules
        ram_bus         = platform.request('ram')
        psram_phy       = HyperRAMPHY(bus=ram_bus)
        psram           = HyperRAMInterface(phy=psram_phy.phy)
        m.submodules   += [psram_phy, psram]

        # HyperRAM status
        depth         = 2 ** 22
        write_address = Signal(range(depth))
        read_address  = Signal(range(depth))
        word_count    = Signal(range(depth + 1))
        empty         = Signal()
        full          = Signal()
        m.d.comb += [
            empty .eq(word_count == 0),
            full  .eq(word_count == depth),
        ]

        # Update word count and pointers using the write and read strobes.
        m.d.sync += word_count.eq(word_count - psram.read_ready + psram.write_ready)
        with m.If(psram.read_ready):
            m.d.sync += read_address.eq(read_address + 1)
        with m.If(psram.write_ready):
            m.d.sync += write_address.eq(write_address + 1)

        # This tiny output buffer prevents data loss during consumer stalls
        m.submodules.out_fifo = out_fifo = SyncFIFO(width=16, depth=2)

        # Hook up our PSRAM.
        m.d.comb += [
            ram_bus.reset.o       .eq(0),
            psram.single_page     .eq(0),
            psram.register_space  .eq(0),
            psram.write_data      .eq(self.input.payload),
            self.input.ready      .eq(psram.write_ready),

            # Wire PSRAM -> output FIFO -> output stream
            out_fifo.w_data       .eq(psram.read_data),
            out_fifo.w_en         .eq(psram.read_ready),
            self.output.payload   .eq(out_fifo.r_data),
            self.output.valid     .eq(out_fifo.r_rdy),
            out_fifo.r_en         .eq(self.output.ready),
        ]

        # Generation of the final word condition.
        is_write = Signal()
        with m.If(is_write):
            # WRITE: Finish when there's no space or incoming data.
            m.d.comb += psram.final_word.eq((word_count == (depth-1)) | self.input.last)
        with m.Else():
            # READ: Finish when PSRAM is empty or the consumer stalls the output stream.
            m.d.comb += psram.final_word.eq((word_count == 1) | ~self.output.ready)

        #
        # HyperRAM Packet FIFO state machine
        #
        with m.FSM(domain="sync"):

            # IDLE: Begin a write / read burst operation when ready.
            with m.State("IDLE"):
                with m.If(self.input.valid & ~full):
                    m.d.comb += [
                        psram.address           .eq(write_address),
                        psram.perform_write     .eq(1),
                        psram.start_transfer    .eq(1),
                    ]
                    m.d.sync += is_write.eq(1)
                    m.next = "BUSY"

                with m.Elif((out_fifo.level == 0) & ~empty):
                    m.d.comb += [
                        psram.address           .eq(read_address),
                        psram.perform_write     .eq(0),
                        psram.start_transfer    .eq(1),
                    ]
                    m.d.sync += is_write.eq(0)
                    m.next = "BUSY"

            # BUSY: Wait for the PSRAM to recover before a new transaction.
            with m.State("BUSY"):
                with m.If(psram.idle):
                    m.next = "IDLE"

        return m


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
