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
        active_block: 0x23,
        data_block_a: 0x24,
        data_block_b: 0x25,
        data_block_c: 0x26
    },
    touch_2: TouchpadTouchConfig {
        active_block: 0x27,
        data_block_a: 0x28,
        data_block_b: 0x29,
        data_block_c: 0x2a
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

pub fn decode(buf: &[u8]) -> Touchpad {
    Touchpad {
        touch_1: decode_touch(&CONFIG.touch_1, buf),
        touch_2: decode_touch(&CONFIG.touch_2, buf)
    }
}

fn decode_touch(config: &TouchpadTouchConfig, buf: &[u8]) -> TouchpadTouch {
    let active = buf[config.active_block] < 0x80;

    let x = if active {
        Some(decode_touch_x(config, buf))
    } else {
        None
    };

    let y = if active {
        Some(decode_touch_y(config, buf))
    } else {
        None
    };

    TouchpadTouch {
        active, x, y
    }
}

fn decode_touch_x(config: &TouchpadTouchConfig, buf: &[u8]) -> u16 {
    let block_a = u16::from(buf[config.data_block_a]);
    let block_b = u16::from(buf[config.data_block_b]);

    ((block_b & 0x0f) << 8) | block_a
}

fn decode_touch_y(config: &TouchpadTouchConfig, buf: &[u8]) -> u16 {
    let block_b = u16::from(buf[config.data_block_b]);
    let block_c = u16::from(buf[config.data_block_c]);

    (block_c << 4) | ((block_b & 0xf0) >> 4)
}
