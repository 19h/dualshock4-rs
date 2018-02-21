use hidapi::{HidApi, HidDevice};

pub mod headset;
pub use self::headset::{Headset};

pub mod buttons;
pub use self::buttons::{Buttons, Button};

pub mod analog_sticks;
pub use self::analog_sticks::{AnalogSticks, AnalogStick};

const DUALSHOCK4_VENDOR_ID:u16 = 0x54C;
const DUALSHOCK4_PRODUCT_ID:u16 = 0x5C4;

// TODO 20.02.2018 nviik - Implement reading bluetooth data
const DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH:usize = 64;

const DUALSHOCK4_DATA_BLOCK_BATTERY_LEVEL:usize = 0x12;

#[derive(PartialEq, Debug)]
pub struct Dualshock4Data {
    pub battery_level: u8,
    pub headset: Headset,
    pub analog_sticks: AnalogSticks,
    pub buttons: Buttons
}

pub type Dualshock4Error = &'static str;
pub type Dualshock4Result<T> = Result<T, Dualshock4Error>;

pub fn get_device(api: &HidApi) -> HidDevice {
    return api.open(DUALSHOCK4_VENDOR_ID, DUALSHOCK4_PRODUCT_ID)
        .expect("Failed to open device");
}

pub fn read(controller: &HidDevice) -> Dualshock4Result<Dualshock4Data> {
    let mut buf = [0; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH];

    match controller.read(&mut buf[..]) {
        Ok(DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH) => (),
        Ok(_res) => return Err("Unexpected data length"),
        Err(err) => return Err(err)
    }

    // TODO 20.02.2018 nviik - should be `return decode_usb_buf(buf)`
    match decode_usb_buf(buf) {
        Ok(data) => return Ok(data),
        Err(err) => return Err(err)
    }
}

fn decode_usb_buf(buf: [u8; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH]) -> Dualshock4Result<Dualshock4Data> {
    let battery_level = buf[DUALSHOCK4_DATA_BLOCK_BATTERY_LEVEL];
    let headset = headset::decode(buf);
    let buttons = buttons::decode(buf);
    let analog_sticks = analog_sticks::decode(buf);

    return Ok(Dualshock4Data {
        battery_level,
        headset,
        analog_sticks,
        buttons
    });
}

#[cfg(test)]
mod tests {
    extern crate rand;

    use self::rand::Rng;
    use dualshock4::*;

    #[test]
    fn test_decode_usb_buf() {
        let mut buf = [0u8; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH];
        let expected = generate_test_data(&mut buf[..]);
        let decoded = decode_usb_buf(buf).expect("Fails");

        assert_eq!(expected, decoded);
    }

    fn generate_test_data(buf: &mut[u8]) -> Dualshock4Data {
        let battery_level = generate_battery_level_data(&mut buf[..]);
        let headset = generate_headeset_data(&mut buf[..]);
        let analog_sticks = generate_analog_sticks_data(&mut buf[..]);
        let buttons = generate_buttons_data(&mut buf[..]);

        return Dualshock4Data {
            battery_level,
            headset,
            analog_sticks,
            buttons
        }
    }

    fn generate_battery_level_data(buf: &mut[u8]) -> u8 {
        let value:u8 = rand::thread_rng().gen_range(0, 22);
        buf[DUALSHOCK4_DATA_BLOCK_BATTERY_LEVEL] = value;
        return value;
    }

    fn generate_headeset_data(buf: &mut [u8]) -> Headset {
        let value = rand::thread_rng().gen_range(0, 3);

        buf[headset::DATA_BLOCK_HEADSET] = match value {
            0 => headset::HEADSET_MASK_NONE,
            1 => headset::HEADSET_MASK_HEADPHONES,
            2 => headset::HEADSET_MASK_HEADSET_WITH_MIC,
            _ => 0
        };

        return match value {
            0 => Headset::None,
            1 => Headset::Headphones,
            2 => Headset::HeadsetWithMic,
            _ => Headset::Unknown
        };
    }

    fn generate_analog_sticks_data(buf: &mut [u8]) -> AnalogSticks {
        let left = generate_analog_stick_data();
        let right = generate_analog_stick_data();

        buf[analog_sticks::ANALOG_STICK_CONFIG_LEFT.block_x] = left.x;
        buf[analog_sticks::ANALOG_STICK_CONFIG_LEFT.block_y] = left.y;

        buf[analog_sticks::ANALOG_STICK_CONFIG_RIGHT.block_x] = right.x;
        buf[analog_sticks::ANALOG_STICK_CONFIG_RIGHT.block_y] = right.y;

        return AnalogSticks {
            left,
            right
        }
    }

    // TODO 20.02.2018 nviik - Maybe pass config and buf as parameters?
    fn generate_analog_stick_data() -> AnalogStick {
        let x:u8 = rand::thread_rng().gen();
        let y:u8 = rand::thread_rng().gen();

        return AnalogStick {
            x, y
        }
    }

    fn generate_buttons_data(buf: &mut [u8]) -> Buttons {
        return Buttons {
            x: generate_button_data(buttons::BUTTONS_CONFIG.x, &mut buf[..]),
            square: generate_button_data(buttons::BUTTONS_CONFIG.square, &mut buf[..]),
            triangle: generate_button_data(buttons::BUTTONS_CONFIG.triangle, &mut buf[..]),
            circle: generate_button_data(buttons::BUTTONS_CONFIG.circle,&mut buf[..]),
            dpad_up: generate_button_data(buttons::BUTTONS_CONFIG.dpad_up,&mut buf[..]),
            dpad_up_right: generate_button_data(buttons::BUTTONS_CONFIG.dpad_up_right, &mut buf[..]),
            dpad_right: generate_button_data(buttons::BUTTONS_CONFIG.dpad_right, &mut buf[..]),
            dpad_down_right: generate_button_data(buttons::BUTTONS_CONFIG.dpad_down_right, &mut buf[..]),
            dpad_down: generate_button_data(buttons::BUTTONS_CONFIG.dpad_down, &mut buf[..]),
            dpad_down_left: generate_button_data(buttons::BUTTONS_CONFIG.dpad_down_left, &mut buf[..]),
            dpad_left: generate_button_data(buttons::BUTTONS_CONFIG.dpad_left, &mut buf[..]),
            dpad_up_left: generate_button_data(buttons::BUTTONS_CONFIG.dpad_up_left, &mut buf[..]),
            share: generate_button_data(buttons::BUTTONS_CONFIG.share, &mut buf[..]),
            options: generate_button_data(buttons::BUTTONS_CONFIG.options, &mut buf[..]),
            psx: generate_button_data(buttons::BUTTONS_CONFIG.psx, &mut buf[..]),
            touchpad: generate_button_data(buttons::BUTTONS_CONFIG.touchpad, &mut buf[..]),
            l1: generate_button_data(buttons::BUTTONS_CONFIG.l1, &mut buf[..]),
            r1: generate_button_data(buttons::BUTTONS_CONFIG.r1, &mut buf[..]),
            left_stick: generate_button_data(buttons::BUTTONS_CONFIG.left_stick, &mut buf[..]),
            right_stick: generate_button_data(buttons::BUTTONS_CONFIG.right_stick, &mut buf[..]),
            l2: generate_button_data(buttons::BUTTONS_CONFIG.l2, &mut buf[..]),
            r2: generate_button_data(buttons::BUTTONS_CONFIG.r2, &mut buf[..])
        }
    }

    fn generate_button_data(config: buttons::ButtonConfig, buf: &mut [u8]) -> Button {
        static mut is_dpad_pressed: bool = false;
        let is_pressed:bool = rand::thread_rng().gen();

        // special case for dpads, because only one can pressed at the time
        if config.block == 0x05 && config.value < 0x08 {
            let mut pressed:bool = is_pressed;
            unsafe {
                if pressed && !is_dpad_pressed {
                    pressed = true;
                    is_dpad_pressed = true;

                    if config.value != 0x00 {
                        buf[config.block] += config.value;
                    }
                } else {
                    pressed = false;
                }
            }


            if !pressed && config.value == 0x00 {
                buf[config.block] += 0x08;
            }

            return Button {
                pressed: pressed,
                analog_value: None
            }
        } else {
            let mut analog_value = None;

            if config.analog_block != None {
                let analog: u8 = rand::thread_rng().gen();
                analog_value = Some(analog);

                match config.analog_block {
                    Some(0x08) => buf[0x08] += analog,
                    Some(0x09) => buf[0x09] += analog,
                    _ => ()
                }
            }

            if !is_pressed {
                return Button {
                    pressed: false,
                    analog_value
                }
            } else {
                buf[config.block] += config.value;

                return Button {
                    pressed: true,
                    analog_value
                }
            }
        }
    }
}
