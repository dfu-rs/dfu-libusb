![Rust](https://github.com/dfu-rs/dfu-libusb/workflows/main/badge.svg)
[![Latest Version](https://img.shields.io/crates/v/dfu-libusb.svg)](https://crates.io/crates/dfu-libusb)
![License](https://img.shields.io/crates/l/dfu-libusb)
[![Docs.rs](https://docs.rs/dfu-libusb/badge.svg)](https://docs.rs/dfu-libusb)
[![Changelog](https://img.shields.io/badge/changelog-CHANGELOG.md-blue)](CHANGELOG.md)
[![Dependency Status](https://deps.rs/repo/github/dfu-rs/dfu-libusb/status.svg)](https://deps.rs/repo/github/dfu-rs/dfu-libusb)

# dfu-libusb

Flash firmware to USB devices using the [DFU] (Device Firmware Upgrade) protocol.

Built on [`dfu-core`] for the protocol implementation and [`rusb`] for USB
access via libusb.

[DFU]: https://www.usb.org/sites/default/files/DFU_1.1.pdf
[`dfu-core`]: https://github.com/dfu-rs/dfu-core
[`rusb`]: https://github.com/a1ien/rusb

## Features

- DFU 1.1 and DFUSe protocol support
- Works with any [`rusb::UsbContext`]
- Open a device by VID/PID or supply your own [`DeviceHandle`] via
  [`DfuLibusb::from_usb_device`]

[`rusb::UsbContext`]: https://docs.rs/rusb/latest/rusb/trait.UsbContext.html
[`DeviceHandle`]: https://docs.rs/rusb/latest/rusb/struct.DeviceHandle.html
[`DfuLibusb::from_usb_device`]: https://docs.rs/dfu-libusb/latest/dfu_libusb/struct.DfuLibusb.html#method.from_usb_device

## Usage

```toml
[dependencies]
dfu-libusb = "0.6"
```

```rust,no_run
use rusb::Context;
use dfu_libusb::DfuLibusb;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = Context::new()?;
    let firmware = std::fs::read("firmware.bin")?;
    let size = firmware.len() as u32;

    let mut dfu = DfuLibusb::open(&context, 0x1234, 0x5678, 0, 0)?;
    dfu.download(std::io::Cursor::new(firmware), size)?;
    Ok(())
}
```

See the [`download`](examples/download.rs) example for a more complete program
with a progress bar, wait-for-device, and post-flash reset; and
[`describe`](examples/describe.rs) to inspect a device's functional descriptor.

## License

MIT OR Apache-2.0
