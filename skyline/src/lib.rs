#![no_std]
#![allow(incomplete_features)]
#![feature(alloc_error_handler, lang_items, start, global_asm, const_generics, impl_trait_in_bindings)]

pub mod alloc;
pub use alloc::Allocator;

pub mod build;

extern "C" {
    fn skyline_tcp_send_raw(bytes: *const u8, usize: u64);
}

pub fn log(message: &str) {
    unsafe {
        skyline_tcp_send_raw(message.as_bytes().as_ptr(), message.as_bytes().len() as _);
    }
}

