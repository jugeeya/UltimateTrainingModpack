use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;

extern "C" {
    fn malloc(size: usize) -> *const c_void;
    fn free(ptr: *const c_void);
    fn calloc(num: usize, size: usize) -> *const c_void;
    fn realloc(ptr: *const c_void, size: usize) -> *const c_void;
    // fn aligned_alloc(align: usize, size: usize) -> *const c_void;
}

pub struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        malloc(layout.size()) as *mut u8
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        calloc(layout.size(), 1) as *mut u8
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free(ptr as *mut c_void)
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
        realloc(ptr as *mut c_void, new_size) as *mut u8
    }
}

#[alloc_error_handler]
fn _alloc_error(_layout: Layout) -> ! {
    panic!("Allocation error");
}
