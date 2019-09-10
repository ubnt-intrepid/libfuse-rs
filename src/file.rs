use crate::common::Ino;
use libc::{c_int, stat};
use libfuse_sys::{fuse_entry_param, fuse_file_info};

pub struct Entry(pub(crate) fuse_entry_param);

impl Default for Entry {
    fn default() -> Self {
        Self(unsafe { std::mem::zeroed() })
    }
}

impl Entry {
    /// Create a new `Entry`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the inode number for this entry.
    pub fn ino(&mut self, ino: Ino) -> &mut Self {
        self.0.ino = ino;
        self
    }

    /// Sets the generation number for this entry.
    pub fn generation(&mut self, gen: u64) -> &mut Self {
        self.0.generation = gen;
        self
    }

    /// Sets the attributes associated with this entry.
    pub fn attr(&mut self, attr: stat) -> &mut Self {
        self.0.attr = attr;
        self
    }

    ///
    pub fn attr_timeout(&mut self, timeout: f64) -> &mut Self {
        self.0.attr_timeout = timeout;
        self
    }

    pub fn entry_timeout(&mut self, timeout: f64) -> &mut Self {
        self.0.entry_timeout = timeout;
        self
    }
}

pub struct OpenOptions<'a>(pub(crate) &'a mut fuse_file_info);

impl<'a> OpenOptions<'a> {
    pub fn flags(&self) -> c_int {
        self.0.flags
    }

    pub fn set_direct_io(&mut self, enabled: bool) -> &mut Self {
        self.0.set_direct_io(if enabled { 1 } else { 0 });
        self
    }

    pub fn set_keep_cache(&mut self, enabled: bool) -> &mut Self {
        self.0.set_keep_cache(if enabled { 1 } else { 0 });
        self
    }

    pub fn set_nonseekable(&mut self, enabled: bool) -> &mut Self {
        self.0.set_nonseekable(if enabled { 1 } else { 0 });
        self
    }
}

pub struct ReadOptions<'a>(pub(crate) &'a mut fuse_file_info);

impl<'a> ReadOptions<'a> {
    pub fn flags(&self) -> c_int {
        self.0.flags
    }

    pub fn lock_owner(&self) -> u64 {
        self.0.lock_owner
    }
}

pub struct WriteOptions<'a>(pub(crate) &'a mut fuse_file_info);

impl<'a> WriteOptions<'a> {
    pub fn writepage(&self) -> bool {
        self.0.writepage() != 0
    }
    pub fn flags(&self) -> c_int {
        self.0.flags
    }

    pub fn lock_owner(&self) -> u64 {
        self.0.lock_owner
    }
}

pub struct FlushOptions<'a>(pub(crate) &'a mut fuse_file_info);

impl<'a> FlushOptions<'a> {
    pub fn lock_owner(&self) -> u64 {
        self.0.lock_owner
    }
}

pub struct ReleaseOptions<'a>(pub(crate) &'a mut fuse_file_info);

impl<'a> ReleaseOptions<'a> {
    pub fn flush(&self) -> bool {
        self.0.flush() != 0
    }

    pub fn lock_owner(&self) -> u64 {
        self.0.lock_owner
    }

    pub fn flock_release(&self) -> bool {
        self.0.flock_release() != 0
    }
}
