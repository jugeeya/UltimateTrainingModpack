#![no_std]
#![feature(proc_macro_hygiene)]

use skyline::nn::account::{self, Uid, GetLastOpenedUser, GetNickname, Nickname};

#[skyline::main]
pub fn main() {
    println!("Hello from Skyline Rust Plugin!\n");

    for i in 0..3 {
        println!("{}", i);
    }

    init_accounts();

    let nickname = unsafe { get_last_user_nickname() };

    println!("Last nickname: {}", nickname);
}

fn init_accounts() {
    unsafe { account::Initialize() };
}

unsafe fn get_last_user_nickname() -> Nickname {
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
