use std::{env, path::PathBuf};

const FUSE_USE_VERSION: &str = "31";

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").map(PathBuf::from).unwrap();

    // Find system libfuse.
    let fuse3_config = pkg_config::Config::new().probe("fuse3").unwrap();

    // Build helper C functions.
    let mut helpers = cc::Build::new();
    helpers.file(manifest_dir.join("src/helpers/common.c"));
    if cfg!(feature = "cache-readdir") {
        helpers.file(manifest_dir.join("src/helpers/cache_readdir.c"));
    }
    helpers.define("FUSE_USE_VERSION", FUSE_USE_VERSION);
    for incpath in &fuse3_config.include_paths {
        helpers.include(incpath);
    }
    helpers.warnings_into_errors(true);
    helpers.compile("helpers");
}
