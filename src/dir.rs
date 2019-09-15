use libc::{c_char, off_t, stat};
use libfuse_sys::{fuse_add_direntry, fuse_file_info, fuse_req};
use std::{ffi::CStr, ptr};

pub struct DirBuf<'a> {
    pub(crate) req: &'a mut fuse_req,
    pub(crate) buf: &'a mut [u8],
    pub(crate) pos: usize,
}

impl<'a> DirBuf<'a> {
    /// Add an directory entry to the send buffer.
    ///
    /// If the size of entry to be added is larger than the send buffer,
    /// no entry is added and a `true` will be returned.
    pub fn add(&mut self, name: &CStr, attr: &stat, offset: off_t) -> bool {
        // calculate the length of new entry.
        let new_entry_len = unsafe {
            fuse_add_direntry(self.req, ptr::null_mut(), 0, name.as_ptr(), ptr::null(), 0)
        };
        if self.buf.len() < self.pos + new_entry_len {
            return true;
        }

        unsafe {
            fuse_add_direntry(
                self.req,
                self.buf[self.pos..].as_mut_ptr() as *mut c_char,
                self.buf.len() - self.pos,
                name.as_ptr(),
                attr,
                offset,
            );
        }

        self.pos += new_entry_len;

        false
    }
}

pub struct OpenDirOptions<'a>(pub(crate) &'a mut fuse_file_info);

impl<'a> OpenDirOptions<'a> {
    #[cfg(feature = "cache_readdir")]
    pub fn set_cache_readdir(&mut self, enabled: bool) -> &mut Self {
        self.0.set_cache_readdir(if enabled { 1 } else { 0 });
        self
    }
}
