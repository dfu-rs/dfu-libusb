# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.6.0] - 2026-05-05

### Fixed

- Release USB interface before reset to avoid failures on macOS (#19)

### Changed

- Bump MSRV to 1.71 and edition to 2021
- Upgrade thiserror to 2.0
- Upgrade indicatif to 0.18
- Upgrade clap to 4.6

## [0.5.5] - 2025-11-12

### Changed

- Bump dfu-core to 0.9

## [0.5.4] - 2025-10-24

### Changed

- Bump dfu-core to 0.8

## [0.5.3] - 2024-11-09

### Changed

- Bump dfu-core to 0.7

## [0.5.2] - 2024-10-14

### Changed

- Avoid dependency on libusb1-sys (#18)

## [0.5.1] - 2023-05-13

### Changed

- Adopt override address in download example (#14)

## [0.5.0] - 2023-03-04

### Added

- Support for both DFU 1.1 and DFUSe (#10)

## [0.4.0] - 2023-01-26

### Added

- Add "describe" example (#12)

### Changed

- Update to dfu-core 0.5

## [0.3.1] - 2022-11-04

### Added

- `DfuLibusb::from_usb_device` constructor (#9)

## [0.3.0] - 2022-07-31

### Changed

- Update rusb (#1)

## [0.2.0] - 2022-01-29

### Changed

- Move CLI binary to example `download.rs`
- Change error type for memory layout from `String` to core's memory layout error type

## [0.1.0] - 2021-10-13

- Initial release

[Unreleased]: https://github.com/dfu-rs/dfu-libusb/compare/v0.6.0...HEAD
[0.6.0]: https://github.com/dfu-rs/dfu-libusb/compare/v0.5.5...v0.6.0
[0.5.5]: https://github.com/dfu-rs/dfu-libusb/compare/v0.5.4...v0.5.5
[0.5.4]: https://github.com/dfu-rs/dfu-libusb/compare/v0.5.3...v0.5.4
[0.5.3]: https://github.com/dfu-rs/dfu-libusb/compare/v0.5.2...v0.5.3
[0.5.2]: https://github.com/dfu-rs/dfu-libusb/compare/v0.5.1...v0.5.2
[0.5.1]: https://github.com/dfu-rs/dfu-libusb/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/dfu-rs/dfu-libusb/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/dfu-rs/dfu-libusb/compare/v0.3.1...v0.4.0
[0.3.1]: https://github.com/dfu-rs/dfu-libusb/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/dfu-rs/dfu-libusb/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/dfu-rs/dfu-libusb/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/dfu-rs/dfu-libusb/releases/tag/v0.1.0
