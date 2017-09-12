extern crate libc;

use self::libc::{c_int, c_void, c_uchar};

#[link(name = "imgtype")]
extern "C" {
    pub fn termWait() -> c_void;
    pub fn termPutImage(buf: *const c_uchar, width: c_int, height: c_int) -> c_void;
    pub fn termInit() -> c_void;
    pub fn termFlush() -> c_void;
    pub fn termClose() -> c_void;
}