use bitflags::bitflags;
use libc::c_uint;
use libfuse_sys::{fuse_cap_flags::*, fuse_conn_info, fuse_ino_t};

/// The type of inode number used in the filesystem.
pub type NodeId = fuse_ino_t;

pub const ROOT_NODEID: NodeId = 1;

/// Connection information passed to `Operations::init` method.
pub struct ConnectionInfo<'a>(pub(crate) &'a mut fuse_conn_info);

impl<'a> ConnectionInfo<'a> {
    /// Returns major version of the protocol.
    pub fn proto_major(&self) -> c_uint {
        self.0.proto_major
    }

    /// Returns minor version of the protocol.
    pub fn proto_minor(&self) -> c_uint {
        self.0.proto_minor
    }

    /// Returns the maximum size of read requests.
    pub fn max_read(&self) -> c_uint {
        self.0.max_read
    }

    /// Returns a mutable reference to the maximum size of read requests.
    pub fn max_read_mut(&mut self) -> &mut c_uint {
        &mut self.0.max_read
    }

    /// Returns capability flags that the kernel supports.
    pub fn capable(&self) -> CapabilityFlags {
        CapabilityFlags::from_bits_truncate(self.0.capable)
    }

    /// Returns capability flags that the filesystem wants to enable.
    pub fn want(&self) -> CapabilityFlags {
        CapabilityFlags::from_bits_truncate(self.0.want)
    }

    /// Sets capability flags that the filesystem wants to enable.
    pub fn set_want(&mut self, flags: CapabilityFlags) {
        self.0.want = flags.bits();
    }

    pub fn max_background_mut(&mut self) -> &mut c_uint {
        &mut self.0.max_background
    }

    pub fn congestion_threshold(&self) -> c_uint {
        self.0.congestion_threshold
    }

    pub fn congestion_threshold_mut(&mut self) -> &mut c_uint {
        &mut self.0.congestion_threshold
    }

    pub fn time_gran(&self) -> c_uint {
        self.0.time_gran
    }

    pub fn time_gran_mut(&mut self) -> &mut c_uint {
        &mut self.0.time_gran
    }
}

bitflags! {
    /// Capability flags.
    pub struct CapabilityFlags: Type {
        const ASYNC_DIO = FUSE_CAP_ASYNC_DIO;
        const ASYNC_READ = FUSE_CAP_ASYNC_READ;
        const ATOMIC_O_TRUNC = FUSE_CAP_ATOMIC_O_TRUNC;
        const AUTO_INVAL_DATA = FUSE_CAP_AUTO_INVAL_DATA;
        const DONT_MASK = FUSE_CAP_DONT_MASK;
        const EXPORT_SUPPORT = FUSE_CAP_EXPORT_SUPPORT;
        const FLOCK_LOCKS = FUSE_CAP_FLOCK_LOCKS;
        const HANDLE_KILLPRIV = FUSE_CAP_HANDLE_KILLPRIV;
        const IOCTL_DIR = FUSE_CAP_IOCTL_DIR;
        const NO_OPENDIR_SUPPORT = FUSE_CAP_NO_OPENDIR_SUPPORT;
        const NO_OPEN_SUPPORT = FUSE_CAP_NO_OPEN_SUPPORT;
        const PARALLEL_DIROPS = FUSE_CAP_PARALLEL_DIROPS;
        const POSIX_ACL = FUSE_CAP_POSIX_ACL;
        const POSIX_LOCKS = FUSE_CAP_POSIX_LOCKS;
        const READDIRPLUS = FUSE_CAP_READDIRPLUS;
        const READDIRPLUS_AUTO = FUSE_CAP_READDIRPLUS_AUTO;
        const SPLICE_MOVE = FUSE_CAP_SPLICE_MOVE;
        const SPLICE_READ = FUSE_CAP_SPLICE_READ;
        const SPLICE_WRITE = FUSE_CAP_SPLICE_WRITE;
        const WRITEBACK_CACHE = FUSE_CAP_WRITEBACK_CACHE;
    }
}
