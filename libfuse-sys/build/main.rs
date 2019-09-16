use std::{env, path::PathBuf};

fn main() {
    let out_dir = env::var("OUT_DIR").map(PathBuf::from).unwrap();

    // Find system libfuse.
    let fuse3_config = pkg_config::Config::new().probe("fuse3").unwrap();

    // Generate Rust bindings.
    generate_bindings(&fuse3_config, out_dir.join("bindings.rs"));
}

#[cfg(not(feature = "bindgen"))]
fn generate_bindings(_: &pkg_config::Library, _: PathBuf) {}

#[cfg(feature = "bindgen")]
fn generate_bindings(fuse3_config: &pkg_config::Library, out_path: PathBuf) {
    use bindgen::EnumVariation;
    use bindgen_crate as bindgen;

    const LIBC_BLACKLIST_TYPES: &str =
        "__(.*)_t|(dev|gid|mode|off|pid|uid)_t|flock|iovec|stat|statvfs|timespec";

    let bindings = bindgen::Builder::default()
        .header("build/bindings.h")
        .clang_args(
            fuse3_config
                .include_paths
                .iter()
                .map(|incpath| format!("-I{}", incpath.display())),
        )
        .whitelist_type("fuse_.*")
        .whitelist_function("fuse_.*")
        .blacklist_type(LIBC_BLACKLIST_TYPES)
        .blacklist_type("fuse_loop_config")
        .blacklist_function("fuse_session_loop_mt(.*)")
        .default_enum_style(EnumVariation::ModuleConsts)
        .ctypes_prefix("libc")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings");
}
