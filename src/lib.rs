extern crate hidapi;

mod dualshock4;

pub use dualshock4::{
    get_device,
    read,
    Dualshock4Data,
    Headset,
    Buttons,
    Button,
    AnalogSticks,
    AnalogStick
};
