use std::env;
use std::fs;
use std::path::Path;

#[cfg(target_os = "windows")]
fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_lib_path = Path::new(&out_dir).join("../../../deps").join(
        "imgtype.lib",
    );
    let dest_dll_path = Path::new(&out_dir).join("../../..").join("imgtype.dll");
    let _ = fs::copy("./vendor/imgtype.dll", dest_dll_path);
    let _ = fs::copy("./vendor/imgtype.lib", dest_lib_path);
}

#[cfg(target_os = "macos")]
fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_lib_path = Path::new(&out_dir).join("lib").join("libimgtype.a");
    let _ = fs::create_dir(Path::new(&out_dir).join("lib"));
    let _ = fs::copy("./vendor/libimgtype.a.macos", dest_lib_path);

    println!("cargo:rustc-link-search=native={}/lib", out_dir);
}
