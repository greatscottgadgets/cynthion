# Changelog

All notable changes to the `cynthion` Python package will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!--
## [Unreleased]
-->


## [0.2.2] - 2025-06-03
### Added
* New libgreat classes for controlling the Cynthion PMODs, USER button and leds from Python.
* Documentation for controlling the Cynthion PMODs, USER button and leds from Python and Facedancer.
### Changed
* Fixed capitalisation of USER LEDs in the device overview documentation.


## [0.2.0] - 2025-05-19
> This is a breaking release which primarily affects usage of the SoC peripherals and their register interfaces.
>
> For migration information see: https://github.com/greatscottgadgets/cynthion/pull/193
>
> Please refer to the [v0.1.x](https://github.com/greatscottgadgets/cynthion/tree/v0.1.x) branch for compatibility with older Cynthion `0.1.x` releases.

### Changed
* luna-usb 0.1.x has been deprecated in favour of luna-usb 0.2.x
* luna-soc 0.2.x has been deprecated in favour of luna-soc 0.3.x
* Amaranth 0.4.x has been deprecated in favour of Amaranth 0.5.x

### Fixed
* Gateware fails to build on new oss-cad-suite releases.

---

## [0.1.9] - 2025-05-16
### Added
* Cynthion Gateware Tutorials
* Cynthion `make assets` will now build gateware if `yowasp-yosys` and `yowasp-nextpnr-ecp5` are installed.
### Changed
* Support for Python 3.8 has been ended. Cynthion now requires Python 3.9 or higher.
### Fixed
* Facedancer endpoints set max packet size for the control endpoint to 64 irrespective of script configuration.


## [0.1.8] - 2024-11-25
### Added
* [USB Emulation Tutorial](https://cynthion.readthedocs.io/en/latest/tutorials/emulation.html)
### Changed
* Apollo firmware has been updated to [1.1.1](https://github.com/greatscottgadgets/apollo/releases/tag/v1.1.1).
* Tutorials have been moved to `docs/source/tutorials/`.
* Bumped `critical-section` crate to version `=1.2.0`.
### Fixed
* Moondancer reliability improvements.


## [0.1.7] - 2024-10-10
### Added
* [USB Analysis Tutorial](https://cynthion.readthedocs.io/en/latest/tutorial_usb_analysis.html)
### Fixed:
* Fix Facedancer USB Timeout error. (#188)
### Changed
* General documentation improvements. (tx for the suggestions @x0rloser!)


## [0.1.6] - 2024-09-19
### Added
* Improved error messages.
* New `make assets` rule to automatically populate the cynthion Python package `assets/` directory for source installs.
### Fixed
* Compilation error and warnings when compiling on rustc `>= 1.8.0`.
* A number of USB Proxy bugs (#71 #134 #156).


## [0.1.5] - 2024-08-20
### Added
* Windows support for Facedancer and USB Proxy.


## [0.1.4] - 2024-08-19
### Added
* Prevent running or flashing Facedancer on Windows until full support is enabled.
### Changed
* Moondancer firmware serial example replaced with an acm serial example that works on all platforms.
* Updated libgreat to v2024.0.2
### Fixed
* Cynthion LED B would activate when taken offline.
* `cynthion setup` required udev rules to be installed in order to install udev rules.
* Fixed the stop condition for final word during analyzer read bursts.


## [0.1.3] - 2024-07-25
### Added
* The `cynthion info` command now shows information from all connected Cynthions.
* Documentation updates:
  - Documented the requirement to install libusb on macOS.
  - Developer instructions for Windows.
### Changed
* Updated the microcontroller firmware to [`apollo v1.1.0`].
### Fixed
* Flash UID returns Zeros after using `--force-offline`.
* `cynthion update` becomes unreliable when device enumeration is slow.
* Some Cynthion `r1.4` devices return `r0.0` for the hardware revision number.
* In some cases the USB analyzer would fall off the bus.

[`apollo v1.1.0`]: https://github.com/greatscottgadgets/apollo/releases/tag/v1.1.0


## [0.1.2] - 2024-07-09
### Fixed
- `cynthion` Python package assets installed to `site-packages/` instead of `site-packages/cynthion/`.
- `usb.core.USBError: [Errno 13] Access denied (insufficient permissions)` error when updating the Cynthion Microcontroller firmware.
- `usb.core.NoBackendError: No backend available` error on Windows. (requires `apollo_fpga>=1.0.7`)


## [0.1.1] - 2024-07-08
### Added
- Rust crates published for `moondancer` and its dependencies: https://crates.io/crates/moondancer
### Fixed
- `[Errno 13] Access denied (insufficient permissions)` when executing `cynthion run selftest` on Windows.
- Duplicate dependency declarations in `cynthion` Python package.


## [0.1.0] - 2024-07-06
### Added
- Initial release


[Unreleased]: https://github.com/greatscottgadgets/cynthion/compare/0.2.2...HEAD
[0.2.2]: https://github.com/greatscottgadgets/cynthion/compare/0.2.0...0.2.2
[0.2.0]: https://github.com/greatscottgadgets/cynthion/compare/0.1.9...0.2.0
[0.1.9]: https://github.com/greatscottgadgets/cynthion/compare/0.1.8...0.1.9
[0.1.8]: https://github.com/greatscottgadgets/cynthion/compare/0.1.7...0.1.8
[0.1.7]: https://github.com/greatscottgadgets/cynthion/compare/0.1.6...0.1.7
[0.1.6]: https://github.com/greatscottgadgets/cynthion/compare/0.1.5...0.1.6
[0.1.5]: https://github.com/greatscottgadgets/cynthion/compare/0.1.4...0.1.5
[0.1.4]: https://github.com/greatscottgadgets/cynthion/compare/0.1.3...0.1.4
[0.1.3]: https://github.com/greatscottgadgets/cynthion/compare/0.1.2...0.1.3
[0.1.2]: https://github.com/greatscottgadgets/cynthion/compare/0.1.1...0.1.2
[0.1.1]: https://github.com/greatscottgadgets/cynthion/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/greatscottgadgets/cynthion/releases/tag/0.1.0
