#[cfg(target_os="windows")]
use std::env;
use std::fs;
use std::path::Path;
use std::fs::File;
use std::io::Write;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_lib_path = Path::new(&out_dir).join("../../../deps").join("imgtype.lib");
    let dest_dll_path = Path::new(&out_dir).join("../../..").join("imgtype.dll");
    fs::copy("./vendor/imgtype.dll", dest_dll_path);
    fs::copy("./vendor/imgtype.lib", dest_lib_path);

    //println!("cargo:rustc-link-search=native=./vendor");
}