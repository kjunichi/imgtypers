extern crate libc;

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
    unsafe {
        ffi::termPutImage(ptr, width, height);
    }
}

#[cfg(windows)]
pub fn term_get_esc_key() -> bool {
    let r: i16 = unsafe { ffi::GetAsyncKeyState(ffi::VK_ESCAPE) } as i16;
    if r != 0 {
        return true;
    }
    return false;
}

#[cfg(not(windows))]
pub fn term_get_esc_key() -> bool {
    let mut buf: [libc::c_char; 1] = [0; 1];
    let ptr = &mut buf;
    let r = unsafe { libc::read(0, ptr.as_ptr() as *mut libc::c_void, 1) };
    if r == 27 {
        return true;
    }
    return false;
}
