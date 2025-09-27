use std::env;
use std::path::PathBuf;

fn main() {
    // Compile C library
    cc::Build::new()
        .file("../c_core/libfastga.c")
        .include("../c_core")
        .compile("fastga_c");

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header("../c_core/libfastga.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Link to C library
    println!("cargo:rustc-link-lib=static=fastga_c");
    println!("cargo:rerun-if-changed=../c_core/libfastga.c");
    println!("cargo:rerun-if-changed=../c_core/libfastga.h");
}