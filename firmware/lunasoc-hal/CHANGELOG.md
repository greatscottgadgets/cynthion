# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!--
## [Unreleased]
-->

## [0.1.8] - 2024-11-25
### Fixed
- Un-prime all OUT endpoints and disable interface when connecting or disconnecting the device controller.

## [0.1.7] - 2024-10-10
### Fixed
- Reset USB Endpoint FIFO's before issuing stall.

## [0.1.6] - 2024-09-19
### Added
- Support USB Low-speed operation.
### Changed
- Refactor `clear_feature_endpoint_halt()` to take an endpoint number and direction rather than endpoint address.
### Fixed
- ZLP was not being sent when sending an empty USB packet.

## [0.1.5] - 2024-08-20
### Fixed
- Control transactions could break if the requested length was a multiple of the packet size.

## [0.1.1] - 2024-07-08
### Added
- Initial release

[Unreleased]: https://github.com/greatscottgadgets/cynthion/compare/0.1.8...HEAD
[0.1.8]: https://github.com/greatscottgadgets/cynthion/compare/0.1.6...0.1.8
[0.1.6]: https://github.com/greatscottgadgets/cynthion/compare/0.1.5...0.1.6
[0.1.5]: https://github.com/greatscottgadgets/cynthion/compare/0.1.4...0.1.5
[0.1.1]: https://github.com/greatscottgadgets/cynthion/releases/tag/0.1.1
