use crate::{
    dir::{FillDir, ReadDirFlags},
    Config, ConnInfo, FileInfo,
};
use libc::{
    c_char, c_int, c_uint, c_void, dev_t, gid_t, mode_t, off_t, stat, statvfs, timespec, uid_t,
};
use libfuse_sys::{
    fuse_config, fuse_conn_info, fuse_file_info, fuse_fill_dir_t, fuse_operations,
    fuse_readdir_flags,
};
use std::{convert::TryFrom, ffi::CStr, mem, ptr::NonNull, slice};

#[allow(nonstandard_style)]
type c_str = *const c_char;

/// A set of functions called by libfuse.
#[allow(unused_variables)]
pub trait Operations: Send + Sync + 'static {
    /// Initialize the filesystem.
    fn init(&mut self, conn: &mut ConnInfo, cfg: &mut Config) {}

    /// Clean up the filesystem.
    fn destroy(&mut self) {}

    /// Check the file access permissions.
    fn access(&self, path: &CStr, mask: c_int) -> c_int {
        -libc::ENOSYS
    }

    /// Read the target of a symbolic link.
    fn readlink(&self, path: &CStr, buf: &mut [u8]) -> c_int {
        -libc::ENOSYS
    }

    /// Create a file node.
    fn mknod(&self, path: &CStr, mode: mode_t, rdev: dev_t) -> c_int {
        -libc::ENOSYS
    }

    /// Create a directory.
    fn mkdir(&self, path: &CStr, mode: mode_t) -> c_int {
        -libc::ENOSYS
    }

    /// Remove a file.
    fn unlink(&self, path: &CStr) -> c_int {
        -libc::ENOSYS
    }

    /// Remove a directory.
    fn rmdir(&self, path: &CStr) -> c_int {
        -libc::ENOSYS
    }

    /// Create a symbolic link.
    fn symlink(&self, path_from: &CStr, path_to: &CStr) -> c_int {
        -libc::ENOSYS
    }

    /// Rename a file.
    fn rename(&self, path_from: &CStr, path_to: &CStr, flags: c_uint) -> c_int {
        -libc::ENOSYS
    }

    /// Create a hard link to a file.
    fn link(&self, path_from: &CStr, path_to: &CStr) -> c_int {
        -libc::ENOSYS
    }

    /// Get file system statistics.
    fn statfs(&self, path: &CStr, stbuf: &mut statvfs) -> c_int {
        -libc::ENOSYS
    }

    /// Set extended attributes.
    fn setxattr(&self, path: &CStr, name: &CStr, value: &[u8], flags: c_int) -> c_int {
        -libc::ENOSYS
    }

    /// Get extended attributes.
    fn getxattr(&self, path: &CStr, name: &CStr, value: &mut [u8]) -> c_int {
        -libc::ENOSYS
    }

    /// List extended attributes.
    fn listxattr(&self, path: &CStr, list: &mut [u8]) -> c_int {
        -libc::ENOSYS
    }

    /// Remove extended attributes.
    fn removexattr(&self, path: &CStr, name: &CStr) -> c_int {
        -libc::ENOSYS
    }

    /// Create and open a file.
    fn create(&self, path: &CStr, mode: mode_t, fi: &mut FileInfo) -> c_int {
        -libc::ENOSYS
    }

    /// Open a file.
    fn open(&self, path: &CStr, fi: &mut FileInfo) -> c_int {
        -libc::ENOSYS
    }

    /// Read data from an opened file.
    fn read(&self, path: &CStr, buf: &mut [u8], offset: off_t, fi: Option<&mut FileInfo>) -> c_int {
        -libc::ENOSYS
    }

    /// Write data to an opened file.
    fn write(&self, path: &CStr, buf: &[u8], offset: off_t, fi: Option<&mut FileInfo>) -> c_int {
        -libc::ENOSYS
    }

    /// Get file attributes.
    fn getattr(&self, path: &CStr, stbuf: &mut stat, fi: Option<&mut FileInfo>) -> c_int {
        -libc::ENOSYS
    }

    /// Change the permission bits of a file.
    fn chmod(&self, path: &CStr, mode: mode_t, fi: Option<&mut FileInfo>) -> c_int {
        -libc::ENOSYS
    }

    /// Change the owner and group of a file.
    fn chown(&self, path: &CStr, uid: uid_t, gid: gid_t, fi: Option<&mut FileInfo>) -> c_int {
        -libc::ENOSYS
    }

    /// Change the size of a file.
    fn truncate(&self, path: &CStr, size: off_t, fi: Option<&mut FileInfo>) -> c_int {
        -libc::ENOSYS
    }

    /// Change the access and modification times of a file with nanosecond resolution.
    fn utimens(&self, path: &CStr, ts: &[timespec; 2], fi: Option<&mut FileInfo>) -> c_int {
        -libc::ENOSYS
    }

    /// Synchronize the file contents.
    fn fsync(&self, path: &CStr, isdatasync: c_int, fi: Option<&mut FileInfo>) -> c_int {
        -libc::ENOSYS
    }

    /// Allocate the space for an opened file.
    fn fallocate(
        &self,
        path: &CStr,
        mode: c_int,
        offset: off_t,
        length: off_t,
        fi: Option<&mut FileInfo>,
    ) -> c_int {
        -libc::ENOSYS
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
    ) -> isize {
        -libc::ENOSYS as isize
    }

    /// Release an opened file.
    fn release(&self, path: &CStr, fi: &mut FileInfo) -> c_int {
        -libc::ENOSYS
    }

    /// Read a directory.
    fn readdir(
        &self,
        path: &CStr,
        filler: &mut FillDir,
        offset: off_t,
        fi: Option<&mut FileInfo>,
        flags: ReadDirFlags,
    ) -> c_int {
        -libc::ENOSYS
    }
}

pub fn make_fuse_operations<F: Operations>() -> fuse_operations {
    fuse_operations {
        access: Some(lib_access::<F>),
        bmap: None,
        chmod: Some(lib_chmod::<F>),
        chown: Some(lib_chown::<F>),
        copy_file_range: Some(lib_copy_file_range::<F>),
        create: Some(lib_create::<F>),
        destroy: Some(lib_destroy::<F>),
        fallocate: Some(lib_fallocate::<F>),
        flock: None,
        flush: None,
        fsync: Some(lib_fsync::<F>),
        fsyncdir: None,
        getattr: Some(lib_getattr::<F>),
        getxattr: Some(lib_getxattr::<F>),
        init: Some(lib_init::<F>),
        ioctl: None,
        link: Some(lib_link::<F>),
        listxattr: Some(lib_listxattr::<F>),
        lock: None,
        mkdir: Some(lib_mkdir::<F>),
        mknod: Some(lib_mknod::<F>),
        open: Some(lib_open::<F>),
        opendir: None,
        poll: None,
        read_buf: None,
        read: Some(lib_read::<F>),
        readdir: Some(lib_readdir::<F>),
        readlink: Some(lib_readlink::<F>),
        release: Some(lib_release::<F>),
        releasedir: None,
        removexattr: Some(lib_removexattr::<F>),
        rename: Some(lib_rename::<F>),
        rmdir: Some(lib_rmdir::<F>),
        setxattr: Some(lib_setxattr::<F>),
        statfs: Some(lib_statfs::<F>),
        symlink: Some(lib_symlink::<F>),
        truncate: Some(lib_truncate::<F>),
        unlink: Some(lib_unlink::<F>),
        utimens: Some(lib_utimens::<F>),
        write_buf: None,
        write: Some(lib_write::<F>),
    }
}

unsafe fn get_private_data<F: Operations, T>(f: impl FnOnce(&F) -> T) -> T {
    let cx = libfuse_sys::fuse_get_context();
    debug_assert!(!cx.is_null());
    let cx = &mut *cx;
    debug_assert!(!cx.private_data.is_null());
    let data = &*(cx.private_data as *mut F as *const F);
    f(data)
}

unsafe extern "C" fn lib_init<F: Operations>(
    conn: *mut fuse_conn_info,
    cfg: *mut fuse_config,
) -> *mut c_void {
    debug_assert!(!conn.is_null());
    let mut conn = NonNull::new_unchecked(conn).cast::<ConnInfo>();

    debug_assert!(!cfg.is_null());
    let mut cfg = NonNull::new_unchecked(cfg).cast::<Config>();

    let cx = libfuse_sys::fuse_get_context();
    debug_assert!(!cx.is_null());

    let data_ptr = (&mut *cx).private_data;
    debug_assert!(!data_ptr.is_null());

    (&mut *(data_ptr as *mut F)).init(conn.as_mut(), cfg.as_mut());

    data_ptr
}

unsafe extern "C" fn lib_destroy<F: Operations>(fs: *mut c_void) {
    let mut data = Box::from_raw(fs as *mut F);
    data.destroy();
    mem::drop(data);
}

unsafe extern "C" fn lib_getattr<F: Operations>(
    path: c_str,
    stbuf: *mut stat,
    fi: *mut fuse_file_info,
) -> c_int {
    debug_assert!(!stbuf.is_null());
    let mut stbuf = NonNull::new_unchecked(stbuf);
    let mut fi = NonNull::new(fi).map(NonNull::cast::<FileInfo>);
    get_private_data(|fs: &F| {
        fs.getattr(
            CStr::from_ptr(path),
            stbuf.as_mut(),
            fi.as_mut().map(|fi| fi.as_mut()),
        )
    })
}

unsafe extern "C" fn lib_access<F: Operations>(path: c_str, mask: c_int) -> c_int {
    get_private_data(|fs: &F| fs.access(CStr::from_ptr(path), mask))
}

unsafe extern "C" fn lib_readlink<F: Operations>(
    path: c_str,
    buf: *mut c_char,
    size: usize,
) -> c_int {
    get_private_data(|fs: &F| {
        let path = CStr::from_ptr(path);
        let buf = slice::from_raw_parts_mut(buf as *mut u8, size);
        fs.readlink(path, buf)
    })
}

unsafe extern "C" fn lib_readdir<F: Operations>(
    path: *const c_char,
    buf: *mut c_void,
    filler: fuse_fill_dir_t,
    offset: off_t,
    fi: *mut fuse_file_info,
    flags: fuse_readdir_flags,
) -> c_int {
    let path = CStr::from_ptr(path);
    let mut filler = FillDir {
        buf,
        filler: filler.expect("filler should not be null"),
    };
    let mut fi = NonNull::new(fi).map(NonNull::cast::<FileInfo>);
    get_private_data(|fs: &F| {
        fs.readdir(
            path,
            &mut filler,
            offset,
            fi.as_mut().map(|fi| fi.as_mut()),
            ReadDirFlags::from_bits_truncate(flags),
        )
    })
}

unsafe extern "C" fn lib_mknod<F: Operations>(path: c_str, mode: mode_t, rdev: dev_t) -> c_int {
    get_private_data(|fs: &F| fs.mknod(CStr::from_ptr(path), mode, rdev))
}

unsafe extern "C" fn lib_mkdir<F: Operations>(path: *const c_char, mode: mode_t) -> c_int {
    get_private_data(|fs: &F| fs.mkdir(CStr::from_ptr(path), mode))
}

unsafe extern "C" fn lib_unlink<F: Operations>(path: *const c_char) -> c_int {
    get_private_data(|fs: &F| fs.unlink(CStr::from_ptr(path)))
}

unsafe extern "C" fn lib_rmdir<F: Operations>(path: *const c_char) -> c_int {
    get_private_data(|fs: &F| fs.rmdir(CStr::from_ptr(path)))
}

unsafe extern "C" fn lib_symlink<F: Operations>(
    path_from: *const c_char,
    path_to: *const c_char,
) -> c_int {
    get_private_data(|fs: &F| fs.symlink(CStr::from_ptr(path_from), CStr::from_ptr(path_to)))
}

unsafe extern "C" fn lib_rename<F: Operations>(
    path_from: *const c_char,
    path_to: *const c_char,
    flags: c_uint,
) -> c_int {
    get_private_data(|fs: &F| fs.rename(CStr::from_ptr(path_from), CStr::from_ptr(path_to), flags))
}

unsafe extern "C" fn lib_link<F: Operations>(
    path_from: *const c_char,
    path_to: *const c_char,
) -> c_int {
    get_private_data(|fs: &F| fs.link(CStr::from_ptr(path_from), CStr::from_ptr(path_to)))
}

unsafe extern "C" fn lib_chmod<F: Operations>(
    path: c_str,
    mode: mode_t,
    fi: *mut fuse_file_info,
) -> c_int {
    let mut fi = NonNull::new(fi).map(NonNull::cast::<FileInfo>);
    get_private_data(|fs: &F| {
        fs.chmod(
            CStr::from_ptr(path),
            mode,
            fi.as_mut().map(|fi| fi.as_mut()),
        )
    })
}

unsafe extern "C" fn lib_chown<F: Operations>(
    path: c_str,
    uid: uid_t,
    gid: gid_t,
    fi: *mut fuse_file_info,
) -> c_int {
    let mut fi = NonNull::new(fi).map(NonNull::cast::<FileInfo>);
    get_private_data(|fs: &F| {
        fs.chown(
            CStr::from_ptr(path),
            uid,
            gid,
            fi.as_mut().map(|fi| fi.as_mut()),
        )
    })
}

unsafe extern "C" fn lib_truncate<F: Operations>(
    path: c_str,
    size: off_t,
    fi: *mut fuse_file_info,
) -> c_int {
    let mut fi = NonNull::new(fi).map(NonNull::cast::<FileInfo>);
    get_private_data(|fs: &F| {
        fs.truncate(
            CStr::from_ptr(path),
            size,
            fi.as_mut().map(|fi| fi.as_mut()),
        )
    })
}

unsafe extern "C" fn lib_utimens<F: Operations>(
    path: c_str,
    ts: *const timespec,
    fi: *mut fuse_file_info,
) -> c_int {
    let ts = <[timespec; 2]>::try_from(std::slice::from_raw_parts(ts, 2)).expect("invalid length");
    let mut fi = NonNull::new(fi).map(NonNull::cast::<FileInfo>);
    get_private_data(|fs: &F| {
        fs.utimens(CStr::from_ptr(path), &ts, fi.as_mut().map(|fi| fi.as_mut()))
    })
}

unsafe extern "C" fn lib_create<F: Operations>(
    path: c_str,
    mode: mode_t,
    fi: *mut fuse_file_info,
) -> c_int {
    debug_assert!(!fi.is_null());
    let mut fi = NonNull::new_unchecked(fi).cast::<FileInfo>();
    get_private_data(|fs: &F| fs.create(CStr::from_ptr(path), mode, fi.as_mut()))
}

unsafe extern "C" fn lib_open<F: Operations>(
    path: *const c_char,
    fi: *mut fuse_file_info,
) -> c_int {
    debug_assert!(!fi.is_null());
    let mut fi = NonNull::new_unchecked(fi).cast::<FileInfo>();
    get_private_data(|fs: &F| fs.open(CStr::from_ptr(path), fi.as_mut()))
}

unsafe extern "C" fn lib_read<F: Operations>(
    path: c_str,
    buf: *mut c_char,
    size: usize,
    offset: off_t,
    fi: *mut fuse_file_info,
) -> c_int {
    let path = CStr::from_ptr(path);
    let buf = std::slice::from_raw_parts_mut(buf as *mut u8, size);
    let mut fi = NonNull::new(fi).map(NonNull::cast::<FileInfo>);
    get_private_data(|fs: &F| fs.read(path, buf, offset, fi.as_mut().map(|fi| fi.as_mut())))
}

unsafe extern "C" fn lib_write<F: Operations>(
    path: c_str,
    buf: *const c_char,
    size: usize,
    offset: off_t,
    fi: *mut fuse_file_info,
) -> c_int {
    let path = CStr::from_ptr(path);
    let buf = std::slice::from_raw_parts(buf as *const u8, size);
    let mut fi = NonNull::new(fi).map(NonNull::cast::<FileInfo>);
    get_private_data(|fs: &F| fs.write(path, buf, offset, fi.as_mut().map(|fi| fi.as_mut())))
}

unsafe extern "C" fn lib_statfs<F: Operations>(path: c_str, stbuf: *mut statvfs) -> c_int {
    debug_assert!(!stbuf.is_null());
    let mut stbuf = NonNull::new_unchecked(stbuf);
    get_private_data(|fs: &F| fs.statfs(CStr::from_ptr(path), stbuf.as_mut()))
}

unsafe extern "C" fn lib_release<F: Operations>(path: c_str, fi: *mut fuse_file_info) -> c_int {
    debug_assert!(!fi.is_null());
    let mut fi = NonNull::new_unchecked(fi).cast::<FileInfo>();
    get_private_data(|fs: &F| fs.release(CStr::from_ptr(path), fi.as_mut()))
}

unsafe extern "C" fn lib_fsync<F: Operations>(
    path: c_str,
    isdatasync: c_int,
    fi: *mut fuse_file_info,
) -> c_int {
    let mut fi = NonNull::new(fi).map(NonNull::cast::<FileInfo>);
    get_private_data(|fs: &F| {
        fs.fsync(
            CStr::from_ptr(path),
            isdatasync,
            fi.as_mut().map(|fi| fi.as_mut()),
        )
    })
}

unsafe extern "C" fn lib_fallocate<F: Operations>(
    path: c_str,
    mode: c_int,
    offset: off_t,
    length: off_t,
    fi: *mut fuse_file_info,
) -> c_int {
    let mut fi = NonNull::new(fi).map(NonNull::cast::<FileInfo>);
    get_private_data(|fs: &F| {
        fs.fallocate(
            CStr::from_ptr(path),
            mode,
            offset,
            length,
            fi.as_mut().map(|fi| fi.as_mut()),
        )
    })
}

unsafe extern "C" fn lib_setxattr<F: Operations>(
    path: c_str,
    name: c_str,
    value: *const c_char,
    size: usize,
    flags: c_int,
) -> c_int {
    let value = std::slice::from_raw_parts(value as *const u8, size);
    get_private_data(|fs: &F| fs.setxattr(CStr::from_ptr(path), CStr::from_ptr(name), value, flags))
}

unsafe extern "C" fn lib_getxattr<F: Operations>(
    path: c_str,
    name: c_str,
    value: *mut c_char,
    size: usize,
) -> c_int {
    let value = std::slice::from_raw_parts_mut(value as *mut u8, size);
    get_private_data(|fs: &F| fs.getxattr(CStr::from_ptr(path), CStr::from_ptr(name), value))
}

unsafe extern "C" fn lib_listxattr<F: Operations>(
    path: c_str,
    list: *mut c_char,
    size: usize,
) -> c_int {
    let list = std::slice::from_raw_parts_mut(list as *mut u8, size);
    get_private_data(|fs: &F| fs.listxattr(CStr::from_ptr(path), list))
}

unsafe extern "C" fn lib_removexattr<F: Operations>(path: c_str, name: c_str) -> c_int {
    get_private_data(|fs: &F| fs.removexattr(CStr::from_ptr(path), CStr::from_ptr(name)))
}

unsafe extern "C" fn lib_copy_file_range<F: Operations>(
    path_in: c_str,
    fi_in: *mut fuse_file_info,
    offset_in: off_t,
    path_out: c_str,
    fi_out: *mut fuse_file_info,
    offset_out: off_t,
    len: usize,
    flags: c_int,
) -> isize {
    let mut fi_in = NonNull::new(fi_in).map(NonNull::cast::<FileInfo>);
    let mut fi_out = NonNull::new(fi_out).map(NonNull::cast::<FileInfo>);
    get_private_data(|fs: &F| {
        fs.copy_file_range(
            CStr::from_ptr(path_in),
            fi_in.as_mut().map(|fi| fi.as_mut()),
            offset_in,
            CStr::from_ptr(path_out),
            fi_out.as_mut().map(|fi| fi.as_mut()),
            offset_out,
            len,
            flags,
        )
    })
}
