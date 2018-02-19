use dualshock4::DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH;

const DATA_BLOCK_HEADSET:usize = 30;

const HEADSET_MASK_NONE:u8 = 0x1b;
const HEADSET_MASK_HEADPHONES:u8 = 0x3b;
const HEADSET_MASK_HEADSET_WITH_MIC:u8 = 0x7b;

#[derive(Debug)]
pub enum Headset {
    None,
    Headphones,
    HeadsetWithMic,
    Unknown
}

pub fn decode(buf: [u8; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH]) -> Headset {
    let val = buf[DATA_BLOCK_HEADSET];

    match val {
        HEADSET_MASK_NONE => return Headset::None,
        HEADSET_MASK_HEADPHONES => return Headset::Headphones,
        HEADSET_MASK_HEADSET_WITH_MIC => return Headset::HeadsetWithMic,
        _ => return Headset::Unknown
    }
}
