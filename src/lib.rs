#![no_std]
#![feature(proc_macro_hygiene)]

use skyline::nn::account::{self, Uid, GetLastOpenedUser, GetNickname, Nickname};
use skyline::smash::hash40;

#[skyline::main(name = "module_name_test")]
pub fn main() {
    println!("Hello from Skyline Rust Plugin!\n");

    for i in 0..3 {
        println!("{}", i);
    }

    let nickname = unsafe { get_last_user_nickname() };

    println!("Last nickname: {}", nickname);

    println!("Compile-time hash40 of 'accel_x': {:010X}", hash40("accel_x"));

    let string = "accel_x";
    println!("Runtime hash40 of '{}': {:010X}", string, hash40(string));
}

unsafe fn get_last_user_nickname() -> Nickname {
    account::Initialize();
    let uid = &mut Uid::new();
    let mut nick = Nickname::new();

    GetLastOpenedUser(uid);
    GetNickname(&mut nick, uid);

    nick
}


/*
use skyline::{
    libc::{fopen, FileOpenMode, fwrite_slice, fclose},
    c_str
};

fn write_to_file(file: &str, contents: &str) {
    unsafe {
        let file = fopen(c_str(file), FileOpenMode::Write);
        fwrite_slice(contents.as_bytes(), file);
        fclose(file);
    }
}*/
