use crate::common::NodeId;
use bitflags::bitflags;
use libc::{c_int, gid_t, mode_t, stat, timespec, uid_t};
use libfuse_sys::{
    fuse_entry_param, //
    fuse_file_info,
    fuse_setattr_flags::*,
    helpers::{
        fuse_entry_param_attr, //
        fuse_entry_param_attr_timeout,
        fuse_entry_param_entry_timeout,
        fuse_entry_param_generation,
        fuse_entry_param_ino,
        fuse_entry_param_new,
        fuse_file_info_flags,
        fuse_file_info_flock_release,
        fuse_file_info_flush,
        fuse_file_info_lock_owner,
        fuse_file_info_set_direct_io,
        fuse_file_info_set_keep_cache,
        fuse_file_info_set_nonseekable,
        fuse_file_info_writepage,
    },
};
use std::{borrow::Cow, ptr::NonNull};

pub struct Entry(pub(crate) NonNull<fuse_entry_param>);

impl Default for Entry {
    fn default() -> Self {
        Self(NonNull::new(unsafe { fuse_entry_param_new() }).unwrap())
    }
}

impl Drop for Entry {
    fn drop(&mut self) {
        unsafe {
            libc::free(self.0.as_ptr() as *mut _);
        }
    }
}

impl Entry {
    /// Create a new `Entry`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the inode number for this entry.
    pub fn nodeid(&mut self, id: NodeId) -> &mut Self {
        unsafe {
            fuse_entry_param_ino(self.0.as_mut(), id);
        }
        self
    }

    /// Sets the generation number for this entry.
    pub fn generation(&mut self, gen: u64) -> &mut Self {
        unsafe {
            fuse_entry_param_generation(self.0.as_mut(), gen);
        }
        self
    }

    /// Sets the attributes associated with this entry.
    pub fn attr(&mut self, attr: &stat) -> &mut Self {
        unsafe {
            fuse_entry_param_attr(self.0.as_mut(), attr);
        }
        self
    }

    ///
    pub fn attr_timeout(&mut self, timeout: f64) -> &mut Self {
        unsafe {
            fuse_entry_param_attr_timeout(self.0.as_mut(), timeout);
        }
        self
    }

    pub fn entry_timeout(&mut self, timeout: f64) -> &mut Self {
        unsafe {
            fuse_entry_param_entry_timeout(self.0.as_mut(), timeout);
        }
        self
    }
}

pub struct OpenOptions<'a>(pub(crate) &'a mut fuse_file_info);

impl<'a> OpenOptions<'a> {
    pub fn flags(&self) -> c_int {
        unsafe { fuse_file_info_flags(self.0) }
    }

    pub fn set_direct_io(&mut self, enabled: bool) -> &mut Self {
        unsafe {
            fuse_file_info_set_direct_io(self.0, if enabled { 1 } else { 0 });
        }
        self
    }

    pub fn set_keep_cache(&mut self, enabled: bool) -> &mut Self {
        unsafe {
            fuse_file_info_set_keep_cache(self.0, if enabled { 1 } else { 0 });
        }
        self
    }

    pub fn set_nonseekable(&mut self, enabled: bool) -> &mut Self {
        unsafe {
            fuse_file_info_set_nonseekable(self.0, if enabled { 1 } else { 0 });
        }
        self
    }
}

pub struct ReadOptions<'a>(pub(crate) &'a mut fuse_file_info);

impl<'a> ReadOptions<'a> {
    pub fn flags(&self) -> c_int {
        unsafe { fuse_file_info_flags(self.0) }
    }

    pub fn lock_owner(&self) -> u64 {
        unsafe { fuse_file_info_lock_owner(self.0) }
    }
}

pub struct WriteOptions<'a>(pub(crate) &'a mut fuse_file_info);

impl<'a> WriteOptions<'a> {
    pub fn flags(&self) -> c_int {
        unsafe { fuse_file_info_flags(self.0) }
    }

    pub fn lock_owner(&self) -> u64 {
        unsafe { fuse_file_info_lock_owner(self.0) }
    }

    pub fn writepage(&self) -> bool {
        unsafe { fuse_file_info_writepage(self.0) != 0 }
    }
}

pub struct FlushOptions<'a>(pub(crate) &'a mut fuse_file_info);

impl<'a> FlushOptions<'a> {
    pub fn lock_owner(&self) -> u64 {
        unsafe { fuse_file_info_lock_owner(self.0) }
    }
}

pub struct ReleaseOptions<'a>(pub(crate) &'a mut fuse_file_info);

impl<'a> ReleaseOptions<'a> {
    pub fn flush(&self) -> bool {
        unsafe { fuse_file_info_flush(self.0) != 0 }
    }

    pub fn lock_owner(&self) -> u64 {
        unsafe { fuse_file_info_lock_owner(self.0) }
    }

    pub fn flock_release(&self) -> bool {
        unsafe { fuse_file_info_flock_release(self.0) != 0 }
    }
}

/// A set of attributes to be set.
pub struct SetAttrs<'a> {
    pub(crate) attr: &'a stat,
    pub(crate) to_set: c_int,
}

impl<'a> SetAttrs<'a> {
    /// Returns the file mode if specified.
    pub fn mode(&self) -> Option<mode_t> {
        if (self.to_set & FUSE_SET_ATTR_MODE) != 0 {
            Some(self.attr.st_mode)
        } else {
            None
        }
    }

    /// Returns the UID if specified.
    pub fn uid(&self) -> Option<uid_t> {
        if (self.to_set & FUSE_SET_ATTR_UID) != 0 {
            Some(self.attr.st_uid)
        } else {
            None
        }
    }

    /// Returns the GID if specified.
    pub fn gid(&self) -> Option<gid_t> {
        if (self.to_set & FUSE_SET_ATTR_GID) != 0 {
            Some(self.attr.st_gid)
        } else {
            None
        }
    }

    /// Returns the file size if specified.
    pub fn size(&self) -> Option<i64> {
        if (self.to_set & FUSE_SET_ATTR_SIZE) != 0 {
            Some(self.attr.st_size)
        } else {
            None
        }
    }

    /// Returns the access time if specified.
    pub fn atime(&self) -> Option<timespec> {
        let mut ts = timespec {
            tv_sec: self.attr.st_atime,
            tv_nsec: 0,
        };
        match self.to_set & (FUSE_SET_ATTR_ATIME | FUSE_SET_ATTR_ATIME_NOW) {
            FUSE_SET_ATTR_ATIME_NOW => {
                ts.tv_nsec = libc::UTIME_NOW;
            }
            FUSE_SET_ATTR_ATIME => {
                ts.tv_nsec = self.attr.st_atime_nsec;
            }
            _ => return None,
        }
        Some(ts)
    }

    /// Returns the modification time if specified.
    pub fn mtime(&self) -> Option<timespec> {
        let mut ts = timespec {
            tv_sec: self.attr.st_mtime,
            tv_nsec: 0,
        };
        match self.to_set & (FUSE_SET_ATTR_MTIME | FUSE_SET_ATTR_MTIME_NOW) {
            FUSE_SET_ATTR_MTIME_NOW => {
                ts.tv_nsec = libc::UTIME_NOW;
            }
            FUSE_SET_ATTR_MTIME => {
                ts.tv_nsec = self.attr.st_mtime_nsec;
            }
            _ => return None,
        }
        Some(ts)
    }

    /// Returns the creation time if specified.
    pub fn ctime(&self) -> Option<timespec> {
        if (self.to_set & FUSE_SET_ATTR_CTIME) != 0 {
            Some(timespec {
                tv_sec: self.attr.st_ctime,
                tv_nsec: self.attr.st_ctime_nsec,
            })
        } else {
            None
        }
    }
}

bitflags! {
    /// Additional option flags provided to `rename`.
    pub struct RenameFlags: c_int {
        /// Don't overwrite the new inode. The `rename`
        /// must return an error if the destination inode
        /// has already exists.
        const NOREPLACE = libc::RENAME_NOREPLACE;

        /// Atomically exchange the old and new inode.
        /// Both inodes must exist but may be of different
        /// types.
        const EXCHANGE = libc::RENAME_EXCHANGE;
    }
}

bitflags! {
    /// Additional option flags provided to `setxattr`.
    pub struct XAttrFlags: c_int {
        /// Perform a pure create, which fails if the named
        /// attribute exists already.
        const CREATE = libc::XATTR_CREATE;

        /// Perform a pure replace operation, which fails
        /// if the named attributes does not exist.
        const REPLACE = libc::XATTR_REPLACE;
    }
}

#[derive(Debug)]
pub enum XAttrReply<'a> {
    Data(Cow<'a, [u8]>),
    Size(usize),
}
