extern crate imgtypers;

fn main() {
    let mut img: Vec<u8> = vec![255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 255, 255];
    imgtypers::term_init();
    let mut ptr = &mut img;
    imgtypers::term_put_image(ptr, 2, 2);
    imgtypers::term_flush();

    if cfg!(not(windows)) {
        imgtypers::term_wait();
    }

    if cfg!(windows) {
        loop {
            if imgtypers::term_get_esc_key() {
                // imgtypers::term_close();
                break;
            }
        }
    }
    imgtypers::term_close();

}