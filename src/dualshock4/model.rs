#[derive(Debug)]
pub enum Headset {
    None,
    Headphones,
    HeadsetWithMic
}

#[derive(Debug)]
pub struct Dualshock4Data {
    pub batteryLevel: u8,
    pub headset: Headset
}
