 usbd-webusb
 ===========

 WebUSB implementation for [usb-device](https://crates.io/crates/usb-device).

 WebUSB is a proposed JavaScript API standard for securely providing access to USB devices from web pages.

 License
 =======

 Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

 at your option.

 Example
 =======

 A full example requires the use of a hardware driver, but the hardware independent part looks like that:

 ```rust
 use usbd_webusb::*;

 // Creates a WebUSB class instance with URL "https://google.com"
 // You can also provide a custom URL with scheme url_scheme::CUSTOM
 let wusb = WebUsb::new(&usb_bus, url_scheme::HTTPS, "google.com");
 let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
     .product("Test product")
     .build();

 loop {
     if !usb_dev.poll(&mut [&mut wusb]) {
         continue;
     }
 }
 ```
