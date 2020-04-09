#![no_std]
#![feature(proc_macro_hygiene)]

use skyline::{
    libc::{fopen, FileOpenMode, fwrite_slice, fclose},
    c_str
};

#[skyline::main]
pub fn main() {
    println!("Hello from Skyline Rust Plugin!\n");

    for i in 0..3 {
        println!("{}", i);
    }

    println!("Writing to file!");
    
    write_to_file("sd:/test.txt\0", "test test test test");
    
    println!("Done writing to file!");
}

fn write_to_file(file: &str, contents: &str) {
    unsafe {
        let file = fopen(c_str(file), FileOpenMode::Write);
        fwrite_slice(contents.as_bytes(), file);
        fclose(file);
    }
}
