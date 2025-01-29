============
Introduction
============


Setting up a Development Environment
------------------------------------

This guide highlights the installation and setup process for setting up a local copy of the Cynthion source code for development.


Prerequisites
-------------

- `Python <https://wiki.python.org/moin/BeginnersGuide/Download>`__ v3.9, or later.
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


.. tab:: Linux / macOS

    Use git to clone the repository:

        .. code-block:: sh

            git clone https://github.com/greatscottgadgets/cynthion.git

.. tab:: Windows

    Please perform the following steps to enable support for symlinks before attempting to clone the repository on Windows:

        1. Open the `"For developers"` page in `System settings` and enable `Developer Mode <https://learn.microsoft.com/en-us/windows/apps/get-started/developer-mode-features-and-debugging#additional-developer-mode-features>`__.
        2. Restart your computer.
        3. Open the Group Policy editor: `gpedit.msc`
        4. Navigate to `Computer Configuration → Windows Settings → Security Settings → Local Policies → User Rights Assignment → Create symbolic links` and check that you have user permission to create symbolic links.
        5. Restart your computer.
        6. Configure git to enable symbolic links on Windows:

           .. code-block:: sh

               git config --global core.symlinks true

    Use git to clone the repository:

        .. code-block:: sh

            git clone https://github.com/greatscottgadgets/cynthion.git


.. note::

    To install the ``cynthion`` Python package and allow for in-place editing of the sources you can use the ``pip --editable`` command:

    .. code-block:: sh

        # change to the 'cynthion' Python package directory
        cd cynthion/python/

        # install the 'cynthion' Python package, including dependencies required for gateware development
        pip install --editable .
