//! libfuse3 bindings.

#![allow(nonstandard_style)]

#[cfg(feature = "bindgen")]
mod bindgen {
    use libc::{
        dev_t, //
        flock,
        gid_t,
        iovec,
        mode_t,
        off_t,
        pid_t,
        stat,
        statvfs,
        uid_t,
    };

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[cfg(feature = "bindgen")]
pub use crate::bindgen::*;

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
