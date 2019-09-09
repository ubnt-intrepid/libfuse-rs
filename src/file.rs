use crate::common::Ino;
use libc::stat;
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

#[derive(Default)]
pub struct OpenOptions {
    direct_io: bool,
    keep_cache: bool,
    nonseekable: bool,
}

impl OpenOptions {
    pub fn direct_io(&mut self, enabled: bool) -> &mut Self {
        self.direct_io = enabled;
        self
    }

    pub fn keep_cache(&mut self, enabled: bool) -> &mut Self {
        self.keep_cache = enabled;
        self
    }

    pub fn nonseekable(&mut self, enabled: bool) -> &mut Self {
        self.nonseekable = enabled;
        self
    }

    pub(crate) fn assign_to(&self, fi: &mut fuse_file_info) {
        fi.set_direct_io(if self.direct_io { 1 } else { 0 });
        fi.set_keep_cache(if self.keep_cache { 1 } else { 0 });
        fi.set_nonseekable(if self.nonseekable { 1 } else { 0 });
    }
}
