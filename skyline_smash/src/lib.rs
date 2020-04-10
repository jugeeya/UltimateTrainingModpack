#![no_std]
#![feature(const_if_match, const_loop)]

pub mod crc32;

#[doc(hidden)]
pub mod cpp;

#[doc(inline)]
pub use cpp::root::*;

// Find the hash40 of a given string
pub const fn hash40(string: &str) -> u64 {
    let bytes = string.as_bytes();

    ((bytes.len() as u64) << 32) + crc32::crc32(bytes) as u64
}

