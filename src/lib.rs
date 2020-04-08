#![no_std]
#![feature(proc_macro_hygiene)]

use skyline::{hook, hooks};

#[hook(sym = "nn::fs::MountSaveData")]
fn test1(path: *const u8, user_id: u64) {
    println!("user id: {}", user_id);
}

#[hook(inline, offset = 0x71000030)]
fn test2(x: u32) -> u64 {
    (x as u64) + 1
}

#[skyline::main]
pub fn main() {
    println!("Hello from Skyline Rust Plugin!\n");

    for i in 0..3 {
        println!("{}", i);
    }

    hooks![test1, test2].install_hooks();
}
