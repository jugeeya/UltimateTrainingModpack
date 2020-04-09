#![no_std]
#![feature(proc_macro_hygiene)]

use skyline::hook;

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

    for hook in skyline::iter_hooks() {
        println!("hook: {}", hook.info.fn_name);
    }
}
