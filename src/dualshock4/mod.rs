extern crate hidapi;
use self::hidapi::{HidApi, HidDevice, HidResult};

mod battery_level;

pub mod headset;
pub use self::headset::{Headset};

pub mod buttons;
pub use self::buttons::{Buttons, Button};

pub mod analog_sticks;
pub use self::analog_sticks::{AnalogSticks, AnalogStick};

pub mod touchpad;
pub use self::touchpad::{Touchpad, TouchpadTouch};

pub mod motion;
pub use self::motion::{Motion};

const DUALSHOCK4_VENDOR_ID:u16 = 0x54c;
const DUALSHOCK4_PRODUCT_ID:u16 = 0x5c4;

// TODO 20.02.2018 nviik - Implement reading bluetooth data
const DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH:usize = 64;

#[derive(PartialEq, Debug)]
pub struct Dualshock4Data {
    pub battery_level: u8,
    pub headset: Headset,
    pub analog_sticks: AnalogSticks,
    pub buttons: Buttons,
    pub touchpad: Touchpad,
    pub motion: Motion
}

// TODO 22.02.2018 nviik - Actually we don't have anything to throw as an error? If there's an error, panic!
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
    let touchpad = touchpad::decode(buf);
    let motion = motion::decode(buf);

    Ok(Dualshock4Data {
        battery_level,
        headset,
        analog_sticks,
        buttons,
        touchpad,
        motion
    })
}

#[cfg(test)]
mod tests {
    extern crate rand;
    extern crate scroll;

    use self::rand::Rng;
    use self::scroll::Pwrite;
    use test::{Bencher, black_box};
    use dualshock4::*;

    #[bench]
    fn bench_test_decode_usb_buf(b: &mut Bencher) {
        b.iter(|| {
            for _i in 1..1000 {
                black_box(test_decode_usb_buf());
            }
        });
    }

    #[test]
    fn test_decode_usb_buf() {
        let mut buf = [0u8; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH];
        let expected = generate_test_data(&mut buf[..]);
        let decoded = decode_usb_buf(buf).expect("Decoding failed");

        assert_eq!(expected, decoded);
    }

    fn generate_test_data(buf: &mut[u8]) -> Dualshock4Data {
        let battery_level = generate_battery_level_data(&mut buf[..]);
        let headset = generate_headset_data(&mut buf[..]);
        let analog_sticks = generate_analog_sticks_data(&mut buf[..]);
        let buttons = generate_buttons_data(&mut buf[..]);
        let touchpad = generate_touchpad_data(&mut buf[..]);
        let motion = generate_motion_data(&mut buf[..]);

        Dualshock4Data {
            battery_level,
            headset,
            analog_sticks,
            buttons,
            touchpad,
            motion
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
            x: generate_button_data(buttons::CONFIG.x, &mut buf[..]),
            square: generate_button_data(buttons::CONFIG.square, &mut buf[..]),
            triangle: generate_button_data(buttons::CONFIG.triangle, &mut buf[..]),
            circle: generate_button_data(buttons::CONFIG.circle, &mut buf[..]),
            dpad_up: generate_button_data(buttons::CONFIG.dpad_up, &mut buf[..]),
            dpad_up_right: generate_button_data(buttons::CONFIG.dpad_up_right, &mut buf[..]),
            dpad_right: generate_button_data(buttons::CONFIG.dpad_right, &mut buf[..]),
            dpad_down_right: generate_button_data(buttons::CONFIG.dpad_down_right, &mut buf[..]),
            dpad_down: generate_button_data(buttons::CONFIG.dpad_down, &mut buf[..]),
            dpad_down_left: generate_button_data(buttons::CONFIG.dpad_down_left, &mut buf[..]),
            dpad_left: generate_button_data(buttons::CONFIG.dpad_left, &mut buf[..]),
            dpad_up_left: generate_button_data(buttons::CONFIG.dpad_up_left, &mut buf[..]),
            share: generate_button_data(buttons::CONFIG.share, &mut buf[..]),
            options: generate_button_data(buttons::CONFIG.options, &mut buf[..]),
            psx: generate_button_data(buttons::CONFIG.psx, &mut buf[..]),
            touchpad: generate_button_data(buttons::CONFIG.touchpad, &mut buf[..]),
            l1: generate_button_data(buttons::CONFIG.l1, &mut buf[..]),
            r1: generate_button_data(buttons::CONFIG.r1, &mut buf[..]),
            left_stick: generate_button_data(buttons::CONFIG.left_stick, &mut buf[..]),
            right_stick: generate_button_data(buttons::CONFIG.right_stick, &mut buf[..]),
            l2: generate_button_data(buttons::CONFIG.l2, &mut buf[..]),
            r2: generate_button_data(buttons::CONFIG.r2, &mut buf[..])
        }
    }

    fn generate_button_data(config: buttons::ButtonConfig, buf: &mut [u8]) -> Button {
        static mut IS_DPAD_PRESSED:bool = false;
        let is_dpad_up_config = config.value == buttons::CONFIG.dpad_up.value;
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

        // special case for dpadUp. If it's pressed then it should contain value.
        if !is_pressed && is_dpad_up_config {
            buf[config.block] += 0x08;
        }

        if is_pressed && !is_dpad_up_config {
            buf[config.block] += config.value;
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

    fn generate_touchpad_data(buf: &mut [u8]) -> Touchpad {
        Touchpad {
            touch_1: generate_touchpad_touch_data(touchpad::CONFIG.touch_1, buf),
            touch_2: generate_touchpad_touch_data(touchpad::CONFIG.touch_2, buf)
        }
    }

    fn generate_touchpad_touch_data(config: touchpad::TouchpadTouchConfig, buf: &mut [u8]) -> TouchpadTouch {
        const TOUCHPAD_RESOLUTION_WIDTH:u16 = 943;
        const TOUCHPAD_RESOLUTION_HEIGHT:u16 = 1920;

        let active:bool = rand::thread_rng().gen();
        let mut x = None;
        let mut y = None;

        if active {
            let temp_x:u16 = rand::thread_rng().gen_range(0, TOUCHPAD_RESOLUTION_WIDTH);
            let temp_y:u16 = rand::thread_rng().gen_range(0, TOUCHPAD_RESOLUTION_HEIGHT);

            buf[config.data_block_a] = (temp_x & 0xff) as u8;
            buf[config.data_block_b] = (((temp_y | 0xf0) << 4) ^ ((temp_x | 0x0f) >> 8)) as u8;
            buf[config.data_block_c] = (temp_y >> 4) as u8;

            x = Some(temp_x);
            y = Some(temp_y);
        }

        buf[config.active_block] = if active { 0x00 } else { 0xff };

        TouchpadTouch {
            active,
            x,
            y
        }
    }

    fn generate_motion_data(buf: &mut[u8]) -> Motion {
        let x:i16 = rand::thread_rng().gen();
        let y:i16 = rand::thread_rng().gen();
        let z:i16 = rand::thread_rng().gen();
        let roll:i16 = rand::thread_rng().gen();
        let yaw:i16 = rand::thread_rng().gen();
        let pitch:i16 = rand::thread_rng().gen();

        buf.pwrite_with::<i16>(x, motion::CONFIG.motion_x, scroll::BE).unwrap();
        buf.pwrite_with::<i16>(y, motion::CONFIG.motion_y, scroll::BE).unwrap();
        buf.pwrite_with::<i16>(z, motion::CONFIG.motion_z, scroll::BE).unwrap();
        buf.pwrite_with::<i16>(roll, motion::CONFIG.gyro_x, scroll::BE).unwrap();
        buf.pwrite_with::<i16>(yaw, motion::CONFIG.gyro_y, scroll::BE).unwrap();
        buf.pwrite_with::<i16>(pitch, motion::CONFIG.gyro_z, scroll::BE).unwrap();

        Motion {
            x, y, z,
            roll, yaw, pitch
        }
    }
}
