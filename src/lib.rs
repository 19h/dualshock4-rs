// TODO 21.02.2018 nviik - Is this in correct place? Should external crate's be defined in lib or in mod?
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
