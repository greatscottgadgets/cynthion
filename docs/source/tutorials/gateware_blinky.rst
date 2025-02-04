Gateware Blinky
###############

This tutorial walks through the process of developing a simple "blinky" example for Cynthion's ECP5 FPGA. We'll use the `Amaranth Language & toolchain <https://amaranth-lang.org>`__ to create the design and generate a FPGA bitstream using the `Yosys Open SYnthesis Suite <https://yosyshq.net/>`__.


Prerequisites
=============

Before you begin, please make sure you have installed the Cynthion tools by following :doc:`/getting_started`.


Install Toolchain
-----------------

To generate bitstreams for Cynthion you will need a synthesis toolchain that can convert the Verilog produced by Amaranth into a bitstream for Cynthion's ECP5 FPGA.

For these tutorials we recommend `YoWASP <https://yowasp.org/>`__ which provides unofficial WebAssembly-based packages for Yosys and NextPNR. It runs a little slower than the `official OSS CAD Suite distribution <https://github.com/YosysHQ/oss-cad-suite-build>`__ but it's platform-independent and much easier to get started with.

Install YoWASP using pip:

.. code-block :: sh

    pip install yowasp-yosys yowasp-nextpnr-ecp5


Create a new Amaranth module
============================

Create a new file called ``gateware-blinky.py`` and add the following code to it:

.. code-block :: python
    :linenos:

    from amaranth import *

    class Top(Elaboratable):
        def elaborate(self, platform):
            m = Module()

            return m

Amaranth designs are built from a hierarchy of smaller modules, which are called `elaboratables`. The ``Top`` class expresses that this will be the top-level or entry-point of your design.

Right now the ``Top`` module does not do anything except to create an Amaranth ``Module()`` and return it from the ``elaborate(...)`` method.

The ``elaborate(...)`` method also takes an argument called ``platform`` that contains resources specific to the board or platform the module is compiled for.

In this case, the argument will be an instance of the `Cynthion Board Description <https://github.com/greatscottgadgets/cynthion/blob/main/cynthion/python/src/gateware/platform/cynthion_r1_4.py>`__ and contain a map of Cynthion resources such as LEDs, USB PHY's, USER PMOD connectors and board constraints.


Obtain a platform resource
==========================

Edit ``gateware-blinky.py`` and add the highlighted line:

.. code-block :: python
    :linenos:
    :emphasize-lines: 7

    from amaranth import *

    class Top(Elaboratable):
        def elaborate(self, platform):
            m = Module()

            leds: Signal(6) = Cat(platform.request("led", n).o for n in range(0, 6))

            return m

Amaranth platform resources can be obtained via the ``platform.request(name, number=0)`` method where ``name`` is the name of the resource and ``number`` is the index of the resource if there are more than one defined.

In this case we use a Python list comprehension to obtain all six FPGA led's and concatenate them into a six-bit addressable Amaranth ``Signal`` using the ``Cat`` operation.


Timer State
===========

To make the LED blink at predictable intervals we'll use a simple timer.

To start with, let's define the timer state by adding the highlighted lines:

.. code-block :: python
    :linenos:
    :emphasize-lines: 9-10

    from amaranth import *

    class Top(Elaboratable):
        def elaborate(self, platform):
            m = Module()

            leds: Signal(6) = Cat(platform.request("led", n).o for n in range(0, 6))

            half_freq: int    = int(60e6 // 2)
            timer: Signal(25) = Signal(range(half_freq))

            return m

First we'll declare a variable ``half_freq`` which is exactly half of Cynthion FPGA's default clock frequency in Hz, next we'll declare ``timer`` to be an Amaranth ``Signal`` which is wide enough to contain a value equal to ``half_freq - 1``.

If we increment the ``timer`` by one for each clock cycle until it reaches ``half_freq - 1`` we get a timer with a 500ms period.


Timer Logic
===========

Now that we have a state definition for our timer we can move forward to the implementation logic, edit your file and add the highlighted lines:

.. code-block :: python
    :linenos:
    :emphasize-lines: 12-17

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

Amaranth combines normal Python expressions with Amaranth in order to describe a design. Whenever you see the prefix ``m.`` you are making a call to the ``Module`` object you created at the beginning of the ``elaborate(...)`` method. These calls are what build the logic which makes up a design.

The ``with m.If(...):`` and ``with m.Else():`` blocks operate much like you'd expect where, every clock-cycle, the expression ``timer == half_freq - 1`` will be evaluated and trigger the corresponding branch.

The first block represents the point at which the timer has expired and we'd like to change the state of the LEDs and then reset the timer back to zero.

In the second block the timer is still active so we simply increment ``timer`` by one.


Put It All Together
===================

The contents of ``gateware-blinky.py`` should now look like this:

.. literalinclude:: ../../../cynthion/python/examples/tutorials/gateware-blinky.py
   :language: python
   :linenos:


Build and Upload FPGA Bitstream
===============================

Make sure your Cynthion CONTROL port is plugged into the host, open a terminal and then run:

.. code-block :: sh

    python gateware-blinky.py

The blinky gateware will now be synthesized, placed, routed and automatically uploaded to the Cynthion's FPGA.

Once this process has completed successfully all six of Cynthion's FPGA LEDs should be flashing on and off.


Exercises
=========

1. Modify the tutorial to turn the FPGA LEDs into a binary counter that increments by one every 250ms.
2. Connect the USER PMOD A port to the output of your counter and use a logic analyzer (e.g. GreatFET One) to view the values as they increment.


More information:
=================

* `Amaranth Language & tool change documentation <https://amaranth-lang.org/docs/amaranth/>`__.
