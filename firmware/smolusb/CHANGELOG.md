# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.2] - 2025-06-03
### Fixed
* Elided unecssary lifetime annotations.

<!--
## [Unreleased]
-->
## [0.2.0] - 2025-05-19
> This is a breaking release which primarily affects usage of the SoC peripherals and their register interfaces.
>
> For migration information see: https://github.com/greatscottgadgets/cynthion/pull/193
>
> Please refer to the [v0.1.x](https://github.com/greatscottgadgets/cynthion/tree/v0.1.x) branch for compatibility with older `0.1.x` releases.

### Changed
* Update luna-soc usage to 0.3.x

---

## [0.1.6] - 2024-09-19
### Added
- New trait method: `clear_feature_endpoint_halt()`.

## [0.1.5] - 2024-08-20
### Added
* Support for Windows Compatible ID Descriptors.

## [0.1.4] - 2024-08-19
### Changed
* Replaced ch341 serial class with acm.

## [0.1.1] - 2024-07-08
### Added
- Initial release

[Unreleased]: https://github.com/greatscottgadgets/cynthion/compare/0.2.2...HEAD
[0.2.2]: https://github.com/greatscottgadgets/cynthion/compare/0.2.0...0.2.2
[0.2.0]: https://github.com/greatscottgadgets/cynthion/compare/0.1.6...0.2.0
[0.1.6]: https://github.com/greatscottgadgets/cynthion/compare/0.1.5...0.1.6
[0.1.5]: https://github.com/greatscottgadgets/cynthion/compare/0.1.4...0.1.5
[0.1.4]: https://github.com/greatscottgadgets/cynthion/compare/0.1.1...0.1.4
[0.1.1]: https://github.com/greatscottgadgets/cynthion/releases/tag/0.1.1
