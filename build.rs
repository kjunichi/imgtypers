#[cfg(target_os="windows")]
use std::env;
use std::fs;
use std::path::Path;
use std::fs::File;
use std::io::Write;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_dll_path = Path::new(&out_dir).join("../../..").join("tmgtype.dll");
    fs::copy("./vendor/imgtype.dll", dest_dll_path);
    let dest_path = Path::new(&out_dir).join("hello.rs");
    let mut f = File::create(&dest_path).unwrap();

    f.write_all(b"
        pub fn message() -> &'static str {
            \"Hello, World!\"
        }
    ").unwrap();
    println!("cargo:rustc-link-search=native=./vendor");
}