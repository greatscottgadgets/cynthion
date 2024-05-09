============
Introduction
============

Setting up a Development Environment
------------------------------------

This guide highlights the installation and setup process for the Cynthion development environment.

Prerequisites
^^^^^^^^^^^^^

- `Python <https://wiki.python.org/moin/BeginnersGuide/Download>`__ v3.8, or later.
- A working FPGA toolchain. We only officially support a toolchain
   composed of the `Project Trellis <https://github.com/YosysHQ/prjtrellis>`__
   ECP5 tools, the `yosys <https://github.com/YosysHQ/yosys>`__
   synthesis suite, and the `NextPNR <https://github.com/YosysHQ/nextpnr>`__
   place-and-route tool. You can obtain the latest binary distribution of this
   software from the `oss-cad-suite-build <https://github.com/YosysHQ/oss-cad-suite-build>`__
   project.
-  A working installation of
   `Amaranth HDL <https://github.com/amaranth-lang/amaranth>`__.

Installation
^^^^^^^^^^^^


TODO

The ``apollo`` Utility
^^^^^^^^^^^^^^^^^^^^^^

The ``cynthion`` distribution depends on ``apollo``, which includes a utility
that can be used to perform various simple functions useful in development;
including simple JTAG operations, SVF playback, manipulating the board’s flash,
and debug comms.

.. code:: sh

   $ apollo
   usage: apollo [-h] command: [[argument]] [[value]]

   Utility for LUNA development via an onboard Debug Controller.

   positional arguments:
     command:    info       -- Prints information about any connected LUNA-compatible boards
                 configure  -- Uploads a bitstream to the device's FPGA over JTAG.
                 erase      -- Clears the attached board's configuration flash.
                 program    -- Programs the target bitstream onto the attached FPGA.
                 jtag-scan  -- Prints information about devices on the onboard JTAG chain.
                 flash-scan -- Attempts to detect any attached configuration flashes.
                 svf        -- Plays a given SVF file over JTAG.
                 spi        -- Sends the given list of bytes over debug-SPI, and returns the response.
                 spi-inv    -- Sends the given list of bytes over SPI with inverted CS.
                 spi-reg    -- Reads or writes to a provided register over the debug-SPI.
     [argument]  the argument to the given command; often a filename
     [value]     the value to a register write command

To have easy access to the ``apollo`` command, you’ll need to ensure
that your python binary directory is in your ``PATH``. For macOS/Linux,
this often means adding ``~/.local/bin`` to your ``PATH``.
