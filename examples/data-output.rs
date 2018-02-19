extern crate hidapi;
extern crate dualshock4;

use hidapi::{HidApi};

fn main() {
    let api = HidApi::new().expect("Failed to create HID API instance.");
    let controller = dualshock4::get_device(&api);

    loop {
        let data = dualshock4::read(&controller)
            .expect("Failed to read data");
        println!("{:?}", data);
    }
}
