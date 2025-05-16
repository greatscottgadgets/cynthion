# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!--
## [Unreleased]
-->

## [0.1.9] - 2025-05-16
### Fixed
* Use correct usb hal write function for facedancer control endpoint.


## [0.1.8] - 2024-11-25
### Added
- New moondancer verb: `ep_out_interface_enable()`.

## [0.1.7] - 2024-10-10
### Fixed
- Shorten connect delay.
- Prime to receive host zlp before writing respone.
- Fix libgreat command abort handling.

## [0.1.6] - 2024-09-19
### Fixed
- Moondancer bulk write operations would fail if the host responded with a halt condition.
- Control writes could fail due to incorrect ZLP behavior.

## [0.1.5] - 2024-08-20
### Added
* Support for Windows Compatible ID Descriptors on the Cynthion Control interface.

## [0.1.4] - 2024-08-19
### Changed
* Replaced ch341 serial example with an acm serial example.

## [0.1.1] - 2024-07-08
### Added
- Initial release

[Unreleased]: https://github.com/greatscottgadgets/cynthion/compare/0.1.8...HEAD
[0.1.8]: https://github.com/greatscottgadgets/cynthion/compare/0.1.6...0.1.8
[0.1.6]: https://github.com/greatscottgadgets/cynthion/compare/0.1.5...0.1.6
[0.1.5]: https://github.com/greatscottgadgets/cynthion/compare/0.1.4...0.1.5
[0.1.4]: https://github.com/greatscottgadgets/cynthion/compare/0.1.1...0.1.4
[0.1.1]: https://github.com/greatscottgadgets/cynthion/releases/tag/0.1.1
