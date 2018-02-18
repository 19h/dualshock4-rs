extern crate hidapi;

use hidapi::{HidApi, HidDevice, HidDeviceInfo};

const DUALSHOCK4_VENDOR_ID:u16 = 0x54C;
const DUALSHOCK4_PRODUCT_ID:u16 = 0x5C4;
const DUALSHOCK4_BLUETOOTH_RAW_BUFFER_DATA_LENGTH:usize = 10;
const DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH:usize = 64;
const DUALSHOCK4_DATA_BLOCK_BATTERY_LEVEL:usize = 12;


enum Dualshock4Result {
    Data(u8),
    Error(&'static str)
}

fn get_device_info(api: &HidApi) -> HidDeviceInfo {
    return api.devices()
        .into_iter()
        .find(|device|
            device.product_id == DUALSHOCK4_PRODUCT_ID && device.vendor_id == DUALSHOCK4_VENDOR_ID)
        .expect("Failed to get device info");
}

fn read_device_data_to_string(controller: &HidDevice) -> String {
    let mut buf = [0; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH];
    let res = controller.read(&mut buf[..])
        .expect("Failed to read buffer");

    let mut data_string = String::new();

    for u in &buf[..res] {
        data_string.push_str(&(u.to_string() + "\t"));
    }

    return data_string;
}

fn decode_usb_buf(buf: [u8; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH]) -> Dualshock4Result {
    return Dualshock4Result::Data(buf[DUALSHOCK4_DATA_BLOCK_BATTERY_LEVEL]);
}

fn main() {
    let api = HidApi::new()
        .expect("Failed to create HID API instance.");

    let device_info = get_device_info(&api);
    println!("{:#?}", device_info);

    let controller = api.open(DUALSHOCK4_VENDOR_ID, DUALSHOCK4_PRODUCT_ID)
        .expect("Failed to open device");

    loop {
//        let device_data_string = read_device_data_to_string(&controller);
//        println!("{}", device_data_string);

        let mut buf = [0; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH];

        match controller.read(&mut buf[..]) {
            Ok(DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH) => (),
            Ok(res) => println!("Error: unexpected data length {}/{}", res, DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH),
            Err(err) => println!("Error: {:}", err)
        }

        match decode_usb_buf(buf) {
            Dualshock4Result::Data(data) => println!("Battery level:\t{:?}", data),
            Dualshock4Result::Error(err) => println!("Error:\t{}", err)
        }
    }
}
