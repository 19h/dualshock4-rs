use dualshock4::DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH;

pub const DATA_BLOCK_HEADSET:usize = 0x1e;

pub const HEADSET_MASK_NONE:u8 = 0x1b;
pub const HEADSET_MASK_HEADPHONES:u8 = 0x3b;
pub const HEADSET_MASK_HEADSET_WITH_MIC:u8 = 0x7b;

#[derive(PartialEq, Debug)]
pub enum Headset {
    None,
    Headphones,
    HeadsetWithMic,
    Unknown
}

pub fn decode(buf: [u8; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH]) -> Headset {
    let mask = buf[DATA_BLOCK_HEADSET];

    return match mask {
        HEADSET_MASK_NONE => Headset::None,
        HEADSET_MASK_HEADPHONES => Headset::Headphones,
        HEADSET_MASK_HEADSET_WITH_MIC => Headset::HeadsetWithMic,
        _ => Headset::Unknown
    }
}
