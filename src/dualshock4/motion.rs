extern crate scroll;
use self::scroll::Pread;

use dualshock4::DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH;

pub struct MotionConfig {
    pub motion_x:usize,
    pub motion_y:usize,
    pub motion_z:usize,
    pub gyro_x:usize,
    pub gyro_y:usize,
    pub gyro_z:usize
}

pub const CONFIG:MotionConfig = MotionConfig {
    motion_z: 13,
    motion_x: 15,
    motion_y: 17,
    gyro_x: 19,
    gyro_y: 21,
    gyro_z: 23
};

#[derive(PartialEq, Debug)]
pub struct Motion {
    pub x:i16,
    pub y:i16,
    pub z:i16,
    pub roll:i16,
    pub yaw:i16,
    pub pitch:i16
}

pub fn decode(buf: [u8; DUALSHOCK4_USB_RAW_BUFFER_DATA_LENGTH]) -> Motion {
    let x = buf.pread_with::<i16>(CONFIG.motion_x, scroll::BE).unwrap();
    let y = buf.pread_with::<i16>(CONFIG.motion_y, scroll::BE).unwrap();
    let z = buf.pread_with::<i16>(CONFIG.motion_z, scroll::BE).unwrap();
    let roll = buf.pread_with::<i16>(CONFIG.gyro_x, scroll::BE).unwrap();
    let yaw = buf.pread_with::<i16>(CONFIG.gyro_y, scroll::BE).unwrap();
    let pitch = buf.pread_with::<i16>(CONFIG.gyro_z, scroll::BE).unwrap();

    Motion {
        x, y, z,
        roll, yaw, pitch
    }
}
