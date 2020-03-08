extern crate bindgen;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=memory.x");
    //libnrf_cc310_bl_0.9.12.a
    println!("cargo:rustc-link-lib=static=nrf_cc310_bl_0.9.12");
    println!("cargo:rustc-link-search=nrfxlib/crypto/nrf_cc310_bl/lib/cortex-m4/hard-float");
    println!("cargo:rerun-if-changed=crypto/nrf_cc310_bl.h");
    let bindings = bindgen::Builder::default()
        .clang_arg("--sysroot=/Users/sigvartmh/Programming/gcc-arm-none-eabi-9-2019-q4-major")
        .ctypes_prefix("cty")
        .use_core()
        .rustified_non_exhaustive_enum(".*")
        .header("crypto/nrf_cc310_bl.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings.");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
