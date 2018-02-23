use dualshock4::DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH;

pub struct TouchpadTouchConfig {
    pub active_block:usize,
    pub data_block_a:usize,
    pub data_block_b:usize,
    pub data_block_c:usize
}

pub struct TouchpadConfig {
    pub touch_1: TouchpadTouchConfig,
    pub touch_2: TouchpadTouchConfig
}

pub const CONFIG:TouchpadConfig = TouchpadConfig {
    touch_1: TouchpadTouchConfig {
        active_block: 35,
        data_block_a: 36,
        data_block_b: 37,
        data_block_c: 38
    },
    touch_2: TouchpadTouchConfig {
        active_block: 39,
        data_block_a: 40,
        data_block_b: 41,
        data_block_c: 42
    }
};

#[derive(PartialEq, Debug)]
pub struct TouchpadTouch {
    pub active:bool,
    pub x: Option<u16>,
    pub y: Option<u16>
}

#[derive(PartialEq, Debug)]
pub struct Touchpad {
    pub touch_1:TouchpadTouch,
    pub touch_2:TouchpadTouch
}

pub fn decode(buf: [u8; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH]) -> Touchpad {
    Touchpad {
        touch_1: decode_touch(&CONFIG.touch_1, buf),
        touch_2: decode_touch(&CONFIG.touch_2, buf)
    }
}

fn decode_touch(config: &TouchpadTouchConfig, buf: [u8; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH]) -> TouchpadTouch {
    let active = buf[config.active_block] < 128;
    let mut x = None;
    let mut y = None;

    if active {
        x = Some(decode_touch_x(config, buf));
        y = Some(decode_touch_y(config, buf));
    }

    TouchpadTouch {
        active,
        x,
        y
    }
}

fn decode_touch_x(config: &TouchpadTouchConfig, buf: [u8; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH]) -> u16 {
    let block_a = buf[config.data_block_a] as u16;
    let block_b = buf[config.data_block_b] as u16;

    ((block_b & 15) << 8) | block_a
}

fn decode_touch_y(config: &TouchpadTouchConfig, buf: [u8; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH]) -> u16 {
    let block_b = buf[config.data_block_b] as u16;
    let block_c = buf[config.data_block_c] as u16;

    (block_c << 4) | ((block_b & 240) >> 4)
}
