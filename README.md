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
