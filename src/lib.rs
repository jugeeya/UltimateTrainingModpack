#![no_std]
#![allow(incomplete_features)]
#![feature(lang_items, start, global_asm, const_generics, impl_trait_in_bindings)]

use core::any::Any;
use skyline::Allocator;

#[global_allocator]
pub static ALLOCATOR: Allocator = Allocator;

extern crate alloc;
use alloc::vec;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {

    log("Panic at the Rust lib!\n");

    loop {}
}

#[lang = "eh_personality"] extern fn eh_personality() {}

extern "C" {
    fn skyline_tcp_send_raw(bytes: *const u8, usize: u64);
}

fn log(message: &str) {
    unsafe {
        skyline_tcp_send_raw(message.as_bytes().as_ptr(), message.as_bytes().len() as _);
    }
}

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

global_asm!(include_str!("mod0.s"));

#[repr(packed)]
#[allow(unused_variables)]
pub struct ModuleName<const LEN: usize> {
    pub unk: u32,
    pub name_length: u32,
    pub name: [u8; LEN],
}

impl<const LEN: usize> ModuleName<LEN> {
    pub const fn new(bytes: &[u8; LEN]) -> Self {
        Self {
            unk: 0,
            name_length: LEN as u32,
            name: *bytes,
        }
    }
}

#[link_section = ".rodata.module_name"]
pub static MODULE_NAME: impl Any = ModuleName::new(b"no_std_test\0");

#[no_mangle] pub extern "C" fn __custom_init() {}
#[no_mangle] pub extern "C" fn __custom_fini() {}
