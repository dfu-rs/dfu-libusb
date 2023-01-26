use std::cell::RefCell;
use std::marker;
use thiserror::Error;

pub type Dfu<C> = dfu_core::sync::DfuSync<DfuLibusb<C>, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Could not find device or an error occurred.")]
    CouldNotOpenDevice,
    #[error(transparent)]
    Dfu(#[from] dfu_core::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Could not parse memory layout: {0}")]
    MemoryLayout(dfu_core::memory_layout::Error),
    #[error("rusb: {0}")]
    LibUsb(#[from] rusb::Error),
    #[error("The device has no languages.")]
    MissingLanguage,
    #[error("Could not find interface.")]
    InvalidInterface,
    #[error("Could not find alt interface.")]
    InvalidAlt,
    #[error("Could not parse interface string.")]
    InvalidInterfaceString,
    #[error("Could not parse address.")]
    InvalidAddress,
    #[error("Could not parse functional descriptor: {0}")]
    FunctionalDescriptor(#[from] dfu_core::functional_descriptor::Error),
    #[error("No DFU capable device found.")]
    NoDfuCapableDeviceFound,
}

pub struct DfuLibusb<C: rusb::UsbContext> {
    usb: RefCell<rusb::DeviceHandle<C>>,
    protocol: dfu_core::DfuProtocol<dfu_core::memory_layout::MemoryLayout>,
    timeout: std::time::Duration,
    iface: u16,
    functional_descriptor: dfu_core::functional_descriptor::FunctionalDescriptor,
    marker: marker::PhantomData<C>,
}

impl<C: rusb::UsbContext> dfu_core::DfuIo for DfuLibusb<C> {
    type Read = usize;
    type Write = usize;
    type Reset = ();
    type Error = Error;
    type MemoryLayout = dfu_core::memory_layout::MemoryLayout;

    #[allow(unused_variables)]
    fn read_control(
        &self,
        request_type: u8,
        request: u8,
        value: u16,
        buffer: &mut [u8],
    ) -> Result<Self::Read, Self::Error> {
        // TODO: do or do not? there is no try
        let request_type = request_type | libusb1_sys::constants::LIBUSB_ENDPOINT_IN;
        let res = self.usb.borrow().read_control(
            request_type,
            request,
            value,
            self.iface,
            buffer,
            self.timeout,
        );
        assert!(
            !matches!(res, Err(rusb::Error::InvalidParam)),
            "invalid param: {:08b} {:?}",
            request_type,
            res,
        );
        Ok(res?)
    }

    #[allow(unused_variables)]
    fn write_control(
        &self,
        request_type: u8,
        request: u8,
        value: u16,
        buffer: &[u8],
    ) -> Result<Self::Write, Self::Error> {
        let res = self.usb.borrow().write_control(
            request_type,
            request,
            value,
            self.iface,
            buffer,
            self.timeout,
        );
        assert!(
            !matches!(res, Err(rusb::Error::InvalidParam)),
            "invalid param: {:08b}",
            request_type,
        );
        Ok(res?)
    }

    fn usb_reset(&self) -> Result<Self::Reset, Self::Error> {
        Ok(self.usb.borrow_mut().reset()?)
    }

    fn protocol(&self) -> &dfu_core::DfuProtocol<Self::MemoryLayout> {
        &self.protocol
    }

    fn functional_descriptor(&self) -> &dfu_core::functional_descriptor::FunctionalDescriptor {
        &self.functional_descriptor
    }
}

impl<C: rusb::UsbContext> DfuLibusb<C> {
    pub fn open(context: &C, vid: u16, pid: u16, iface: u8, alt: u8) -> Result<Dfu<C>, Error> {
        let (device, handle) = Self::open_device(context, vid, pid)?;
        Self::from_usb_device(device, handle, iface, alt)
    }

    pub fn from_usb_device(
        device: rusb::Device<C>,
        mut handle: rusb::DeviceHandle<C>,
        iface: u8,
        alt: u8,
    ) -> Result<Dfu<C>, Error> {
        use std::convert::TryFrom;

        let timeout = std::time::Duration::from_secs(3);
        handle.claim_interface(iface)?;
        handle.set_alternate_setting(iface, alt)?;
        let device_descriptor = device.device_descriptor()?;
        let languages = handle.read_languages(timeout)?;
        let lang = languages.get(0).ok_or(Error::MissingLanguage)?;

        for index in 0..device_descriptor.num_configurations() {
            let config_descriptor = device.config_descriptor(index)?;

            let interface = config_descriptor
                .interfaces()
                .find(|x| x.number() == iface)
                .ok_or(Error::InvalidInterface)?;
            let iface_desc = interface
                .descriptors()
                .find(|x| x.setting_number() == alt)
                .ok_or(Error::InvalidAlt)?;
            let interface_string = handle.read_interface_string(*lang, &iface_desc, timeout)?;

            let (rest, memory_layout) = interface_string
                .rsplit_once('/')
                .ok_or(Error::InvalidInterfaceString)?;
            let memory_layout = dfu_core::memory_layout::MemoryLayout::try_from(memory_layout)
                .map_err(Error::MemoryLayout)?;
            let (_rest, address) = rest.rsplit_once('/').ok_or(Error::InvalidInterfaceString)?;
            let address = address
                .strip_prefix("0x")
                .and_then(|s| u32::from_str_radix(s, 16).ok())
                .ok_or(Error::InvalidAddress)?;
            let protocol = dfu_core::DfuProtocol::Dfuse {
                address,
                memory_layout,
            };

            if let Some(functional_descriptor) =
                Self::find_functional_descriptor(&handle, &config_descriptor, timeout)
                    .transpose()?
            {
                let io = DfuLibusb {
                    usb: RefCell::new(handle),
                    protocol,
                    timeout,
                    iface: iface as u16,
                    functional_descriptor,
                    marker: marker::PhantomData,
                };

                return Ok(dfu_core::sync::DfuSync::new(io));
            }
        }

        Err(Error::NoDfuCapableDeviceFound)
    }

    fn open_device(
        context: &C,
        vid: u16,
        pid: u16,
    ) -> Result<(rusb::Device<C>, rusb::DeviceHandle<C>), Error> {
        for device in context.devices()?.iter() {
            let device_desc = match device.device_descriptor() {
                Ok(x) => x,
                Err(_) => continue,
            };

            if device_desc.vendor_id() == vid && device_desc.product_id() == pid {
                let handle = device.open()?;
                return Ok((device, handle));
            }
        }

        Err(Error::CouldNotOpenDevice)
    }

    fn find_functional_descriptor(
        handle: &rusb::DeviceHandle<C>,
        config: &rusb::ConfigDescriptor,
        timeout: std::time::Duration,
    ) -> Option<Result<dfu_core::functional_descriptor::FunctionalDescriptor, Error>> {
        macro_rules! find_func_desc {
            ($data:expr) => {{
                if let Some(func_desc) =
                    dfu_core::functional_descriptor::FunctionalDescriptor::from_bytes($data)
                {
                    return Some(func_desc.map_err(Into::into));
                }
            }};
        }

        find_func_desc!(config.extra());

        for if_desc in config.interfaces().flat_map(|x| x.descriptors()) {
            find_func_desc!(if_desc.extra());
        }

        let mut buffer = [0x00; 9];
        match handle.read_control(
            libusb1_sys::constants::LIBUSB_ENDPOINT_IN,
            libusb1_sys::constants::LIBUSB_REQUEST_GET_DESCRIPTOR,
            0x2100,
            0,
            &mut buffer,
            timeout,
        ) {
            Ok(n) => find_func_desc!(&buffer[..n]),
            Err(err) => return Some(Err(err.into())),
        }

        None
    }
}
