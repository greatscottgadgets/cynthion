# USB Analyzer

This directory contains the gateware to use a Cynthion device as a USB analyzer.

## Dependencies

### Python environment

Install the python dependencies needed with:

    cd /path/to/cynthion.git/device/gateware
    pip install .

### Yosys Toolchain

Install the latest release from:

    https://github.com/YosysHQ/oss-cad-suite-build/releases/latest

Remember to mollify Gatekeeper if you're on macOS:

    oss-cad-suite/activate

Enable environment with:

    source <path-to>/oss-cad-suite/environment

## Building and usage

Build the bitstream by running:

    python top.py -o analyzer.bit

Use Apollo to flash the bitstream to Cynthion:

    apollo flash-program analyzer.bit

Or to load it temporarily:

    apollo configure analyzer.bit

The analyzer can then be used with the [Packetry](https://github.com/greatscottgadgets/packetry/) host software.
