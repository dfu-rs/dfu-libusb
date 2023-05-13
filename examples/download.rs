use anyhow::{Context, Result};
use dfu_libusb::*;
use std::convert::TryFrom;
use std::io;
use std::io::Seek;
use std::path::PathBuf;

#[derive(clap::Parser)]
pub struct Cli {
    /// Path to the firmware file to write to the device.
    path: PathBuf,

    /// Wait for the device to appear.
    #[clap(short, long)]
    wait: bool,

    /// Reset after download.
    #[clap(short, long)]
    reset: bool,

    /// Specify Vendor/Product ID(s) of DFU device.
    #[clap(
        long,
        short,
        parse(try_from_str = Self::parse_vid_pid), name = "vendor>:<product",
    )]
    device: (u16, u16),

    /// Specify the DFU Interface number.
    #[clap(long, short, default_value = "0")]
    intf: u8,

    /// Specify the Altsetting of the DFU Interface by number.
    #[clap(long, short, default_value = "0")]
    alt: u8,

    /// Enable verbose logs.
    #[clap(long, short)]
    verbose: bool,

    /// Override start address (e.g. 0x0800C000)
    #[clap(long, short, value_parser=Self::parse_address, name="address")]
    override_address: Option<u32>,
}

impl Cli {
    pub fn run(self) -> Result<()> {
        let Cli {
            path,
            wait,
            reset,
            device,
            intf,
            alt,
            verbose,
            override_address,
        } = self;
        let log_level = if verbose {
            simplelog::LevelFilter::Trace
        } else {
            simplelog::LevelFilter::Info
        };
        simplelog::SimpleLogger::init(log_level, Default::default())?;
        let (vid, pid) = device;
        let context = rusb::Context::new()?;
        let mut file = std::fs::File::open(path).context("could not open firmware file")?;

        let file_size = u32::try_from(file.seek(io::SeekFrom::End(0))?)
            .context("the firmware file is too big")?;
        file.seek(io::SeekFrom::Start(0))?;

        let mut device: Dfu<rusb::Context> = match DfuLibusb::open(&context, vid, pid, intf, alt) {
            Err(Error::CouldNotOpenDevice) if wait => {
                let bar = indicatif::ProgressBar::new_spinner();
                bar.set_message("Waiting for device");

                loop {
                    std::thread::sleep(std::time::Duration::from_millis(250));
                    match DfuLibusb::open(&context, vid, pid, intf, alt) {
                        Err(Error::CouldNotOpenDevice) => bar.tick(),
                        r => {
                            bar.finish();
                            break r;
                        }
                    }
                }
            }
            r => r,
        }
        .context("could not open device")?;

        let bar = indicatif::ProgressBar::new(file_size as u64);
        bar.set_style(
            indicatif::ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:27.cyan/blue}] \
                        {bytes}/{total_bytes} ({bytes_per_sec}) ({eta}) {msg:10}",
                )
                .progress_chars("#>-"),
        );

        device.with_progress({
            let bar = bar.clone();
            move |count| {
                bar.inc(count as u64);
                if bar.position() == file_size as u64 {
                    bar.finish();
                }
            }
        });

        if let Some(address) = override_address {
            device.override_address(address);
        }

        match device.download(file, file_size) {
            Ok(_) => (),
            Err(Error::LibUsb(..)) if bar.is_finished() => {
                println!("USB error after upload; Device reset itself?");
                return Ok(());
            }
            e => return e.context("could not write firmware to the device"),
        }

        if reset {
            // Detach isn't strictly meant to be sent after a download, however u-boot in
            // particular will only switch to the downloaded firmware if it saw a detach before
            // a usb reset. So send a detach blindly...
            //
            // This matches the behaviour of dfu-util so should be safe
            let _ = device.detach();
            println!("Resetting device");
            device.usb_reset()?;
        }

        Ok(())
    }

    pub fn parse_vid_pid(s: &str) -> Result<(u16, u16)> {
        let (vid, pid) = s
            .split_once(':')
            .context("could not parse VID/PID (missing `:')")?;
        let vid = u16::from_str_radix(vid, 16).context("could not parse VID")?;
        let pid = u16::from_str_radix(pid, 16).context("could not parse PID")?;

        Ok((vid, pid))
    }

    pub fn parse_address(s: &str) -> Result<u32> {
        if s.to_ascii_lowercase().starts_with("0x") {
            u32::from_str_radix(&s[2..], 16).context("could not parse override address")
        } else {
            s.parse().context("could not parse override address")
        }
    }
}

fn main() -> Result<()> {
    <Cli as clap::Parser>::from_args().run()
}
