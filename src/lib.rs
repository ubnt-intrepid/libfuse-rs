//! A wrapper for libfuse.

pub mod dir;
pub mod lowlevel;

mod common;
mod fuse;
mod ops;

pub use crate::common::{Config, ConnInfo, FileInfo};
pub use crate::fuse::Fuse;
pub use crate::ops::Operations;

pub type Result<T> = std::result::Result<T, libc::c_int>;
