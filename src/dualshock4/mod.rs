use hidapi::{HidApi, HidDevice, HidResult};

mod battery_level;

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

#[derive(PartialEq, Debug)]
pub struct Dualshock4Data {
    pub battery_level: u8,
    pub headset: Headset,
    pub analog_sticks: AnalogSticks,
    pub buttons: Buttons
}

pub type Dualshock4Error = &'static str;
pub type Dualshock4Result<T> = Result<T, Dualshock4Error>;

pub fn get_device(api: &HidApi) -> HidResult<HidDevice> {
    api.open(DUALSHOCK4_VENDOR_ID, DUALSHOCK4_PRODUCT_ID)
}

pub fn read(controller: &HidDevice) -> Dualshock4Result<Dualshock4Data> {
    let mut buf = [0; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH];

    match controller.read(&mut buf[..]) {
        Ok(DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH) => (),
        Ok(_res) => return Err("Unexpected data length"),
        Err(err) => return Err(err)
    }

    decode_usb_buf(buf)
}

fn decode_usb_buf(buf: [u8; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH]) -> Dualshock4Result<Dualshock4Data> {
    let battery_level = battery_level::decode(buf);
    let headset = headset::decode(buf);
    let buttons = buttons::decode(buf);
    let analog_sticks = analog_sticks::decode(buf);

    Ok(Dualshock4Data {
        battery_level,
        headset,
        analog_sticks,
        buttons
    })
}

#[cfg(test)]
mod tests {
    extern crate rand;

    use self::rand::Rng;
    use dualshock4::*;

    // TODO 21.02.2018 nviik - Figure out how to run this test like 1000 times.
    #[test]
    fn test_decode_usb_buf() {
        let mut buf = [0u8; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH];
        let expected = generate_test_data(&mut buf[..]);
        let decoded = decode_usb_buf(buf).expect("Fails");

        assert_eq!(expected, decoded);
    }

    fn generate_test_data(buf: &mut[u8]) -> Dualshock4Data {
        let battery_level = generate_battery_level_data(&mut buf[..]);
        let headset = generate_headset_data(&mut buf[..]);
        let analog_sticks = generate_analog_sticks_data(&mut buf[..]);
        let buttons = generate_buttons_data(&mut buf[..]);

        Dualshock4Data {
            battery_level,
            headset,
            analog_sticks,
            buttons
        }
    }

    fn generate_battery_level_data(buf: &mut[u8]) -> u8 {
        let value:u8 = rand::thread_rng().gen_range(0, 22);
        buf[battery_level::DATA_BLOCK_BATTERY_LEVEL] = value;
        return value;
    }

    fn generate_headset_data(buf: &mut [u8]) -> Headset {
        let value = rand::thread_rng().gen_range(0, 3);

        buf[headset::DATA_BLOCK_HEADSET] = match value {
            0 => headset::HEADSET_MASK_NONE,
            1 => headset::HEADSET_MASK_HEADPHONES,
            2 => headset::HEADSET_MASK_HEADSET_WITH_MIC,
            _ => 0
        };

        match value {
            0 => Headset::None,
            1 => Headset::Headphones,
            2 => Headset::HeadsetWithMic,
            _ => Headset::Unknown
        }
    }

    fn generate_analog_sticks_data(buf: &mut [u8]) -> AnalogSticks {
        let left = generate_analog_stick_data(&analog_sticks::CONFIG.left, buf);
        let right = generate_analog_stick_data(&analog_sticks::CONFIG.right, buf);

        AnalogSticks {
            left,
            right
        }
    }

    // TODO 20.02.2018 nviik - This should get config and buf as parameters.
    fn generate_analog_stick_data(config: &analog_sticks::AnalogStickConfig, buf: &mut [u8]) -> AnalogStick {
        let x:u8 = rand::thread_rng().gen();
        let y:u8 = rand::thread_rng().gen();

        buf[config.block_x] = x;
        buf[config.block_y] = y;

        AnalogStick {
            x, y
        }
    }

    fn generate_buttons_data(buf: &mut [u8]) -> Buttons {
        Buttons {
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
        static mut IS_DPAD_PRESSED:bool = false;
        let mut is_pressed:bool = rand::thread_rng().gen();

        // special case for dpads, because only one can pressed at the time.
        if config.block == 0x05 && config.value < 0x08 {
            unsafe {
                if IS_DPAD_PRESSED {
                    is_pressed = false;
                } else if is_pressed {
                    IS_DPAD_PRESSED = true;
                }
            }
        }

        if is_pressed && config.value != 0x00 {
            buf[config.block] += config.value;
        }

        // special case for dpadUp. If it's pressed then it should contain value.
        if !is_pressed && config.value == 0x00 {
            buf[config.block] += 0x08;
        }

        let mut analog_value = None;

        if config.analog_block != None {
            let analog: u8 = rand::thread_rng().gen();
            analog_value = Some(analog);

            let block = config.analog_block.unwrap();
            buf[block] += analog;
        }

        Button {
            pressed: is_pressed,
            analog_value
        }
    }
}
