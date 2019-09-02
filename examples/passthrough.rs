use nix::errno::errno;
use std::{
    borrow::Cow,
    env,
    ffi::{CStr, CString},
    mem,
    os::raw::{c_char, c_int, c_uint, c_void},
    ptr,
};

fn main() {
    env::set_var("RUST_LOG", "trace");
    pretty_env_logger::init();

    unsafe {
        libc::umask(0);
    }

    fuse::main(Filesystem {
        source: env::current_dir().unwrap(),
    })
}

#[derive(Debug)]
struct Filesystem {
    source: std::path::PathBuf,
}

impl Filesystem {
    unsafe fn resolve_path(&self, path: &CStr) -> CString {
        let path = path.to_string_lossy();
        let path = if path == "/" {
            Cow::Borrowed(&*self.source)
        } else {
            Cow::Owned(self.source.join(&*path.trim_start_matches("/")))
        };
        CString::from_vec_unchecked(path.to_string_lossy().into_owned().into())
    }
}

impl fuse::FS for Filesystem {
    unsafe fn init(&self, _conn: *mut fuse::sys::fuse_conn_info, cfg: *mut fuse::sys::fuse_config) {
        log::trace!("init()");

        let cfg = &mut *cfg;

        cfg.use_ino = 1;

        cfg.entry_timeout = 0.0;
        cfg.attr_timeout = 0.0;
        cfg.negative_timeout = 0.0;
    }

    unsafe fn getattr(
        &self,
        path: &CStr,
        stbuf: *mut libc::stat,
        _fi: *mut fuse::sys::fuse_file_info,
    ) -> c_int {
        let path = self.resolve_path(path);
        log::trace!("getattr(path={:?})", path);

        let res = libc::lstat(path.as_ptr(), stbuf);
        if res == -1 {
            return -errno();
        }

        0
    }

    unsafe fn access(&self, path: &CStr, mask: c_int) -> c_int {
        let path = self.resolve_path(path);
        log::trace!("access(path={:?})", path);

        let res = libc::access(path.as_ptr(), mask);
        if res == -1 {
            return -errno();
        }
        0
    }

    unsafe fn readlink(&self, path: &CStr, buf: &mut [u8]) -> c_int {
        let path = self.resolve_path(path);
        log::trace!("readlink(path={:?})", path);

        let res = libc::readlink(
            path.as_ptr(),
            buf.as_mut_ptr() as *mut c_char,
            buf.len() - 1,
        );
        if res == -1 {
            return -errno();
        }
        buf[res as usize] = 0;

        0
    }

    unsafe fn readdir(
        &self,
        path: &CStr,
        buf: *mut c_void,
        filler: fuse::sys::fuse_fill_dir_t,
        _offset: libc::off_t,
        _fi: *mut fuse::sys::fuse_file_info,
        _flags: fuse::sys::fuse_readdir_flags,
    ) -> c_int {
        let path = self.resolve_path(path);
        let filler = filler.expect("filler should not be null");
        log::trace!("readdir(path={:?})", path);

        let dp = libc::opendir(path.as_ptr());
        if dp == ptr::null_mut() {
            return -errno();
        }
        let dp = &mut *dp;

        loop {
            let de = libc::readdir(dp);
            if de == ptr::null_mut() {
                break;
            }
            let de = &mut *de;

            let mut st: libc::stat = mem::zeroed();
            st.st_ino = de.d_ino;
            st.st_mode = (de.d_type as c_uint) << 12;

            if filler(buf, de.d_name.as_ptr(), &st, 0, 0) != 0 {
                break;
            }
        }

        libc::closedir(dp);
        0
    }

    unsafe fn mknod(&self, path: &CStr, mode: libc::mode_t, rdev: libc::dev_t) -> c_int {
        let path = self.resolve_path(path);
        log::trace!("mknod(path={:?}, mode={}, rdev={})", path, mode, rdev);

        let res = mknod_wrapper(libc::AT_FDCWD, &path, None, mode, rdev);
        if res == -1 {
            return -errno();
        }
        0
    }

    unsafe fn mkdir(&self, path: &CStr, mode: libc::mode_t) -> c_int {
        let path = self.resolve_path(path);
        log::trace!("mkdir(path={:?}, mode={})", path, mode);

        let res = libc::mkdir(path.as_ptr(), mode);
        if res == -1 {
            return -errno();
        }
        0
    }

    unsafe fn unlink(&self, path: &CStr) -> c_int {
        let path = self.resolve_path(path);
        log::trace!("unlink(path={:?})", path);

        let res = libc::unlink(path.as_ptr());
        if res == -1 {
            return -errno();
        }
        0
    }

    unsafe fn rmdir(&self, path: &CStr) -> c_int {
        let path = self.resolve_path(path);
        log::trace!("open(path={:?})", path);

        let res = libc::rmdir(path.as_ptr());
        if res == -1 {
            return -errno();
        }
        0
    }

    unsafe fn symlink(&self, path_from: &CStr, path_to: &CStr) -> c_int {
        let path_to = self.resolve_path(path_to);
        log::trace!("symlink(from={:?}, to={:?})", path_from, path_to);

        let res = libc::symlink(path_from.as_ptr(), path_to.as_ptr());
        if res == -1 {
            return -errno();
        }
        0
    }

    unsafe fn rename(&self, path_from: &CStr, path_to: &CStr, flags: c_uint) -> c_int {
        let path_to = self.resolve_path(path_to);
        log::trace!(
            "rename(from={:?}, to={:?}, flags={})",
            path_from,
            path_to,
            flags
        );

        if flags != 0 {
            return -libc::EINVAL;
        }

        let res = libc::rename(path_from.as_ptr(), path_to.as_ptr());
        if res == -1 {
            return -errno();
        }
        0
    }

    unsafe fn link(&self, path_from: &CStr, path_to: &CStr) -> c_int {
        let path_to = self.resolve_path(path_to);
        log::trace!("link(from={:?}, to={:?})", path_from, path_to);

        let res = libc::link(path_from.as_ptr(), path_to.as_ptr());
        if res == -1 {
            return -errno();
        }

        0
    }

    unsafe fn chmod(
        &self,
        path: &CStr,
        mode: libc::mode_t,
        _fi: *mut fuse::sys::fuse_file_info,
    ) -> c_int {
        let path = self.resolve_path(path);
        log::trace!("chmod(path={:?}, mode={})", path, mode);

        let res = libc::chmod(path.as_ptr(), mode);
        if res == -1 {
            return -errno();
        }

        0
    }

    unsafe fn chown(
        &self,
        path: &CStr,
        uid: libc::uid_t,
        gid: libc::gid_t,
        _fi: *mut fuse::sys::fuse_file_info,
    ) -> c_int {
        let path = self.resolve_path(path);
        log::trace!("chown(path={:?}, uid={}, gid={})", path, uid, gid);

        let res = libc::lchown(path.as_ptr(), uid, gid);
        if res == -1 {
            return -errno();
        }

        0
    }

    unsafe fn truncate(
        &self,
        path: &CStr,
        size: libc::off_t,
        fi: *mut fuse::sys::fuse_file_info,
    ) -> c_int {
        let path = self.resolve_path(path);
        log::trace!("truncate(path={:?}, size={})", path, size);

        let res;
        if fi != ptr::null_mut() {
            res = libc::ftruncate((&mut *fi).fh as i32, size);
        } else {
            res = libc::truncate(path.as_ptr(), size);
        }

        if res == -1 {
            return -errno();
        }

        0
    }

    unsafe fn create(
        &self,
        path: &CStr,
        mode: libc::mode_t,
        fi: *mut fuse::sys::fuse_file_info,
    ) -> c_int {
        let path = self.resolve_path(path);
        log::trace!("create(path={:?}, mode={})", path, mode);

        let fi = &mut *fi;

        let res = libc::open(path.as_ptr(), fi.flags, mode);
        if res == -1 {
            return -errno();
        }

        fi.fh = res as u64;
        0
    }

    unsafe fn open(&self, path: &CStr, fi: *mut fuse::sys::fuse_file_info) -> c_int {
        let path = self.resolve_path(path);
        log::trace!("open(path={:?})", path);

        let fi = &mut *fi;

        let res = libc::open(path.as_ptr(), fi.flags);
        if res == -1 {
            return nix::errno::errno() * -1;
        }

        fi.fh = res as u64;
        0
    }

    unsafe fn read(
        &self,
        path: &CStr,
        buf: &mut [u8],
        offset: libc::off_t,
        fi: *mut fuse::sys::fuse_file_info,
    ) -> c_int {
        let path = self.resolve_path(path);
        log::trace!("read(path={:?})", path);

        let fd;
        if fi == ptr::null_mut() {
            fd = libc::open(path.as_ptr(), libc::O_RDONLY as i32);
        } else {
            fd = (&mut *fi).fh as c_int;
        }
        if fd == -1 {
            return -errno();
        }

        let mut res = libc::pread(fd, buf.as_mut_ptr() as *mut c_void, buf.len(), offset) as c_int;
        if res == -1 {
            res = -errno();
        }

        if fi == ptr::null_mut() {
            libc::close(fd);
        }

        res
    }

    unsafe fn write(
        &self,
        path: &CStr,
        buf: &[u8],
        offset: libc::off_t,
        fi: *mut fuse::sys::fuse_file_info,
    ) -> c_int {
        let path = self.resolve_path(path);
        log::trace!("write(path={:?})", path);

        let fd;
        if fi == ptr::null_mut() {
            fd = libc::open(path.as_ptr(), libc::O_WRONLY);
        } else {
            fd = (&mut *fi).fh as c_int;
        }

        let mut res = libc::pwrite(fd, buf.as_ptr() as *const c_void, buf.len(), offset) as c_int;
        if res == -1 {
            res = -errno();
        }

        if fi == ptr::null_mut() {
            libc::close(fd);
        }

        res
    }

    unsafe fn statfs(&self, path: &CStr, stbuf: *mut libc::statvfs) -> c_int {
        let path = self.resolve_path(path);
        log::trace!("statfs(path={:?})", path);

        let res = libc::statvfs(path.as_ptr(), stbuf);
        if res == -1 {
            return -errno();
        }
        0
    }

    unsafe fn release(&self, path: &CStr, fi: *mut fuse::sys::fuse_file_info) -> c_int {
        let path = self.resolve_path(path);
        log::trace!("release(path={:?})", path);
        debug_assert!(!fi.is_null());
        libc::close((&mut *fi).fh as i32);
        0
    }
}

unsafe fn mknod_wrapper(
    dirfd: c_int,
    path: &CStr,
    link: Option<&CStr>,
    mode: libc::mode_t,
    rdev: libc::dev_t,
) -> c_int {
    match mode & libc::S_IFMT {
        libc::S_IFREG => {
            let res = libc::openat(
                dirfd,
                path.as_ptr(),
                libc::O_CREAT | libc::O_EXCL | libc::O_WRONLY,
                mode,
            );
            if res >= 0 {
                return libc::close(res);
            }
            res
        }
        libc::S_IFDIR => libc::mkdirat(dirfd, path.as_ptr(), mode),
        libc::S_IFLNK => libc::symlinkat(
            link.map(|s| s.as_ptr()).unwrap_or_else(ptr::null),
            dirfd,
            path.as_ptr(),
        ),
        libc::S_IFIFO => libc::mkfifoat(dirfd, path.as_ptr(), mode),
        _ => libc::mknodat(dirfd, path.as_ptr(), mode, rdev),
    }
}
