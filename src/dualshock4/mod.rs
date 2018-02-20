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
//    pub buttons: Buttons,
//    pub analog_sticks: AnalogSticks
}

pub type Dualshock4Error = &'static str;
pub type Dualshock4Result<T> = Result<T, Dualshock4Error>;

pub fn read(controller: &HidDevice) -> Dualshock4Result<Dualshock4Data> {
    let mut buf = [0; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH];

    match controller.read(&mut buf[..]) {
        Ok(DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH) => (),
        Ok(_res) => return Err("Unexpected data length"),
        Err(err) => return Err(err)
    }

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
//        buttons,
//        analog_sticks
    });
}

pub fn get_device(api: &HidApi) -> HidDevice {
    return api.open(DUALSHOCK4_VENDOR_ID, DUALSHOCK4_PRODUCT_ID)
        .expect("Failed to open device");
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

        return Dualshock4Data {
            battery_level,
            headset
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
}
