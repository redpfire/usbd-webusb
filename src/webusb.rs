#![allow(non_snake_case)]

use usb_device::{
    class_prelude::*,
    Result,
};

use core::marker::PhantomData;
use core::mem;

pub(crate) const GET_URL: u16 = 2;
pub(crate) const DESC_LN: usize = 128;

/// URL scheme used in URL descriptor.
pub mod url_scheme {
    pub const HTTP: u8 = 0;
    pub const HTTPS: u8 = 1;
    pub const CUSTOM: u8 = 0xff;
}

/// WebUSB class
pub struct WebUsb<B: UsbBus> {
    //comm_ep: EndpointIn<'a, B>,
    p: PhantomData<B>,
    iLandingPage: u8,
    bVendorCode: u8,
    desc: [u8; DESC_LN],
    desc_len: usize,
}

impl<B: UsbBus> WebUsb<B> {
    /// Creates new WebUSB class with provided `url` and `scheme`.
    pub fn new(_alloc: &UsbBusAllocator<B>, scheme: u8, url: &'static str) -> WebUsb<B> {
        let mut desc: [u8; DESC_LN] = unsafe { mem::zeroed() };
        let ln: u8 = url.len() as u8;
        desc[0..2].copy_from_slice(&[ln+3, 0x03]);
        desc[2] = scheme;
        desc[3..3+(ln as usize)].copy_from_slice(url.as_bytes());
        WebUsb {
            //comm_ep: alloc.interrupt(8, 255),
            p: PhantomData,
            iLandingPage: 1,
            bVendorCode: 1,
            desc: desc,
            desc_len: (ln+3) as usize,
        }
    }
}

impl<B: UsbBus> UsbClass<B> for WebUsb<B> {
    fn get_bos_descriptors(&self, w: &mut BosWriter) -> Result<()> {
        w.capability(0x05, &[
            0x0,
            0x38, 0xb6, 0x08, 0x34, 0xa9,
            0x09, 0xa0, 0x47, 0x8b, 0xfd,
            0xa0, 0x76, 0x88, 0x15, 0xb6,
            0x65,  // PlatformCapabilityUUID

            0x00, 0x01, // bcdVersion of WebUSB
            self.bVendorCode, // bVendorCode
            self.iLandingPage, // iLandingPage
        ])
    }

    fn control_in(&mut self, xfer: ControlIn<B>) {
        let req = *xfer.request();

        if !(req.request_type == control::RequestType::Vendor
             && req.recipient == control::Recipient::Device
             && req.request == self.bVendorCode
             && req.value == self.iLandingPage as u16
             )
        {
            return;
        }

        match req.index {
            GET_URL => {
                xfer.accept_with(&self.desc[..self.desc_len]).ok()
            },
            _ => xfer.reject().ok(),
        };
    }
}
