use crate::{
    common::{ConnectionInfo, Ino},
    dir::{DirBuf, DirOperations},
    file::{Entry, FileOperations},
    util::*,
};
use libc::{c_char, c_int, c_uint, c_void, dev_t, mode_t, off_t, stat};
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
    /// Associated `DirOperations` during opening an directory.
    type File: FileOperations<Ops = Self>;

    /// Associated `DirOperations` during opening an directory.
    type Dir: DirOperations<Ops = Self>;

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

    /// Get a file attributes.
    #[allow(unused_variables)]
    fn getattr(&mut self, ino: Ino) -> OperationResult<(stat, f64)> {
        Err(libc::ENOSYS)
    }

    // TODO: setattr

    /// Open a file.
    #[allow(unused_variables)]
    fn open(&mut self, ino: Ino, fi: &mut fuse_file_info) -> OperationResult<Self::File> {
        Err(libc::ENOSYS)
    }

    /// Create and open a file.
    #[allow(unused_variables)]
    fn create(
        &mut self,
        parent: Ino,
        name: &CStr,
        mode: mode_t,
        fi: &mut fuse_file_info,
    ) -> OperationResult<(Self::File, Entry)> {
        Err(libc::ENOSYS)
    }

    /// Open a directory.
    #[allow(unused_variables)]
    fn opendir(&mut self, ino: Ino, fi: &mut fuse_file_info) -> OperationResult<Self::Dir> {
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

unsafe extern "C" fn ops_open<T: Operations>(
    req: fuse_req_t,
    ino: fuse_ino_t,
    fi: *mut fuse_file_info,
) {
    call_with_ops(req, |ops: &mut T, req| {
        let fi = make_mut_unchecked(fi);
        match ops.open(ino, fi) {
            Ok(file) => {
                fi.fh = into_fh(file);
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
        match ops.create(parent, CStr::from_ptr(name), mode, fi) {
            Ok((file, Entry(entry))) => {
                fi.fh = into_fh(file);
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
        let file = make_mut_unchecked(fi.fh as *mut c_void as *mut T::File);
        let mut buf = Vec::with_capacity(size);
        buf.resize(size, 0u8);
        match file.read(ops, ino, &mut buf[..], off, fi) {
            Ok(size) => {
                let out = &buf[..size];
                match out.len() {
                    0 => fuse_reply_buf(req, ptr::null_mut(), 0),
                    n => fuse_reply_buf(req, out.as_ptr() as *const c_char, n),
                }
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
        let fi = make_mut_unchecked(fi);
        let file = make_mut_unchecked(fi.fh as *mut c_void as *mut T::File);
        let buf = std::slice::from_raw_parts(buf as *const u8, size);
        match file.write(ops, ino, &buf[..], off, fi) {
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
        let file = make_mut_unchecked(fi.fh as *mut c_void as *mut T::File);
        match file.flush(ops, ino, fi) {
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
        let res = match make_mut(fi) {
            Some(fi) => {
                let file = make_mut_unchecked(fi.fh as *mut c_void as *mut T::File);
                file.getattr(ops, ino)
            }
            None => ops.getattr(ino),
        };
        match res {
            Ok((stat, timeout)) => fuse_reply_attr(req, &stat, timeout),
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
        let mut file = from_fh::<T::File>(mem::replace(&mut fi.fh, 0u64));
        match file.release(ops, ino, fi) {
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
        match ops.opendir(ino, fi) {
            Ok(dir) => {
                // FIXME: avoid to use boxing.
                fi.fh = into_fh(dir);
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
        let dir = make_mut_unchecked(fi.fh as *mut c_void as *mut T::Dir);
        let mut buf = Vec::with_capacity(size);
        buf.set_len(size);

        let mut dir_buf = DirBuf {
            req: &mut *req,
            buf: &mut buf[..],
            pos: 0,
        };

        let res = dir.readdir(ops, ino, offset, &mut dir_buf);
        let DirBuf { pos, .. } = dir_buf;

        match res {
            Ok(()) => {
                let out = &buf[..pos];
                match out.len() {
                    0 => fuse_reply_buf(req, ptr::null_mut(), 0),
                    n => fuse_reply_buf(req, out.as_ptr() as *const c_char, n),
                }
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
        let mut dir = from_fh::<T::Dir>(mem::replace(&mut fi.fh, 0u64));
        match dir.releasedir(ops, ino) {
            Ok(()) => {
                0 /* do nothing */
            }
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}
