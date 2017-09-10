extern crate imgtypers;

fn main() {
    let img: Vec<u8> = vec![0, 0, 0, 128, 128, 0, 255, 0, 0, 0, 255, 255];
    imgtypers::term_init();
    imgtypers::term_put_image(img, 2, 2);
    imgtypers::term_flush();
    imgtypers::term_wait();
    imgtypers::term_close();
}