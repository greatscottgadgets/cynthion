#!/usr/bin/env python3
#
# This file is part of Cynthion.
#
# Copyright (c) 2024 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

from amaranth import *

class Top(Elaboratable):
    def elaborate(self, platform):
        m = Module()

        leds: Signal(6) = Cat(platform.request("led", n).o for n in range(0, 6))

        half_freq: int    = int(60e6 // 2)
        timer: Signal(25) = Signal(range(half_freq))

        with m.If(timer == half_freq - 1):
            m.d.sync += leds.eq(~leds)
            m.d.sync += timer.eq(0)

        with m.Else():
            m.d.sync += timer.eq(timer + 1)

        return m

if __name__ == "__main__":
    from luna import top_level_cli
    top_level_cli(Top)
