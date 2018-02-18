extern crate hidapi;

use hidapi::{HidApi, HidDevice, HidDeviceInfo};

const DUALSHOCK4_VENDOR_ID:u16 = 0x54C; // 1356;
const DUALSHOCK4_PRODUCT_ID:u16 = 0x5C4; // 1476;
const DUALSHOCK4_BLUETOOTH_RAW_BUFFER_DATA_LENGTH:usize = 10;
const DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH:usize = 64;

fn get_device_info(api: &HidApi) -> HidDeviceInfo {
    return api.devices()
        .into_iter()
        .find(|device|
            device.product_id == DUALSHOCK4_PRODUCT_ID && device.vendor_id == DUALSHOCK4_VENDOR_ID)
        .expect("Failed to get dualshock device info");
}

fn main() {
    let api = HidApi::new().expect("Failed to create HID API instance.");

    let device_info = get_device_info(&api);
    println!("{:#?}", device_info);

    let controller = api
        .open(DUALSHOCK4_VENDOR_ID, DUALSHOCK4_PRODUCT_ID)
        .expect("Failed to open device");

}
