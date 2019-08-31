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
    os::raw::{c_char, c_int, c_uint, c_void},
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
        open: Some(passthrough_open),
        opendir: None,
        poll: None,
        read_buf: None,
        read: Some(passthrough_read),
        readdir: Some(passthrough_readdir),
        readlink: Some(passthrough_readlink),
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

unsafe extern "C" fn passthrough_readlink(
    path: *const c_char,
    buf: *mut c_char,
    size: usize,
) -> c_int {
    log::trace!("called passthrough_readlink()");

    let res = readlink(path, buf, size - 1);
    if res == -1 {
        return errno() * -1;
    }

    *buf.offset(res) = 0;
    0
}

unsafe extern "C" fn passthrough_readdir(
    path: *const c_char,
    buf: *mut c_void,
    filler: fuse_fill_dir_t,
    _offset: off_t,
    _fi: *mut fuse_file_info,
    _flags: fuse_readdir_flags,
) -> c_int {
    log::trace!("called passthrough_readdir()");

    let filler = filler.expect("filler should not be null");

    let dp = opendir(path);
    if dp == ptr::null_mut() {
        return errno() * -1;
    }

    loop {
        let de = readdir(dp);
        if de == ptr::null_mut() {
            break;
        }
        let de = &mut *de;

        let mut st: stat = mem::zeroed();
        st.st_ino = de.d_ino;
        st.st_mode = (de.d_type as c_uint) << 12;

        if filler(buf, de.d_name.as_ptr(), &mut st, 0, 0) != 0 {
            break;
        }
    }

    closedir(dp);
    0
}

unsafe extern "C" fn passthrough_open(path: *const c_char, fi: *mut fuse_file_info) -> c_int {
    log::trace!("called passthrough_open()");

    let fi = &mut *fi;

    let res = open(path, fi.flags);
    if res == -1 {
        return errno() * -1;
    }

    fi.fh = res as u64;
    0
}

unsafe extern "C" fn passthrough_read(
    path: *const c_char,
    buf: *mut c_char,
    size: usize,
    offset: off_t,
    fi: *mut fuse_file_info,
) -> c_int {
    log::trace!("called passthrough_read()");

    let fd;
    if fi == ptr::null_mut() {
        fd = open(path, O_RDONLY as i32);
    } else {
        let fi = &mut *fi;
        fd = fi.fh as c_int;
    }
    if fd == -1 {
        return errno() * -1;
    }

    let mut res = pread(fd, buf as *mut c_void, size, offset) as c_int;
    if res == -1 {
        res = errno() * -1;
    }

    if fi == ptr::null_mut() {
        close(fd);
    }

    res
}
