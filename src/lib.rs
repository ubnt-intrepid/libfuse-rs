//! A wrapper for libfuse3 using bindgen.

#[allow(nonstandard_style, dead_code)]
pub mod sys {
    use libc::{flock, iovec, stat, statvfs, timespec};
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use libc::{
    c_char, c_double, c_int, c_uint, c_void, dev_t, gid_t, mode_t, off_t, stat, statvfs, uid_t,
};
use std::{
    env,
    ffi::{self, CStr},
    mem,
};
use sys::{
    fuse_config, fuse_conn_info, fuse_file_info, fuse_fill_dir_flags, fuse_operations,
    fuse_readdir_flags,
};

#[repr(C)]
pub struct FileInfo(fuse_file_info);

impl FileInfo {
    pub fn fh(&self) -> u64 {
        self.0.fh
    }

    pub fn fh_mut(&mut self) -> &mut u64 {
        &mut self.0.fh
    }

    pub fn flags(&self) -> c_int {
        self.0.flags
    }
}

#[repr(C)]
pub struct ConnInfo(fuse_conn_info);

#[repr(C)]
pub struct Config(fuse_config);

impl Config {
    pub fn use_ino(&mut self, ino: c_int) -> &mut Self {
        self.0.use_ino = ino;
        self
    }

    pub fn entry_timeout(&mut self, timeout: c_double) -> &mut Self {
        self.0.entry_timeout = timeout;
        self
    }

    pub fn attr_timeout(&mut self, timeout: c_double) -> &mut Self {
        self.0.attr_timeout = timeout;
        self
    }

    pub fn negative_timeout(&mut self, timeout: c_double) -> &mut Self {
        self.0.negative_timeout = timeout;
        self
    }
}

pub struct FillDir {
    buf: *mut c_void,
    filler: unsafe extern "C" fn(
        *mut c_void,
        *const c_char,
        *const stat,
        off_t,
        fuse_fill_dir_flags,
    ) -> c_int,
}

impl FillDir {
    pub unsafe fn fill(
        &mut self,
        name: &CStr,
        stbuf: &stat,
        off: off_t,
        flags: fuse_fill_dir_flags,
    ) -> c_int {
        (self.filler)(self.buf, name.as_ptr(), stbuf, off, flags)
    }
}

#[allow(unused_variables)]
pub trait FS {
    /// Initialize the filesystem.
    fn init(&mut self, conn: &mut ConnInfo, cfg: &mut Config) {}

    /// Clean up the filesystem.
    fn destroy(&mut self) {}

    /// Get file attributes.
    fn getattr(&self, path: &CStr, stbuf: &mut stat, fi: Option<&mut FileInfo>) -> c_int;

    /// Check the file access permissions.
    fn access(&self, path: &CStr, mask: c_int) -> c_int;

    /// Read the target of a symbolic link.
    fn readlink(&self, path: &CStr, buf: &mut [u8]) -> c_int;

    /// Read a directory.
    fn readdir(
        &self,
        path: &CStr,
        filler: &mut FillDir,
        offset: off_t,
        fi: Option<&mut FileInfo>,
        flags: fuse_readdir_flags,
    ) -> c_int;

    /// Create a file node.
    fn mknod(&self, path: &CStr, mode: mode_t, rdev: dev_t) -> c_int;

    /// Create a directory.
    fn mkdir(&self, path: &CStr, mode: mode_t) -> c_int;

    /// Remove a file.
    fn unlink(&self, path: &CStr) -> c_int;

    /// Remove a directory.
    fn rmdir(&self, path: &CStr) -> c_int;

    /// Create a symbolic link.
    fn symlink(&self, path_from: &CStr, path_to: &CStr) -> c_int;

    /// Rename a file.
    fn rename(&self, path_from: &CStr, path_to: &CStr, flags: c_uint) -> c_int;

    /// Create a hard link to a file.
    fn link(&self, path_from: &CStr, path_to: &CStr) -> c_int;

    /// Change the permission bits of a file.
    fn chmod(&self, path: &CStr, mode: mode_t, fi: Option<&mut FileInfo>) -> c_int;

    /// Change the owner and group of a file.
    fn chown(&self, path: &CStr, uid: uid_t, gid: gid_t, fi: Option<&mut FileInfo>) -> c_int;

    /// Change the size of a file.
    fn truncate(&self, path: &CStr, size: off_t, fi: Option<&mut FileInfo>) -> c_int;

    /// Create and open a file.
    fn create(&self, path: &CStr, mode: mode_t, fi: &mut FileInfo) -> c_int;

    /// Open a file.
    fn open(&self, path: &CStr, fi: &mut FileInfo) -> c_int;

    /// Read data from an opened file.
    fn read(&self, path: &CStr, buf: &mut [u8], offset: off_t, fi: Option<&mut FileInfo>) -> c_int;

    /// Write data to an opened file.
    fn write(&self, path: &CStr, buf: &[u8], offset: off_t, fi: Option<&mut FileInfo>) -> c_int;

    /// Get file system statistics.
    fn statfs(&self, path: &CStr, stbuf: &mut statvfs) -> c_int;

    /// Release an opened file.
    fn release(&self, path: &CStr, fi: &mut FileInfo) -> c_int;
}

pub fn main<F: FS>(fuse: F) -> ! {
    let args: Vec<ffi::CString> = env::args()
        .map(ffi::CString::new)
        .collect::<Result<_, _>>()
        .expect("failed to construct C-style arguments list");
    let mut c_args: Vec<*const c_char> = args.iter().map(|a| a.as_ptr()).collect();

    let ops = ops::make_operations::<F>();

    let code = unsafe {
        crate::sys::fuse_main_real(
            c_args.len() as i32,
            c_args.as_mut_ptr() as *mut *mut c_char,
            &ops,
            mem::size_of::<fuse_operations>(),
            Box::into_raw(Box::new(fuse)) as *mut c_void,
        )
    };

    std::process::exit(code)
}

mod ops {
    use crate::{
        sys::{
            fuse_config, fuse_conn_info, fuse_file_info, fuse_fill_dir_t, fuse_operations,
            fuse_readdir_flags,
        },
        Config, ConnInfo, FileInfo, FillDir, FS,
    };
    use libc::{c_char, c_int, c_uint, c_void, dev_t, gid_t, mode_t, off_t, stat, statvfs, uid_t};
    use std::{ffi::CStr, mem, ptr::NonNull, slice};

    #[allow(nonstandard_style)]
    type c_str = *const c_char;

    pub fn make_operations<F: FS>() -> fuse_operations {
        fuse_operations {
            access: Some(lib_access::<F>),
            bmap: None,
            chmod: Some(lib_chmod::<F>),
            chown: Some(lib_chown::<F>),
            copy_file_range: None,
            create: Some(lib_create::<F>),
            destroy: Some(lib_destroy::<F>),
            fallocate: None,
            flock: None,
            flush: None,
            fsync: None,
            fsyncdir: None,
            getattr: Some(lib_getattr::<F>),
            getxattr: None,
            init: Some(lib_init::<F>),
            ioctl: None,
            link: Some(lib_link::<F>),
            listxattr: None,
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
            removexattr: None,
            rename: Some(lib_rename::<F>),
            rmdir: Some(lib_rmdir::<F>),
            setxattr: None,
            statfs: Some(lib_statfs::<F>),
            symlink: Some(lib_symlink::<F>),
            truncate: Some(lib_truncate::<F>),
            unlink: Some(lib_unlink::<F>),
            utimens: None,
            write_buf: None,
            write: Some(lib_write::<F>),
        }
    }

    unsafe fn get_private_data<F: FS, T>(f: impl FnOnce(&F) -> T) -> T {
        let cx = crate::sys::fuse_get_context();
        debug_assert!(!cx.is_null());
        let cx = &mut *cx;
        debug_assert!(!cx.private_data.is_null());
        let data = &*(cx.private_data as *mut F as *const F);
        f(data)
    }

    unsafe extern "C" fn lib_init<F: FS>(
        conn: *mut fuse_conn_info,
        cfg: *mut fuse_config,
    ) -> *mut c_void {
        debug_assert!(!conn.is_null());
        let mut conn = NonNull::new_unchecked(conn).cast::<ConnInfo>();

        debug_assert!(!cfg.is_null());
        let mut cfg = NonNull::new_unchecked(cfg).cast::<Config>();

        let cx = crate::sys::fuse_get_context();
        debug_assert!(!cx.is_null());

        let data_ptr = (&mut *cx).private_data;
        debug_assert!(!data_ptr.is_null());

        (&mut *(data_ptr as *mut F)).init(conn.as_mut(), cfg.as_mut());

        data_ptr
    }

    unsafe extern "C" fn lib_destroy<F: FS>(fs: *mut c_void) {
        let mut data = Box::from_raw(fs as *mut F);
        data.destroy();
        mem::drop(data);
    }

    unsafe extern "C" fn lib_getattr<F: FS>(
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

    unsafe extern "C" fn lib_access<F: FS>(path: c_str, mask: c_int) -> c_int {
        get_private_data(|fs: &F| fs.access(CStr::from_ptr(path), mask))
    }

    unsafe extern "C" fn lib_readlink<F: FS>(path: c_str, buf: *mut c_char, size: usize) -> c_int {
        get_private_data(|fs: &F| {
            let path = CStr::from_ptr(path);
            let buf = slice::from_raw_parts_mut(buf as *mut u8, size);
            fs.readlink(path, buf)
        })
    }

    unsafe extern "C" fn lib_readdir<F: FS>(
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
                flags,
            )
        })
    }

    unsafe extern "C" fn lib_mknod<F: FS>(path: c_str, mode: mode_t, rdev: dev_t) -> c_int {
        get_private_data(|fs: &F| fs.mknod(CStr::from_ptr(path), mode, rdev))
    }

    unsafe extern "C" fn lib_mkdir<F: FS>(path: *const c_char, mode: mode_t) -> c_int {
        get_private_data(|fs: &F| fs.mkdir(CStr::from_ptr(path), mode))
    }

    unsafe extern "C" fn lib_unlink<F: FS>(path: *const c_char) -> c_int {
        get_private_data(|fs: &F| fs.unlink(CStr::from_ptr(path)))
    }

    unsafe extern "C" fn lib_rmdir<F: FS>(path: *const c_char) -> c_int {
        get_private_data(|fs: &F| fs.rmdir(CStr::from_ptr(path)))
    }

    unsafe extern "C" fn lib_symlink<F: FS>(
        path_from: *const c_char,
        path_to: *const c_char,
    ) -> c_int {
        get_private_data(|fs: &F| fs.symlink(CStr::from_ptr(path_from), CStr::from_ptr(path_to)))
    }

    unsafe extern "C" fn lib_rename<F: FS>(
        path_from: *const c_char,
        path_to: *const c_char,
        flags: c_uint,
    ) -> c_int {
        get_private_data(|fs: &F| {
            fs.rename(CStr::from_ptr(path_from), CStr::from_ptr(path_to), flags)
        })
    }

    unsafe extern "C" fn lib_link<F: FS>(
        path_from: *const c_char,
        path_to: *const c_char,
    ) -> c_int {
        get_private_data(|fs: &F| fs.link(CStr::from_ptr(path_from), CStr::from_ptr(path_to)))
    }

    unsafe extern "C" fn lib_chmod<F: FS>(
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

    unsafe extern "C" fn lib_chown<F: FS>(
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

    unsafe extern "C" fn lib_truncate<F: FS>(
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

    unsafe extern "C" fn lib_create<F: FS>(
        path: c_str,
        mode: mode_t,
        fi: *mut fuse_file_info,
    ) -> c_int {
        debug_assert!(!fi.is_null());
        let mut fi = NonNull::new_unchecked(fi).cast::<FileInfo>();
        get_private_data(|fs: &F| fs.create(CStr::from_ptr(path), mode, fi.as_mut()))
    }

    unsafe extern "C" fn lib_open<F: FS>(path: *const c_char, fi: *mut fuse_file_info) -> c_int {
        debug_assert!(!fi.is_null());
        let mut fi = NonNull::new_unchecked(fi).cast::<FileInfo>();
        get_private_data(|fs: &F| fs.open(CStr::from_ptr(path), fi.as_mut()))
    }

    unsafe extern "C" fn lib_read<F: FS>(
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

    unsafe extern "C" fn lib_write<F: FS>(
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

    unsafe extern "C" fn lib_statfs<F: FS>(path: c_str, stbuf: *mut statvfs) -> c_int {
        debug_assert!(!stbuf.is_null());
        let mut stbuf = NonNull::new_unchecked(stbuf);
        get_private_data(|fs: &F| fs.statfs(CStr::from_ptr(path), stbuf.as_mut()))
    }

    unsafe extern "C" fn lib_release<F: FS>(path: c_str, fi: *mut fuse_file_info) -> c_int {
        debug_assert!(!fi.is_null());
        let mut fi = NonNull::new_unchecked(fi).cast::<FileInfo>();
        get_private_data(|fs: &F| fs.release(CStr::from_ptr(path), fi.as_mut()))
    }
}
