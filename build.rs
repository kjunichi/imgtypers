#[cfg(target_os="windows")]
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_lib_path = Path::new(&out_dir).join("../../../deps").join("imgtype.lib");
    let dest_dll_path = Path::new(&out_dir).join("../../..").join("imgtype.dll");
    let _ = fs::copy("./vendor/imgtype.dll", dest_dll_path);
    let _ = fs::copy("./vendor/imgtype.lib", dest_lib_path);
}