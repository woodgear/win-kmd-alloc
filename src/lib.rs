#![no_std]
#![feature(alloc_error_handler)]

mod alloc_sys;

use core::alloc::{GlobalAlloc, Layout};
use self::alloc_sys::*;

const KMRS_TAG: u32 = 0x4B4D5253; // 'KMRS'

pub struct KernelAlloc;

unsafe impl GlobalAlloc for KernelAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let res = ExAllocatePoolWithTag(POOL_TYPE::NonPagedPool, layout.size(), KMRS_TAG);
        if res.is_null() {
            panic!("alloc fail");
        }
        res
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        ExFreePoolWithTag(ptr, KMRS_TAG);
    }
}

#[alloc_error_handler]
fn alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("{:?} alloc memory error", layout);
}