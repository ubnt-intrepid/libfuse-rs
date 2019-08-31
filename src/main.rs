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
    env, ffi, mem,
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
        destroy: Some(passthrough_destroy),
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

    let data = Data {
        current_dir: env::current_dir().unwrap(),
    };
    fuse_main(&ops, data);
}

fn fuse_main(ops: &fuse_operations, data: Data) -> c_int {
    let args: Vec<ffi::CString> = env::args()
        .map(ffi::CString::new)
        .collect::<Result<_, _>>()
        .expect("failed to construct C-style arguments list");
    let mut c_args: Vec<*const c_char> = args.iter().map(|a| a.as_ptr()).collect();
    let argc = c_args.len() as i32;
    let argv = c_args.as_mut_ptr() as *mut *mut c_char;

    let data_ptr = Box::into_raw(Box::new(data));

    unsafe {
        fuse_main_real(
            argc,
            argv,
            ops,
            mem::size_of::<fuse_operations>(),
            mem::transmute(data_ptr),
        )
    }
}

#[derive(Debug)]
struct Data {
    current_dir: std::path::PathBuf,
}

unsafe extern "C" fn passthrough_destroy(data: *mut c_void) {
    mem::drop(Box::from_raw(data as *mut Data));
}

unsafe fn resolve_path(path: *const c_char) -> ffi::CString {
    let cx = fuse_get_context();
    debug_assert!(!cx.is_null());
    let cx = &mut *cx;
    debug_assert!(!cx.private_data.is_null());
    let data = &mut *(cx.private_data as *mut Data);

    let path = ffi::CStr::from_ptr(path);
    let path = path.to_string_lossy();

    let path = if path == "/" {
        std::borrow::Cow::Borrowed(&*data.current_dir)
    } else {
        std::borrow::Cow::Owned(data.current_dir.join(&*path.trim_start_matches("/")))
    };

    ffi::CString::from_vec_unchecked(path.to_string_lossy().into_owned().into())
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

    let cx = fuse_get_context();
    debug_assert!(!cx.is_null());
    let data_ptr = (&mut *cx).private_data;
    debug_assert!(!data_ptr.is_null());

    data_ptr
}

unsafe extern "C" fn passthrough_getattr(
    path: *const c_char,
    stbuf: *mut stat,
    _fi: *mut fuse_file_info,
) -> c_int {
    let path = resolve_path(path);
    log::trace!("called passthrough_getattr(path={:?})", path);

    let res = lstat(path.as_ptr(), stbuf);
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
    let path = resolve_path(path);
    log::trace!("called passthrough_readlink(path={:?})", path);

    let res = readlink(path.as_ptr(), buf, size - 1);
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
    let path = resolve_path(path);
    let filler = filler.expect("filler should not be null");
    log::trace!("called passthrough_readdir(path={:?})", path);

    let dp = opendir(path.as_ptr());
    if dp == ptr::null_mut() {
        return errno() * -1;
    }
    let dp = &mut *dp;

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
    let path = resolve_path(path);
    log::trace!("called passthrough_open(path={:?})", path);

    let fi = &mut *fi;

    let res = open(path.as_ptr(), fi.flags);
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
    let path = resolve_path(path);
    log::trace!("called passthrough_read(path={:?})", path);

    let fd;
    if fi == ptr::null_mut() {
        fd = open(path.as_ptr(), O_RDONLY as i32);
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
