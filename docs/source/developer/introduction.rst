============
Introduction
============


Setting up a Development Environment
------------------------------------

This guide highlights the installation and setup process for setting up a local copy of the Cynthion source code for development.


Prerequisites
-------------

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
- A working `Rust development environment <https://www.rust-lang.org/learn/get-started>`__ if you want to develop firmware for Cynthion's SoC bitstream.
- A `RISC-V GNU Compiler Toolchain <https://github.com/riscv-collab/riscv-gnu-toolchain>`__ if you want to use ``gdb`` for SoC firmware debugging over JTAG.


Installation
------------

For development you'll need a local copy of the cynthion repository:

.. code-block:: sh

    git clone https://github.com/greatscottgadgets/cynthion.git
    cd cynthion/

To install the ``cynthion`` python package to allow for in-place editing of the sources you can use the ``pip --editable`` command:

.. code-block:: sh

    # change to the cynthion python package directory
    cd cynthion/python/

    # install the cynthion python package, including dependencies required for gateware development
    pip install --editable ".[gateware, gateware-soc]"


Bitstream Generation
--------------------

The Cynthion repository contains gateware for two designs:

- ``analyzer`` -- USB analyzer for using Cynthion with Packetry.
- ``soc``      -- System-on-Chip for using Cynthion with Facedancer.

Bitstreams can be generated from the ``cynthion/python/`` sub-directory as follows:

Analyzer Gateware
^^^^^^^^^^^^^^^^^

.. code-block:: sh

    # change to the cynthion python package directory
    cd cynthion/python/

    # generate bitstream
    python3 -m cynthion.gateware.analyzer.top

SoC Gateware
^^^^^^^^^^^^

In the ``cynthion/cynthion/python/`` directory run the following command:

.. code-block:: sh

    # change to the cynthion python package directory
    cd cynthion/python/

    # generate bitstream
    python3 -m cynthion.gateware.soc.top

Additional Options
^^^^^^^^^^^^^^^^^^

Additional options for bitstream generation can be listed by appending ``--help`` to the command:

.. code-block:: text

    $ python3 -m cynthion.gateware.analyzer.top --help

    usage: top.py [-h] [--output filename] [--erase] [--upload] [--flash]
                  [--dry-run] [--keep-files] [--fpga part_number] [--console port]

    Gateware generation/upload script for 'USBAnalyzerApplet' gateware.

    optional arguments:
      -h, --help            show this help message and exit
      --output filename, -o filename
                            Build and output a bitstream to the given file.
      --erase, -E           Clears the relevant FPGA's flash before performing
                            other options.
      --upload, -U          Uploads the relevant design to the target hardware.
                            Default if no options are provided.
      --flash, -F           Flashes the relevant design to the target hardware's
                            configuration flash.
      --dry-run, -D         When provided as the only option; builds the relevant
                            bitstream without uploading or flashing it.
      --keep-files          Keeps the local files in the default `build` folder.
      --fpga part_number    Overrides build configuration to build for a given
                            FPGA. Useful if no FPGA is connected during build.
      --console port        Attempts to open a convenience 115200 8N1 UART console
                            on the specified port immediately after uploading.

Firmware Compilation
--------------------

Firmware for the Cynthion SoC can be found in the ``firmware/moondancer/`` sub-directory.

You can rebuild the firmware using ``cargo`` as follows:

.. code-block:: text

    # change to the cynthion firmware directory
    cd firmware/moondancer/

    # rebuild the firmware
    cargo build --release

To upload the firmware binary to Cynthion and flash the SoC bitstream you can run:

.. code-block:: text

    # change to the cynthion firmware directory
    cd firmware/moondancer/

    # upload firmware and run it
    cargo run --release

.. note::

    By default the firmware's flash script will look for the SoC UART
    on ``/dev/ttyACM0``, if this is not the case on your machine you
    will need to specify the correct path using the ``UART`` environment
    variable, for example:

    .. code-block:: sh

        UART=/dev/cu.usbmodem22401 cargo run --release

    By default the SoC bitstream is obtained from the latest build in
    ``cynthion/python/build/top.bit`` but you can override
    it with:

    .. code-block:: sh

        BITSTREAM=path/to/bitstream.bit cargo run --release


Running Firmware Unit Tests
^^^^^^^^^^^^^^^^^^^^^^^^^^^

Once the firmware is running on the SoC you can execute some unittests to exercise the firmware.

In order to do this you will need to connect both the ``control`` and
``aux`` ports of the Cynthion to the host and then run:

.. code-block:: sh

    # change to the cynthion firmware directory
    cd firmware/moondancer/

    # run firmware unit tests
    python -m unittest


Firmware Examples
^^^^^^^^^^^^^^^^^

There are a number of firmware examples in the ``firmware/moondancer/examples/`` sub-directory.

.. code-block:: sh

    # change to the cynthion firmware directory
    cd firmware/moondancer/

    # run example
    cargo run --release --example <example name>
