![Rust](https://github.com/dfu-rs/dfu-libusb/workflows/main/badge.svg)
[![Latest Version](https://img.shields.io/crates/v/dfu-libusb.svg)](https://crates.io/crates/dfu-libusb)
![License](https://img.shields.io/crates/l/dfu-libusb)
[![Docs.rs](https://docs.rs/dfu-libusb/badge.svg)](https://docs.rs/dfu-libusb)
[![LOC](https://tokei.rs/b1/github/dfu-rs/dfu-libusb)](https://github.com/dfu-rs/dfu-libusb)
[![Dependency Status](https://deps.rs/repo/github/dfu-rs/dfu-libusb/status.svg)](https://deps.rs/repo/github/dfu-rs/dfu-libusb)

dfu-libusb
==========

Implementation of DFU using libusb and dfu-core.

Library
-------

You can use this crate as a library to your projects. It depends on
[`dfu-core`](https://github.com/dfu-rs/dfu-core)
for the actual DFU implementation and on
[`rusb`](https://github.com/a1ien/rusb)
for the libusb Rust wrapper library.

CLI
---

You can use this crate as a CLI:

```
cargo install --features cli dfu-libusb
```

This will install a binary `dfu` to your cargo binary PATH which you can use to
write firmwares to your devices.

Please run `dfu --help` for more information about how to use it.

License
-------

MIT OR Apache-2.0
