extern crate imgtypers;

fn main() {
    let mut img: Vec<u8> = vec![0, 0, 0, 128, 128, 0, 255, 0, 0, 0, 255, 255];
    imgtypers::term_init();
    let mut ptr = &mut img;
    imgtypers::term_put_image(ptr, 2, 2);
    imgtypers::term_flush();
    imgtypers::term_wait();
    imgtypers::term_close();
}