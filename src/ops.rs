use crate::{
    common::{ConnectionInfo, Ino},
    dir::{DirBuf, OpenOptions as DirOpenOptions},
    file::{Entry, OpenOptions},
    util::*,
};
use libc::{c_char, c_int, c_uint, c_void, dev_t, mode_t, off_t, stat, statvfs};
use libfuse_sys::{
    fuse_conn_info, //
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
    fuse_reply_statfs,
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
    /// The type of context value during opening a file.
    type File;

    /// The type of context value during opening a directory.
    type Dir;

    /// Initialize the filesystem.
    #[allow(unused_variables)]
    fn init(&mut self, conn: &mut ConnectionInfo) {}

    /// Look up a directory entry by name and get its attributes.
    #[allow(unused_variables)]
    fn lookup(&mut self, parent: Ino, name: &CStr) -> OperationResult<Entry> {
        Err(libc::ENOSYS)
    }

    /// Forget about an inode.
    #[allow(unused_variables)]
    fn forget(&mut self, ino: Ino, nlookup: u64) -> OperationResult<()> {
        Err(libc::ENOSYS)
    }

    /// Read a symbolic link.
    #[allow(unused_variables)]
    fn readlink(&mut self, ino: Ino) -> OperationResult<CString> {
        Err(libc::ENOSYS)
    }

    /// Create a file node.
    #[allow(unused_variables)]
    fn mknod(
        &mut self,
        parent: Ino,
        name: &CStr,
        mode: mode_t,
        rdev: dev_t,
    ) -> OperationResult<Entry> {
        Err(libc::ENOSYS)
    }

    /// Create a directory.
    #[allow(unused_variables)]
    fn mkdir(&mut self, parent: Ino, name: &CStr, mode: mode_t) -> OperationResult<Entry> {
        Err(libc::ENOSYS)
    }

    /// Remove a file.
    #[allow(unused_variables)]
    fn unlink(&mut self, parent: Ino, name: &CStr) -> OperationResult<()> {
        Err(libc::ENOSYS)
    }

    /// Remove a directory.
    #[allow(unused_variables)]
    fn rmdir(&mut self, parent: Ino, name: &CStr) -> OperationResult<()> {
        Err(libc::ENOSYS)
    }

    /// Create a symbolic link.
    #[allow(unused_variables)]
    fn symlink(&mut self, link: &CStr, parent: Ino, name: &CStr) -> OperationResult<Entry> {
        Err(libc::ENOSYS)
    }

    /// Rename a file.
    #[allow(unused_variables)]
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
    #[allow(unused_variables)]
    fn link(&mut self, ino: Ino, newparent: Ino, newname: &CStr) -> OperationResult<Entry> {
        Err(libc::ENOSYS)
    }

    /// Get file system statistics.
    #[allow(unused_variables)]
    fn statfs(&mut self, ino: Ino) -> OperationResult<statvfs> {
        Err(libc::ENOSYS)
    }

    #[allow(unused_variables)]
    fn setxattr(
        &mut self,
        ino: Ino,
        name: &CStr,
        value: &[u8],
        flags: c_int,
    ) -> OperationResult<()> {
        Err(libc::ENOSYS)
    }

    #[allow(unused_variables)]
    fn getxattr(&mut self, ino: Ino, name: &CStr, buf: &mut [u8]) -> OperationResult<usize> {
        Err(libc::ENOSYS)
    }

    #[allow(unused_variables)]
    fn listxattr(&mut self, ino: Ino, buf: &mut [u8]) -> OperationResult<usize> {
        Err(libc::ENOSYS)
    }

    #[allow(unused_variables)]
    fn removexattr(&mut self, ino: Ino, name: &CStr) -> OperationResult<()> {
        Err(libc::ENOSYS)
    }

    #[allow(unused_variables)]
    fn access(&mut self, ino: Ino, mask: c_int) -> OperationResult<()> {
        Err(libc::ENOSYS)
    }

    /// Open a file.
    #[allow(unused_variables)]
    fn open(
        &mut self,
        ino: Ino,
        flags: c_int,
        options: &mut OpenOptions,
    ) -> OperationResult<Option<Self::File>> {
        Ok(None)
    }

    /// Create and open a file.
    #[allow(unused_variables)]
    fn create(
        &mut self,
        parent: Ino,
        name: &CStr,
        mode: mode_t,
        flags: c_int,
        options: &mut OpenOptions,
    ) -> OperationResult<(Entry, Option<Self::File>)> {
        Err(libc::ENOSYS)
    }

    /// Read data from an opened file.
    #[allow(unused_variables)]
    fn read(
        &mut self,
        ino: Ino,
        buf: &mut [u8],
        off: off_t,
        fi: &mut fuse_file_info,
        ctx: Option<&mut Self::File>,
    ) -> OperationResult<usize> {
        Err(libc::ENOSYS)
    }

    /// Write data to a file.
    #[allow(unused_variables)]
    fn write(
        &mut self,
        ino: Ino,
        buf: &[u8],
        off: off_t,
        fi: &mut fuse_file_info,
        ctx: Option<&mut Self::File>,
    ) -> OperationResult<usize> {
        Err(libc::ENOSYS)
    }

    /// Flush an opened file.
    #[allow(unused_variables)]
    fn flush(
        &mut self,
        ino: Ino,
        fi: &mut fuse_file_info,
        ctx: Option<&mut Self::File>,
    ) -> OperationResult<()> {
        Err(libc::ENOSYS)
    }

    /// Get attributes from an opened file.
    #[allow(unused_variables)]
    fn getattr(&mut self, ino: Ino, ctx: Option<&mut Self::File>) -> OperationResult<(stat, f64)> {
        Err(libc::ENOSYS)
    }

    #[allow(unused_variables)]
    fn setattr(
        &mut self,
        ino: Ino,
        attr: &stat,
        to_set: c_int,
        ctx: Option<&mut Self::File>,
    ) -> OperationResult<(stat, f64)> {
        Err(libc::ENOSYS)
    }

    /// Synchronisze the file contents.
    #[allow(unused_variables)]
    fn fsync(
        &mut self,
        ino: Ino,
        datasync: c_int,
        ctx: Option<&mut Self::File>,
    ) -> OperationResult<()> {
        Err(libc::ENOSYS)
    }

    /// Release an opened file.
    #[allow(unused_variables)]
    fn release(
        &mut self,
        ino: Ino,
        fi: &mut fuse_file_info,
        ctx: Option<Self::File>,
    ) -> OperationResult<()> {
        Err(libc::ENOSYS)
    }

    /// Open a directory.
    #[allow(unused_variables)]
    fn opendir(
        &mut self,
        ino: Ino,
        options: &mut DirOpenOptions,
    ) -> OperationResult<Option<Self::Dir>> {
        Ok(None)
    }

    /// Read a directory.
    #[allow(unused_variables)]
    fn readdir(
        &mut self,
        ino: Ino,
        offset: off_t,
        buf: &mut DirBuf<'_>,
        ctx: Option<&mut Self::Dir>,
    ) -> OperationResult<()> {
        Err(libc::ENOSYS)
    }

    /// Synchronisze the file contents.
    #[allow(unused_variables)]
    fn fsyncdir(
        &mut self,
        ino: Ino,
        datasync: c_int,
        ctx: Option<&mut Self::Dir>,
    ) -> OperationResult<()> {
        Err(libc::ENOSYS)
    }

    /// Release an opened directory.
    #[allow(unused_variables)]
    fn releasedir(&mut self, ino: Ino, ctx: Option<Self::Dir>) -> OperationResult<()> {
        Err(libc::ENOSYS)
    }
}

pub(super) fn make_fuse_lowlevel_ops<T: Operations>() -> fuse_lowlevel_ops {
    let mut ops = unsafe { mem::zeroed::<fuse_lowlevel_ops>() };

    ops.init = Some(ops_init::<T>);
    ops.destroy = Some(ops_destroy::<T>);
    ops.lookup = Some(ops_lookup::<T>);
    ops.forget = Some(ops_forget::<T>);

    // TODO: bmap
    ops.readlink = Some(ops_readlink::<T>);
    ops.mknod = Some(ops_mknod::<T>);
    ops.mkdir = Some(ops_mkdir::<T>);
    ops.unlink = Some(ops_unlink::<T>);
    ops.rmdir = Some(ops_rmdir::<T>);
    ops.symlink = Some(ops_symlink::<T>);
    ops.rename = Some(ops_rename::<T>);
    ops.link = Some(ops_link::<T>);
    ops.statfs = Some(ops_statfs::<T>);
    ops.setxattr = Some(ops_setxattr::<T>);
    ops.getxattr = Some(ops_getxattr::<T>);
    ops.listxattr = Some(ops_listxattr::<T>);
    ops.removexattr = Some(ops_removexattr::<T>);
    ops.access = Some(ops_access::<T>);

    // TODO: getlk, setlk, ioctl, poll, flock, fallocate, copy_file_range
    ops.open = Some(ops_open::<T>);
    ops.create = Some(ops_create::<T>);
    ops.read = Some(ops_read::<T>);
    ops.write = Some(ops_write::<T>);
    ops.flush = Some(ops_flush::<T>);
    ops.getattr = Some(ops_getattr::<T>);
    ops.setattr = Some(ops_setattr::<T>);
    ops.fsync = Some(ops_fsync::<T>);
    ops.release = Some(ops_release::<T>);

    ops.opendir = Some(ops_opendir::<T>);
    ops.readdir = Some(ops_readdir::<T>);
    ops.fsyncdir = Some(ops_fsyncdir::<T>);
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
    let conn = make_mut_unchecked(conn as *mut ConnectionInfo);
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
            Ok(Entry(entry)) => fuse_reply_entry(req, &entry),
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
            Ok(Entry(entry)) => fuse_reply_entry(req, &entry),
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
            Ok(Entry(entry)) => fuse_reply_entry(req, &entry),
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
            Ok(Entry(entry)) => fuse_reply_entry(req, &entry),
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
            Ok(Entry(entry)) => fuse_reply_entry(req, &entry),
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn ops_statfs<T: Operations>(req: fuse_req_t, ino: fuse_ino_t) {
    call_with_ops(req, |ops: &mut T, req| match ops.statfs(ino) {
        Ok(stat) => fuse_reply_statfs(req, &stat),
        Err(errno) => fuse_reply_err(req, errno),
    })
}

unsafe extern "C" fn ops_setxattr<T: Operations>(
    req: fuse_req_t,
    ino: fuse_ino_t,
    name: *const c_char,
    value: *const c_char,
    size: usize,
    flags: c_int,
) {
    call_with_ops(req, |ops: &mut T, req| {
        let value = std::slice::from_raw_parts(value as *const u8, size);
        match ops.setxattr(ino, CStr::from_ptr(name), value, flags) {
            Ok(()) => {
                0 /* do nothing */
            }
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn ops_getxattr<T: Operations>(
    req: fuse_req_t,
    ino: fuse_ino_t,
    name: *const c_char,
    size: usize,
) {
    call_with_ops(req, |ops: &mut T, req| {
        let mut buf = Vec::with_capacity(size);
        buf.resize(size, 0u8);
        match ops.getxattr(ino, CStr::from_ptr(name), &mut buf[..]) {
            Ok(size) => reply_buf_limited(req, &buf[..size]),
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn ops_listxattr<T: Operations>(req: fuse_req_t, ino: fuse_ino_t, size: usize) {
    call_with_ops(req, |ops: &mut T, req| {
        let mut buf = Vec::with_capacity(size);
        buf.resize(size, 0u8);
        match ops.listxattr(ino, &mut buf[..]) {
            Ok(size) => reply_buf_limited(req, &buf[..size]),
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn ops_removexattr<T: Operations>(
    req: fuse_req_t,
    ino: fuse_ino_t,
    name: *const c_char,
) {
    call_with_ops(req, |ops: &mut T, req| {
        match ops.removexattr(ino, CStr::from_ptr(name)) {
            Ok(()) => {
                0 /* do nothing */
            }
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn ops_access<T: Operations>(req: fuse_req_t, ino: fuse_ino_t, mask: c_int) {
    call_with_ops(req, |ops: &mut T, req| match ops.access(ino, mask) {
        Ok(()) => {
            0 /* do nothing */
        }
        Err(errno) => fuse_reply_err(req, errno),
    })
}

unsafe extern "C" fn ops_open<T: Operations>(
    req: fuse_req_t,
    ino: fuse_ino_t,
    fi: *mut fuse_file_info,
) {
    call_with_ops(req, |ops: &mut T, req| {
        let fi = make_mut_unchecked(fi);
        let mut options = OpenOptions::default();
        match ops.open(ino, fi.flags, &mut options) {
            Ok(file) => {
                fi.fh = into_fh(file);
                options.assign_to(fi);
                fuse_reply_open(req, fi)
            }
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
        let mut options = OpenOptions::default();
        match ops.create(parent, CStr::from_ptr(name), mode, fi.flags, &mut options) {
            Ok((Entry(entry), file)) => {
                fi.fh = into_fh(file);
                options.assign_to(fi);
                fuse_reply_create(req, &entry, fi)
            }
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
        let fi = make_mut_unchecked(fi);
        let file = make_mut(fi.fh as *mut c_void as *mut T::File);
        let mut buf = Vec::with_capacity(size);
        buf.resize(size, 0u8);
        match ops.read(ino, &mut buf[..], off, fi, file) {
            Ok(size) => reply_buf_limited(req, &buf[..size]),
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
        let fi = make_mut_unchecked(fi);
        let file = make_mut(fi.fh as *mut c_void as *mut T::File);
        let buf = std::slice::from_raw_parts(buf as *const u8, size);
        match ops.write(ino, &buf[..], off, fi, file) {
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
        let fi = make_mut_unchecked(fi);
        let file = make_mut(fi.fh as *mut c_void as *mut T::File);
        match ops.flush(ino, fi, file) {
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
    call_with_ops(req, |ops: &mut T, req| {
        let fi = make_mut(fi);
        let file = fi.and_then(|fi| make_mut(fi.fh as *mut c_void as *mut T::File));
        match ops.getattr(ino, file) {
            Ok((stat, timeout)) => fuse_reply_attr(req, &stat, timeout),
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn ops_setattr<T: Operations>(
    req: fuse_req_t,
    ino: fuse_ino_t,
    attr: *mut stat,
    to_set: c_int,
    fi: *mut fuse_file_info,
) {
    call_with_ops(req, |ops: &mut T, req| {
        let fi = make_mut(fi);
        let attr = make_mut_unchecked(attr);
        let file = fi.and_then(|fi| make_mut(fi.fh as *mut c_void as *mut T::File));
        match ops.setattr(ino, &*attr, to_set, file) {
            Ok((stat, timeout)) => fuse_reply_attr(req, &stat, timeout),
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn ops_fsync<T: Operations>(
    req: fuse_req_t,
    ino: fuse_ino_t,
    datasync: c_int,
    fi: *mut fuse_file_info,
) {
    call_with_ops(req, |ops: &mut T, req| {
        let fi = make_mut_unchecked(fi);
        let file = make_mut(fi.fh as *mut c_void as *mut T::File);
        match ops.fsync(ino, datasync, file) {
            Ok(()) => {
                0 /* do nothing */
            }
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn ops_release<T: Operations>(
    req: fuse_req_t,
    ino: fuse_ino_t,
    fi: *mut fuse_file_info,
) {
    call_with_ops(req, |ops: &mut T, req| {
        let fi = make_mut_unchecked(fi);
        let file = from_fh::<T::File>(mem::replace(&mut fi.fh, 0u64));
        match ops.release(ino, fi, file) {
            Ok(()) => {
                fuse_reply_none(req);
                0
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
        let fi = make_mut_unchecked(fi);
        let mut options = DirOpenOptions::default();
        match ops.opendir(ino, &mut options) {
            Ok(dir) => {
                // FIXME: avoid to use boxing.
                fi.fh = into_fh(dir);
                options.assign_to(fi);
                fuse_reply_open(req, fi)
            }
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
        let fi = make_mut_unchecked(fi);
        let dir = make_mut(fi.fh as *mut c_void as *mut T::Dir);
        let mut buf = Vec::with_capacity(size);
        buf.set_len(size);

        let mut dir_buf = DirBuf {
            req: &mut *req,
            buf: &mut buf[..],
            pos: 0,
        };

        let res = ops.readdir(ino, offset, &mut dir_buf, dir);
        let DirBuf { pos, .. } = dir_buf;

        match res {
            Ok(()) => reply_buf_limited(req, &buf[..pos]),
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn ops_fsyncdir<T: Operations>(
    req: fuse_req_t,
    ino: fuse_ino_t,
    datasync: c_int,
    fi: *mut fuse_file_info,
) {
    call_with_ops(req, |ops: &mut T, req| {
        let fi = make_mut_unchecked(fi);
        let dir = make_mut(fi.fh as *mut c_void as *mut T::Dir);
        match ops.fsyncdir(ino, datasync, dir) {
            Ok(()) => {
                0 /* do nothing */
            }
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn ops_releasedir<T: Operations>(
    req: fuse_req_t,
    ino: fuse_ino_t,
    fi: *mut fuse_file_info,
) {
    call_with_ops(req, |ops: &mut T, req| {
        let fi = make_mut_unchecked(fi);
        let dir = from_fh::<T::Dir>(mem::replace(&mut fi.fh, 0u64));
        match ops.releasedir(ino, dir) {
            Ok(()) => {
                0 /* do nothing */
            }
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe fn reply_buf_limited(req: &mut fuse_req, buf: &[u8]) -> c_int {
    match buf.len() {
        0 => fuse_reply_buf(req, ptr::null_mut(), 0),
        n => fuse_reply_buf(req, buf.as_ptr() as *const c_char, n),
    }
}
