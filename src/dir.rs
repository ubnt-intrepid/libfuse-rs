use bitflags::bitflags;
use libc::{c_char, c_int, c_void, off_t, stat};
use libfuse_sys::{fuse_fill_dir_flags, fuse_readdir_flags};
use std::{ffi::CStr, ptr};

type FillDirFn = unsafe extern "C" fn(
    *mut c_void,
    *const c_char,
    *const stat,
    off_t,
    fuse_fill_dir_flags,
) -> c_int;

pub struct FillDir {
    pub(crate) buf: *mut c_void,
    pub(crate) filler: FillDirFn,
}

impl FillDir {
    pub fn add(
        &mut self,
        name: &CStr,
        stbuf: Option<&stat>,
        off: off_t,
        flags: FillDirFlags,
    ) -> bool {
        unsafe {
            (self.filler)(
                self.buf,
                name.as_ptr(),
                stbuf.map_or(ptr::null(), |s| s as *const _),
                off,
                flags.bits(),
            ) == 1
        }
    }
}

bitflags! {
    pub struct FillDirFlags: fuse_fill_dir_flags {
        const PLUS = libfuse_sys::fuse_fill_dir_flags_FUSE_FILL_DIR_PLUS;
    }
}

impl Default for FillDirFlags {
    fn default() -> Self {
        Self::empty()
    }
}

bitflags! {
    pub struct ReadDirFlags: fuse_readdir_flags {
        const PLUS = libfuse_sys::fuse_readdir_flags_FUSE_READDIR_PLUS;
    }
}
