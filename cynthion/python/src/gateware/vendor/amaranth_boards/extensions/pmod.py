# Reference: https://www.digilentinc.com/Pmods/Digilent-Pmod_%20Interface_Specification.pdf

from amaranth.build import *


__all__ = [
    "PmodGPIOType1Resource",
    "PmodSPIType2Resource",
    "PmodSPIType2AResource",
    "PmodUARTType3Resource",
    "PmodUARTType4Resource",
    "PmodUARTType4AResource",
    "PmodHBridgeType5Resource",
    "PmodDualHBridgeType6Resource",
]


def PmodGPIOType1Resource(name, number, *args, pmod):
    return Resource(name, number,
        Pins("1 2 3 4", dir="io", conn=("pmod", pmod)),
        *args
    )


def PmodSPIType2Resource(name, number, *args, pmod):
    return Resource(name, number,
        Subsignal("cs",   PinsN("1", dir="o", conn=("pmod", pmod))),
        Subsignal("clk",   Pins("2", dir="o", conn=("pmod", pmod))),
        Subsignal("copi",  Pins("3", dir="o", conn=("pmod", pmod))),
        Subsignal("cipo",  Pins("4", dir="i", conn=("pmod", pmod))),
        *args
    )


def PmodSPIType2AResource(name, number, *args, pmod):
    return Resource(name, number,
        Subsignal("cs",   PinsN("1", dir="o", conn=("pmod", pmod))),
        Subsignal("clk",   Pins("2", dir="o", conn=("pmod", pmod))),
        Subsignal("copi",  Pins("3", dir="o", conn=("pmod", pmod))),
        Subsignal("cipo",  Pins("4", dir="i", conn=("pmod", pmod))),
        Subsignal("int",   Pins("7", dir="i", conn=("pmod", pmod))),
        Subsignal("reset", Pins("8", dir="o", conn=("pmod", pmod))),
        *args
        )


def PmodUARTType3Resource(name, number, *args, pmod):
    return Resource(name, number,
        Subsignal("cts",   Pins("1", dir="o", conn=("pmod", pmod))),
        Subsignal("rts",   Pins("2", dir="i", conn=("pmod", pmod))),
        Subsignal("rx",    Pins("3", dir="i", conn=("pmod", pmod))),
        Subsignal("tx",    Pins("4", dir="o", conn=("pmod", pmod))),
        *args
    )


def PmodUARTType4Resource(name, number, *args, pmod):
    return Resource(name, number,
        Subsignal("cts",   Pins("1", dir="i", conn=("pmod", pmod))),
        Subsignal("tx",    Pins("2", dir="o", conn=("pmod", pmod))),
        Subsignal("rx",    Pins("3", dir="i", conn=("pmod", pmod))),
        Subsignal("rts",   Pins("4", dir="o", conn=("pmod", pmod))),
        *args
    )


def PmodUARTType4AResource(name, number, *args, pmod):
    return Resource(name, number,
        Subsignal("cts",   Pins("1", dir="i", conn=("pmod", pmod))),
        Subsignal("tx",    Pins("2", dir="o", conn=("pmod", pmod))),
        Subsignal("rx",    Pins("3", dir="i", conn=("pmod", pmod))),
        Subsignal("rts",   Pins("4", dir="o", conn=("pmod", pmod))),
        Subsignal("int",   Pins("7", dir="i", conn=("pmod", pmod))),
        Subsignal("reset", Pins("8", dir="o", conn=("pmod", pmod))),
        *args
    )


def PmodHBridgeType5Resource(name, number, *args, pmod):
    return Resource(name, number,
        Subsignal("dir",   Pins("1", dir="o", conn=("pmod", pmod))),
        Subsignal("en",    Pins("2", dir="o", conn=("pmod", pmod))),
        Subsignal("sa",    Pins("3", dir="i", conn=("pmod", pmod))),
        Subsignal("sb",    Pins("4", dir="i", conn=("pmod", pmod))),
        *args
    )


def PmodDualHBridgeType6Resource(name, number, *args, pmod):
    return Resource(name, number,
        Subsignal("dir",   Pins("1 3", dir="o", conn=("pmod", pmod))),
        Subsignal("en",    Pins("2 4", dir="o", conn=("pmod", pmod))),
        *args
    )
