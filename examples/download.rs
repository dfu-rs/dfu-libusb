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
}

impl Cli {
    pub fn run(self) -> Result<()> {
        let Cli {
            path,
            device,
            intf,
            alt,
            verbose,
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

        let bar = indicatif::ProgressBar::new(file_size as u64);
        bar.set_style(
            indicatif::ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:27.cyan/blue}] \
                        {bytes}/{total_bytes} ({bytes_per_sec}) ({eta}) {msg:10}",
                )
                .progress_chars("#>-"),
        );

        let mut device: Dfu<rusb::Context> =
            DfuLibusb::open(&context, vid, pid, intf, alt).context("could not open device")?;
        device.with_progress({
            let bar = bar.clone();
            move |count| {
                bar.inc(count as u64);
            }
        });

        device
            .download(file, file_size)
            .context("could not write firmware to the device")?;

        bar.finish();

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
}

fn main() -> Result<()> {
    <Cli as clap::Parser>::from_args().run()
}
