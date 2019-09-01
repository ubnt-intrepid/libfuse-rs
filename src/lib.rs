#[allow(nonstandard_style, dead_code)]
pub mod sys {
    use libc::{flock, iovec, stat, statvfs, timespec};
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use libc::{c_char, c_int, c_void, stat};
use std::{env, ffi, mem};
use sys::*;

pub trait FS {
    unsafe fn init(&self, _conn: *mut fuse_conn_info, cfg: *mut fuse_config);

    unsafe fn getattr(
        &self,
        path: *const c_char,
        stbuf: *mut stat,
        _fi: *mut fuse_file_info,
    ) -> c_int;

    unsafe fn readlink(&self, path: *const c_char, buf: *mut c_char, size: usize) -> c_int;

    unsafe fn readdir(
        &self,
        path: *const c_char,
        buf: *mut c_void,
        filler: fuse_fill_dir_t,
        _offset: off_t,
        _fi: *mut fuse_file_info,
        _flags: fuse_readdir_flags,
    ) -> c_int;

    unsafe fn open(&self, path: *const c_char, fi: *mut fuse_file_info) -> c_int;

    unsafe fn read(
        &self,
        path: *const c_char,
        buf: *mut c_char,
        size: usize,
        offset: off_t,
        fi: *mut fuse_file_info,
    ) -> c_int;
}

pub fn main<F: FS>(fuse: F) -> ! {
    let ops = fuse_operations {
        access: None,
        bmap: None,
        chmod: None,
        chown: None,
        copy_file_range: None,
        create: None,
        destroy: Some(lib_destroy::<F>),
        fallocate: None,
        flock: None,
        flush: None,
        fsync: None,
        fsyncdir: None,
        getattr: Some(lib_getattr::<F>),
        getxattr: None,
        init: Some(lib_init::<F>),
        ioctl: None,
        link: None,
        listxattr: None,
        lock: None,
        mkdir: None,
        mknod: None,
        open: Some(lib_open::<F>),
        opendir: None,
        poll: None,
        read_buf: None,
        read: Some(lib_read::<F>),
        readdir: Some(lib_readdir::<F>),
        readlink: Some(lib_readlink::<F>),
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

    let args: Vec<ffi::CString> = env::args()
        .map(ffi::CString::new)
        .collect::<Result<_, _>>()
        .expect("failed to construct C-style arguments list");
    let mut c_args: Vec<*const c_char> = args.iter().map(|a| a.as_ptr()).collect();
    let argc = c_args.len() as i32;
    let argv = c_args.as_mut_ptr() as *mut *mut c_char;

    let data_ptr = Box::into_raw(Box::new(fuse));

    let code = unsafe {
        fuse_main_real(
            argc,
            argv,
            &ops,
            mem::size_of::<fuse_operations>(),
            mem::transmute(data_ptr),
        )
    };

    std::process::exit(code)
}

unsafe fn with_get_cx<F: FS, T>(f: impl FnOnce(&F) -> T) -> T {
    let cx = fuse_get_context();
    debug_assert!(!cx.is_null());
    let cx = &mut *cx;
    debug_assert!(!cx.private_data.is_null());
    let data = &*(cx.private_data as *mut F as *const F);
    f(data)
}

unsafe extern "C" fn lib_init<F: FS>(
    conn: *mut fuse_conn_info,
    cfg: *mut fuse_config,
) -> *mut c_void {
    with_get_cx(|fs: &F| {
        fs.init(conn, cfg);

        let cx = fuse_get_context();
        debug_assert!(!cx.is_null());
        let data_ptr = (&mut *cx).private_data;
        debug_assert!(!data_ptr.is_null());
        data_ptr
    })
}

unsafe extern "C" fn lib_destroy<F: FS>(fs: *mut c_void) {
    mem::drop(Box::from_raw(fs as *mut F));
}

unsafe extern "C" fn lib_getattr<F: FS>(
    path: *const c_char,
    stbuf: *mut stat,
    fi: *mut fuse_file_info,
) -> c_int {
    with_get_cx(|fs: &F| fs.getattr(path, stbuf, fi))
}

unsafe extern "C" fn lib_readlink<F: FS>(
    path: *const c_char,
    buf: *mut c_char,
    size: usize,
) -> c_int {
    with_get_cx(|fs: &F| fs.readlink(path, buf, size))
}

unsafe extern "C" fn lib_readdir<F: FS>(
    path: *const c_char,
    buf: *mut c_void,
    filler: fuse_fill_dir_t,
    offset: off_t,
    fi: *mut fuse_file_info,
    flags: fuse_readdir_flags,
) -> c_int {
    with_get_cx(|fs: &F| fs.readdir(path, buf, filler, offset, fi, flags))
}

unsafe extern "C" fn lib_open<F: FS>(path: *const c_char, fi: *mut fuse_file_info) -> c_int {
    with_get_cx(|fs: &F| fs.open(path, fi))
}

unsafe extern "C" fn lib_read<F: FS>(
    path: *const c_char,
    buf: *mut c_char,
    size: usize,
    offset: off_t,
    fi: *mut fuse_file_info,
) -> c_int {
    with_get_cx(|fs: &F| fs.read(path, buf, size, offset, fi))
}
