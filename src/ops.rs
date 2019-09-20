use crate::{
    common::{ConnectionInfo, NodeId},
    dir::{DirBuf, OpenDirOptions},
    file::{
        Entry, //
        FlushOptions,
        OpenOptions,
        ReadOptions,
        ReleaseOptions,
        RenameFlags,
        SetAttrs,
        WriteOptions,
        XAttrFlags,
        XAttrReply,
    },
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
    fuse_reply_xattr,
    fuse_req,
    fuse_req_t,
    fuse_req_userdata,
    helpers::{fuse_file_info_fh, fuse_file_info_set_fh},
};
use std::{
    borrow::Cow,
    ffi::{CStr, CString},
    mem, ptr,
};

pub type OperationResult<T> = std::result::Result<T, c_int>;

pub trait Operations {
    /// Initialize the filesystem.
    #[allow(unused_variables)]
    fn init(&mut self, conn: &mut ConnectionInfo<'_>) {}

    /// Look up a directory entry by name and get its attributes.
    #[allow(unused_variables)]
    fn lookup(&mut self, parent: NodeId, name: &CStr) -> OperationResult<Entry> {
        Err(libc::ENOSYS)
    }

    /// Forget about an inode.
    #[allow(unused_variables)]
    fn forget(&mut self, id: NodeId, nlookup: u64) {}

    /// Read a symbolic link.
    #[allow(unused_variables)]
    fn readlink(&mut self, id: NodeId) -> OperationResult<CString> {
        Err(libc::ENOSYS)
    }

    /// Create a file node.
    #[allow(unused_variables)]
    fn mknod(
        &mut self,
        parent: NodeId,
        name: &CStr,
        mode: mode_t,
        rdev: dev_t,
    ) -> OperationResult<Entry> {
        Err(libc::ENOSYS)
    }

    /// Create a directory.
    #[allow(unused_variables)]
    fn mkdir(&mut self, parent: NodeId, name: &CStr, mode: mode_t) -> OperationResult<Entry> {
        Err(libc::ENOSYS)
    }

    /// Remove a file.
    #[allow(unused_variables)]
    fn unlink(&mut self, parent: NodeId, name: &CStr) -> OperationResult<()> {
        Err(libc::ENOSYS)
    }

    /// Remove a directory.
    #[allow(unused_variables)]
    fn rmdir(&mut self, parent: NodeId, name: &CStr) -> OperationResult<()> {
        Err(libc::ENOSYS)
    }

    /// Create a symbolic link.
    #[allow(unused_variables)]
    fn symlink(&mut self, link: &CStr, parent: NodeId, name: &CStr) -> OperationResult<Entry> {
        Err(libc::ENOSYS)
    }

    /// Rename a file.
    #[allow(unused_variables)]
    fn rename(
        &mut self,
        oldparent: NodeId,
        oldname: &CStr,
        newparent: NodeId,
        newname: &CStr,
        flags: RenameFlags,
    ) -> OperationResult<()> {
        Err(libc::ENOSYS)
    }

    /// Create a hard link.
    #[allow(unused_variables)]
    fn link(&mut self, id: NodeId, newparent: NodeId, newname: &CStr) -> OperationResult<Entry> {
        Err(libc::ENOSYS)
    }

    /// Get file system statistics.
    #[allow(unused_variables)]
    fn statfs(&mut self, id: NodeId) -> OperationResult<statvfs> {
        Err(libc::ENOSYS)
    }

    /// Set an extended attribute.
    #[allow(unused_variables)]
    fn setxattr(
        &mut self,
        id: NodeId,
        name: &CStr,
        value: &[u8],
        flags: XAttrFlags,
    ) -> OperationResult<()> {
        Err(libc::ENOSYS)
    }

    /// Get an extended attribute.
    #[allow(unused_variables)]
    fn getxattr(
        &mut self,
        id: NodeId,
        name: &CStr,
        size: usize,
    ) -> OperationResult<XAttrReply<'_>> {
        Err(libc::ENOSYS)
    }

    /// List extended attribute names.
    #[allow(unused_variables)]
    fn listxattr(&mut self, id: NodeId, size: usize) -> OperationResult<XAttrReply<'_>> {
        Err(libc::ENOSYS)
    }

    /// Remove an extended attribute.
    #[allow(unused_variables)]
    fn removexattr(&mut self, id: NodeId, name: &CStr) -> OperationResult<()> {
        Err(libc::ENOSYS)
    }

    #[allow(unused_variables)]
    fn access(&mut self, id: NodeId, mask: c_int) -> OperationResult<()> {
        Err(libc::ENOSYS)
    }

    /// Open a file.
    #[allow(unused_variables)]
    fn open(&mut self, id: NodeId, options: &mut OpenOptions<'_>) -> OperationResult<u64> {
        Ok(0)
    }

    /// Create and open a file.
    #[allow(unused_variables)]
    fn create(
        &mut self,
        parent: NodeId,
        name: &CStr,
        mode: mode_t,
        options: &mut OpenOptions<'_>,
    ) -> OperationResult<(Entry, u64)> {
        Err(libc::ENOSYS)
    }

    /// Read data from an opened file.
    ///
    /// If the size of returned data is larger than `bufsize`,
    /// the remaining part is ignored and the method must be
    /// read again them at the next call.
    #[allow(unused_variables)]
    fn read(
        &mut self,
        id: NodeId,
        off: off_t,
        bufsize: usize,
        opts: &mut ReadOptions<'_>,
        fh: u64,
    ) -> OperationResult<Cow<'_, [u8]>> {
        Err(libc::ENOSYS)
    }

    /// Write data to a file.
    #[allow(unused_variables)]
    fn write(
        &mut self,
        id: NodeId,
        buf: &[u8],
        off: off_t,
        opts: &mut WriteOptions<'_>,
        fh: u64,
    ) -> OperationResult<usize> {
        Err(libc::ENOSYS)
    }

    /// Flush an opened file.
    #[allow(unused_variables)]
    fn flush(&mut self, id: NodeId, opts: &mut FlushOptions<'_>, fh: u64) -> OperationResult<()> {
        Ok(())
    }

    /// Get file attributes.
    #[allow(unused_variables)]
    fn getattr(&mut self, id: NodeId, fh: Option<u64>) -> OperationResult<(stat, f64)> {
        Err(libc::ENOSYS)
    }

    /// Set file attributes.
    #[allow(unused_variables)]
    fn setattr(
        &mut self,
        id: NodeId,
        attrs: &SetAttrs<'_>,
        fh: Option<u64>,
    ) -> OperationResult<(stat, f64)> {
        Err(libc::ENOSYS)
    }

    /// Synchronisze the file contents.
    #[allow(unused_variables)]
    fn fsync(&mut self, id: NodeId, datasync: c_int, fh: u64) -> OperationResult<()> {
        Err(libc::ENOSYS)
    }

    /// Release an opened file.
    #[allow(unused_variables)]
    fn release(
        &mut self,
        id: NodeId,
        options: &mut ReleaseOptions<'_>,
        fh: u64,
    ) -> OperationResult<()> {
        Ok(())
    }

    /// Open a directory.
    #[allow(unused_variables)]
    fn opendir(&mut self, id: NodeId, options: &mut OpenDirOptions) -> OperationResult<u64> {
        Ok(0)
    }

    /// Read a directory.
    #[allow(unused_variables)]
    fn readdir(
        &mut self,
        id: NodeId,
        offset: off_t,
        buf: &mut DirBuf<'_>,
        fh: u64,
    ) -> OperationResult<()> {
        Err(libc::ENOSYS)
    }

    /// Synchronisze the file contents.
    #[allow(unused_variables)]
    fn fsyncdir(&mut self, id: NodeId, datasync: c_int, fh: u64) -> OperationResult<()> {
        Err(libc::ENOSYS)
    }

    /// Release an opened directory.
    #[allow(unused_variables)]
    fn releasedir(&mut self, id: NodeId, fh: u64) -> OperationResult<()> {
        Ok(())
    }
}

pub(super) unsafe fn assign_ops<T: Operations>(op: &mut fuse_lowlevel_ops, _: &T) {
    macro_rules! map_ops {
        ($( $op:ident => $f:ident, )*) => {$(
            libfuse_sys::helpers::$f(op, Some($op::<T>));
        )*}
    }

    map_ops! {
        on_init => fuse_ll_ops_on_init,
        on_destroy => fuse_ll_ops_on_destroy,
        on_lookup => fuse_ll_ops_on_lookup,
        on_forget => fuse_ll_ops_on_forget,
        on_getattr => fuse_ll_ops_on_getattr,
        on_setattr => fuse_ll_ops_on_setattr,
        on_readlink => fuse_ll_ops_on_readlink,
        on_mknod => fuse_ll_ops_on_mknod,
        on_mkdir => fuse_ll_ops_on_mkdir,
        on_unlink => fuse_ll_ops_on_unlink,
        on_rmdir => fuse_ll_ops_on_rmdir,
        on_symlink => fuse_ll_ops_on_symlink,
        on_rename => fuse_ll_ops_on_rename,
        on_link => fuse_ll_ops_on_link,
        on_open => fuse_ll_ops_on_open,
        on_read => fuse_ll_ops_on_read,
        on_write => fuse_ll_ops_on_write,
        on_flush => fuse_ll_ops_on_flush,
        on_release => fuse_ll_ops_on_release,
        on_fsync => fuse_ll_ops_on_fsync,
        on_opendir => fuse_ll_ops_on_opendir,
        on_readdir => fuse_ll_ops_on_readdir,
        on_releasedir => fuse_ll_ops_on_releasedir,
        on_fsyncdir => fuse_ll_ops_on_fsyncdir,
        on_statfs => fuse_ll_ops_on_statfs,
        on_setxattr => fuse_ll_ops_on_setxattr,
        on_getxattr => fuse_ll_ops_on_getxattr,
        on_listxattr => fuse_ll_ops_on_listxattr,
        on_removexattr => fuse_ll_ops_on_removexattr,
        on_access => fuse_ll_ops_on_access,
        on_create => fuse_ll_ops_on_create,

        // TODO: getlk, setlk, bmap, ioctl, poll, write_buf, retrieve_reply,
        //       forget_multi, flock, fallocate, readdirplus, copy_file_range
    }
}

unsafe extern "C" fn on_init<T: Operations>(user_data: *mut c_void, conn: *mut fuse_conn_info) {
    let ops = make_mut_unchecked(user_data as *mut T);
    let conn = make_mut_unchecked(conn);
    ops.init(&mut ConnectionInfo(conn));
}

unsafe extern "C" fn on_destroy<T: Operations>(user_data: *mut c_void) {
    mem::drop(Box::from_raw(user_data as *mut T));
}

unsafe extern "C" fn on_lookup<T: Operations>(
    req: fuse_req_t,
    parent: fuse_ino_t,
    name: *const c_char,
) {
    call_with_ops(req, |ops: &mut T, req| {
        match ops.lookup(parent, CStr::from_ptr(name)) {
            Ok(Entry(entry)) => fuse_reply_entry(req, entry.as_ref()),
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn on_forget<T: Operations>(req: fuse_req_t, ino: fuse_ino_t, nlookup: u64) {
    call_with_ops(req, |ops: &mut T, req| {
        ops.forget(ino, nlookup);
        fuse_reply_none(req);
        0
    })
}

unsafe extern "C" fn on_getattr<T: Operations>(
    req: fuse_req_t,
    ino: fuse_ino_t,
    fi: *mut fuse_file_info,
) {
    call_with_ops(req, |ops: &mut T, req| {
        let fi = make_mut(fi);
        match ops.getattr(ino, fi.map(|fi| fuse_file_info_fh(fi))) {
            Ok((stat, timeout)) => fuse_reply_attr(req, &stat, timeout),
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn on_setattr<T: Operations>(
    req: fuse_req_t,
    ino: fuse_ino_t,
    attr: *const stat,
    to_set: c_int,
    fi: *mut fuse_file_info,
) {
    call_with_ops(req, |ops: &mut T, req| {
        let fi = make_mut(fi);
        let attr = make_ref_unchecked(attr);
        match ops.setattr(
            ino,
            &SetAttrs {
                attr: &*attr,
                to_set,
            },
            fi.map(|fi| fuse_file_info_fh(fi)),
        ) {
            Ok((stat, timeout)) => fuse_reply_attr(req, &stat, timeout),
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn on_readlink<T: Operations>(req: fuse_req_t, ino: fuse_ino_t) {
    call_with_ops(req, |ops: &mut T, req| match ops.readlink(ino) {
        Ok(content) => fuse_reply_readlink(req, content.as_ptr()),
        Err(errno) => fuse_reply_err(req, errno),
    })
}

unsafe extern "C" fn on_mknod<T: Operations>(
    req: fuse_req_t,
    parent: fuse_ino_t,
    name: *const c_char,
    mode: mode_t,
    rdev: dev_t,
) {
    call_with_ops(req, |ops: &mut T, req| {
        match ops.mknod(parent, CStr::from_ptr(name), mode, rdev) {
            Ok(Entry(entry)) => fuse_reply_entry(req, entry.as_ref()),
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn on_mkdir<T: Operations>(
    req: fuse_req_t,
    parent: fuse_ino_t,
    name: *const c_char,
    mode: mode_t,
) {
    call_with_ops(req, |ops: &mut T, req| {
        match ops.mkdir(parent, CStr::from_ptr(name), mode) {
            Ok(Entry(entry)) => fuse_reply_entry(req, entry.as_ref()),
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn on_unlink<T: Operations>(
    req: fuse_req_t,
    parent: fuse_ino_t,
    name: *const c_char,
) {
    call_with_ops(req, |ops: &mut T, req| {
        match ops.unlink(parent, CStr::from_ptr(name)) {
            Ok(()) => fuse_reply_err(req, 0),
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn on_rmdir<T: Operations>(
    req: fuse_req_t,
    parent: fuse_ino_t,
    name: *const c_char,
) {
    call_with_ops(req, |ops: &mut T, req| {
        match ops.rmdir(parent, CStr::from_ptr(name)) {
            Ok(()) => fuse_reply_err(req, 0),
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn on_symlink<T: Operations>(
    req: fuse_req_t,
    link: *const c_char,
    parent: fuse_ino_t,
    name: *const c_char,
) {
    call_with_ops(req, |ops: &mut T, req| {
        match ops.symlink(CStr::from_ptr(link), parent, CStr::from_ptr(name)) {
            Ok(Entry(entry)) => fuse_reply_entry(req, entry.as_ref()),
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn on_rename<T: Operations>(
    req: fuse_req_t,
    oldparent: fuse_ino_t,
    oldname: *const c_char,
    newparent: fuse_ino_t,
    newname: *const c_char,
    flags: c_uint,
) {
    call_with_ops(req, |ops: &mut T, req| {
        match ops.rename(
            oldparent,
            CStr::from_ptr(oldname),
            newparent,
            CStr::from_ptr(newname),
            RenameFlags::from_bits_truncate(flags as c_int),
        ) {
            Ok(()) => fuse_reply_err(req, 0),
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn on_link<T: Operations>(
    req: fuse_req_t,
    ino: fuse_ino_t,
    newparent: fuse_ino_t,
    newname: *const c_char,
) {
    call_with_ops(req, |ops: &mut T, req| {
        match ops.link(ino, newparent, CStr::from_ptr(newname)) {
            Ok(Entry(entry)) => fuse_reply_entry(req, entry.as_ref()),
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn on_open<T: Operations>(
    req: fuse_req_t,
    ino: fuse_ino_t,
    fi: *mut fuse_file_info,
) {
    call_with_ops(req, |ops: &mut T, req| {
        let fi = make_mut_unchecked(fi);
        match ops.open(ino, &mut OpenOptions(fi)) {
            Ok(fh) => {
                fuse_file_info_set_fh(fi, fh);
                fuse_reply_open(req, fi)
            }
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn on_read<T: Operations>(
    req: fuse_req_t,
    ino: fuse_ino_t,
    bufsize: usize,
    off: off_t,
    fi: *mut fuse_file_info,
) {
    call_with_ops(req, |ops: &mut T, req| {
        let fi = make_mut_unchecked(fi);
        let fh = fuse_file_info_fh(fi);
        match ops.read(ino, off, bufsize, &mut ReadOptions(fi), fh) {
            Ok(data) => reply_buf_limited(req, &data[..std::cmp::min(data.len(), bufsize)]),
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn on_write<T: Operations>(
    req: fuse_req_t,
    ino: fuse_ino_t,
    buf: *const c_char,
    size: usize,
    off: off_t,
    fi: *mut fuse_file_info,
) {
    call_with_ops(req, |ops: &mut T, req| {
        let fi = make_mut_unchecked(fi);
        let buf = std::slice::from_raw_parts(buf as *const u8, size);
        let fh = fuse_file_info_fh(fi);
        match ops.write(ino, &buf[..], off, &mut WriteOptions(fi), fh) {
            Ok(count) => fuse_reply_write(req, count),
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn on_flush<T: Operations>(
    req: fuse_req_t,
    ino: fuse_ino_t,
    fi: *mut fuse_file_info,
) {
    call_with_ops(req, |ops: &mut T, req| {
        let fi = make_mut_unchecked(fi);
        let fh = fuse_file_info_fh(fi);
        match ops.flush(ino, &mut FlushOptions(fi), fh) {
            Ok(()) => fuse_reply_err(req, 0),
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn on_release<T: Operations>(
    req: fuse_req_t,
    ino: fuse_ino_t,
    fi: *mut fuse_file_info,
) {
    call_with_ops(req, |ops: &mut T, req| {
        let fi = make_mut_unchecked(fi);
        let fh = fuse_file_info_fh(fi);
        match ops.release(ino, &mut ReleaseOptions(fi), fh) {
            Ok(()) => fuse_reply_err(req, 0),
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn on_fsync<T: Operations>(
    req: fuse_req_t,
    ino: fuse_ino_t,
    datasync: c_int,
    fi: *mut fuse_file_info,
) {
    call_with_ops(req, |ops: &mut T, req| {
        let fi = make_mut_unchecked(fi);
        let fh = fuse_file_info_fh(fi);
        match ops.fsync(ino, datasync, fh) {
            Ok(()) => fuse_reply_err(req, 0),
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn on_opendir<T: Operations>(
    req: fuse_req_t,
    ino: fuse_ino_t,
    fi: *mut fuse_file_info,
) {
    call_with_ops(req, |ops: &mut T, req| {
        let fi = make_mut_unchecked(fi);
        match ops.opendir(ino, &mut OpenDirOptions(fi)) {
            Ok(fh) => {
                fuse_file_info_set_fh(fi, fh);
                fuse_reply_open(req, fi)
            }
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn on_readdir<T: Operations>(
    req: fuse_req_t,
    ino: fuse_ino_t,
    size: usize,
    offset: off_t,
    fi: *mut fuse_file_info,
) {
    call_with_ops(req, |ops: &mut T, req| {
        let fi = make_mut_unchecked(fi);
        let fh = fuse_file_info_fh(fi);
        let mut buf = Vec::with_capacity(size);
        buf.set_len(size);

        let mut dir_buf = DirBuf {
            req: &mut *req,
            buf: &mut buf[..],
            pos: 0,
        };

        let res = ops.readdir(ino, offset, &mut dir_buf, fh);
        let DirBuf { pos, .. } = dir_buf;

        match res {
            Ok(()) => reply_buf_limited(req, &buf[..pos]),
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn on_releasedir<T: Operations>(
    req: fuse_req_t,
    ino: fuse_ino_t,
    fi: *mut fuse_file_info,
) {
    call_with_ops(req, |ops: &mut T, req| {
        let fi = make_mut_unchecked(fi);
        let fh = fuse_file_info_fh(fi);
        match ops.releasedir(ino, fh) {
            Ok(()) => fuse_reply_err(req, 0),
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn on_fsyncdir<T: Operations>(
    req: fuse_req_t,
    ino: fuse_ino_t,
    datasync: c_int,
    fi: *mut fuse_file_info,
) {
    call_with_ops(req, |ops: &mut T, req| {
        let fi = make_mut_unchecked(fi);
        let fh = fuse_file_info_fh(fi);
        match ops.fsyncdir(ino, datasync, fh) {
            Ok(()) => fuse_reply_err(req, 0),
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn on_statfs<T: Operations>(req: fuse_req_t, ino: fuse_ino_t) {
    call_with_ops(req, |ops: &mut T, req| match ops.statfs(ino) {
        Ok(stat) => fuse_reply_statfs(req, &stat),
        Err(errno) => fuse_reply_err(req, errno),
    })
}

unsafe extern "C" fn on_setxattr<T: Operations>(
    req: fuse_req_t,
    ino: fuse_ino_t,
    name: *const c_char,
    value: *const c_char,
    size: usize,
    flags: c_int,
) {
    call_with_ops(req, |ops: &mut T, req| {
        let value = std::slice::from_raw_parts(value as *const u8, size);
        match ops.setxattr(
            ino,
            CStr::from_ptr(name),
            value,
            XAttrFlags::from_bits_truncate(flags),
        ) {
            Ok(()) => fuse_reply_err(req, 0),
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn on_getxattr<T: Operations>(
    req: fuse_req_t,
    ino: fuse_ino_t,
    name: *const c_char,
    size: usize,
) {
    call_with_ops(req, |ops: &mut T, req| {
        match ops.getxattr(ino, CStr::from_ptr(name), size) {
            Ok(XAttrReply::Size(size)) => fuse_reply_xattr(req, size),
            Ok(XAttrReply::Data(ref data)) if data.len() <= size => reply_buf_limited(req, &*data),
            Ok(XAttrReply::Data(..)) => fuse_reply_err(req, libc::ERANGE),
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn on_listxattr<T: Operations>(req: fuse_req_t, ino: fuse_ino_t, size: usize) {
    call_with_ops(req, |ops: &mut T, req| match ops.listxattr(ino, size) {
        Ok(XAttrReply::Size(size)) => fuse_reply_xattr(req, size),
        Ok(XAttrReply::Data(ref data)) if data.len() <= size => reply_buf_limited(req, &*data),
        Ok(XAttrReply::Data(..)) => fuse_reply_err(req, libc::ERANGE),
        Err(errno) => fuse_reply_err(req, errno),
    })
}

unsafe extern "C" fn on_removexattr<T: Operations>(
    req: fuse_req_t,
    ino: fuse_ino_t,
    name: *const c_char,
) {
    call_with_ops(req, |ops: &mut T, req| {
        match ops.removexattr(ino, CStr::from_ptr(name)) {
            Ok(()) => fuse_reply_err(req, 0),
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

unsafe extern "C" fn on_access<T: Operations>(req: fuse_req_t, ino: fuse_ino_t, mask: c_int) {
    call_with_ops(req, |ops: &mut T, req| match ops.access(ino, mask) {
        Ok(()) => fuse_reply_err(req, 0),
        Err(errno) => fuse_reply_err(req, errno),
    })
}

unsafe extern "C" fn on_create<T: Operations>(
    req: fuse_req_t,
    parent: fuse_ino_t,
    name: *const c_char,
    mode: mode_t,
    fi: *mut fuse_file_info,
) {
    call_with_ops(req, |ops: &mut T, req| {
        let fi = make_mut_unchecked(fi);
        match ops.create(parent, CStr::from_ptr(name), mode, &mut OpenOptions(fi)) {
            Ok((Entry(entry), fh)) => {
                fuse_file_info_set_fh(fi, fh);
                fuse_reply_create(req, entry.as_ref(), fi)
            }
            Err(errno) => fuse_reply_err(req, errno),
        }
    })
}

// ==== helpers ====

unsafe fn call_with_ops<T: Operations>(
    req: fuse_req_t,
    f: impl FnOnce(&mut T, &mut fuse_req) -> c_int,
) {
    let req = make_mut_unchecked(req);
    let ops = make_mut_unchecked(fuse_req_userdata(req) as *mut T);
    f(ops, req);
}

unsafe fn reply_buf_limited(req: &mut fuse_req, buf: &[u8]) -> c_int {
    match buf.len() {
        0 => fuse_reply_buf(req, ptr::null_mut(), 0),
        n => fuse_reply_buf(req, buf.as_ptr() as *const c_char, n),
    }
}

fn make_mut<'a, T>(ptr: *mut T) -> Option<&'a mut T> {
    if !ptr.is_null() {
        Some(unsafe { &mut *ptr })
    } else {
        None
    }
}

unsafe fn make_ref_unchecked<'a, T>(ptr: *const T) -> &'a T {
    debug_assert!(!ptr.is_null());
    &*ptr
}

unsafe fn make_mut_unchecked<'a, T>(ptr: *mut T) -> &'a mut T {
    debug_assert!(!ptr.is_null());
    &mut *ptr
}
