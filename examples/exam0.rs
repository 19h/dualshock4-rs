extern crate hidapi;
extern crate dualshock4;

use hidapi::{HidApi, HidDevice};

const DUALSHOCK4_VENDOR_ID:u16 = 0x54C;
const DUALSHOCK4_PRODUCT_ID:u16 = 0x5C4;

fn main() {
    let api = HidApi::new()
        .expect("Failed to create HID API instance.");

    let controller = api.open(DUALSHOCK4_VENDOR_ID, DUALSHOCK4_PRODUCT_ID)
        .expect("Failed to open device");

    loop {
        let ds4_data = dualshock4::read_ds4_data(&controller)
            .expect("Failed to read data");
        println!("{:?}", ds4_data);
    }
}



//extern crate dualshock4;
//
//fn main() {
//    dualshock4::it_works();
//}