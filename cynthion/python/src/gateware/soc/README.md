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

This is needed to build litex-bios:

    # macOS - https://github.com/riscv-software-src/homebrew-riscv
    brew tap riscv-software-src/riscv
    brew install riscv-gnu-toolchain

    # debian
    apt install gcc-riscv64-unknown-elf


## Build SoC bitstream

Build the bitstream with:

    make top

You can load the bitstream with `apollo` using:

    apollo configure build/top.bit

Finally, you can check if the soc is working with something like:

    # linux
    picocom --imap lfcrlf -b 115200 /dev/ttyACM0

    # macos
    picocom --imap lfcrlf -b 115200 /dev/cu.usbserial-D00137

Hit enter after connecting and you should see a prompt like:

    BIOS>


## Known Issues

### Firmware compilation errors on x86_64 Mac:

See:

* https://github.com/riscv-software-src/homebrew-riscv/issues/99
* https://github.com/riscv-software-src/homebrew-riscv/pull/101

Use this for now:

    brew tap gmerlino/homebrew-riscv
    brew install riscv-gnu-toolchain

Also try:

1. Get homebrew cache directory with: `brew --cache`
1. Then delete: `riscv-gnu-toolchain--git`
