extern crate hidapi;

mod dualshock4;

// TODO 19.02.2018 nviik - expose only data model and parser function
pub use dualshock4::*;
