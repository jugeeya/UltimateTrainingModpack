#![no_std]
#![allow(incomplete_features)]
#![feature(alloc_error_handler, lang_items, start, global_asm, const_generics, impl_trait_in_bindings, proc_macro_hygiene, alloc_prelude)]

pub extern crate alloc;

pub mod hooks;
pub mod build;
pub mod extern_alloc;
pub use extern_alloc::Allocator;
pub use skyline_macro::{main, hook};
pub use hooks::iter_hooks;

pub use skyline_libc as libc;

extern "C" {
    fn skyline_tcp_send_raw(bytes: *const u8, usize: u64);
}

pub fn log(message: &str) {
    unsafe {
        skyline_tcp_send_raw(message.as_bytes().as_ptr(), message.as_bytes().len() as _);
    }
}

pub fn c_str(string: &str) -> *const u8 {
    string.as_bytes().as_ptr()
}

#[macro_export] macro_rules! setup {
    () => {
        #[global_allocator]
        pub static ALLOCATOR: $crate::Allocator = $crate::Allocator;
    };
}

#[macro_export] macro_rules! println {
    () => {
        $crate::log();
    };
    ($($arg:tt)*) => {
        {
            use $crate::alloc::format;
            $crate::log(&format!(
                $($arg)*
            ));
        }
    };
}

pub mod prelude {
    pub use crate::Allocator;
    pub use crate::println;
    pub use crate::alloc::prelude::v1::*;
}
