use std::{
    env,
    ffi::OsStr,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

fn main() {
    // Build libfuse3.
    let libfuse_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("libfuse");
    let build_dir = PathBuf::from(env::var("OUT_DIR").unwrap()).join("build");
    run_meson(&libfuse_dir, &build_dir);

    println!("cargo:rerun-if-changed={}", libfuse_dir.display());
    println!("cargo:rustc-link-lib=static=fuse3");
    println!("cargo:rustc-link-search=native={}", build_dir.join("lib").display());

    // Generate Rust bindings.
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(&["-Ilibfuse/include"])
        .whitelist_type("fuse_.*")
        .whitelist_function("fuse_.*")
        .blacklist_type("flock|iovec|stat|statvfs|timespec")
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}

fn run_meson(project_dir: &Path, build_dir: &Path) {
    if !build_dir.join("build.ninja").exists() {
        run_command(project_dir, "meson", &[".".as_ref(), build_dir.as_os_str()]);
    }
    run_command(build_dir, "ninja", &[]);
}

fn run_command(dir: &Path, name: &str, args: &[&OsStr]) {
    let mut cmd = Command::new(name);
    cmd.current_dir(dir);
    cmd.stdout(Stdio::null());
    if args.len() > 0 {
        cmd.args(args);
    }
    let status = cmd.status().expect("cannot run command");
    assert!(status.success());
}
