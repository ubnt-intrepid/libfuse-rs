use std::{env, path::PathBuf};

fn main() {
    pkg_config::probe_library("fuse3").expect("Failed to probe libfuse3");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(&["-I/usr/include/fuse3"])
        .whitelist_type("fuse_.*")
        .whitelist_function("fuse_.*")
        .whitelist_var("fuse_.*")
        .blacklist_type("flock|iovec|stat|statvfs|timespec")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
