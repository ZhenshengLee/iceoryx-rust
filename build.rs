// SPDX-License-Identifier: Apache-2.0
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let link_search_path: String = if let Ok(path) = std::env::var("ICEORYX_LIB") {
        format!("cargo:rustc-link-search={}", path)
    } else {
        "cargo:rustc-link-search=/usr/local/lib".into()
    };
    println!("{}", link_search_path);
    println!("cargo:rustc-link-lib=iceoryx_posh_roudi");
    println!("cargo:rustc-link-lib=iceoryx_posh");
    println!("cargo:rustc-link-lib=iceoryx_utils");
    println!("cargo:rustc-link-lib=iceoryx_platform");
    println!("cargo:rustc-link-lib=iceoryx_binding_c");
    println!("cargo:rustc-link-lib=stdc++");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_arg("-std=c++14")
        .clang_arg("-x")
        .clang_arg("c++")
        // blocklist
        // .blocklist_type("iox_notification_info_t")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        // .write_to_file("/home/zs/bindings.rs")
        .expect("Couldn't write bindings!");
}
