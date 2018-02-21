use dualshock4::DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH;

pub struct ButtonConfig {
    pub block: usize,
    pub value: u8,
    pub mask: u8,
    pub analog_block: Option<usize>
}

pub struct ButtonsConfig {
    pub x: ButtonConfig,
    pub square: ButtonConfig,
    pub triangle: ButtonConfig,
    pub circle: ButtonConfig,
    pub dpad_up: ButtonConfig,
    pub dpad_up_right: ButtonConfig,
    pub dpad_right: ButtonConfig,
    pub dpad_down_right: ButtonConfig,
    pub dpad_down: ButtonConfig,
    pub dpad_down_left: ButtonConfig,
    pub dpad_left: ButtonConfig,
    pub dpad_up_left: ButtonConfig,
    pub share: ButtonConfig,
    pub options: ButtonConfig,
    pub psx: ButtonConfig,
    pub touchpad: ButtonConfig,
    pub l1: ButtonConfig,
    pub r1: ButtonConfig,
    pub left_stick: ButtonConfig,
    pub right_stick: ButtonConfig,
    pub l2: ButtonConfig,
    pub r2: ButtonConfig
}

// TODO 21.02.2018 nviik - Rename to `CONFIG`. Outside from this file we can use `buttons::CONFIG`
pub const BUTTONS_CONFIG:ButtonsConfig = ButtonsConfig {
    x: ButtonConfig {
        block: 0x05,
        value: 0x20,
        mask: 0xff,
        analog_block: Option::None
    },
    square: ButtonConfig {
        block: 0x05,
        value: 0x10,
        mask: 0xff,
        analog_block: Option::None
    },
    triangle: ButtonConfig {
        block: 0x05,
        value: 0x80,
        mask: 0xff,
        analog_block: Option::None
    },
    circle: ButtonConfig {
        block: 0x05,
        value: 0x40,
        mask: 0xff,
        analog_block: Option::None
    },
    dpad_up: ButtonConfig {
        block: 0x05,
        value: 0x00,
        mask: 0xf,
        analog_block: Option::None
    },
    dpad_up_right: ButtonConfig {
        block: 0x05,
        value: 0x01,
        mask: 0xf,
        analog_block: Option::None
    },
    dpad_right: ButtonConfig {
        block: 0x05,
        value: 0x02,
        mask: 0xf,
        analog_block: Option::None
    },
    dpad_down_right: ButtonConfig {
        block: 0x05,
        value: 0x03,
        mask: 0xf,
        analog_block: Option::None
    },
    dpad_down: ButtonConfig {
        block: 0x05,
        value: 0x04,
        mask: 0xf,
        analog_block: Option::None
    },
    dpad_down_left: ButtonConfig {
        block: 0x05,
        value: 0x05,
        mask: 0xf,
        analog_block: Option::None
    },
    dpad_left: ButtonConfig {
        block: 0x05,
        value: 0x06,
        mask: 0xf,
        analog_block: Option::None
    },
    dpad_up_left: ButtonConfig {
        block: 0x05,
        value: 0x07,
        mask: 0xf,
        analog_block: Option::None
    },
    share: ButtonConfig {
        block: 0x06,
        value: 0x10,
        mask: 0xff,
        analog_block: Option::None
    },
    options: ButtonConfig {
        block: 0x06,
        value: 0x20,
        mask: 0xff,
        analog_block: Option::None
    },
    psx: ButtonConfig {
        block: 0x07,
        value: 0x01,
        mask: 0xff,
        analog_block: Option::None
    },
    touchpad: ButtonConfig {
        block: 0x07,
        value: 0x02,
        mask: 0xff,
        analog_block: Option::None
    },
    l1: ButtonConfig {
        block: 0x06,
        value: 0x01,
        mask: 0xff,
        analog_block: Option::None
    },
    r1: ButtonConfig {
        block: 0x06,
        value: 0x02,
        mask: 0xff,
        analog_block: Option::None
    },
    left_stick: ButtonConfig {
        block: 0x06,
        value: 0x40,
        mask: 0xff,
        analog_block: Option::None
    },
    right_stick: ButtonConfig {
        block: 0x06,
        value: 0x80,
        mask: 0xff,
        analog_block: Option::None
    },
    l2: ButtonConfig {
        block: 0x06,
        value: 0x04,
        mask: 0xff,
        analog_block: Some(0x08)
    },
    r2: ButtonConfig {
        block: 0x06,
        value: 0x08,
        mask: 0xff,
        analog_block: Some(0x09)
    }
};

#[derive(PartialEq, Debug)]
pub struct Button {
    pub pressed: bool,
    pub analog_value: Option<u8>
}

#[derive(PartialEq, Debug)]
pub struct Buttons {
    pub x: Button,
    pub square: Button,
    pub triangle: Button,
    pub circle: Button,
    pub dpad_up: Button,
    pub dpad_up_right: Button,
    pub dpad_right: Button,
    pub dpad_down_right: Button,
    pub dpad_down: Button,
    pub dpad_down_left: Button,
    pub dpad_left: Button,
    pub dpad_up_left: Button,
    pub share: Button,
    pub options: Button,
    pub psx: Button,
    pub touchpad: Button,
    pub l1: Button,
    pub r1: Button,
    pub left_stick: Button,
    pub right_stick: Button,
    pub l2: Button,
    pub r2: Button
}

pub fn decode(buf: [u8; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH]) -> Buttons {
    Buttons {
        x: decode_button(BUTTONS_CONFIG.x, buf),
        square: decode_button(BUTTONS_CONFIG.square, buf),
        triangle: decode_button(BUTTONS_CONFIG.triangle, buf),
        circle: decode_button(BUTTONS_CONFIG.circle, buf),
        dpad_up: decode_button(BUTTONS_CONFIG.dpad_up, buf),
        dpad_up_right: decode_button(BUTTONS_CONFIG.dpad_up_right, buf),
        dpad_right: decode_button(BUTTONS_CONFIG.dpad_right, buf),
        dpad_down_right: decode_button(BUTTONS_CONFIG.dpad_down_right, buf),
        dpad_down: decode_button(BUTTONS_CONFIG.dpad_down, buf),
        dpad_down_left: decode_button(BUTTONS_CONFIG.dpad_down_left, buf),
        dpad_left: decode_button(BUTTONS_CONFIG.dpad_left, buf),
        dpad_up_left: decode_button(BUTTONS_CONFIG.dpad_up_left, buf),
        share: decode_button(BUTTONS_CONFIG.share, buf),
        options: decode_button(BUTTONS_CONFIG.options, buf),
        psx: decode_button(BUTTONS_CONFIG.psx, buf),
        touchpad: decode_button(BUTTONS_CONFIG.touchpad, buf),
        l1: decode_button(BUTTONS_CONFIG.l1, buf),
        r1: decode_button(BUTTONS_CONFIG.r1, buf),
        left_stick: decode_button(BUTTONS_CONFIG.left_stick, buf),
        right_stick: decode_button(BUTTONS_CONFIG.right_stick, buf),
        l2: decode_button(BUTTONS_CONFIG.l2, buf),
        r2: decode_button(BUTTONS_CONFIG.r2, buf)
    }
}

fn decode_button(config: ButtonConfig, buf: [u8; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH]) -> Button {
    let is_pressed = is_button_pressed(&config, buf);
    let analog_value = get_analog_value(&config, buf);

    Button {
        pressed: is_pressed,
        analog_value
    }
}

fn is_button_pressed(config: &ButtonConfig, buf: [u8; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH]) -> bool {
    let block = buf[config.block] & config.mask;

    // special case for dpadUp
    if config.block == 0x05 && config.value == 0x00 {
        return block == 0
    }

    // special case for dpads, only one can be pressed at time
    if config.block == 0x05 && block < 0x08 {
        return block == config.value
    }

    (block & config.value) == config.value
}

fn get_analog_value(config: &ButtonConfig, buf: [u8; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH]) -> Option<u8> {
    match config.analog_block {
        Some(block) => Some(buf[block]),
        None => None
    }
}
