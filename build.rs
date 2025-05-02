/// Build libspiro binding for Rust.
extern crate bindgen;
extern crate cc;
extern crate dos2unix;

use std::env;
use std::path::PathBuf;

fn do_bindgen() {
    // Based on `bindgen` manual.
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .derive_partialeq(true)
        .derive_debug(true)
        .derive_copy(true)
        .derive_default(true)
        // This function doesn't exist! Struct must be created.
        .blocklist_function("new_bezctx")
        // Avoids silly warnings about case of types. They're from C...
        .raw_line("#![allow(non_camel_case_types, non_snake_case)]")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("src");
    let out_file = out_path.join("lib.rs");
    bindings
        .write_to_file(out_file.clone())
        .expect("Couldn't write bindings!");

    // Windows sanity - use \n even on Windows.
    dos2unix::Dos2Unix::convert(&out_file.into_os_string().into_string().unwrap(), false);
}

fn main() {
    cc::Build::new()
        // We aren't using libspiro's autotools build, so I wrote a generic spiro-config.h which
        // will work on all the systems I care about.
        .include(".") // for spiro-config.h
        .file("libspiro/bezctx.c")
        .file("libspiro/spiro.c")
        .file("libspiro/spiroentrypoints.c")
        .static_flag(true)
        .shared_flag(true)
        .static_crt(true)
        .compile("libspiro.a");

    // Link built library. "lib" and ".a" are added back by Cargo.
    println!("cargo:rustc-link-lib=static=spiro");

    if env::var("DO_BINDGEN").is_ok() {
        do_bindgen();
    }
}
