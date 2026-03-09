use std::env;
use std::path::Path;

fn main() {
    let watch_files = ["c_src", "redirect.c", "redirect.h", "wrapper.h"];

    for path in &watch_files {
        println!("cargo:rerun-if-changed={}", path);
    }

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir);
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("Failed to get MANIFEST_DIR");
    let manifest_dir = Path::new(&manifest_dir);
    let redirect_header_path = manifest_dir.join("redirect.h");

    // We need to use the standard <stdio.h> here, so do not force-include the redirect header
    cc::Build::new().file("redirect.c").compile("redirect_impl");

    let mut build = cc::Build::new();
    build.file("c_src/lib.c");

    let redirect_flag = redirect_header_path.as_os_str();

    // Force-include the header for the following compilation
    if build.get_compiler().is_like_msvc() {
        build.flag("/FI");
        build.flag(redirect_flag);
    } else {
        build.flag("-include");
        build.flag(redirect_flag);
    }

    build.compile("redirect_c_src");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
