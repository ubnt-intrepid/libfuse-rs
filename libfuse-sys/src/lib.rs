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
