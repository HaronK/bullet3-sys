extern crate bindgen;
extern crate cc;
extern crate cmake;
extern crate walkdir;

use cmake::Config;
use std::env;
use std::path::PathBuf;

fn main() {
    if cfg!(feature = "build") {
        let dst = Config::new("bullet3")
            .define("BUILD_PYBULLET", "OFF")
            .define("BUILD_UNIT_TESTS", "OFF")
            .define("BUILD_CPU_DEMOS", "OFF")
            .define("BUILD_BULLET2_DEMOS", "OFF")
            .define("USE_DOUBLE_PRECISION", "ON")
            .generator("Visual Studio 14 2015 Win64")
            .build();

        // println!("cargo:rustc-link-search=native={}", dst.display());
        // println!("cargo:rustc-link-lib=static=bullet3");
    }

    if cfg!(feature = "bind") {
        println!("cargo:rustc-link-lib=bullet3");

        let bindings = bindgen::Builder::default()
            .clang_arg("-x")
            .clang_arg("c++")
            .clang_arg("-Ibullet3/src")
            .header("bullet3.h")
            .generate()
            .expect("Unable to generate bindings");

        //let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        let out_path = PathBuf::from("src/");
        bindings
            .write_to_file(out_path.join("bullet3.rs"))
            .expect("Couldn't write bindings!");
    }
}
