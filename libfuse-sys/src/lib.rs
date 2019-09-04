//! libfuse3 bindings.

#![allow(nonstandard_style)]

use libc::{flock, iovec, stat, statvfs, timespec};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
