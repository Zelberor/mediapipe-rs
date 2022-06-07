extern crate bindgen;

use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=opencv4");
    println!("cargo:rustc-link-lib=mediagraph");
    // println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .clang_arg("-xc++")
        .clang_arg("-std=c++14")
        .clang_arg("-I/usr/local/include/opencv4")
        .generate_comments(true)
        .header("/usr/local/include/mediagraph.h")
        .allowlist_function("mediagraph.*")
        .allowlist_type("mediagraph.*")
        .allowlist_var("mediagraph.*")
        .detect_include_paths(true)
        .generate_inline_functions(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("./src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
