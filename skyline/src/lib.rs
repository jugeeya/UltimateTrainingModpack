#![no_std]
#![allow(incomplete_features)]
#![feature(alloc_error_handler, lang_items, start, global_asm, const_generics, impl_trait_in_bindings)]

pub extern crate alloc;

pub mod libc;
pub mod build;
pub mod extern_alloc;
pub use extern_alloc::Allocator;

extern "C" {
    fn skyline_tcp_send_raw(bytes: *const u8, usize: u64);
}

pub fn log(message: &str) {
    unsafe {
        skyline_tcp_send_raw(message.as_bytes().as_ptr(), message.as_bytes().len() as _);
    }
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
}
