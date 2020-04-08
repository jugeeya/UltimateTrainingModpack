#![no_std]
use skyline::prelude::*;
skyline::setup!();

#[no_mangle]
pub fn main() {
    println!("Hello from Skyline Rust Plugin!\n");

    for i in 0..3 {
        println!("{}", i);
    }
}
