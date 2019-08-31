#[allow(nonstandard_style, dead_code)]
mod bindings {
    use std::os::raw::c_int;

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    pub unsafe fn errno() -> c_int {
        *__errno_location()
    }
}

use bindings::*;
use std::{
    env, ffi, fs, mem,
    os::raw::{c_char, c_int, c_void},
    ptr,
};

fn main() {
    env::set_var("RUST_LOG", "trace");
    pretty_env_logger::init();

    let ops = fuse_operations {
        access: None,
        bmap: None,
        chmod: None,
        chown: None,
        copy_file_range: None,
        create: None,
        destroy: None,
        fallocate: None,
        flock: None,
        flush: None,
        fsync: None,
        fsyncdir: None,
        getattr: Some(passthrough_getattr),
        getxattr: None,
        init: Some(passthrough_init),
        ioctl: None,
        link: None,
        listxattr: None,
        lock: None,
        mkdir: None,
        mknod: None,
        open: None,
        opendir: None,
        poll: None,
        read_buf: None,
        read: None,
        readdir: None,
        readlink: None,
        release: None,
        releasedir: None,
        removexattr: None,
        rename: None,
        rmdir: None,
        setxattr: None,
        statfs: None,
        symlink: None,
        truncate: None,
        unlink: None,
        utimens: None,
        write_buf: None,
        write: None,
    };

    unsafe {
        umask(0);
    }

    fs::create_dir_all("/tmp/mount").unwrap();
    env::set_current_dir("/tmp/mount").unwrap();

    fuse_main(&["passthrough", ".", "-d"], &ops);
}

fn fuse_main(args: impl IntoIterator<Item = impl AsRef<str>>, ops: &fuse_operations) -> c_int {
    let args: Vec<ffi::CString> = args
        .into_iter()
        .map(|s| ffi::CString::new(s.as_ref()))
        .collect::<Result<_, _>>()
        .expect("failed to construct C-style arguments list");
    let mut c_args: Vec<*const c_char> = args.iter().map(|a| a.as_ptr()).collect();
    let argc = c_args.len() as i32;
    let argv = c_args.as_mut_ptr() as *mut *mut c_char;
    unsafe {
        fuse_main_real(
            argc,
            argv,
            ops,
            mem::size_of::<fuse_operations>(),
            ptr::null_mut(),
        )
    }
}

unsafe extern "C" fn passthrough_init(
    _conn: *mut fuse_conn_info,
    cfg: *mut fuse_config,
) -> *mut c_void {
    log::trace!("called passthrough_init()");

    let cfg = &mut *cfg;

    cfg.use_ino = 1;

    cfg.entry_timeout = 0.0;
    cfg.attr_timeout = 0.0;
    cfg.negative_timeout = 0.0;

    ptr::null_mut()
}

unsafe extern "C" fn passthrough_getattr(
    path: *const c_char,
    stbuf: *mut stat,
    _fi: *mut fuse_file_info,
) -> c_int {
    log::trace!("called passthrough_getattr()");

    let res = lstat(path, stbuf);
    if res == -1 {
        return errno() * -1;
    }

    0
}
