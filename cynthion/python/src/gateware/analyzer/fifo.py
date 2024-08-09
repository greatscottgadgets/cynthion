#
# This file is part of Cynthion.
#
# Copyright (c) 2024 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

from amaranth                       import Elaboratable, Module, Signal, Cat, Memory, ResetSignal
from amaranth.lib.fifo              import SyncFIFOBuffered, FIFOInterface
from amaranth.lib.coding            import GrayDecoder, GrayEncoder
from amaranth.hdl.ast               import Assume, Initial
from amaranth.lib.cdc               import FFSynchronizer
from amaranth.utils                 import log2_int

from luna.gateware.stream           import StreamInterface
from luna.gateware.interface.psram  import HyperRAMInterface, HyperRAMPHY


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
    def __init__(self, out_fifo_depth=None):
        self.input  = StreamInterface(payload_width=16)
        self.output = StreamInterface(payload_width=16)
        # A minimum output FIFO depth of 2 prevents data loss during consumer stalls.
        self.out_fifo_depth = max(out_fifo_depth, 2) if out_fifo_depth is not None else 2

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

        # This output buffer prevents data loss during consumer stalls. It can also be used
        # to gather entire bursts from the HyperRAM if `out_fifo_depth` is big enough.
        m.submodules.out_fifo = out_fifo = SyncFIFOBuffered(width=16, depth=self.out_fifo_depth)

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
            # READ: Finish when PSRAM is empty or the output FIFO is full.
            m.d.comb += psram.final_word.eq((word_count == 1) | (out_fifo.level == out_fifo.depth - 1))

        #
        # HyperRAM Packet FIFO state machine
        #
        with m.FSM(domain="sync"):

            # IDLE: Begin a write / read burst operation when ready.
            with m.State("IDLE"):
                # Write whenever we have input data...
                with m.If(self.input.valid & ~full):
                    m.d.comb += [
                        psram.address           .eq(write_address),
                        psram.perform_write     .eq(1),
                        psram.start_transfer    .eq(1),
                    ]
                    m.d.sync += is_write.eq(1)
                    m.next = "BUSY"

                # ...otherwise, read when FIFO is less than half full.
                with m.Elif(~empty & (out_fifo.level[-1] == 0)):
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


class AsyncFIFOReadReset(Elaboratable, FIFOInterface):
    def __init__(self, *, width, depth, r_domain="read", w_domain="write", exact_depth=False):
        if depth != 0:
            try:
                depth_bits = log2_int(depth, need_pow2=exact_depth)
                depth = 1 << depth_bits
            except ValueError:
                raise ValueError("AsyncFIFO only supports depths that are powers of 2; requested "
                                 "exact depth {} is not"
                                 .format(depth)) from None
        else:
            depth_bits = 0
        super().__init__(width=width, depth=depth)

        self.ext_rst = Signal()
        self._r_domain = r_domain
        self._w_domain = w_domain
        self._ctr_bits = depth_bits + 1

    def elaborate(self, platform):
        m = Module()
        if self.depth == 0:
            m.d.comb += [
                self.w_rdy.eq(0),
                self.r_rdy.eq(0),
            ]
            return m

        # The design of this queue is the "style #2" from Clifford E. Cummings' paper "Simulation
        # and Synthesis Techniques for Asynchronous FIFO Design":
        # http://www.sunburst-design.com/papers/CummingsSNUG2002SJ_FIFO1.pdf

        do_write = self.w_rdy & self.w_en
        do_read  = self.r_rdy & self.r_en

        # TODO: extract this pattern into lib.cdc.GrayCounter
        produce_w_bin = Signal(self._ctr_bits)
        produce_w_nxt = Signal(self._ctr_bits)
        m.d.comb += produce_w_nxt.eq(produce_w_bin + do_write)
        m.d[self._w_domain] += produce_w_bin.eq(produce_w_nxt)

        # Note: Both read-domain counters must be reset_less (see comments below)
        consume_r_bin = Signal(self._ctr_bits, reset_less=True)
        consume_r_nxt = Signal(self._ctr_bits)
        m.d.comb += consume_r_nxt.eq(consume_r_bin + do_read)
        m.d[self._r_domain] += consume_r_bin.eq(consume_r_nxt)

        produce_w_gry = Signal(self._ctr_bits)
        produce_r_gry = Signal(self._ctr_bits)
        produce_enc = m.submodules.produce_enc = \
            GrayEncoder(self._ctr_bits)
        produce_cdc = m.submodules.produce_cdc = \
            FFSynchronizer(produce_w_gry, produce_r_gry, o_domain=self._r_domain)
        m.d.comb += produce_enc.i.eq(produce_w_nxt),
        m.d[self._w_domain] += produce_w_gry.eq(produce_enc.o)

        consume_r_gry = Signal(self._ctr_bits, reset_less=True)
        consume_w_gry = Signal(self._ctr_bits)
        consume_enc = m.submodules.consume_enc = \
            GrayEncoder(self._ctr_bits)
        consume_cdc = m.submodules.consume_cdc = \
            FFSynchronizer(consume_r_gry, consume_w_gry, o_domain=self._w_domain)
        m.d.comb += consume_enc.i.eq(consume_r_nxt)
        m.d[self._r_domain] += consume_r_gry.eq(consume_enc.o)

        consume_w_bin = Signal(self._ctr_bits)
        consume_dec = m.submodules.consume_dec = \
            GrayDecoder(self._ctr_bits)
        m.d.comb += consume_dec.i.eq(consume_w_gry),
        m.d[self._w_domain] += consume_w_bin.eq(consume_dec.o)

        produce_r_bin = Signal(self._ctr_bits)
        produce_dec = m.submodules.produce_dec = \
            GrayDecoder(self._ctr_bits)
        m.d.comb += produce_dec.i.eq(produce_r_gry),
        m.d.comb += produce_r_bin.eq(produce_dec.o)

        w_full  = Signal()
        r_empty = Signal()
        m.d.comb += [
            w_full.eq((produce_w_gry[-1]  != consume_w_gry[-1]) &
                      (produce_w_gry[-2]  != consume_w_gry[-2]) &
                      (produce_w_gry[:-2] == consume_w_gry[:-2])),
            r_empty.eq(consume_r_gry == produce_r_gry),
        ]

        m.d[self._w_domain] += self.w_level.eq(produce_w_bin - consume_w_bin)
        m.d.comb += self.r_level.eq(produce_r_bin - consume_r_bin)

        storage = Memory(width=self.width, depth=self.depth)
        w_port  = m.submodules.w_port = storage.write_port(domain=self._w_domain)
        r_port  = m.submodules.r_port = storage.read_port (domain=self._r_domain,
                                                           transparent=False)
        m.d.comb += [
            w_port.addr.eq(produce_w_bin[:-1]),
            w_port.data.eq(self.w_data),
            w_port.en.eq(do_write),
            self.w_rdy.eq(~w_full),
        ]
        m.d.comb += [
            r_port.addr.eq(consume_r_nxt[:-1]),
            self.r_data.eq(r_port.data),
            r_port.en.eq(1),
            self.r_rdy.eq(~r_empty),
        ]

        # Reset handling differs from Amaranth's AsyncFIFO: reset control rests entirely with 
        # the read domain. An additional synchronous external reset signal is also included.
        r_rst = ResetSignal(domain=self._r_domain, allow_reset_less=True)

        # Decode Gray code counter to overwrite binary counter in read domain.
        rst_dec = m.submodules.rst_dec = \
            GrayDecoder(self._ctr_bits)
        m.d.comb += rst_dec.i.eq(produce_r_gry)
        with m.If(r_rst | self.ext_rst):
            m.d.comb += r_empty.eq(1)
            m.d[self._r_domain] += consume_r_gry.eq(produce_r_gry)
            m.d[self._r_domain] += consume_r_bin.eq(rst_dec.o)

        if platform == "formal":
            with m.If(Initial()):
                m.d.comb += Assume(produce_w_gry == (produce_w_bin ^ produce_w_bin[1:]))
                m.d.comb += Assume(consume_r_gry == (consume_r_bin ^ consume_r_bin[1:]))

        return m
