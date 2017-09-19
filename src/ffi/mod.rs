extern crate libc;

#[cfg(target_os="windows")]
use self::libc::{c_int, c_void, c_uchar, c_short};

#[cfg(not(target_os="windows"))]
use self::libc::{c_int, c_void, c_uchar};

mod link;

#[cfg(target_os="windows")]
pub static VK_ESCAPE: c_int = 0x1b;

#[cfg(target_os="windows")]
extern "system" {
    pub fn termWait() -> c_void;
    pub fn termPutImage(buf: *const c_uchar, width: c_int, height: c_int) -> c_void;
    pub fn termInit() -> c_void;
    pub fn termFlush() -> c_void;
    pub fn termClose() -> c_void;   
    pub fn GetAsyncKeyState(v_key: c_int) -> c_short;
}

#[cfg(not(target_os="windows"))]
extern "C" {
    pub fn termWait() -> c_void;
    pub fn termPutImage(buf: *const c_uchar, width: c_int, height: c_int) -> c_void;
    pub fn termInit() -> c_void;
    pub fn termFlush() -> c_void;
    pub fn termClose() -> c_void;   
}
