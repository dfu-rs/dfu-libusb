use anyhow::{Context, Result};
use dfu_core::DfuIo;
use dfu_libusb::*;

#[derive(clap::Parser)]
pub struct Cli {
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

        let device: Dfu<rusb::Context> =
            DfuLibusb::open(&context, vid, pid, intf, alt).context("could not open device")?;

        println!("{:?}", device.into_inner().functional_descriptor());

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
