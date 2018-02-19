use dualshock4::DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH;

const DUALSHOCK4_DATA_BLOCK_HEADSET:usize = 30;

const DUALSHOCK4_HEADSET_MASK_NONE:u8 = 0x1b;
const DUALSHOCK4_HEADSET_MASK_HEADPHONES:u8 = 0x3b;
const DUALSHOCK4_HEADSET_MASK_HEADSET_WITH_MIC:u8 = 0x7b;

#[derive(Debug)]
pub enum Headset {
    None,
    Headphones,
    HeadsetWithMic
}

pub fn decode(buf: [u8; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH]) -> Headset {
    let val = buf[DUALSHOCK4_DATA_BLOCK_HEADSET];

    match val {
        DUALSHOCK4_HEADSET_MASK_NONE => return Headset::None,
        DUALSHOCK4_HEADSET_MASK_HEADPHONES => return Headset::Headphones,
        DUALSHOCK4_HEADSET_MASK_HEADSET_WITH_MIC => return Headset::HeadsetWithMic,
        _ => return Headset::None
    }
}
