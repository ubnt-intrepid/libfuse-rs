//! A wrapper for libfuse3 using bindgen.

#[allow(nonstandard_style, dead_code)]
pub mod sys {
    use libc::{flock, iovec, stat, statvfs, timespec};
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use libc::{c_char, c_int, c_uint, c_void, stat};
use std::{
    env,
    ffi::{self, CStr},
    mem,
};
use sys::*;

pub trait FS {
    unsafe fn init(&self, conn: *mut fuse_conn_info, cfg: *mut fuse_config);

    /// Get file attributes.
    unsafe fn getattr(&self, path: &CStr, stbuf: *mut stat, fi: *mut fuse_file_info) -> c_int;

    /// Check the file access permissions.
    unsafe fn access(&self, path: &CStr, mask: c_int) -> c_int;

    /// Read the target of a symbolic link.
    unsafe fn readlink(&self, path: &CStr, buf: &mut [u8]) -> c_int;

    /// Read a directory.
    unsafe fn readdir(
        &self,
        path: &CStr,
        buf: *mut c_void,
        filler: fuse_fill_dir_t,
        offset: off_t,
        fi: *mut fuse_file_info,
        flags: fuse_readdir_flags,
    ) -> c_int;

    /// Create a file node.
    unsafe fn mknod(&self, path: &CStr, mode: libc::mode_t, rdev: libc::dev_t) -> c_int;

    /// Create a directory.
    unsafe fn mkdir(&self, path: &CStr, mode: libc::mode_t) -> c_int;

    /// Remove a file.
    unsafe fn unlink(&self, path: &CStr) -> c_int;

    /// Remove a directory.
    unsafe fn rmdir(&self, path: &CStr) -> c_int;

    /// Create a symbolic link.
    unsafe fn symlink(&self, path_from: &CStr, path_to: &CStr) -> c_int;

    /// Rename a file.
    unsafe fn rename(&self, path_from: &CStr, path_to: &CStr, flags: c_uint) -> c_int;

    /// Create a hard link to a file.
    unsafe fn link(&self, path_from: &CStr, path_to: &CStr) -> c_int;

    /// Change the permission bits of a file.
    unsafe fn chmod(&self, path: &CStr, mode: libc::mode_t, fi: *mut fuse_file_info) -> c_int;

    /// Change the owner and group of a file.
    unsafe fn chown(
        &self,
        path: &CStr,
        uid: libc::uid_t,
        gid: libc::gid_t,
        fi: *mut fuse_file_info,
    ) -> c_int;

    /// Change the size of a file.
    unsafe fn truncate(&self, path: &CStr, size: libc::off_t, fi: *mut fuse_file_info) -> c_int;

    /// Create and open a file.
    unsafe fn create(&self, path: &CStr, mode: libc::mode_t, fi: *mut fuse_file_info) -> c_int;

    /// Open a file.
    unsafe fn open(&self, path: &CStr, fi: *mut fuse_file_info) -> c_int;

    /// Read data from an opened file.
    unsafe fn read(
        &self,
        path: &CStr,
        buf: &mut [u8],
        offset: off_t,
        fi: *mut fuse_file_info,
    ) -> c_int;

    /// Write data to an opened file.
    unsafe fn write(
        &self,
        path: &CStr,
        buf: &[u8],
        offset: off_t,
        fi: *mut fuse_file_info,
    ) -> c_int;

    /// Get file system statistics.
    unsafe fn statfs(&self, path: &CStr, stbuf: *mut libc::statvfs) -> c_int;

    /// Release an opened file.
    unsafe fn release(&self, path: &CStr, fi: *mut fuse_file_info) -> c_int;
}

pub fn main<F: FS>(fuse: F) -> ! {
    let ops = fuse_operations {
        access: Some(lib_access::<F>),
        bmap: None,
        chmod: Some(lib_chmod::<F>),
        chown: Some(lib_chown::<F>),
        copy_file_range: None,
        create: Some(lib_create::<F>),
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
        link: Some(lib_link::<F>),
        listxattr: None,
        lock: None,
        mkdir: Some(lib_mkdir::<F>),
        mknod: Some(lib_mknod::<F>),
        open: Some(lib_open::<F>),
        opendir: None,
        poll: None,
        read_buf: None,
        read: Some(lib_read::<F>),
        readdir: Some(lib_readdir::<F>),
        readlink: Some(lib_readlink::<F>),
        release: Some(lib_release::<F>),
        releasedir: None,
        removexattr: None,
        rename: Some(lib_rename::<F>),
        rmdir: Some(lib_rmdir::<F>),
        setxattr: None,
        statfs: Some(lib_statfs::<F>),
        symlink: Some(lib_symlink::<F>),
        truncate: Some(lib_truncate::<F>),
        unlink: Some(lib_unlink::<F>),
        utimens: None,
        write_buf: None,
        write: Some(lib_write::<F>),
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
    with_get_cx(|fs: &F| fs.getattr(CStr::from_ptr(path), stbuf, fi))
}

unsafe extern "C" fn lib_access<F: FS>(path: *const c_char, mask: c_int) -> c_int {
    with_get_cx(|fs: &F| fs.access(CStr::from_ptr(path), mask))
}

unsafe extern "C" fn lib_readlink<F: FS>(
    path: *const c_char,
    buf: *mut c_char,
    size: usize,
) -> c_int {
    with_get_cx(|fs: &F| {
        let path = CStr::from_ptr(path);
        let buf = std::slice::from_raw_parts_mut(buf as *mut u8, size);
        fs.readlink(path, buf)
    })
}

unsafe extern "C" fn lib_readdir<F: FS>(
    path: *const c_char,
    buf: *mut c_void,
    filler: fuse_fill_dir_t,
    offset: off_t,
    fi: *mut fuse_file_info,
    flags: fuse_readdir_flags,
) -> c_int {
    with_get_cx(|fs: &F| fs.readdir(CStr::from_ptr(path), buf, filler, offset, fi, flags))
}

unsafe extern "C" fn lib_mknod<F: FS>(
    path: *const c_char,
    mode: libc::mode_t,
    rdev: libc::dev_t,
) -> c_int {
    with_get_cx(|fs: &F| fs.mknod(CStr::from_ptr(path), mode, rdev))
}

unsafe extern "C" fn lib_mkdir<F: FS>(path: *const c_char, mode: libc::mode_t) -> c_int {
    with_get_cx(|fs: &F| fs.mkdir(CStr::from_ptr(path), mode))
}

unsafe extern "C" fn lib_unlink<F: FS>(path: *const c_char) -> c_int {
    with_get_cx(|fs: &F| fs.unlink(CStr::from_ptr(path)))
}

unsafe extern "C" fn lib_rmdir<F: FS>(path: *const c_char) -> c_int {
    with_get_cx(|fs: &F| fs.rmdir(CStr::from_ptr(path)))
}

unsafe extern "C" fn lib_symlink<F: FS>(path_from: *const c_char, path_to: *const c_char) -> c_int {
    with_get_cx(|fs: &F| fs.symlink(CStr::from_ptr(path_from), CStr::from_ptr(path_to)))
}

unsafe extern "C" fn lib_rename<F: FS>(
    path_from: *const c_char,
    path_to: *const c_char,
    flags: c_uint,
) -> c_int {
    with_get_cx(|fs: &F| fs.rename(CStr::from_ptr(path_from), CStr::from_ptr(path_to), flags))
}

unsafe extern "C" fn lib_link<F: FS>(path_from: *const c_char, path_to: *const c_char) -> c_int {
    with_get_cx(|fs: &F| fs.link(CStr::from_ptr(path_from), CStr::from_ptr(path_to)))
}

unsafe extern "C" fn lib_chmod<F: FS>(
    path: *const c_char,
    mode: libc::mode_t,
    fi: *mut fuse_file_info,
) -> c_int {
    with_get_cx(|fs: &F| fs.chmod(CStr::from_ptr(path), mode, fi))
}

unsafe extern "C" fn lib_chown<F: FS>(
    path: *const c_char,
    uid: libc::uid_t,
    gid: libc::gid_t,
    fi: *mut fuse_file_info,
) -> c_int {
    with_get_cx(|fs: &F| fs.chown(CStr::from_ptr(path), uid, gid, fi))
}

unsafe extern "C" fn lib_truncate<F: FS>(
    path: *const c_char,
    size: libc::off_t,
    fi: *mut fuse_file_info,
) -> c_int {
    with_get_cx(|fs: &F| fs.truncate(CStr::from_ptr(path), size, fi))
}

unsafe extern "C" fn lib_create<F: FS>(
    path: *const c_char,
    mode: libc::mode_t,
    fi: *mut fuse_file_info,
) -> c_int {
    with_get_cx(|fs: &F| fs.create(CStr::from_ptr(path), mode, fi))
}

unsafe extern "C" fn lib_open<F: FS>(path: *const c_char, fi: *mut fuse_file_info) -> c_int {
    with_get_cx(|fs: &F| fs.open(CStr::from_ptr(path), fi))
}

unsafe extern "C" fn lib_read<F: FS>(
    path: *const c_char,
    buf: *mut c_char,
    size: usize,
    offset: off_t,
    fi: *mut fuse_file_info,
) -> c_int {
    with_get_cx(|fs: &F| {
        let path = CStr::from_ptr(path);
        let buf = std::slice::from_raw_parts_mut(buf as *mut u8, size);
        fs.read(path, buf, offset, fi)
    })
}

unsafe extern "C" fn lib_write<F: FS>(
    path: *const c_char,
    buf: *const c_char,
    size: usize,
    offset: off_t,
    fi: *mut fuse_file_info,
) -> c_int {
    with_get_cx(|fs: &F| {
        let path = CStr::from_ptr(path);
        let buf = std::slice::from_raw_parts(buf as *const u8, size);
        fs.write(path, buf, offset, fi)
    })
}

unsafe extern "C" fn lib_statfs<F: FS>(path: *const c_char, stbuf: *mut libc::statvfs) -> c_int {
    with_get_cx(|fs: &F| fs.statfs(CStr::from_ptr(path), stbuf))
}

unsafe extern "C" fn lib_release<F: FS>(path: *const c_char, fi: *mut fuse_file_info) -> c_int {
    with_get_cx(|fs: &F| fs.release(CStr::from_ptr(path), fi))
}
