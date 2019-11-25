#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

#![feature(test)]
extern crate test;

mod dualshock4;

pub use dualshock4::{
    get_device,
    get_device_old,
    read,
    Dualshock4Data,
    Headset,
    Buttons,
    Button,
    AnalogSticks,
    AnalogStick,
};
