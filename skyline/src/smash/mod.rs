pub mod crc32;

// Find the hash40 of a given string
pub const fn hash40(string: &str) -> u64 {
    let bytes = string.as_bytes();

    ((bytes.len() as u64) << 32) + crc32::crc32(bytes) as u64
}
