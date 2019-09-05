//! A wrapper for libfuse.

mod common;
pub mod dir;
mod fuse;
mod ops;

pub use crate::common::{Config, ConnInfo, FileInfo};
pub use crate::fuse::Fuse;
pub use crate::ops::Operations;

pub type Result<T> = std::result::Result<T, libc::c_int>;
