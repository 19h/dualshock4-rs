use dualshock4::DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH;

pub struct AnalogStickConfig {
    pub block_x: usize,
    pub block_y: usize
}

pub struct AnalogSticksConfig {
    pub left: AnalogStickConfig,
    pub right: AnalogStickConfig
}

pub const CONFIG:AnalogSticksConfig = AnalogSticksConfig {
    left: AnalogStickConfig {
        block_x: 0x01,
        block_y: 0x02
    },
    right: AnalogStickConfig {
        block_x: 0x03,
        block_y: 0x04
    }
};

#[derive(PartialEq, Debug)]
pub struct AnalogStick {
    pub x: u8,
    pub y: u8
}

#[derive(PartialEq, Debug)]
pub struct AnalogSticks {
    pub left: AnalogStick,
    pub right: AnalogStick
}

pub fn decode(buf: [u8; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH]) -> AnalogSticks {
    let left = decode_analog_stick(&CONFIG.left, buf);
    let right = decode_analog_stick(&CONFIG.right, buf);

    AnalogSticks {
        left,
        right
    }
}

fn decode_analog_stick(config: &AnalogStickConfig, buf: [u8; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH]) -> AnalogStick {
    AnalogStick {
        x: buf[config.block_x],
        y: buf[config.block_y]
    }
}
