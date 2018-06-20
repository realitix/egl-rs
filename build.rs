extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let current_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let clang_arg = ["-I", &current_dir, "/include"].join("");

    println!("cargo:rustc-link-lib=EGL");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(clang_arg)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
