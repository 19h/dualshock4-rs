use dualshock4::DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH;

pub struct AnalogStickConfig {
    pub block_x: usize,
    pub block_y: usize
}

// TODO 21.02.2018 nviik - Add struct `AnalogStickConfig` which contains configurations
//   for both left and right analog sticks. Use and define config in `CONFIG` property.

pub const ANALOG_STICK_CONFIG_LEFT: AnalogStickConfig = AnalogStickConfig {
    block_x: 0x01,
    block_y: 0x02
};

pub const ANALOG_STICK_CONFIG_RIGHT: AnalogStickConfig = AnalogStickConfig {
    block_x: 0x03,
    block_y: 0x04
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
    let left = decode_analog_stick(ANALOG_STICK_CONFIG_LEFT, buf);
    let right = decode_analog_stick(ANALOG_STICK_CONFIG_RIGHT, buf);

    return AnalogSticks {
        left,
        right
    }
}

fn decode_analog_stick(config: AnalogStickConfig, buf: [u8; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH]) -> AnalogStick {
    return AnalogStick {
        x: buf[config.block_x],
        y: buf[config.block_y]
    };
}
