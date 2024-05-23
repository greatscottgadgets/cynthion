from amaranth.build import *


__all__ = [
    "UARTResource", "IrDAResource", "SPIResource", "I2CResource",
    "DirectUSBResource", "ULPIResource", "PS2Resource",
]


def UARTResource(*args, rx, tx, rts=None, cts=None, dtr=None, dsr=None, dcd=None, ri=None,
                 conn=None, attrs=None, role=None):
    if any(line is not None for line in (rts, cts, dtr, dsr, dcd, ri)):
        assert role in ("dce", "dte")
    if role == "dte":
        dce_to_dte = "i"
        dte_to_dce = "o"
    else:
        dce_to_dte = "o"
        dte_to_dce = "i"

    io = []
    io.append(Subsignal("rx", Pins(rx, dir="i", conn=conn, assert_width=1)))
    io.append(Subsignal("tx", Pins(tx, dir="o", conn=conn, assert_width=1)))
    if rts is not None:
        io.append(Subsignal("rts", Pins(rts, dir=dte_to_dce, conn=conn, assert_width=1)))
    if cts is not None:
        io.append(Subsignal("cts", Pins(cts, dir=dce_to_dte, conn=conn, assert_width=1)))
    if dtr is not None:
        io.append(Subsignal("dtr", Pins(dtr, dir=dte_to_dce, conn=conn, assert_width=1)))
    if dsr is not None:
        io.append(Subsignal("dsr", Pins(dsr, dir=dce_to_dte, conn=conn, assert_width=1)))
    if dcd is not None:
        io.append(Subsignal("dcd", Pins(dcd, dir=dce_to_dte, conn=conn, assert_width=1)))
    if ri is not None:
        io.append(Subsignal("ri", Pins(ri, dir=dce_to_dte, conn=conn, assert_width=1)))
    if attrs is not None:
        io.append(attrs)
    return Resource.family(*args, default_name="uart", ios=io)


def IrDAResource(number, *, rx, tx, en=None, sd=None,
                 conn=None, attrs=None):
    # Exactly one of en (active-high enable) or sd (shutdown, active-low enable) should
    # be specified, and it is mapped to a logic level en subsignal.
    assert (en is not None) ^ (sd is not None)

    io = []
    io.append(Subsignal("rx", Pins(rx, dir="i", conn=conn, assert_width=1)))
    io.append(Subsignal("tx", Pins(tx, dir="o", conn=conn, assert_width=1)))
    if en is not None:
        io.append(Subsignal("en", Pins(en, dir="o", conn=conn, assert_width=1)))
    if sd is not None:
        io.append(Subsignal("en", PinsN(sd, dir="o", conn=conn, assert_width=1)))
    if attrs is not None:
        io.append(attrs)
    return Resource("irda", number, *io)


def SPIResource(*args, cs_n, clk, copi, cipo, int=None, reset=None,
                conn=None, attrs=None, role="controller"):
    assert role in ("controller", "peripheral")
    assert copi is not None or cipo is not None # support unidirectional SPI

    io = []
    if role == "controller":
        io.append(Subsignal("cs", PinsN(cs_n, dir="o", conn=conn)))
        io.append(Subsignal("clk", Pins(clk, dir="o", conn=conn, assert_width=1)))
        if copi is not None:
            io.append(Subsignal("copi", Pins(copi, dir="o", conn=conn, assert_width=1)))
        if cipo is not None:
            io.append(Subsignal("cipo", Pins(cipo, dir="i", conn=conn, assert_width=1)))
    else:  # peripheral
        io.append(Subsignal("cs", PinsN(cs_n, dir="i", conn=conn, assert_width=1)))
        io.append(Subsignal("clk", Pins(clk, dir="i", conn=conn, assert_width=1)))
        if copi is not None:
            io.append(Subsignal("copi", Pins(copi, dir="i", conn=conn, assert_width=1)))
        if cipo is not None:
            io.append(Subsignal("cipo", Pins(cipo, dir="oe", conn=conn, assert_width=1)))
    if int is not None:
        if role == "controller":
            io.append(Subsignal("int", Pins(int, dir="i", conn=conn)))
        else:
            io.append(Subsignal("int", Pins(int, dir="oe", conn=conn, assert_width=1)))
    if reset is not None:
        if role == "controller":
            io.append(Subsignal("reset", Pins(reset, dir="o", conn=conn)))
        else:
            io.append(Subsignal("reset", Pins(reset, dir="i", conn=conn, assert_width=1)))
    if attrs is not None:
        io.append(attrs)
    return Resource.family(*args, default_name="spi", ios=io)


def I2CResource(*args, scl, sda, conn=None, attrs=None):
    io = []
    io.append(Subsignal("scl", Pins(scl, dir="io", conn=conn, assert_width=1)))
    io.append(Subsignal("sda", Pins(sda, dir="io", conn=conn, assert_width=1)))
    if attrs is not None:
        io.append(attrs)
    return Resource.family(*args, default_name="i2c", ios=io)


def DirectUSBResource(*args, d_p, d_n, pullup=None, vbus_valid=None, conn=None, attrs=None):

    io = []
    io.append(Subsignal("d_p", Pins(d_p, dir="io", conn=conn, assert_width=1)))
    io.append(Subsignal("d_n", Pins(d_n, dir="io", conn=conn, assert_width=1)))
    if pullup:
        io.append(Subsignal("pullup", Pins(pullup, dir="o", conn=conn, assert_width=1)))
    if vbus_valid:
        io.append(Subsignal("vbus_valid", Pins(vbus_valid, dir="i", conn=conn, assert_width=1)))
    if attrs is not None:
        io.append(attrs)
    return Resource.family(*args, default_name="usb", ios=io)


def ULPIResource(*args, data, clk, dir, nxt, stp, rst=None,
            clk_dir='i', rst_invert=False, attrs=None, conn=None):
    assert clk_dir in ('i', 'o',)

    io = []
    io.append(Subsignal("data", Pins(data, dir="io", conn=conn, assert_width=8)))
    io.append(Subsignal("clk", Pins(clk, dir=clk_dir, conn=conn, assert_width=1)))
    io.append(Subsignal("dir", Pins(dir, dir="i", conn=conn, assert_width=1)))
    io.append(Subsignal("nxt", Pins(nxt, dir="i", conn=conn, assert_width=1)))
    io.append(Subsignal("stp", Pins(stp, dir="o", conn=conn, assert_width=1)))
    if rst is not None:
        io.append(Subsignal("rst", Pins(rst, dir="o", invert=rst_invert,
            conn=conn, assert_width=1)))
    if attrs is not None:
        io.append(attrs)
    return Resource.family(*args, default_name="usb", ios=io)


def PS2Resource(*args, clk, dat, conn=None, attrs=None):
    ios = []

    ios.append(Subsignal("clk", Pins(clk, dir="i", conn=conn, assert_width=1))),
    ios.append(Subsignal("dat", Pins(dat, dir="io", conn=conn, assert_width=1))),

    if attrs is not None:
        ios.append(attrs)

    return Resource.family(*args, default_name="ps2", ios=ios)
