# soc

This directory contains the custom system-on-chip (SoC) design that runs the Moondancer firmware.


## Dependencies

### Python environment

Install the python dependencies needed to build the SoC bitstream with:

    cd /path/to/cynthion.git/device/gateware
    pip install .

### Yosys Toolchain

Install the latest release from:

    https://github.com/YosysHQ/oss-cad-suite-build/releases/latest

Remember to mollify Gatekeeper if you're on macOS:

    oss-cad-suite/activate

Enable environment with:

    source <path-to>/oss-cad-suite/environment

### RiscV GNU Toolchain

This is needed if you want to use gdb for firmware debugging over JTAG.

    # macOS - https://github.com/riscv-software-src/homebrew-riscv
    brew tap riscv-software-src/riscv
    brew install riscv-gnu-toolchain

    # debian
    apt install gcc-riscv64-unknown-elf


## Build SoC bitstream

Build the bitstream by running the following command from the `cynthion/package/` directory:

    make facedancer

You can load the bitstream with the `apollo` command-line tool using:

    apollo configure build/facedancer.bit
