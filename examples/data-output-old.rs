extern crate hidapi;
extern crate dualshock4;

use hidapi::{HidApi};

/// This example is used to read dualshock4 data before playstation firmaware update 5.50
fn main() {
    let api = HidApi::new().expect("Failed to create HID API instance.");
    let controller = dualshock4::get_device_old(&api).expect("Failed to open device");

    loop {
        let data = dualshock4::read(&controller)
            .expect("Failed to read data");
        println!("{:?}", data);
    }
}
