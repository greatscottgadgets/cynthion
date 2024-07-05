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
- A working `Rust development environment <https://www.rust-lang.org/learn/get-started>`__ if you want to develop firmware for Cynthion's SoC bitstream.
- A `RISC-V GNU Compiler Toolchain <https://github.com/riscv-collab/riscv-gnu-toolchain>`__ if you want to use ``gdb`` for SoC firmware debugging over JTAG.


Installation
------------

For development you'll need a local copy of the Cynthion repository:

.. code-block:: sh

    git clone https://github.com/greatscottgadgets/cynthion.git

To install the ``cynthion`` Python package to allow for in-place editing of the sources you can use the ``pip --editable`` command:

.. code-block:: sh

    # change to the 'cynthion' Python package directory
    cd cynthion/python/

    # install the 'cynthion' Python package, including dependencies required for gateware development
    pip install --editable .
