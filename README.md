# Dualshock4 controller

This library is used to read _dualshock4_ controller data.

Dualshock4 event data documentation: http://www.psdevwiki.com/ps4/DS4-USB

# Usage / Example

Connect dualshock4 controller to your computer by using USB or bluetooth connection.

Add the following dependencies:
```
[dependencies]
hidapi = "0.4.1"
dualshock4 = "0.1.0"
```

Start reading device data:
```
extern crate hidapi;
extern crate dualshock4;

use hidapi::{HidApi};

fn main() {
    let api = HidApi::new().expect("Failed to create HID API instance.");
    let controller = dualshock4::get_device(&api).expect("Failed to open device");

    loop {
        let data = dualshock4::read(&controller)
            .expect("Failed to read data");
        println!("{:?}", data);
    }
}
```
