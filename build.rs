extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Check the target operating system
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    // Use different wrapper files based on the OS
    let header_file = if target_os == "windows" {
        "windows_wrapper.h"  // Use custom wrapper for Windows
    } else {
        "wrapper.h"  // Default wrapper for Unix-like systems
    };

    // Tell cargo to tell rustc to link the system ta_lib shared library.
    println!("cargo:rustc-link-lib=ta_lib");

    // Set up the include path explicitly for Windows (if needed)
    if target_os == "windows" {
        println!("cargo:rustc-link-search=native=C:/TA-Lib/c/lib");
    }

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        .header(header_file)
        // Generate rustified enums
        .rustified_enum(".*")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
