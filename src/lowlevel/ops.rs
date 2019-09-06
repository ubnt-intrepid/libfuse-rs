#![allow(unused_variables)]

use libc::{c_char, c_void, off_t};
use libfuse_sys::{
    fuse_file_info, //
    fuse_ino_t,
    fuse_lowlevel_ops,
    fuse_reply_err,
    fuse_req,
    fuse_req_t,
    fuse_req_userdata,
};
use std::mem;

pub trait Operations {
    /// Look up a directory entry by name and get its attributes.
    unsafe fn lookup(&mut self, req: &mut fuse_req, parent: fuse_ino_t, name: *const c_char) {
        fuse_reply_err(req, libc::ENOSYS);
    }

    /// Get a file attributes.
    unsafe fn getattr(&mut self, req: &mut fuse_req, ino: fuse_ino_t, _: *mut fuse_file_info) {
        fuse_reply_err(req, libc::ENOSYS);
    }

    /// Open a file.
    unsafe fn open(&mut self, req: &mut fuse_req, ino: fuse_ino_t, fi: *mut fuse_file_info) {
        fuse_reply_err(req, libc::ENOSYS);
    }

    /// Read data from a file.
    unsafe fn read(
        &mut self,
        req: &mut fuse_req,
        ino: fuse_ino_t,
        size: usize,
        off: off_t,
        fi: *mut fuse_file_info,
    ) {
        fuse_reply_err(req, libc::ENOSYS);
    }

    /// Read a directory.
    unsafe fn readdir(
        &mut self,
        req: &mut fuse_req,
        ino: fuse_ino_t,
        size: usize,
        off: off_t,
        fi: *mut fuse_file_info,
    ) {
        fuse_reply_err(req, libc::ENOSYS);
    }
}

pub(super) fn make_fuse_lowlevel_ops<T: Operations>() -> fuse_lowlevel_ops {
    let mut ops = unsafe { mem::zeroed::<fuse_lowlevel_ops>() };

    ops.destroy = Some(ops_destroy::<T>);
    ops.getattr = Some(ops_getattr::<T>);
    ops.lookup = Some(ops_lookup::<T>);
    ops.readdir = Some(ops_readdir::<T>);
    ops.open = Some(ops_open::<T>);
    ops.read = Some(ops_read::<T>);

    ops
}

unsafe fn call_with_ops<T: Operations>(req: fuse_req_t, f: impl FnOnce(&mut T, &mut fuse_req)) {
    debug_assert!(!req.is_null());
    let req = &mut *req;

    let ops = fuse_req_userdata(req) as *mut T;
    debug_assert!(!ops.is_null());
    f(&mut *ops, req)
}

unsafe extern "C" fn ops_destroy<T: Operations>(user_data: *mut c_void) {
    mem::drop(Box::from_raw(user_data as *mut T));
}

unsafe extern "C" fn ops_getattr<T: Operations>(
    req: fuse_req_t,
    ino: fuse_ino_t,
    fi: *mut fuse_file_info,
) {
    call_with_ops(req, |ops: &mut T, req| ops.getattr(req, ino, fi))
}

unsafe extern "C" fn ops_lookup<T: Operations>(
    req: fuse_req_t,
    parent: fuse_ino_t,
    name: *const c_char,
) {
    call_with_ops(req, |ops: &mut T, req| ops.lookup(req, parent, name))
}

unsafe extern "C" fn ops_readdir<T: Operations>(
    req: fuse_req_t,
    ino: fuse_ino_t,
    size: usize,
    off: off_t,
    fi: *mut fuse_file_info,
) {
    call_with_ops(req, |ops: &mut T, req| ops.readdir(req, ino, size, off, fi))
}

unsafe extern "C" fn ops_open<T: Operations>(
    req: fuse_req_t,
    ino: fuse_ino_t,
    fi: *mut fuse_file_info,
) {
    call_with_ops(req, |ops: &mut T, req| ops.open(req, ino, fi))
}

unsafe extern "C" fn ops_read<T: Operations>(
    req: fuse_req_t,
    ino: fuse_ino_t,
    size: usize,
    off: off_t,
    fi: *mut fuse_file_info,
) {
    call_with_ops(req, |ops: &mut T, req| ops.read(req, ino, size, off, fi))
}
