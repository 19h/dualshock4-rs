use dualshock4::DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH;

pub const DATA_BLOCK_BATTERY_LEVEL:usize = 0x12;

pub fn decode(buf: [u8; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH]) -> u8 {
    buf[DATA_BLOCK_BATTERY_LEVEL]
}
