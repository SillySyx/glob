pub fn to_u32_array(x: u32) -> [u8; 4] {
    [
        ((x >> 24) & 0xff) as u8,
        ((x >> 16) & 0xff) as u8,
        ((x >> 8)  & 0xff) as u8,
        ((x >> 0)  & 0xff) as u8,
    ]
}

pub fn to_u64_array(x: u64) -> [u8; 8] {
    [
        ((x >> 56) & 0xff) as u8,
        ((x >> 48) & 0xff) as u8,
        ((x >> 40) & 0xff) as u8,
        ((x >> 32) & 0xff) as u8,
        ((x >> 24) & 0xff) as u8,
        ((x >> 16) & 0xff) as u8,
        ((x >> 8)  & 0xff) as u8,
        ((x >> 0)  & 0xff) as u8,
    ]
}

pub fn to_u32(bytes: &[u8; 4]) -> u32 {
    ((bytes[0] as u32) << 24) +
    ((bytes[1] as u32) << 16) +
    ((bytes[2] as u32) << 8)  +
    ((bytes[3] as u32) << 0)
}

pub fn to_u64(bytes: &[u8; 8]) -> u64 {
    ((bytes[0] as u64) << 56) +
    ((bytes[1] as u64) << 48) +
    ((bytes[2] as u64) << 40) +
    ((bytes[3] as u64) << 32) +
    ((bytes[4] as u64) << 24) +
    ((bytes[5] as u64) << 16) +
    ((bytes[6] as u64) << 8)  +
    ((bytes[7] as u64) << 0)
}