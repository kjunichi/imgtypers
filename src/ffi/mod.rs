extern crate libc;

use self::libc::{c_int, c_void, c_uchar, c_short};

mod link;

#[cfg(windows)]
pub static VK_ESCAPE: c_int = 0x1b;

extern "C" {
    pub fn termWait() -> c_void;
    pub fn termPutImage(buf: *const c_uchar, width: c_int, height: c_int) -> c_void;
    pub fn termInit() -> c_void;
    pub fn termFlush() -> c_void;
    pub fn termClose() -> c_void;

    #[cfg(windows)]
    pub fn GetAsyncKeyState(v_key: c_int) -> c_short;
}
