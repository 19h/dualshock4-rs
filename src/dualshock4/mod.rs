use hidapi::{HidApi, HidDevice};

pub mod headset;
pub use headset::*;

const DUALSHOCK4_VENDOR_ID:u16 = 0x54C;
const DUALSHOCK4_PRODUCT_ID:u16 = 0x5C4;

const DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH:usize = 64;
const DUALSHOCK4_DATA_BLOCK_BATTERY_LEVEL:usize = 12;

#[derive(Debug)]
pub struct Dualshock4Data {
    pub battery_level: u8,
    pub headset: Headset
}

pub type Dualshock4Error = &'static str;
pub type Dualshock4Result<T> = Result<T, Dualshock4Error>;

pub fn read(controller: &HidDevice) -> Dualshock4Result<Dualshock4Data> {
    let mut buf = [0; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH];

    match controller.read(&mut buf[..]) {
        Ok(DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH) => (),
        Ok(_res) => return Err("Unexpected data length"),
        Err(err) => return Err(err)
    }

    match decode_usb_buf(buf) {
        Ok(data) => return Ok(data),
        Err(err) => return Err(err)
    }
}

fn decode_usb_buf(buf: [u8; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH]) -> Dualshock4Result<Dualshock4Data> {
    let battery_level = buf[DUALSHOCK4_DATA_BLOCK_BATTERY_LEVEL];
    let headset = headset::decode(buf);

    return Ok(Dualshock4Data {
        battery_level,
        headset
    });
}

pub fn get_device(api: &HidApi) -> HidDevice {
    return api.open(DUALSHOCK4_VENDOR_ID, DUALSHOCK4_PRODUCT_ID)
        .expect("Failed to open device");
}
