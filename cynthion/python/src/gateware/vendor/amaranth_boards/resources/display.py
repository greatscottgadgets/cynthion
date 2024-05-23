from amaranth.build import *


__all__ = ["Display7SegResource", "VGAResource"]


def Display7SegResource(*args, a, b, c, d, e, f, g, dp=None, invert=False,
                        conn=None, attrs=None):
    ios = []
    ios.append(Subsignal("a", Pins(a, dir="o", invert=invert, conn=conn, assert_width=1)))
    ios.append(Subsignal("b", Pins(b, dir="o", invert=invert, conn=conn, assert_width=1)))
    ios.append(Subsignal("c", Pins(c, dir="o", invert=invert, conn=conn, assert_width=1)))
    ios.append(Subsignal("d", Pins(d, dir="o", invert=invert, conn=conn, assert_width=1)))
    ios.append(Subsignal("e", Pins(e, dir="o", invert=invert, conn=conn, assert_width=1)))
    ios.append(Subsignal("f", Pins(f, dir="o", invert=invert, conn=conn, assert_width=1)))
    ios.append(Subsignal("g", Pins(g, dir="o", invert=invert, conn=conn, assert_width=1)))
    if dp is not None:
        ios.append(Subsignal("dp", Pins(dp, dir="o", invert=invert, conn=conn, assert_width=1)))
    if attrs is not None:
        ios.append(attrs)
    return Resource.family(*args, default_name="display_7seg", ios=ios)


def VGAResource(*args, r, g, b, vs, hs, invert_sync=False, conn=None, attrs=None):
    ios = []

    ios.append(Subsignal("r", Pins(r, dir="o", conn=conn)))
    ios.append(Subsignal("g", Pins(g, dir="o", conn=conn)))
    ios.append(Subsignal("b", Pins(b, dir="o", conn=conn)))
    ios.append(Subsignal("hs", Pins(hs, dir="o", invert=invert_sync, conn=conn, assert_width=1)))
    ios.append(Subsignal("vs", Pins(vs, dir="o", invert=invert_sync, conn=conn, assert_width=1)))

    if attrs is not None:
        ios.append(attrs)

    return Resource.family(*args, default_name="vga", ios=ios)
