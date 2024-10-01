extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::fs;

fn main() {
    // Default include and library paths (system default)
    let mut include_path = None;
    let mut lib_path = None;

    // Check if TA-Lib header and library are in default system paths
    let default_include = "/usr/include/ta-lib"; // Default for Unix-like systems
    let default_lib = "/usr/lib"; // Default for Unix-like systems

    // Check for TA-Lib on Windows or a custom location
    let custom_include = "C:/ta-lib/c/include"; // Custom path for Windows
    let custom_lib = "C:/ta-lib/c/lib";         // Custom library path for Windows

    // Check if the default include path exists
    if fs::metadata(default_include).is_ok() {
        include_path = Some(default_include);
        lib_path = Some(default_lib);
    } else if fs::metadata(custom_include).is_ok() {
        // Fallback to custom TA-Lib location (Windows)
        include_path = Some(custom_include);
        lib_path = Some(custom_lib);
    } else {
        // If neither are found, you can panic or handle this gracefully
        panic!("Unable to find TA-Lib headers or libraries. Please install TA-Lib.");
    }

    // Tell cargo to tell rustc to link the system ta_lib shared library.
    if let Some(lib) = lib_path {
        println!("cargo:rustc-link-search=native={}", lib);
    }
    println!("cargo:rustc-link-lib=ta_lib");

    // Set the include path for bindgen if it's found
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        .header("wrapper.h")
        // Add the include path if found
        .clang_arg(format!("-I{}", include_path.unwrap()))
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
