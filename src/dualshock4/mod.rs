use hidapi::{HidDevice};

pub mod model;
pub use model::{Headset,Dualshock4Data};

const DUALSHOCK4_VENDOR_ID:u16 = 0x54C;
const DUALSHOCK4_PRODUCT_ID:u16 = 0x5C4;

const DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH:usize = 64;

const DUALSHOCK4_DATA_BLOCK_BATTERY_LEVEL:usize = 12;
const DUALSHOCK4_DATA_BLOCK_HEADSET:usize = 30;

const DUALSHOCK4_HEADSET_MASK_NONE:u8 = 0x1b;
const DUALSHOCK4_HEADSET_MASK_HEADPHONES:u8 = 0x3b;
const DUALSHOCK4_HEADSET_MASK_HEADSET_WITH_MIC:u8 = 0x7b;


pub type Dualshock4Error = &'static str;
pub type Dualshock4Result<T> = Result<T, Dualshock4Error>;

pub fn read_ds4_data(controller: &HidDevice) -> Dualshock4Result<Dualshock4Data> {
    let mut buf = [0; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH];

    match controller.read(&mut buf[..]) {
        Ok(DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH) => (),
        Ok(_res) => return Err("Unexepted data length"),
        Err(err) => return Err(err)
    }

    match decode_usb_buf(buf) {
        Ok(data) => return Ok(data),
        Err(err) => return Err(err)
    }
}

fn decode_usb_buf(buf: [u8; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH]) -> Dualshock4Result<Dualshock4Data> {
    let batteryLevel = buf[DUALSHOCK4_DATA_BLOCK_BATTERY_LEVEL];
    let headset = decode_headset_value(buf[DUALSHOCK4_DATA_BLOCK_HEADSET]);

    return Ok(Dualshock4Data {
        batteryLevel,
        headset
    });
}

fn decode_headset_value(val:u8) -> Headset {
    match val {
        DUALSHOCK4_HEADSET_MASK_NONE => return Headset::None,
        DUALSHOCK4_HEADSET_MASK_HEADPHONES => return Headset::Headphones,
        DUALSHOCK4_HEADSET_MASK_HEADSET_WITH_MIC => return Headset::HeadsetWithMic,
        _ => return Headset::None
    }
}
