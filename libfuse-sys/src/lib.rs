//! libfuse3 bindings.

#![allow(nonstandard_style)]

use libc::{
    c_int, //
    dev_t,
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

include!(concat!(env!("OUT_DIR"), "/fuse_lowlevel.rs"));

pub const FUSE_SET_ATTR_MODE: c_int = 1 << 0;
pub const FUSE_SET_ATTR_UID: c_int = 1 << 1;
pub const FUSE_SET_ATTR_GID: c_int = 1 << 2;
pub const FUSE_SET_ATTR_SIZE: c_int = 1 << 3;
pub const FUSE_SET_ATTR_ATIME: c_int = 1 << 4;
pub const FUSE_SET_ATTR_MTIME: c_int = 1 << 5;
pub const FUSE_SET_ATTR_ATIME_NOW: c_int = 1 << 7;
pub const FUSE_SET_ATTR_MTIME_NOW: c_int = 1 << 8;
pub const FUSE_SET_ATTR_CTIME: c_int = 1 << 10;
