use libc::c_uint;
use libfuse_sys::{fuse_conn_info, fuse_ino_t};

/// The type of inode number used in the filesystem.
pub type Ino = fuse_ino_t;

#[repr(C)]
pub struct ConnectionInfo(fuse_conn_info);

impl ConnectionInfo {
    /// Returns major version of the protocol.
    pub fn proto_major(&self) -> c_uint {
        self.0.proto_major
    }

    /// Returns the maximum size of read requests.
    pub fn max_read(&self) -> c_uint {
        self.0.max_read
    }

    /// Returns a mutable reference to the maximum size of read requests.
    pub fn max_read_mut(&mut self) -> &mut c_uint {
        &mut self.0.max_read
    }

    /// Returns minor version of the protocol.
    pub fn proto_minor(&self) -> c_uint {
        self.0.proto_minor
    }

    /// Returns capability flags that the kernel supports.
    pub fn capable(&self) -> c_uint {
        self.0.capable
    }
}
