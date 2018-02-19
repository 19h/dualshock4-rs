use dualshock4::DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH;

struct ButtonConfig {
    block: usize,
    value: u8,
    mask: u8,
    analog_block: Option<usize>
}

struct ButtonsConfig {
    x: ButtonConfig,
    square: ButtonConfig,
    triangle: ButtonConfig,
    circle: ButtonConfig
}

const BUTTONS_CONFIG:ButtonsConfig = ButtonsConfig {
    x: ButtonConfig {
        block: 5,
        value: 0x20,
        mask: 0xff,
        analog_block: Option::None
    },
    square: ButtonConfig {
        block: 5,
        value: 0x10,
        mask: 0xff,
        analog_block: Option::None
    },
    triangle: ButtonConfig {
        block: 5,
        value: 0x80,
        mask: 0xff,
        analog_block: Option::None
    },
    circle: ButtonConfig {
        block: 5,
        value: 0x40,
        mask: 0xff,
        analog_block: Option::None
    },
};

#[derive(Debug)]
pub struct Button {
    pressed: bool,
    analog_value: Option<u8>
}

#[derive(Debug)]
pub struct Buttons {
    x: Button,
    square: Button,
    triangle: Button,
    circle: Button
}

pub fn decode(buf: [u8; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH]) -> Buttons {
    return Buttons {
        x: decode_button(BUTTONS_CONFIG.x, buf),
        square: decode_button(BUTTONS_CONFIG.square, buf),
        triangle: decode_button(BUTTONS_CONFIG.triangle, buf),
        circle: decode_button(BUTTONS_CONFIG.circle, buf)
    }
}

fn decode_button(config: ButtonConfig, buf: [u8; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH]) -> Button {
    let is_pressed = is_button_pressed(&config, buf);
    let analog_value = get_analog_value(&config, buf);

    return Button {
        pressed: is_pressed,
        analog_value
    }
}

fn is_button_pressed(config: &ButtonConfig, buf: [u8; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH]) -> bool {
    let block = buf[config.block] & config.mask;

    // special case: dpadUp
    if config.value == 0x00 {
        return !(block == 0);
    }

    // special case for dpads, only one can be pressed at time
    if config.block == 5 && block < 0x08 {
        return block == config.value;
    }

    return (block & config.value) == config.value;
}

fn get_analog_value(config: &ButtonConfig, buf: [u8; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH]) -> Option<u8> {
    match config.analog_block {
        Some(block) => Some(buf[block]),
        None => None
    }
}
