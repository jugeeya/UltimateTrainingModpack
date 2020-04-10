#![no_std]
#![allow(incomplete_features)]
#![feature(alloc_error_handler, lang_items, start, global_asm, const_generics, impl_trait_in_bindings, proc_macro_hygiene, alloc_prelude, const_if_match, const_loop)]

/// The rust core allocation and collections library
pub extern crate alloc;

#[doc(hidden)]
pub use skyline_macro;

/// Types and functions for working with hooking
pub mod hooks;
pub mod logging;
pub mod smash;

#[doc(hidden)]
pub mod extern_alloc;

#[doc(hidden)]
pub mod build;

// nnsdk API bindings
pub mod nn {
    pub use nnsdk::root::nn::*;
}

#[doc(inline)]
pub use {
    skyline_libc as libc,
    skyline_macro::{main, hook}, 
    hooks::iter_hooks,
};

/// Helper to convert a str to a *const u8 (to be replaced)
pub fn c_str(string: &str) -> *const u8 {
    string.as_bytes().as_ptr()
}

/// A set of items that will likely be useful to import anyway
///
/// Designed to be used as such:
/// ```
/// use skyline::prelude::*;
/// ```
pub mod prelude {
    pub use crate::println;
    pub use alloc::format;
    pub use alloc::vec;
    pub use crate::alloc::prelude::v1::*;
}
