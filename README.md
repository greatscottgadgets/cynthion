# Cynthion: a USB Test Instrument

Cynthion is an all-in-one tool for building, testing, monitoring, and experimenting with USB devices. Built around a unique FPGA-based architecture, Cynthion’s digital hardware can be fully customized to suit the application at hand. As a result, it can act as a no-compromise High-Speed USB protocol analyzer, a USB-hacking multi-tool, or a USB development platform.

Out-of-the-box, Cynthion acts as a USB protocol analyzer capable of capturing and analyzing traffic between a host and any Low-, Full-, or High-Speed ("USB 2.0") USB device. It works seamlessly with our open-source analysis software.

Combined with our LUNA gateware and Facedancer libraries, Cynthion becomes a versatile USB-hacking and development tool. Facedancer makes it quick and easy to create or tamper with real USB devices—not just emulations—even if you don’t have experience with digital-hardware design, HDL, or FPGA architecture!

For more information, see the [Cynthion Crowd Supply](https://www.crowdsupply.com/great-scott-gadgets/cynthion) page.

## Project Structure

This project is broken down into several directories:

* [`docs/`](docs/) -- sources for the Cynthion Sphinx documentation
* [`firmware/`](firmware/) -- sources for Cynthion's Rust firmware
* [`gateware/`](gateware/) -- sources for Cynthion's Amaranth gateware
* [`host/`](host/) -- sources for Cynthion's Python host-side tools

## Project Documentation

Cynthion's documentation is captured on [Read the Docs](https://cynthion.readthedocs.io/en/latest/). Raw documentation sources are in the [`docs/`](docs/) folder.

## Related Projects

Cynthion is supported by two additional firmware projects:

* [Apollo](https://github.com/greatscottgadgets/apollo/), the firmware that runs on Cynthion's debug controller, and which is responsible for configuring its FPGA.
* [Saturn-V](https://github.com/greatscottgadgets/saturn-v/), a DFU bootloader created for Cynthion.
