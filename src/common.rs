use bitflags::bitflags;
use libc::c_uint;
use libfuse_sys::{
    fuse_cap_flags::*,
    fuse_conn_info, fuse_ino_t,
    helpers::{
        fuse_conn_info_capable, //
        fuse_conn_info_congestion_threshold,
        fuse_conn_info_max_background,
        fuse_conn_info_max_read,
        fuse_conn_info_proto_major,
        fuse_conn_info_proto_minor,
        fuse_conn_info_set_congestion_threshold,
        fuse_conn_info_set_max_background,
        fuse_conn_info_set_max_read,
        fuse_conn_info_set_time_gran,
        fuse_conn_info_set_want,
        fuse_conn_info_time_gran,
        fuse_conn_info_want,
    },
};

/// The type of inode number used in the filesystem.
pub type NodeId = fuse_ino_t;

pub const ROOT_NODEID: NodeId = 1;

/// Connection information passed to `Operations::init` method.
pub struct ConnectionInfo<'a>(pub(crate) &'a mut fuse_conn_info);

impl<'a> ConnectionInfo<'a> {
    /// Returns major version of the protocol.
    pub fn proto_major(&self) -> c_uint {
        unsafe { fuse_conn_info_proto_major(self.0) }
    }

    /// Returns minor version of the protocol.
    pub fn proto_minor(&self) -> c_uint {
        unsafe { fuse_conn_info_proto_minor(self.0) }
    }

    /// Returns the maximum size of read requests.
    pub fn max_read(&self) -> c_uint {
        unsafe { fuse_conn_info_max_read(self.0) }
    }

    /// Returns a mutable reference to the maximum size of read requests.
    pub fn set_max_read(&mut self, max_read: c_uint) {
        unsafe {
            fuse_conn_info_set_max_read(self.0, max_read);
        }
    }

    /// Returns capability flags that the kernel supports.
    pub fn capable(&self) -> CapabilityFlags {
        CapabilityFlags::from_bits_truncate(unsafe { fuse_conn_info_capable(self.0) })
    }

    /// Returns capability flags that the filesystem wants to enable.
    pub fn want(&self) -> CapabilityFlags {
        CapabilityFlags::from_bits_truncate(unsafe { fuse_conn_info_want(self.0) })
    }

    /// Sets capability flags that the filesystem wants to enable.
    pub fn set_want(&mut self, flags: CapabilityFlags) {
        unsafe {
            fuse_conn_info_set_want(self.0, flags.bits());
        }
    }

    pub fn max_background(&self) -> c_uint {
        unsafe { fuse_conn_info_max_background(self.0) }
    }

    pub fn set_max_background(&mut self, max_background: c_uint) {
        unsafe {
            fuse_conn_info_set_max_background(self.0, max_background);
        }
    }

    pub fn congestion_threshold(&self) -> c_uint {
        unsafe { fuse_conn_info_congestion_threshold(self.0) }
    }

    pub fn set_congestion_threshold(&mut self, threshold: c_uint) {
        unsafe {
            fuse_conn_info_set_congestion_threshold(self.0, threshold);
        }
    }

    pub fn time_gran(&self) -> c_uint {
        unsafe { fuse_conn_info_time_gran(self.0) }
    }

    pub fn set_time_gran(&mut self, time_gran: c_uint) {
        unsafe {
            fuse_conn_info_set_time_gran(self.0, time_gran);
        }
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
