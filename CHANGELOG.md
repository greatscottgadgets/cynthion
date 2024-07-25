# Changelog

All notable changes to the `cynthion` Python package will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!--
## [Unreleased]
-->

## [0.1.3] - 2024-08-25
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

[Unreleased]: https://github.com/greatscottgadgets/cynthion/compare/0.1.3...HEAD
[0.1.3]: https://github.com/greatscottgadgets/cynthion/compare/0.1.2...0.1.3
[0.1.2]: https://github.com/greatscottgadgets/cynthion/compare/0.1.1...0.1.2
[0.1.1]: https://github.com/greatscottgadgets/cynthion/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/greatscottgadgets/cynthion/releases/tag/0.1.0
