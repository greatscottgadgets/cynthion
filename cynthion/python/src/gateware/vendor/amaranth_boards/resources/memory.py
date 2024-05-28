from amaranth.build import *


__all__ = [
    "SPIFlashResources", "SDCardResources",
    "SRAMResource", "SDRAMResource", "NORFlashResources",
    "DDR3Resource",
]


def SPIFlashResources(*args, cs_n, clk, copi, cipo, wp_n=None, hold_n=None,
                      conn=None, attrs=None):
    resources = []

    io_all = []
    if attrs is not None:
        io_all.append(attrs)
    io_all.append(Subsignal("cs",  PinsN(cs_n, dir="o", conn=conn)))
    io_all.append(Subsignal("clk", Pins(clk, dir="o", conn=conn, assert_width=1)))

    io_1x = list(io_all)
    io_1x.append(Subsignal("copi", Pins(copi, dir="o", conn=conn, assert_width=1)))
    io_1x.append(Subsignal("cipo", Pins(cipo, dir="i", conn=conn, assert_width=1)))
    if wp_n is not None and hold_n is not None:
        io_1x.append(Subsignal("wp",   PinsN(wp_n,   dir="o", conn=conn, assert_width=1)))
        io_1x.append(Subsignal("hold", PinsN(hold_n, dir="o", conn=conn, assert_width=1)))
    resources.append(Resource.family(*args, default_name="spi_flash", ios=io_1x,
                                     name_suffix="1x"))

    io_2x = list(io_all)
    io_2x.append(Subsignal("dq", Pins(" ".join([copi, cipo]), dir="io", conn=conn,
                                      assert_width=2)))
    resources.append(Resource.family(*args, default_name="spi_flash", ios=io_2x,
                                     name_suffix="2x"))

    if wp_n is not None and hold_n is not None:
        io_4x = list(io_all)
        io_4x.append(Subsignal("dq", Pins(" ".join([copi, cipo, wp_n, hold_n]), dir="io", conn=conn,
                                          assert_width=4)))
        resources.append(Resource.family(*args, default_name="spi_flash", ios=io_4x,
                                         name_suffix="4x"))

    return resources


def SDCardResources(*args, clk, cmd, dat0, dat1=None, dat2=None, dat3=None, cd=None, wp_n=None,
                    conn=None, attrs=None):
    resources = []

    io_common = []
    if attrs is not None:
        io_common.append(attrs)
    if cd is not None:
        io_common.append(Subsignal("cd", Pins(cd, dir="i", conn=conn, assert_width=1)))
    if wp_n is not None:
        io_common.append(Subsignal("wp", PinsN(wp_n, dir="i", conn=conn, assert_width=1)))

    io_native = list(io_common)
    io_native.append(Subsignal("clk", Pins(clk, dir="o", conn=conn, assert_width=1)))
    io_native.append(Subsignal("cmd", Pins(cmd, dir="o", conn=conn, assert_width=1)))

    io_1bit = list(io_native)
    io_1bit.append(Subsignal("dat", Pins(dat0, dir="io", conn=conn, assert_width=1)))
    if dat3 is not None:
        # DAT3 has a pullup and works as electronic card detect
        io_1bit.append(Subsignal("ecd", Pins(dat3, dir="i", conn=conn, assert_width=1)))
    resources.append(Resource.family(*args, default_name="sd_card", ios=io_1bit,
                                     name_suffix="1bit"))

    if dat1 is not None and dat2 is not None and dat3 is not None:
        io_4bit = list(io_native)
        io_4bit.append(Subsignal("dat", Pins(" ".join((dat0, dat1, dat2, dat3)), dir="io",
                                             conn=conn, assert_width=4)))
        resources.append(Resource.family(*args, default_name="sd_card", ios=io_4bit,
                                         name_suffix="4bit"))

    if dat3 is not None:
        io_spi = list(io_common)
        # DAT3/CS# has a pullup and doubles as electronic card detect
        io_spi.append(Subsignal("cs", PinsN(dat3, dir="io", conn=conn, assert_width=1)))
        io_spi.append(Subsignal("clk", Pins(clk, dir="o", conn=conn, assert_width=1)))
        io_spi.append(Subsignal("copi", Pins(cmd, dir="o", conn=conn, assert_width=1)))
        io_spi.append(Subsignal("cipo", Pins(dat0, dir="i", conn=conn, assert_width=1)))
        resources.append(Resource.family(*args, default_name="sd_card", ios=io_spi,
                                         name_suffix="spi"))

    return resources


def SRAMResource(*args, cs_n, oe_n=None, we_n, a, d, dm_n=None,
                 conn=None, attrs=None):
    io = []
    io.append(Subsignal("cs", PinsN(cs_n, dir="o", conn=conn, assert_width=1)))
    if oe_n is not None:
        # Asserted WE# deactivates the D output buffers, so WE# can be used to replace OE#.
        io.append(Subsignal("oe", PinsN(oe_n, dir="o", conn=conn, assert_width=1)))
    io.append(Subsignal("we", PinsN(we_n, dir="o", conn=conn, assert_width=1)))
    io.append(Subsignal("a", Pins(a, dir="o", conn=conn)))
    io.append(Subsignal("d", Pins(d, dir="io", conn=conn)))
    if dm_n is not None:
        io.append(Subsignal("dm", PinsN(dm_n, dir="o", conn=conn))) # dm="LB# UB#"
    if attrs is not None:
        io.append(attrs)
    return Resource.family(*args, default_name="sram", ios=io)


def SDRAMResource(*args, clk, cke=None, cs_n=None, we_n, ras_n, cas_n, ba, a, dq, dqm=None,
                  conn=None, attrs=None):
    io = []
    io.append(Subsignal("clk", Pins(clk, dir="o", conn=conn, assert_width=1)))
    if cke is not None:
        io.append(Subsignal("clk_en", Pins(cke, dir="o", conn=conn, assert_width=1)))
    if cs_n is not None:
        io.append(Subsignal("cs", PinsN(cs_n,  dir="o", conn=conn, assert_width=1)))
    io.append(Subsignal("we",  PinsN(we_n,  dir="o", conn=conn, assert_width=1)))
    io.append(Subsignal("ras", PinsN(ras_n, dir="o", conn=conn, assert_width=1)))
    io.append(Subsignal("cas", PinsN(cas_n, dir="o", conn=conn, assert_width=1)))
    io.append(Subsignal("ba", Pins(ba, dir="o", conn=conn)))
    io.append(Subsignal("a",  Pins(a,  dir="o", conn=conn)))
    io.append(Subsignal("dq", Pins(dq, dir="io", conn=conn)))
    if dqm is not None:
        io.append(Subsignal("dqm", Pins(dqm, dir="o", conn=conn))) # dqm="LDQM# UDQM#"
    if attrs is not None:
        io.append(attrs)
    return Resource.family(*args, default_name="sdram", ios=io)


def NORFlashResources(*args, rst=None, byte_n=None, cs_n, oe_n, we_n, wp_n, by, a, dq,
                      conn=None, attrs=None):
    resources = []

    io_common = []
    if rst is not None:
        io_common.append(Subsignal("rst", Pins(rst, dir="o", conn=conn, assert_width=1)))
    io_common.append(Subsignal("cs", PinsN(cs_n, dir="o", conn=conn, assert_width=1)))
    io_common.append(Subsignal("oe", PinsN(oe_n, dir="o", conn=conn, assert_width=1)))
    io_common.append(Subsignal("we", PinsN(we_n, dir="o", conn=conn, assert_width=1)))
    io_common.append(Subsignal("wp", PinsN(wp_n, dir="o", conn=conn, assert_width=1)))
    io_common.append(Subsignal("rdy", Pins(by, dir="i", conn=conn, assert_width=1)))

    if byte_n is None:
        io_8bit = list(io_common)
        io_8bit.append(Subsignal("a", Pins(a, dir="o", conn=conn)))
        io_8bit.append(Subsignal("dq", Pins(dq, dir="io", conn=conn, assert_width=8)))
        resources.append(Resource.family(*args, default_name="nor_flash", ios=io_8bit,
                                         name_suffix="8bit"))
    else:
        *dq_0_14, dq15_am1 = dq.split()

        # If present in a requested resource, this pin needs to be strapped correctly.
        io_common.append(Subsignal("byte", PinsN(byte_n, dir="o", conn=conn, assert_width=1)))

        io_8bit = list(io_common)
        io_8bit.append(Subsignal("a", Pins(" ".join((dq15_am1, a)), dir="o", conn=conn)))
        io_8bit.append(Subsignal("dq", Pins(" ".join(dq_0_14[:8]), dir="io", conn=conn,
                                            assert_width=8)))
        resources.append(Resource.family(*args, default_name="nor_flash", ios=io_8bit,
                                         name_suffix="8bit"))

        io_16bit = list(io_common)
        io_16bit.append(Subsignal("a", Pins(a, dir="o", conn=conn)))
        io_16bit.append(Subsignal("dq", Pins(dq, dir="io", conn=conn, assert_width=16)))
        resources.append(Resource.family(*args, default_name="nor_flash", ios=io_16bit,
                                         name_suffix="16bit"))

    return resources


def DDR3Resource(*args, rst_n=None, clk_p, clk_n, clk_en, cs_n, we_n, ras_n, cas_n, a, ba, dqs_p, dqs_n, dq, dm, odt,
                 conn=None, diff_attrs=None, attrs=None):
    ios = []

    ios.append(Subsignal("rst", PinsN(rst_n, dir="o", conn=conn, assert_width=1)))
    ios.append(Subsignal("clk", DiffPairs(clk_p, clk_n, dir="o", conn=conn, assert_width=1), diff_attrs))
    ios.append(Subsignal("clk_en", Pins(clk_en, dir="o", conn=conn, assert_width=1)))
    ios.append(Subsignal("cs", PinsN(cs_n, dir="o", conn=conn, assert_width=1)))
    ios.append(Subsignal("we", PinsN(we_n, dir="o", conn=conn, assert_width=1)))
    ios.append(Subsignal("ras", PinsN(ras_n, dir="o", conn=conn, assert_width=1)))
    ios.append(Subsignal("cas", PinsN(cas_n, dir="o", conn=conn, assert_width=1)))
    ios.append(Subsignal("a", Pins(a, dir="o", conn=conn)))
    ios.append(Subsignal("ba", Pins(ba, dir="o", conn=conn)))
    ios.append(Subsignal("dqs", DiffPairs(dqs_p, dqs_n, dir="io", conn=conn), diff_attrs))
    ios.append(Subsignal("dq", Pins(dq, dir="io", conn=conn)))
    ios.append(Subsignal("dm", Pins(dm, dir="o", conn=conn)))
    ios.append(Subsignal("odt", Pins(odt, dir="o", conn=conn, assert_width=1)))

    if attrs is not None:
        ios.append(attrs)

    return Resource.family(*args, default_name="ddr3", ios=ios)
