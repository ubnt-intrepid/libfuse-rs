use crate::{
    dir::{FillDir, ReadDirFlags},
    Config, ConnInfo, FileInfo, Result,
};
use libc::{
    c_char, c_int, c_uint, c_void, dev_t, gid_t, mode_t, off_t, stat, statvfs, timespec, uid_t,
};
use libfuse_sys::{
    fuse_config, fuse_conn_info, fuse_file_info, fuse_fill_dir_t, fuse_operations,
    fuse_readdir_flags,
};
use std::{convert::TryFrom, ffi::CStr, mem, slice};

#[allow(nonstandard_style)]
type c_str = *const c_char;

/// A set of functions called by libfuse.
#[allow(unused_variables)]
pub trait Operations: Send + Sync + 'static {
    /// Initialize the filesystem.
    fn init(&mut self, conn: &mut ConnInfo, cfg: &mut Config) {}

    /// Check the file access permissions.
    fn access(&self, path: &CStr, mask: c_int) -> Result<()> {
        Err(libc::ENOSYS)
    }

    /// Read the target of a symbolic link.
    fn readlink(&self, path: &CStr, buf: &mut [u8]) -> Result<()> {
        Err(libc::ENOSYS)
    }

    /// Create a file node.
    fn mknod(&self, path: &CStr, mode: mode_t, rdev: dev_t) -> Result<()> {
        Err(libc::ENOSYS)
    }

    /// Create a directory.
    fn mkdir(&self, path: &CStr, mode: mode_t) -> Result<()> {
        Err(libc::ENOSYS)
    }

    /// Remove a file.
    fn unlink(&self, path: &CStr) -> Result<()> {
        Err(libc::ENOSYS)
    }

    /// Remove a directory.
    fn rmdir(&self, path: &CStr) -> Result<()> {
        Err(libc::ENOSYS)
    }

    /// Create a symbolic link.
    fn symlink(&self, path_from: &CStr, path_to: &CStr) -> Result<()> {
        Err(libc::ENOSYS)
    }

    /// Rename a file.
    fn rename(&self, path_from: &CStr, path_to: &CStr, flags: c_uint) -> Result<()> {
        Err(libc::ENOSYS)
    }

    /// Create a hard link to a file.
    fn link(&self, path_from: &CStr, path_to: &CStr) -> Result<()> {
        Err(libc::ENOSYS)
    }

    /// Get file system statistics.
    fn statfs(&self, path: &CStr, stbuf: &mut statvfs) -> Result<()> {
        Err(libc::ENOSYS)
    }

    /// Set extended attributes.
    fn setxattr(&self, path: &CStr, name: &CStr, value: &[u8], flags: c_int) -> Result<()> {
        Err(libc::ENOSYS)
    }

    /// Get extended attributes.
    fn getxattr(&self, path: &CStr, name: &CStr, value: &mut [u8]) -> Result<()> {
        Err(libc::ENOSYS)
    }

    /// List extended attributes.
    fn listxattr(&self, path: &CStr, list: &mut [u8]) -> Result<()> {
        Err(libc::ENOSYS)
    }

    /// Remove extended attributes.
    fn removexattr(&self, path: &CStr, name: &CStr) -> Result<()> {
        Err(libc::ENOSYS)
    }

    /// Create and open a file.
    fn create(&self, path: &CStr, mode: mode_t, fi: &mut FileInfo) -> Result<()> {
        Err(libc::ENOSYS)
    }

    /// Open a file.
    fn open(&self, path: &CStr, fi: &mut FileInfo) -> Result<()> {
        Err(libc::ENOSYS)
    }

    /// Read data from an opened file.
    fn read(
        &self,
        path: &CStr,
        buf: &mut [u8],
        offset: off_t,
        fi: Option<&mut FileInfo>,
    ) -> Result<usize> {
        Err(libc::ENOSYS)
    }

    /// Write data to an opened file.
    fn write(
        &self,
        path: &CStr,
        buf: &[u8],
        offset: off_t,
        fi: Option<&mut FileInfo>,
    ) -> Result<usize> {
        Err(libc::ENOSYS)
    }

    /// Get file attributes.
    fn getattr(&self, path: &CStr, fi: Option<&mut FileInfo>) -> Result<stat> {
        Err(libc::ENOSYS)
    }

    /// Change the permission bits of a file.
    fn chmod(&self, path: &CStr, mode: mode_t, fi: Option<&mut FileInfo>) -> Result<()> {
        Err(libc::ENOSYS)
    }

    /// Change the owner and group of a file.
    fn chown(&self, path: &CStr, uid: uid_t, gid: gid_t, fi: Option<&mut FileInfo>) -> Result<()> {
        Err(libc::ENOSYS)
    }

    /// Change the size of a file.
    fn truncate(&self, path: &CStr, size: off_t, fi: Option<&mut FileInfo>) -> Result<()> {
        Err(libc::ENOSYS)
    }

    /// Change the access and modification times of a file with nanosecond resolution.
    fn utimens(&self, path: &CStr, ts: &[timespec; 2], fi: Option<&mut FileInfo>) -> Result<()> {
        Err(libc::ENOSYS)
    }

    /// Synchronize the file contents.
    fn fsync(&self, path: &CStr, isdatasync: c_int, fi: Option<&mut FileInfo>) -> Result<()> {
        Err(libc::ENOSYS)
    }

    /// Allocate the space for an opened file.
    fn fallocate(
        &self,
        path: &CStr,
        mode: c_int,
        offset: off_t,
        length: off_t,
        fi: Option<&mut FileInfo>,
    ) -> Result<()> {
        Err(libc::ENOSYS)
    }

    /// Copy a range of data from one file to another.
    fn copy_file_range(
        &self,
        path_in: &CStr,
        fi_in: Option<&mut FileInfo>,
        offset_in: off_t,
        path_out: &CStr,
        fi_out: Option<&mut FileInfo>,
        offset_out: off_t,
        len: usize,
        flags: c_int,
    ) -> Result<isize> {
        Err(libc::ENOSYS)
    }

    /// Release an opened file.
    fn release(&self, path: &CStr, fi: &mut FileInfo) -> Result<()> {
        Err(libc::ENOSYS)
    }

    /// Read a directory.
    fn readdir(
        &self,
        path: &CStr,
        filler: &mut FillDir,
        offset: off_t,
        fi: Option<&mut FileInfo>,
        flags: ReadDirFlags,
    ) -> Result<()> {
        Err(libc::ENOSYS)
    }
}

pub fn make_fuse_operations<T: Operations>() -> fuse_operations {
    fuse_operations {
        access: Some(lib_access::<T>),
        bmap: None,
        chmod: Some(lib_chmod::<T>),
        chown: Some(lib_chown::<T>),
        copy_file_range: Some(lib_copy_file_range::<T>),
        create: Some(lib_create::<T>),
        destroy: Some(lib_destroy::<T>),
        fallocate: Some(lib_fallocate::<T>),
        flock: None,
        flush: None,
        fsync: Some(lib_fsync::<T>),
        fsyncdir: None,
        getattr: Some(lib_getattr::<T>),
        getxattr: Some(lib_getxattr::<T>),
        init: Some(lib_init::<T>),
        ioctl: None,
        link: Some(lib_link::<T>),
        listxattr: Some(lib_listxattr::<T>),
        lock: None,
        mkdir: Some(lib_mkdir::<T>),
        mknod: Some(lib_mknod::<T>),
        open: Some(lib_open::<T>),
        opendir: None,
        poll: None,
        read_buf: None,
        read: Some(lib_read::<T>),
        readdir: Some(lib_readdir::<T>),
        readlink: Some(lib_readlink::<T>),
        release: Some(lib_release::<T>),
        releasedir: None,
        removexattr: Some(lib_removexattr::<T>),
        rename: Some(lib_rename::<T>),
        rmdir: Some(lib_rmdir::<T>),
        setxattr: Some(lib_setxattr::<T>),
        statfs: Some(lib_statfs::<T>),
        symlink: Some(lib_symlink::<T>),
        truncate: Some(lib_truncate::<T>),
        unlink: Some(lib_unlink::<T>),
        utimens: Some(lib_utimens::<T>),
        write_buf: None,
        write: Some(lib_write::<T>),
    }
}

fn make_mut<'a, T>(ptr: *mut T) -> Option<&'a mut T> {
    if !ptr.is_null() {
        Some(unsafe { &mut *ptr })
    } else {
        None
    }
}

unsafe fn make_mut_unchecked<'a, T>(ptr: *mut T) -> &'a mut T {
    debug_assert!(!ptr.is_null());
    &mut *ptr
}

unsafe fn call_with_ops<F: Operations, T>(f: impl FnOnce(&F) -> T) -> T {
    let cx = make_mut_unchecked(libfuse_sys::fuse_get_context());

    debug_assert!(!cx.private_data.is_null());
    let data = &*(cx.private_data as *mut F as *const F);

    f(data)
}

unsafe extern "C" fn lib_init<T: Operations>(
    conn: *mut fuse_conn_info,
    cfg: *mut fuse_config,
) -> *mut c_void {
    let conn = make_mut_unchecked(conn as *mut ConnInfo);
    let cfg = make_mut_unchecked(cfg as *mut Config);

    let cx = make_mut_unchecked(libfuse_sys::fuse_get_context());
    let data = make_mut_unchecked(cx.private_data as *mut T);
    data.init(conn, cfg);

    // Returning null_mut() here means that libfuse initializes
    // the pointer of user data with NULL and the data cannot
    // longer be used in the other callbacks.
    data as *mut T as *mut c_void
}

unsafe extern "C" fn lib_destroy<F: Operations>(fs: *mut c_void) {
    let data = Box::from_raw(fs as *mut F);
    mem::drop(data);
}

unsafe extern "C" fn lib_getattr<T: Operations>(
    path: c_str,
    stbuf: *mut stat,
    fi: *mut fuse_file_info,
) -> c_int {
    call_with_ops(|ops: &T| {
        let path = CStr::from_ptr(path);
        let stbuf = make_mut_unchecked(stbuf);
        let fi = make_mut(fi as *mut FileInfo);
        match ops.getattr(path, fi) {
            Ok(stat) => {
                mem::replace(stbuf, stat);
                0
            }
            Err(errno) => -errno,
        }
    })
}

unsafe extern "C" fn lib_access<T: Operations>(path: c_str, mask: c_int) -> c_int {
    call_with_ops(|ops: &T| match ops.access(CStr::from_ptr(path), mask) {
        Ok(()) => 0,
        Err(errno) => -errno,
    })
}

unsafe extern "C" fn lib_readlink<T: Operations>(
    path: c_str,
    buf: *mut c_char,
    size: usize,
) -> c_int {
    call_with_ops(|ops: &T| {
        let path = CStr::from_ptr(path);
        let buf = slice::from_raw_parts_mut(buf as *mut u8, size);
        match ops.readlink(path, buf) {
            Ok(()) => 0,
            Err(errno) => -errno,
        }
    })
}

unsafe extern "C" fn lib_readdir<T: Operations>(
    path: c_str,
    buf: *mut c_void,
    filler: fuse_fill_dir_t,
    offset: off_t,
    fi: *mut fuse_file_info,
    flags: fuse_readdir_flags,
) -> c_int {
    call_with_ops(|ops: &T| {
        let path = CStr::from_ptr(path);
        let mut filler = FillDir {
            buf,
            filler: filler.expect("filler should not be null"),
        };
        let fi = make_mut(fi as *mut FileInfo);
        let flags = ReadDirFlags::from_bits_truncate(flags);

        match ops.readdir(path, &mut filler, offset, fi, flags) {
            Ok(()) => 0,
            Err(errno) => -errno,
        }
    })
}

unsafe extern "C" fn lib_mknod<T: Operations>(path: c_str, mode: mode_t, rdev: dev_t) -> c_int {
    call_with_ops(
        |ops: &T| match ops.mknod(CStr::from_ptr(path), mode, rdev) {
            Ok(()) => 0,
            Err(errno) => -errno,
        },
    )
}

unsafe extern "C" fn lib_mkdir<T: Operations>(path: c_str, mode: mode_t) -> c_int {
    call_with_ops(|ops: &T| match ops.mkdir(CStr::from_ptr(path), mode) {
        Ok(()) => 0,
        Err(errno) => -errno,
    })
}

unsafe extern "C" fn lib_unlink<T: Operations>(path: c_str) -> c_int {
    call_with_ops(|ops: &T| match ops.unlink(CStr::from_ptr(path)) {
        Ok(()) => 0,
        Err(errno) => -errno,
    })
}

unsafe extern "C" fn lib_rmdir<T: Operations>(path: c_str) -> c_int {
    call_with_ops(|ops: &T| match ops.rmdir(CStr::from_ptr(path)) {
        Ok(()) => 0,
        Err(errno) => -errno,
    })
}

unsafe extern "C" fn lib_symlink<T: Operations>(path_from: c_str, path_to: c_str) -> c_int {
    call_with_ops(
        |fs: &T| match fs.symlink(CStr::from_ptr(path_from), CStr::from_ptr(path_to)) {
            Ok(()) => 0,
            Err(errno) => -errno,
        },
    )
}

unsafe extern "C" fn lib_rename<T: Operations>(
    path_from: c_str,
    path_to: c_str,
    flags: c_uint,
) -> c_int {
    call_with_ops(|ops: &T| {
        match ops.rename(CStr::from_ptr(path_from), CStr::from_ptr(path_to), flags) {
            Ok(()) => 0,
            Err(errno) => -errno,
        }
    })
}

unsafe extern "C" fn lib_link<T: Operations>(path_from: c_str, path_to: c_str) -> c_int {
    call_with_ops(
        |ops: &T| match ops.link(CStr::from_ptr(path_from), CStr::from_ptr(path_to)) {
            Ok(()) => 0,
            Err(errno) => -errno,
        },
    )
}

unsafe extern "C" fn lib_chmod<T: Operations>(
    path: c_str,
    mode: mode_t,
    fi: *mut fuse_file_info,
) -> c_int {
    call_with_ops(|ops: &T| {
        let fi = make_mut(fi as *mut FileInfo);
        match ops.chmod(CStr::from_ptr(path), mode, fi) {
            Ok(()) => 0,
            Err(errno) => -errno,
        }
    })
}

unsafe extern "C" fn lib_chown<T: Operations>(
    path: c_str,
    uid: uid_t,
    gid: gid_t,
    fi: *mut fuse_file_info,
) -> c_int {
    call_with_ops(|ops: &T| {
        let fi = make_mut(fi as *mut FileInfo);
        match ops.chown(CStr::from_ptr(path), uid, gid, fi) {
            Ok(()) => 0,
            Err(errno) => -errno,
        }
    })
}

unsafe extern "C" fn lib_truncate<T: Operations>(
    path: c_str,
    size: off_t,
    fi: *mut fuse_file_info,
) -> c_int {
    call_with_ops(|ops: &T| {
        let fi = make_mut(fi as *mut FileInfo);
        match ops.truncate(CStr::from_ptr(path), size, fi) {
            Ok(()) => 0,
            Err(errno) => -errno,
        }
    })
}

unsafe extern "C" fn lib_utimens<T: Operations>(
    path: c_str,
    ts: *const timespec,
    fi: *mut fuse_file_info,
) -> c_int {
    call_with_ops(|ops: &T| {
        let ts = <[timespec; 2]>::try_from(std::slice::from_raw_parts(ts, 2)) //
            .expect("invalid length");
        let fi = make_mut(fi as *mut FileInfo);
        match ops.utimens(CStr::from_ptr(path), &ts, fi) {
            Ok(()) => 0,
            Err(errno) => -errno,
        }
    })
}

unsafe extern "C" fn lib_create<T: Operations>(
    path: c_str,
    mode: mode_t,
    fi: *mut fuse_file_info,
) -> c_int {
    call_with_ops(|ops: &T| {
        let fi = make_mut_unchecked(fi as *mut FileInfo);
        match ops.create(CStr::from_ptr(path), mode, fi) {
            Ok(()) => 0,
            Err(errno) => -errno,
        }
    })
}

unsafe extern "C" fn lib_open<T: Operations>(path: c_str, fi: *mut fuse_file_info) -> c_int {
    call_with_ops(|ops: &T| {
        let fi = make_mut_unchecked(fi as *mut FileInfo);
        match ops.open(CStr::from_ptr(path), fi) {
            Ok(()) => 0,
            Err(errno) => -errno,
        }
    })
}

unsafe extern "C" fn lib_read<T: Operations>(
    path: c_str,
    buf: *mut c_char,
    size: usize,
    offset: off_t,
    fi: *mut fuse_file_info,
) -> c_int {
    call_with_ops(|ops: &T| {
        let path = CStr::from_ptr(path);
        let buf = slice::from_raw_parts_mut(buf as *mut u8, size);
        let fi = make_mut(fi as *mut FileInfo);
        match ops.read(path, buf, offset, fi) {
            Ok(len) => len as c_int,
            Err(errno) => -errno,
        }
    })
}

unsafe extern "C" fn lib_write<T: Operations>(
    path: c_str,
    buf: *const c_char,
    size: usize,
    offset: off_t,
    fi: *mut fuse_file_info,
) -> c_int {
    call_with_ops(|ops: &T| {
        let path = CStr::from_ptr(path);
        let buf = slice::from_raw_parts(buf as *const u8, size);
        let fi = make_mut(fi as *mut FileInfo);
        match ops.write(path, buf, offset, fi) {
            Ok(len) => len as c_int,
            Err(errno) => -errno,
        }
    })
}

unsafe extern "C" fn lib_statfs<T: Operations>(path: c_str, stbuf: *mut statvfs) -> c_int {
    call_with_ops(|ops: &T| {
        let stbuf = make_mut_unchecked(stbuf);
        match ops.statfs(CStr::from_ptr(path), stbuf) {
            Ok(()) => 0,
            Err(errno) => -errno,
        }
    })
}

unsafe extern "C" fn lib_release<T: Operations>(path: c_str, fi: *mut fuse_file_info) -> c_int {
    call_with_ops(|ops: &T| {
        let fi = make_mut_unchecked(fi as *mut FileInfo);
        match ops.release(CStr::from_ptr(path), fi) {
            Ok(()) => 0,
            Err(errno) => -errno,
        }
    })
}

unsafe extern "C" fn lib_fsync<T: Operations>(
    path: c_str,
    isdatasync: c_int,
    fi: *mut fuse_file_info,
) -> c_int {
    call_with_ops(|ops: &T| {
        let fi = make_mut(fi as *mut FileInfo);
        match ops.fsync(CStr::from_ptr(path), isdatasync, fi) {
            Ok(()) => 0,
            Err(errno) => -errno,
        }
    })
}

unsafe extern "C" fn lib_fallocate<T: Operations>(
    path: c_str,
    mode: c_int,
    offset: off_t,
    length: off_t,
    fi: *mut fuse_file_info,
) -> c_int {
    call_with_ops(|ops: &T| {
        let fi = make_mut(fi as *mut FileInfo);
        match ops.fallocate(CStr::from_ptr(path), mode, offset, length, fi) {
            Ok(()) => 0,
            Err(errno) => -errno,
        }
    })
}

unsafe extern "C" fn lib_setxattr<T: Operations>(
    path: c_str,
    name: c_str,
    value: *const c_char,
    size: usize,
    flags: c_int,
) -> c_int {
    call_with_ops(|ops: &T| {
        let value = slice::from_raw_parts(value as *const u8, size);
        match ops.setxattr(CStr::from_ptr(path), CStr::from_ptr(name), value, flags) {
            Ok(()) => 0,
            Err(errno) => -errno,
        }
    })
}

unsafe extern "C" fn lib_getxattr<T: Operations>(
    path: c_str,
    name: c_str,
    value: *mut c_char,
    size: usize,
) -> c_int {
    call_with_ops(|ops: &T| {
        let value = slice::from_raw_parts_mut(value as *mut u8, size);
        match ops.getxattr(CStr::from_ptr(path), CStr::from_ptr(name), value) {
            Ok(()) => 0,
            Err(errno) => -errno,
        }
    })
}

unsafe extern "C" fn lib_listxattr<T: Operations>(
    path: c_str,
    list: *mut c_char,
    size: usize,
) -> c_int {
    call_with_ops(|ops: &T| {
        let list = slice::from_raw_parts_mut(list as *mut u8, size);
        match ops.listxattr(CStr::from_ptr(path), list) {
            Ok(()) => 0,
            Err(errno) => -errno,
        }
    })
}

unsafe extern "C" fn lib_removexattr<T: Operations>(path: c_str, name: c_str) -> c_int {
    call_with_ops(
        |ops: &T| match ops.removexattr(CStr::from_ptr(path), CStr::from_ptr(name)) {
            Ok(()) => 0,
            Err(errno) => -errno,
        },
    )
}

unsafe extern "C" fn lib_copy_file_range<T: Operations>(
    path_in: c_str,
    fi_in: *mut fuse_file_info,
    offset_in: off_t,
    path_out: c_str,
    fi_out: *mut fuse_file_info,
    offset_out: off_t,
    len: usize,
    flags: c_int,
) -> isize {
    call_with_ops(|ops: &T| {
        let fi_in = make_mut(fi_in as *mut FileInfo);
        let fi_out = make_mut(fi_out as *mut FileInfo);
        match ops.copy_file_range(
            CStr::from_ptr(path_in),
            fi_in,
            offset_in,
            CStr::from_ptr(path_out),
            fi_out,
            offset_out,
            len,
            flags,
        ) {
            Ok(len) => len,
            Err(errno) => -errno as isize,
        }
    })
}
