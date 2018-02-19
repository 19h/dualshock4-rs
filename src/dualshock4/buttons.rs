use dualshock4::DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH;

struct ButtonConfig {
    block: usize,
    value: u8,
    mask: u8,
    analog_block: Option<u8>
}

struct ButtonsConfig {
    x: ButtonConfig,
    square: ButtonConfig
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
    }
};

#[derive(Debug)]
pub struct Button {
    pressed: bool
}

#[derive(Debug)]
pub struct Buttons {
    x: Button,
    square: Button
}

pub fn decode(buf: [u8; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH]) -> Buttons {
    return Buttons {
        x: decode_button(BUTTONS_CONFIG.x, buf),
        square: decode_button(BUTTONS_CONFIG.square, buf)
    }
}

fn decode_button(config: ButtonConfig, buf: [u8; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH]) -> Button {
    let block = buf[config.block] & config.mask;
    let is_pressed = (block & config.value) == config.value;

    return Button {
        pressed: is_pressed
    }
}
