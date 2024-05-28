from amaranth.build import *


__all__ = ["LEDResources", "RGBLEDResource", "ButtonResources", "SwitchResources"]


def _SplitResources(*args, pins, invert=False, conn=None, attrs=None, default_name, dir):
    assert isinstance(pins, (str, list, dict))

    if isinstance(pins, str):
        pins = pins.split()
    if isinstance(pins, list):
        pins = dict(enumerate(pins))

    resources = []
    for number, pin in pins.items():
        ios = [Pins(pin, dir=dir, invert=invert, conn=conn)]
        if attrs is not None:
            ios.append(attrs)
        resources.append(Resource.family(*args, number, default_name=default_name, ios=ios))
    return resources


def LEDResources(*args, **kwargs):
    return _SplitResources(*args, **kwargs, default_name="led", dir="o")


def RGBLEDResource(*args, r, g, b, invert=False, conn=None, attrs=None):
    ios = []
    ios.append(Subsignal("r", Pins(r, dir="o", invert=invert, conn=conn, assert_width=1)))
    ios.append(Subsignal("g", Pins(g, dir="o", invert=invert, conn=conn, assert_width=1)))
    ios.append(Subsignal("b", Pins(b, dir="o", invert=invert, conn=conn, assert_width=1)))
    if attrs is not None:
        ios.append(attrs)
    return Resource.family(*args, default_name="rgb_led", ios=ios)


def ButtonResources(*args, **kwargs):
    return _SplitResources(*args, **kwargs, default_name="button", dir="i")


def SwitchResources(*args, **kwargs):
    return _SplitResources(*args, **kwargs, default_name="switch", dir="i")
