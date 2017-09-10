extern crate libc;

use std::ffi::CString;
use std::mem;

pub mod ffi;
pub fn term_wait() -> () {
    unsafe {
        ffi::termWait();
    }
}

pub fn term_init() -> () {
    unsafe {
        ffi::termInit();
    }
}

pub fn term_flush() -> () {
    unsafe {
        ffi::termFlush();
    }
}
pub fn term_close() -> () {
    unsafe {
        ffi::termClose();
    }
}

pub fn term_put_image(mut img: &mut Vec<u8>, width: i32, height: i32) -> () {
    let ptr = (*img).as_mut_ptr();
    let len = (*img).len();
    // mem::forget(img); // prevent deallocation in Rust
    unsafe {
        ffi::termPutImage(ptr, width, height);
    }
}
