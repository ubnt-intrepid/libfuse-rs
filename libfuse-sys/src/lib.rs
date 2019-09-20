//! libfuse3 bindings.

#![allow(nonstandard_style)]

pub mod helpers;

use libc::{c_char, c_double, c_int, c_void, off_t, size_t, stat, statvfs};

#[repr(C)]
pub struct fuse_conn_info {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct fuse_ctx {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct fuse_entry_param {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct fuse_file_info {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct fuse_lowlevel_ops {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct fuse_req {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct fuse_session {
    _unused: [u8; 0],
}

pub type fuse_ino_t = u64;
pub type fuse_req_t = *mut fuse_req;

extern "C" {
    pub fn fuse_add_direntry(
        req: fuse_req_t,
        buf: *mut c_char,
        bufsize: size_t,
        name: *const c_char,
        stbuf: *const stat,
        off: off_t,
    ) -> size_t;

    pub fn fuse_remove_signal_handlers(se: *mut fuse_session);

    pub fn fuse_reply_attr(req: fuse_req_t, attr: *const stat, attr_timeout: c_double) -> c_int;

    pub fn fuse_reply_buf(req: fuse_req_t, buf: *const c_char, size: size_t) -> c_int;

    pub fn fuse_reply_create(
        req: fuse_req_t,
        e: *const fuse_entry_param,
        fi: *const fuse_file_info,
    ) -> c_int;

    pub fn fuse_reply_entry(req: fuse_req_t, e: *const fuse_entry_param) -> c_int;

    pub fn fuse_reply_err(req: fuse_req_t, err: c_int) -> c_int;

    pub fn fuse_reply_none(req: fuse_req_t);

    pub fn fuse_reply_open(req: fuse_req_t, fi: *const fuse_file_info) -> c_int;

    pub fn fuse_reply_readlink(req: fuse_req_t, link: *const c_char) -> c_int;

    pub fn fuse_reply_statfs(req: fuse_req_t, stbuf: *const statvfs) -> c_int;

    pub fn fuse_reply_write(req: fuse_req_t, count: size_t) -> c_int;

    pub fn fuse_reply_xattr(req: fuse_req_t, count: size_t) -> c_int;

    pub fn fuse_req_ctx(req: fuse_req_t) -> *const fuse_ctx;

    pub fn fuse_req_userdata(req: fuse_req_t) -> *mut c_void;

    pub fn fuse_session_destroy(se: *mut fuse_session);

    pub fn fuse_session_fd(se: *mut fuse_session) -> c_int;

    pub fn fuse_session_loop(se: *mut fuse_session) -> c_int;

    pub fn fuse_session_mount(se: *mut fuse_session, mountpoint: *const c_char) -> c_int;

    pub fn fuse_session_unmount(se: *mut fuse_session);

    pub fn fuse_set_signal_handlers(se: *mut fuse_session) -> c_int;
}

/// Capability bits for `fuse_conn_info.capable` and `fuse_conn_info.want`.
pub mod fuse_cap_flags {
    use libc::c_uint;

    pub type Type = c_uint;

    pub const FUSE_CAP_ASYNC_READ: Type = 1 << 0;
    pub const FUSE_CAP_POSIX_LOCKS: Type = 1 << 1;
    // 2
    pub const FUSE_CAP_ATOMIC_O_TRUNC: Type = 1 << 3;
    pub const FUSE_CAP_EXPORT_SUPPORT: Type = 1 << 4;
    // 5
    pub const FUSE_CAP_DONT_MASK: Type = 1 << 6;
    pub const FUSE_CAP_SPLICE_WRITE: Type = 1 << 7;
    pub const FUSE_CAP_SPLICE_MOVE: Type = 1 << 8;
    pub const FUSE_CAP_SPLICE_READ: Type = 1 << 9;
    pub const FUSE_CAP_FLOCK_LOCKS: Type = 1 << 10;
    pub const FUSE_CAP_IOCTL_DIR: Type = 1 << 11;
    pub const FUSE_CAP_AUTO_INVAL_DATA: Type = 1 << 12;
    pub const FUSE_CAP_READDIRPLUS: Type = 1 << 13;
    pub const FUSE_CAP_READDIRPLUS_AUTO: Type = 1 << 14;
    pub const FUSE_CAP_ASYNC_DIO: Type = 1 << 15;
    pub const FUSE_CAP_WRITEBACK_CACHE: Type = 1 << 16;
    pub const FUSE_CAP_NO_OPEN_SUPPORT: Type = 1 << 17;
    pub const FUSE_CAP_PARALLEL_DIROPS: Type = 1 << 18;
    pub const FUSE_CAP_POSIX_ACL: Type = 1 << 19;
    pub const FUSE_CAP_HANDLE_KILLPRIV: Type = 1 << 20;
    // 21
    // 22
    // 23
    pub const FUSE_CAP_NO_OPENDIR_SUPPORT: Type = 1 << 24;
}

/// Ioctl flags.
pub mod fuse_ioctl_flags {
    use libc::c_int;

    pub type Type = c_int;

    pub const FUSE_IOCTL_COMPAT: Type = 1 << 0;
    pub const FUSE_IOCTL_UNRESTRICTED: Type = 1 << 1;
    pub const FUSE_IOCTL_RETRY: Type = 1 << 2;
    // 3
    pub const FUSE_IOCTL_DIR: Type = 1 << 4;

    /// Maximum of in_iovecs + out_iovecs.
    pub const FUSE_IOCTL_MAX_IOV: usize = 256;
}

pub mod fuse_setattr_flags {
    use libc::c_int;

    pub type Type = c_int;

    pub const FUSE_SET_ATTR_MODE: Type = 1 << 0;
    pub const FUSE_SET_ATTR_UID: Type = 1 << 1;
    pub const FUSE_SET_ATTR_GID: Type = 1 << 2;
    pub const FUSE_SET_ATTR_SIZE: Type = 1 << 3;
    pub const FUSE_SET_ATTR_ATIME: Type = 1 << 4;
    pub const FUSE_SET_ATTR_MTIME: Type = 1 << 5;
    pub const FUSE_SET_ATTR_ATIME_NOW: Type = 1 << 7;
    pub const FUSE_SET_ATTR_MTIME_NOW: Type = 1 << 8;
    pub const FUSE_SET_ATTR_CTIME: Type = 1 << 10;
}
