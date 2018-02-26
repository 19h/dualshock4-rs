pub const DATA_BLOCK_BATTERY_LEVEL:usize = 0x0c;

pub fn decode(buf: &[u8]) -> u8 {
    buf[DATA_BLOCK_BATTERY_LEVEL]
}
