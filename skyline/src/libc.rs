//! Public exports of libc functions
pub use core::ffi::c_void;

#[allow(non_camel_case_types)]
type size_t = usize;

extern "C" {
    pub fn malloc(size: size_t) -> *const c_void;
    pub fn free(ptr: *const c_void);
    pub fn calloc(num: size_t, size: size_t) -> *const c_void;
    pub fn realloc(ptr: *const c_void, size: size_t) -> *const c_void;
    // fn aligned_alloc(align: usize, size: usize) -> *const c_void;
}
