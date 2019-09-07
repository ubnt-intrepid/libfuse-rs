#![allow(unused_variables)]

use crate::{
    common::{ConnInfo, DirEntry, Ino},
    util::*,
};
use libc::{c_char, c_int, c_uint, c_void, dev_t, mode_t, off_t, stat};
use libfuse_sys::{
    fuse_add_direntry, //
    fuse_conn_info,
    fuse_file_info,
    fuse_ino_t,
    fuse_lowlevel_ops,
    fuse_reply_attr,
    fuse_reply_buf,
    fuse_reply_create,
    fuse_reply_entry,
    fuse_reply_err,
    fuse_reply_none,
    fuse_reply_open,
    fuse_reply_readlink,
    fuse_reply_write,
    fuse_req,
    fuse_req_t,
    fuse_req_userdata,
};
use std::{
    ffi::{CStr, CString},
    mem, ptr,
};

pub type OperationResult<T> = std::result::Result<T, c_int>;

pub trait Operations {
    /// Initialize the filesystem.
    fn init(&mut self, conn: &mut ConnInfo) {}

    /// Look up a directory entry by name and get its attributes.
    fn lookup(&mut self, parent: Ino, name: &CStr) -> OperationResult<DirEntry> {
        Err(libc::ENOSYS)
    }

    /// Forget about an inode.
    fn forget(&mut self, ino: Ino, nlookup: u64) -> OperationResult<()> {
        Err(libc::ENOSYS)
    }

    /// Read a symbolic link.
    fn readlink(&mut self, ino: Ino) -> OperationResult<CString> {
        Err(libc::ENOSYS)
    }

    /// Create a file node.
    fn mknod(
        &mut self,
        parent: Ino,
        name: &CStr,
        mode: mode_t,
        rdev: dev_t,
    ) -> OperationResult<DirEntry> {
        Err(libc::ENOSYS)
    }

    /// Create a directory.
    fn mkdir(&mut self, parent: Ino, name: &CStr, mode: mode_t) -> OperationResult<DirEntry> {
        Err(libc::ENOSYS)
    }

    /// Remove a file.
    fn unlink(&mut self, parent: Ino, name: &CStr) -> OperationResult<()> {
        Err(libc::ENOSYS)
    }

    /// Remove a directory.
    fn rmdir(&mut self, parent: Ino, name: &CStr) -> OperationResult<()> {
        Err(libc::ENOSYS)
    }

    /// Create a symbolic link.
    fn symlink(&mut self, link: &CStr, parent: Ino, name: &CStr) -> OperationResult<DirEntry> {
        Err(libc::ENOSYS)
    }

    /// Rename a file.
    fn rename(
        &mut self,
        parent: Ino,
        name: &CStr,
        newparent: Ino,
        newname: &CStr,
        flags: c_uint,
    ) -> OperationResult<()> {
        Err(libc::ENOSYS)
    }

    /// Create a hard link.
    fn link(&mut self, ino: Ino, newparent: Ino, newname: &CStr) -> OperationResult<DirEntry> {
        Err(libc::ENOSYS)
    }

    /// Open a file.
    fn open(&mut self, ino: Ino, fi: &mut fuse_file_info) -> OperationResult<()> {
        Err(libc::ENOSYS)
    }

    /// Create and open a file.
    fn create(
        &mut self,
        parent: Ino,
        name: &CStr,
        mode: mode_t,
        fi: &mut fuse_file_info,
    ) -> OperationResult<DirEntry> {
        Err(libc::ENOSYS)
    }

    /// Read data from a file.
    fn read(
        &mut self,
        ino: Ino,
        buf: &mut [u8],
        off: off_t,
        fi: *mut fuse_file_info,
    ) -> OperationResult<usize> {
        Err(libc::ENOSYS)
    }

    /// Write data to a file.
    fn write(
        &mut self,
        ino: Ino,
        buf: &[u8],
        off: off_t,
        fi: *mut fuse_file_info,
    ) -> OperationResult<usize> {
        Err(libc::ENOSYS)
    }

    /// Flush a file.
    fn flush(&mut self, ino: Ino, fi: *mut fuse_file_info) -> OperationResult<()> {
        Err(libc::ENOSYS)
    }

    /// Get a file attributes.
    fn getattr(&mut self, ino: Ino, fi: *mut fuse_file_info) -> OperationResult<(stat, f64)> {
        Err(libc::ENOSYS)
    }

    fn release(&mut self, ino: Ino, fi: *mut fuse_file_info) -> OperationResult<()> {
        Err(libc::ENOSYS)
    }

    // TODO: setattr

    /// Open a directory.
    fn opendir(&mut self, ino: Ino, fi: *mut fuse_file_info) -> OperationResult<()> {
        Err(libc::ENOSYS)
    }

    /// Read a directory.
    fn readdir(
        &mut self,
        ino: Ino,
        buf: &mut DirBuf<'_>,
        fi: *mut fuse_file_info,
    ) -> OperationResult<()> {
        Err(libc::ENOSYS)
    }

    /// Release an opened directory.
    fn releasedir(&mut self, ino: Ino, fi: *mut fuse_file_info) -> OperationResult<()> {
        Err(libc::ENOSYS)
    }
}

pub(super) fn make_fuse_lowlevel_ops<T: Operations>() -> fuse_lowlevel_ops {
    let mut ops = unsafe { mem::zeroed::<fuse_lowlevel_ops>() };

    ops.init = Some(ops_init::<T>);
    ops.destroy = Some(ops_destroy::<T>);
    ops.lookup = Some(ops_lookup::<T>);
    ops.forget = Some(ops_forget::<T>);

    ops.readlink = Some(ops_readlink::<T>);
    ops.mknod = Some(ops_mknod::<T>);
    ops.mkdir = Some(ops_mkdir::<T>);
    ops.unlink = Some(ops_unlink::<T>);
    ops.rmdir = Some(ops_rmdir::<T>);
    ops.symlink = Some(ops_symlink::<T>);
    ops.rename = Some(ops_rename::<T>);
    ops.link = Some(ops_link::<T>);

    // TODO: setattr
    ops.open = Some(ops_open::<T>);
    ops.create = Some(ops_create::<T>);
    ops.read = Some(ops_read::<T>);
    ops.write = Some(ops_write::<T>);
    ops.flush = Some(ops_flush::<T>);
    ops.getattr = Some(ops_getattr::<T>);
    ops.release = Some(ops_release::<T>);

    ops.opendir = Some(ops_opendir::<T>);
    ops.readdir = Some(ops_readdir::<T>);
    ops.releasedir = Some(ops_releasedir::<T>);

    ops
}

unsafe fn call_with_ops<T: Operations>(
    req: fuse_req_t,
    f: impl FnOnce(&mut T, &mut fuse_req) -> c_int,
) {
    debug_assert!(!req.is_null());
    let req = &mut *req;

    let ops = fuse_req_userdata(req) as *mut T;
    debug_assert!(!ops.is_null());

    f(&mut *ops, req);
}

unsafe extern "C" fn ops_init<T: Operations>(user_data: *mut c_void, conn: *mut fuse_conn_info) {
    let ops = make_mut_unchecked(user_data as *mut T);
    let conn = make_mut_unchecked(conn as *mut ConnInfo);
    ops.init(conn);
}

unsafe extern "C" fn ops_destroy<T: Operations>(user_data: *mut c_void) {
    mem::drop(Box::from_raw(user_data as *mut T));
}

unsafe extern "C" fn ops_lookup<T: Operations>(
    req: fuse_req_t,
    parent: fuse_ino_t,
    name: *const c_char,
) {
    call_with_ops(req, |ops: &mut T, req| {
        match ops.lookup(parent, CStr::from_ptr(name)) {
            Ok(DirEntry(entry)) => fuse_reply_entry(req, &entry),
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn ops_forget<T: Operations>(req: fuse_req_t, ino: fuse_ino_t, nlookup: u64) {
    call_with_ops(req, |ops: &mut T, req| match ops.forget(ino, nlookup) {
        Ok(()) => {
            fuse_reply_none(req);
            0
        }
        Err(errno) => fuse_reply_err(req, errno),
    })
}

unsafe extern "C" fn ops_readlink<T: Operations>(req: fuse_req_t, ino: fuse_ino_t) {
    call_with_ops(req, |ops: &mut T, req| match ops.readlink(ino) {
        Ok(content) => fuse_reply_readlink(req, content.as_ptr()),
        Err(errno) => fuse_reply_err(req, errno),
    })
}

unsafe extern "C" fn ops_mknod<T: Operations>(
    req: fuse_req_t,
    parent: fuse_ino_t,
    name: *const c_char,
    mode: mode_t,
    rdev: dev_t,
) {
    call_with_ops(req, |ops: &mut T, req| {
        match ops.mknod(parent, CStr::from_ptr(name), mode, rdev) {
            Ok(DirEntry(entry)) => fuse_reply_entry(req, &entry),
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn ops_mkdir<T: Operations>(
    req: fuse_req_t,
    parent: fuse_ino_t,
    name: *const c_char,
    mode: mode_t,
) {
    call_with_ops(req, |ops: &mut T, req| {
        match ops.mkdir(parent, CStr::from_ptr(name), mode) {
            Ok(DirEntry(entry)) => fuse_reply_entry(req, &entry),
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn ops_unlink<T: Operations>(
    req: fuse_req_t,
    parent: fuse_ino_t,
    name: *const c_char,
) {
    call_with_ops(req, |ops: &mut T, req| {
        match ops.unlink(parent, CStr::from_ptr(name)) {
            Ok(()) => {
                0 /* do nothing */
            }
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn ops_rmdir<T: Operations>(
    req: fuse_req_t,
    parent: fuse_ino_t,
    name: *const c_char,
) {
    call_with_ops(req, |ops: &mut T, req| {
        match ops.rmdir(parent, CStr::from_ptr(name)) {
            Ok(()) => {
                0 /* do nothing */
            }
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn ops_symlink<T: Operations>(
    req: fuse_req_t,
    link: *const c_char,
    parent: fuse_ino_t,
    name: *const c_char,
) {
    call_with_ops(req, |ops: &mut T, req| {
        match ops.symlink(CStr::from_ptr(link), parent, CStr::from_ptr(name)) {
            Ok(DirEntry(entry)) => fuse_reply_entry(req, &entry),
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn ops_rename<T: Operations>(
    req: fuse_req_t,
    parent: fuse_ino_t,
    name: *const c_char,
    newparent: fuse_ino_t,
    newname: *const c_char,
    flags: c_uint,
) {
    call_with_ops(req, |ops: &mut T, req| {
        match ops.rename(
            parent,
            CStr::from_ptr(name),
            newparent,
            CStr::from_ptr(newname),
            flags,
        ) {
            Ok(()) => {
                0 /* do nothing */
            }
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn ops_link<T: Operations>(
    req: fuse_req_t,
    ino: fuse_ino_t,
    newparent: fuse_ino_t,
    newname: *const c_char,
) {
    call_with_ops(req, |ops: &mut T, req| {
        match ops.link(ino, newparent, CStr::from_ptr(newname)) {
            Ok(DirEntry(entry)) => fuse_reply_entry(req, &entry),
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn ops_open<T: Operations>(
    req: fuse_req_t,
    ino: fuse_ino_t,
    fi: *mut fuse_file_info,
) {
    call_with_ops(req, |ops: &mut T, req| {
        let fi = make_mut_unchecked(fi);
        match ops.open(ino, fi) {
            Ok(()) => fuse_reply_open(req, fi),
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn ops_create<T: Operations>(
    req: fuse_req_t,
    parent: fuse_ino_t,
    name: *const c_char,
    mode: mode_t,
    fi: *mut fuse_file_info,
) {
    call_with_ops(req, |ops: &mut T, req| {
        let fi = make_mut_unchecked(fi);
        match ops.create(parent, CStr::from_ptr(name), mode, fi) {
            Ok(DirEntry(entry)) => fuse_reply_create(req, &entry, fi),
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn ops_read<T: Operations>(
    req: fuse_req_t,
    ino: fuse_ino_t,
    size: usize,
    off: off_t,
    fi: *mut fuse_file_info,
) {
    call_with_ops(req, |ops: &mut T, req| {
        let mut buf = Vec::with_capacity(size);
        buf.resize(size, 0u8);
        match ops.read(ino, &mut buf[..], off, fi) {
            Ok(size) => {
                let out = &buf[..size];
                fuse_reply_buf(req, out.as_ptr() as *const c_char, out.len())
            }
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn ops_write<T: Operations>(
    req: fuse_req_t,
    ino: fuse_ino_t,
    buf: *const c_char,
    size: usize,
    off: off_t,
    fi: *mut fuse_file_info,
) {
    call_with_ops(req, |ops: &mut T, req| {
        let buf = std::slice::from_raw_parts(buf as *const u8, size);
        match ops.write(ino, &buf[..], off, fi) {
            Ok(count) => fuse_reply_write(req, count),
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn ops_flush<T: Operations>(
    req: fuse_req_t,
    ino: fuse_ino_t,
    fi: *mut fuse_file_info,
) {
    call_with_ops(req, |ops: &mut T, req| {
        match ops.flush(ino, fi) {
            Ok(()) => {
                0 /* do nothing */
            }
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn ops_getattr<T: Operations>(
    req: fuse_req_t,
    ino: fuse_ino_t,
    fi: *mut fuse_file_info,
) {
    call_with_ops(req, |ops: &mut T, req| match ops.getattr(ino, fi) {
        Ok((stat, timeout)) => fuse_reply_attr(req, &stat, timeout),
        Err(errno) => fuse_reply_err(req, errno),
    })
}

unsafe extern "C" fn ops_release<T: Operations>(
    req: fuse_req_t,
    ino: fuse_ino_t,
    fi: *mut fuse_file_info,
) {
    call_with_ops(req, |ops: &mut T, req| {
        match ops.release(ino, fi) {
            Ok(()) => {
                0 /* do nothing */
            }
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn ops_opendir<T: Operations>(
    req: fuse_req_t,
    ino: fuse_ino_t,
    fi: *mut fuse_file_info,
) {
    call_with_ops(req, |ops: &mut T, req| {
        debug_assert!(!fi.is_null());
        let fi = &mut *fi;
        match ops.opendir(ino, fi) {
            Ok(()) => fuse_reply_open(req, fi),
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn ops_readdir<T: Operations>(
    req: fuse_req_t,
    ino: fuse_ino_t,
    size: usize,
    offset: off_t,
    fi: *mut fuse_file_info,
) {
    call_with_ops(req, |ops: &mut T, req| {
        let mut buf = DirBuf {
            req: &mut *req,
            p: ptr::null_mut(),
            size: 0,
            offset,
        };
        match ops.readdir(ino, &mut buf, fi) {
            Ok(()) => {
                if (offset as usize) <= buf.size {
                    fuse_reply_buf(
                        buf.req,
                        buf.p.offset(offset as isize),
                        std::cmp::min(buf.size - offset as usize, size),
                    )
                } else {
                    fuse_reply_buf(buf.req, ptr::null(), 0)
                }
            }
            Err(errno) => fuse_reply_err(buf.req, errno),
        }
    })
}

unsafe extern "C" fn ops_releasedir<T: Operations>(
    req: fuse_req_t,
    ino: fuse_ino_t,
    fi: *mut fuse_file_info,
) {
    call_with_ops(req, |ops: &mut T, req| {
        match ops.releasedir(ino, fi) {
            Ok(()) => {
                0 /* do nothing */
            }
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

pub struct DirBuf<'a> {
    req: &'a mut fuse_req,
    p: *mut c_char,
    size: usize,
    #[allow(dead_code)]
    offset: off_t,
}

impl<'a> Drop for DirBuf<'a> {
    fn drop(&mut self) {
        unsafe {
            if !self.p.is_null() {
                libc::free(self.p as *mut _);
            }
            self.p = ptr::null_mut();
        }
    }
}

impl<'a> DirBuf<'a> {
    pub fn add(&mut self, name: &str, ino: fuse_ino_t) {
        let name = CString::new(name).unwrap();
        let oldsize = self.size;
        self.size += unsafe {
            fuse_add_direntry(self.req, ptr::null_mut(), 0, name.as_ptr(), ptr::null(), 0)
        };
        self.p = unsafe { libc::realloc(self.p as *mut _, self.size) as *mut c_char };

        let mut stbuf = unsafe { mem::zeroed::<stat>() };
        stbuf.st_ino = ino;

        unsafe {
            fuse_add_direntry(
                self.req,
                self.p.offset(oldsize as isize),
                self.size - oldsize,
                name.as_ptr(),
                &stbuf,
                self.size as i64,
            );
        }
    }
}
