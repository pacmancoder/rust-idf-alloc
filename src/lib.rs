#![no_std]

use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

pub struct IdfAllocator;

extern "C" {
    pub fn malloc(count: usize) -> *mut core::ffi::c_void;
    pub fn realloc(old_ptr: *mut core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    pub fn free(ptr: *mut core::ffi::c_void);
}


unsafe impl GlobalAlloc for IdfAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        malloc(layout.size()) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free(ptr as *mut core::ffi::c_void)
    }

    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
        realloc(ptr as *mut core::ffi::c_void, new_size) as *mut u8
    }
}
