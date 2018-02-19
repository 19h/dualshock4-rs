// TOIMII
//pub fn it_works() {
//    println!("hello world!");
//}




extern crate hidapi;

use hidapi::{HidApi, HidDevice};

const DUALSHOCK4_VENDOR_ID:u16 = 0x54C;
const DUALSHOCK4_PRODUCT_ID:u16 = 0x5C4;

const DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH:usize = 64;

const DUALSHOCK4_DATA_BLOCK_BATTERY_LEVEL:usize = 12;
const DUALSHOCK4_DATA_BLOCK_HEADSET:usize = 30;

const DUALSHOCK4_HEADSET_MASK_NONE:u8 = 0x1b;
const DUALSHOCK4_HEADSET_MASK_HEADPHONES:u8 = 0x3b;
const DUALSHOCK4_HEADSET_MASK_HEADSET_WITH_MIC:u8 = 0x7b;


pub type Dualshock4Error = &'static str;
pub type Dualshock4Result<T> = Result<T, Dualshock4Error>;

#[derive(Debug)]
pub enum Headset {
    None,
    Headphones,
    HeadsetWithMic
}

#[derive(Debug)]
pub struct Dualshock4Data {
    batteryLevel: u8,
    headset: Headset
}

pub fn read_ds4_data(controller: &HidDevice) -> Dualshock4Result<Dualshock4Data> {
    let mut buf = [0; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH];

    match controller.read(&mut buf[..]) {
        Ok(DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH) => (),
        Ok(_res) => return Err("Unexepted data length"),
        Err(err) => return Err(err)
    }

    match decode_usb_buf(buf) {
        Ok(data) => return Ok(data),
        Err(err) => return Err(err)
    }
}

fn decode_usb_buf(buf: [u8; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH]) -> Dualshock4Result<Dualshock4Data> {
    let batteryLevel = buf[DUALSHOCK4_DATA_BLOCK_BATTERY_LEVEL];
    let headset = decode_headset_value(buf[DUALSHOCK4_DATA_BLOCK_HEADSET]);

    return Ok(Dualshock4Data {
        batteryLevel,
        headset
    });
}

fn decode_headset_value(val:u8) -> Headset {
    match val {
        DUALSHOCK4_HEADSET_MASK_NONE => return Headset::None,
        DUALSHOCK4_HEADSET_MASK_HEADPHONES => return Headset::Headphones,
        DUALSHOCK4_HEADSET_MASK_HEADSET_WITH_MIC => return Headset::HeadsetWithMic,
        _ => return Headset::None
    }
}

//fn main() {
//    let api = HidApi::new()
//        .expect("Failed to create HID API instance.");
//
//    let controller = api.open(DUALSHOCK4_VENDOR_ID, DUALSHOCK4_PRODUCT_ID)
//        .expect("Failed to open device");
//
//    loop {
//        let ds4_data = read_ds4_data(&controller);
//        println!("{:?}", ds4_data);
//    }
//}






//extern crate hidapi;
//
//use hidapi::{HidApi, HidDevice, HidDeviceInfo};
//
//const DUALSHOCK4_VENDOR_ID:u16 = 0x54C;
//const DUALSHOCK4_PRODUCT_ID:u16 = 0x5C4;
//
//const DUALSHOCK4_BLUETOOTH_RAW_BUFFER_DATA_LENGTH:usize = 10;
//const DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH:usize = 64;
//
//const DUALSHOCK4_DATA_BLOCK_BATTERY_LEVEL:usize = 12;
//const DUALSHOCK4_DATA_BLOCK_HEADSET:usize = 30;
//
//const DUALSHOCK4_HEADSET_MASK_NONE:u8 = 0x1b;
//const DUALSHOCK4_HEADSET_MASK_HEADPHONES:u8 = 0x3b;
//const DUALSHOCK4_HEADSET_MASK_HEADSET_WITH_MIC:u8 = 0x7b;
//
//#[derive(Debug)]
//enum Headset {
//    None,
//    Headphones,
//    HeadsetWithMic
//}
//
//#[derive(Debug)]
//struct Dualshock4Data {
//    batteryLevel: u8,
//    headset: Headset
//}
//
//enum Dualshock4Result {
//    Data(Dualshock4Data),
//    Error(&'static str)
//}
//
//fn get_device_info(api: &HidApi) -> HidDeviceInfo {
//    return api.devices()
//        .into_iter()
//        .find(|device|
//            device.product_id == DUALSHOCK4_PRODUCT_ID && device.vendor_id == DUALSHOCK4_VENDOR_ID)
//        .expect("Failed to get device info");
//}
//
//fn read_device_data_to_string(controller: &HidDevice) -> String {
//    let mut buf = [0; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH];
//    let res = controller.read(&mut buf[..])
//        .expect("Failed to read buffer");
//
//    let mut data_string = String::new();
//
//    for u in &buf[..res] {
//        data_string.push_str(&(u.to_string() + "\t"));
//    }
//
//    return data_string;
//}
//
//fn decode_usb_buf(buf: [u8; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH]) -> Dualshock4Result {
//    let batteryLevel = buf[DUALSHOCK4_DATA_BLOCK_BATTERY_LEVEL];
//    let headset = decode_headset_value(buf[DUALSHOCK4_DATA_BLOCK_HEADSET]);
//
//    let data = Dualshock4Data {
//        batteryLevel,
//        headset
//    };
//
//    return Dualshock4Result::Data(data);
//}
//
//fn decode_headset_value(val:u8) -> Headset {
//    match val {
//        DUALSHOCK4_HEADSET_MASK_NONE => return Headset::None,
//        DUALSHOCK4_HEADSET_MASK_HEADPHONES => return Headset::Headphones,
//        DUALSHOCK4_HEADSET_MASK_HEADSET_WITH_MIC => return Headset::HeadsetWithMic,
//        _ => return Headset::None
//    }
//}

//fn main() {
//    let api = HidApi::new()
//        .expect("Failed to create HID API instance.");
//
//    let device_info = get_device_info(&api);
//    println!("{:#?}", device_info);
//
//    let controller = api.open(DUALSHOCK4_VENDOR_ID, DUALSHOCK4_PRODUCT_ID)
//        .expect("Failed to open device");
//
//    loop {
////        let device_data_string = read_device_data_to_string(&controller);
////        println!("{}", device_data_string);
//
//        let mut buf = [0; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH];
//
//        match controller.read(&mut buf[..]) {
//            Ok(DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH) => (),
//            Ok(res) => {
//              println!("Error: unexpected data length {}/{}", res, DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH),
//              continue;
//            Err(err) => {
//              println!("Error: {:}", err)
//              continue;
//            }
//        }
//
//        match decode_usb_buf(buf) {
//            Dualshock4Result::Data(data) => println!("Data:\t{:?}", data),
//            Dualshock4Result::Error(err) => println!("Error:\t{}", err)
//        }
//    }
//}
