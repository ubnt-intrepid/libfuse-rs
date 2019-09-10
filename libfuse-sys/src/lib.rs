//! libfuse3 bindings.

#![allow(nonstandard_style)]

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

include!(concat!(env!("OUT_DIR"), "/fuse_lowlevel.rs"));
