use std::{env, path::PathBuf};

const FUSE_USE_VERSION: &str = "31";

fn main() {
    let out_dir = env::var("OUT_DIR").map(PathBuf::from).unwrap();

    // Find system libfuse.
    let fuse3_config = pkg_config::Config::new().probe("fuse3").unwrap();

    // Generate Rust bindings.
    generate_bindings(&fuse3_config, out_dir.join("bindings.rs"));

    // Build helper C functions.
    let mut helpers = cc::Build::new();
    helpers.file("src/helpers.c");
    if cfg!(feature = "cache-readdir") {
        helpers.file("src/cache_readdir.c");
    }
    helpers.define("FUSE_USE_VERSION", FUSE_USE_VERSION);
    for incpath in &fuse3_config.include_paths {
        helpers.include(incpath);
    }
    helpers.warnings_into_errors(true);
    helpers.compile("helpers");
}

#[cfg(not(feature = "bindgen"))]
fn generate_bindings(_: &pkg_config::Library, _: PathBuf) {}

#[cfg(feature = "bindgen")]
fn generate_bindings(fuse3_config: &pkg_config::Library, out_path: PathBuf) {
    use bindgen::EnumVariation;
    use bindgen_crate as bindgen;

    const LIBC_BLACKLIST_TYPES: &str =
        "__(.*)_t|(dev|gid|mode|off|pid|uid)_t|flock|iovec|stat|statvfs|timespec";

    const FUSE_WHITELIST_TYPES: &[&str] = &[
        "^fuse_conn_info$",
        "^fuse_ctx$",
        "^fuse_entry_param$",
        "^fuse_file_info$",
        "^fuse_ino_t$",
        "^fuse_lowlevel_ops$",
        "^fuse_req$",
        "^fuse_session$",
    ];

    const FUSE_WHITELIST_FUNCTIONS: &[&str] = &[
        "^fuse_add_direntry$",
        "^fuse_reply_(attr|buf|create|entry|err|none|open|readlink|statfs|write|xattr)$",
        "^fuse_req_(ctx|userdata)$",
        "^fuse_session_(destroy|mount|unmount|loop|fd)$",
        "^fuse_(set|remove)_signal_handlers$",
    ];

    let bindings = bindgen::Builder::default()
        .header("src/bindings.h")
        .clang_arg(format!("-DFUSE_USE_VERSION={}", FUSE_USE_VERSION))
        .clang_args(
            fuse3_config
                .include_paths
                .iter()
                .map(|incpath| format!("-I{}", incpath.display())),
        )
        .whitelist_type(FUSE_WHITELIST_TYPES.join("|"))
        .whitelist_function(FUSE_WHITELIST_FUNCTIONS.join("|"))
        .blacklist_type(LIBC_BLACKLIST_TYPES)
        .blacklist_type("fuse_loop_config")
        .blacklist_function("fuse_session_loop_mt(.*)")
        .opaque_type(FUSE_WHITELIST_TYPES.join("|"))
        .default_enum_style(EnumVariation::ModuleConsts)
        .ctypes_prefix("libc")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings");
}
