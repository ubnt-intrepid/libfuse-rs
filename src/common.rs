use libc::{c_double, c_int, stat};
use libfuse_sys::{fuse_config, fuse_conn_info, fuse_entry_param, fuse_file_info, fuse_ino_t};

/// The type of inode number used in the filesystem.
pub type Ino = fuse_ino_t;

#[repr(C)]
pub struct ConnInfo(fuse_conn_info);

#[repr(C)]
pub struct Config(fuse_config);

impl Config {
    pub fn use_ino(&mut self, ino: c_int) -> &mut Self {
        self.0.use_ino = ino;
        self
    }

    pub fn kernel_cache(&mut self, enabled: bool) -> &mut Self {
        self.0.kernel_cache = if enabled { 1 } else { 0 };
        self
    }

    pub fn entry_timeout(&mut self, timeout: c_double) -> &mut Self {
        self.0.entry_timeout = timeout;
        self
    }

    pub fn attr_timeout(&mut self, timeout: c_double) -> &mut Self {
        self.0.attr_timeout = timeout;
        self
    }

    pub fn negative_timeout(&mut self, timeout: c_double) -> &mut Self {
        self.0.negative_timeout = timeout;
        self
    }
}

#[repr(C)]
pub struct FileInfo(fuse_file_info);

impl FileInfo {
    pub fn flags(&self) -> c_int {
        self.0.flags
    }

    pub fn lock_owner(&self) -> u64 {
        self.0.lock_owner
    }

    pub fn poll_events(&self) -> u32 {
        self.0.poll_events
    }

    pub fn fh(&self) -> u64 {
        self.0.fh
    }

    pub fn fh_mut(&mut self) -> &mut u64 {
        &mut self.0.fh
    }
}

#[repr(C)]
pub struct DirEntry(pub(crate) fuse_entry_param);

impl DirEntry {
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
