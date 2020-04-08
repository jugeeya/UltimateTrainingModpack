#![no_std]
use skyline::{Allocator, log};

#[global_allocator]
pub static ALLOCATOR: Allocator = Allocator;

extern crate alloc;
use alloc::vec;

#[no_mangle]
pub fn main() {
    let x = vec![1, 2, 3];
    log("Hello from Skyline Rust Plugin!\n");

    if x[1] == 2 {
        log("x[1] == 2\n");
    } else {
        log("x[1] != 2\n");
    }
}
