use crate::{
    common::Ino,
    ops::{OperationResult, Operations},
};
use libc::{off_t, stat};
use libfuse_sys::{fuse_entry_param, fuse_file_info};

/// A set of operations associated with an opened file.
pub trait FileOperations: Sized {
    type Ops: ?Sized + Operations<File = Self>;

    /// Read data from an opened file.
    #[allow(unused_variables)]
    fn read(
        &mut self,
        ops: &mut Self::Ops,
        ino: Ino,
        buf: &mut [u8],
        off: off_t,
        fi: &mut fuse_file_info,
    ) -> OperationResult<usize> {
        Err(libc::ENOSYS)
    }

    /// Write data to a file.
    #[allow(unused_variables)]
    fn write(
        &mut self,
        ops: &mut Self::Ops,
        ino: Ino,
        buf: &[u8],
        off: off_t,
        fi: &mut fuse_file_info,
    ) -> OperationResult<usize> {
        Err(libc::ENOSYS)
    }

    /// Flush an opened file.
    #[allow(unused_variables)]
    fn flush(
        &mut self,
        ops: &mut Self::Ops,
        ino: Ino,
        fi: &mut fuse_file_info,
    ) -> OperationResult<()> {
        Err(libc::ENOSYS)
    }

    /// Get attributes from an opened file.
    #[allow(unused_variables)]
    fn getattr(&mut self, ops: &mut Self::Ops, ino: Ino) -> OperationResult<(stat, f64)> {
        ops.getattr(ino)
    }

    // TODO: setattr

    #[allow(unused_variables)]
    fn release(
        &mut self,
        ops: &mut Self::Ops,
        ino: Ino,
        fi: &mut fuse_file_info,
    ) -> OperationResult<()> {
        Err(libc::ENOSYS)
    }
}

pub struct Entry(pub(crate) fuse_entry_param);

impl Entry {
    /// Create a new `DirEntry` with the specified inode number.
    pub fn new(ino: Ino) -> Self {
        let mut entry: fuse_entry_param = unsafe { std::mem::zeroed() };
        entry.ino = ino;
        Self(entry)
    }

    /// Sets the generation number for this entry.
    pub fn generation(mut self, gen: u64) -> Self {
        self.0.generation = gen;
        self
    }

    /// Sets the attributes associated with this entry.
    pub fn attr(mut self, attr: stat) -> Self {
        self.0.attr = attr;
        self
    }

    ///
    pub fn attr_timeout(mut self, timeout: f64) -> Self {
        self.0.attr_timeout = timeout;
        self
    }

    pub fn entry_timeout(mut self, timeout: f64) -> Self {
        self.0.entry_timeout = timeout;
        self
    }
}
