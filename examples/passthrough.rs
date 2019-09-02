use fuse::{FileInfo, FS};
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
    fn resolve_path(&self, path: &CStr) -> CString {
        let path = path.to_string_lossy();
        let path = if path == "/" {
            Cow::Borrowed(&*self.source)
        } else {
            Cow::Owned(self.source.join(&*path.trim_start_matches("/")))
        };
        let path = path.to_string_lossy().into_owned();
        unsafe { CString::from_vec_unchecked(path.into()) }
    }
}

impl FS for Filesystem {
    fn init(&mut self, _conn: &mut fuse::ConnInfo, cfg: &mut fuse::Config) {
        log::trace!("init()");
        cfg.use_ino(1);
        cfg.entry_timeout(0.0);
        cfg.attr_timeout(0.0);
        cfg.negative_timeout(0.0);
    }

    fn getattr(&self, path: &CStr, stbuf: &mut libc::stat, _fi: Option<&mut FileInfo>) -> c_int {
        let path = self.resolve_path(path);
        log::trace!("getattr(path={:?})", path);

        let res = unsafe { libc::lstat(path.as_ptr(), stbuf) };
        if res == -1 {
            return -errno();
        }

        0
    }

    fn access(&self, path: &CStr, mask: c_int) -> c_int {
        let path = self.resolve_path(path);
        log::trace!("access(path={:?})", path);

        let res = unsafe { libc::access(path.as_ptr(), mask) };
        if res == -1 {
            return -errno();
        }
        0
    }

    fn readlink(&self, path: &CStr, buf: &mut [u8]) -> c_int {
        let path = self.resolve_path(path);
        log::trace!("readlink(path={:?})", path);

        let res = unsafe {
            libc::readlink(
                path.as_ptr(),
                buf.as_mut_ptr() as *mut c_char,
                buf.len() - 1,
            )
        };
        if res == -1 {
            return -errno();
        }
        buf[res as usize] = 0;

        0
    }

    fn readdir(
        &self,
        path: &CStr,
        filler: &mut fuse::FillDir,
        _offset: libc::off_t,
        _fi: Option<&mut FileInfo>,
        _flags: fuse::sys::fuse_readdir_flags,
    ) -> c_int {
        let path = self.resolve_path(path);
        log::trace!("readdir(path={:?})", path);

        let dp = unsafe { libc::opendir(path.as_ptr()) };
        if dp == ptr::null_mut() {
            return -errno();
        }
        let dp = unsafe { &mut *dp };

        loop {
            let de = unsafe { libc::readdir(dp) };
            if de == ptr::null_mut() {
                break;
            }
            let de = unsafe { &mut *de };

            let mut st: libc::stat = unsafe { mem::zeroed() };
            st.st_ino = de.d_ino;
            st.st_mode = (de.d_type as c_uint) << 12;

            let name =
                unsafe { CStr::from_bytes_with_nul_unchecked(mem::transmute(&de.d_name[..])) };
            if unsafe { filler.fill(name, &st, 0, 0) } != 0 {
                break;
            }
        }

        unsafe { libc::closedir(dp) };
        0
    }

    fn mknod(&self, path: &CStr, mode: libc::mode_t, rdev: libc::dev_t) -> c_int {
        let path = self.resolve_path(path);
        log::trace!("mknod(path={:?}, mode={}, rdev={})", path, mode, rdev);

        let res = unsafe { mknod_wrapper(libc::AT_FDCWD, &path, None, mode, rdev) };
        if res == -1 {
            return -errno();
        }
        0
    }

    fn mkdir(&self, path: &CStr, mode: libc::mode_t) -> c_int {
        let path = self.resolve_path(path);
        log::trace!("mkdir(path={:?}, mode={})", path, mode);

        let res = unsafe { libc::mkdir(path.as_ptr(), mode) };
        if res == -1 {
            return -errno();
        }
        0
    }

    fn unlink(&self, path: &CStr) -> c_int {
        let path = self.resolve_path(path);
        log::trace!("unlink(path={:?})", path);

        let res = unsafe { libc::unlink(path.as_ptr()) };
        if res == -1 {
            return -errno();
        }
        0
    }

    fn rmdir(&self, path: &CStr) -> c_int {
        let path = self.resolve_path(path);
        log::trace!("open(path={:?})", path);

        let res = unsafe { libc::rmdir(path.as_ptr()) };
        if res == -1 {
            return -errno();
        }
        0
    }

    fn symlink(&self, path_from: &CStr, path_to: &CStr) -> c_int {
        let path_to = self.resolve_path(path_to);
        log::trace!("symlink(from={:?}, to={:?})", path_from, path_to);

        let res = unsafe { libc::symlink(path_from.as_ptr(), path_to.as_ptr()) };
        if res == -1 {
            return -errno();
        }
        0
    }

    fn rename(&self, path_from: &CStr, path_to: &CStr, flags: c_uint) -> c_int {
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

        let res = unsafe { libc::rename(path_from.as_ptr(), path_to.as_ptr()) };
        if res == -1 {
            return -errno();
        }
        0
    }

    fn link(&self, path_from: &CStr, path_to: &CStr) -> c_int {
        let path_to = self.resolve_path(path_to);
        log::trace!("link(from={:?}, to={:?})", path_from, path_to);

        let res = unsafe { libc::link(path_from.as_ptr(), path_to.as_ptr()) };
        if res == -1 {
            return -errno();
        }

        0
    }

    fn chmod(&self, path: &CStr, mode: libc::mode_t, _fi: Option<&mut FileInfo>) -> c_int {
        let path = self.resolve_path(path);
        log::trace!("chmod(path={:?}, mode={})", path, mode);

        let res = unsafe { libc::chmod(path.as_ptr(), mode) };
        if res == -1 {
            return -errno();
        }

        0
    }

    fn chown(
        &self,
        path: &CStr,
        uid: libc::uid_t,
        gid: libc::gid_t,
        _fi: Option<&mut FileInfo>,
    ) -> c_int {
        let path = self.resolve_path(path);
        log::trace!("chown(path={:?}, uid={}, gid={})", path, uid, gid);

        let res = unsafe { libc::lchown(path.as_ptr(), uid, gid) };
        if res == -1 {
            return -errno();
        }

        0
    }

    fn truncate(&self, path: &CStr, size: libc::off_t, fi: Option<&mut FileInfo>) -> c_int {
        let path = self.resolve_path(path);
        log::trace!("truncate(path={:?}, size={})", path, size);

        let res = if let Some(fi) = fi {
            unsafe { libc::ftruncate(fi.fh() as i32, size) }
        } else {
            unsafe { libc::truncate(path.as_ptr(), size) }
        };

        if res == -1 {
            return -errno();
        }

        0
    }

    fn create(&self, path: &CStr, mode: libc::mode_t, fi: &mut FileInfo) -> c_int {
        let path = self.resolve_path(path);
        log::trace!("create(path={:?}, mode={})", path, mode);

        let res = unsafe { libc::open(path.as_ptr(), fi.flags(), mode) };
        if res == -1 {
            return -errno();
        }

        *fi.fh_mut() = res as u64;
        0
    }

    fn open(&self, path: &CStr, fi: &mut FileInfo) -> c_int {
        let path = self.resolve_path(path);
        log::trace!("open(path={:?})", path);

        let res = unsafe { libc::open(path.as_ptr(), fi.flags()) };
        if res == -1 {
            return nix::errno::errno() * -1;
        }
        *fi.fh_mut() = res as u64;

        0
    }

    fn read(
        &self,
        path: &CStr,
        buf: &mut [u8],
        offset: libc::off_t,
        fi: Option<&mut FileInfo>,
    ) -> c_int {
        let path = self.resolve_path(path);
        log::trace!("read(path={:?})", path);

        let (fd, oneshot) = if let Some(fi) = fi {
            (fi.fh() as c_int, false)
        } else {
            let fh = unsafe { libc::open(path.as_ptr(), libc::O_RDONLY) };
            (fh, true)
        };
        if fd == -1 {
            return -errno();
        }

        let mut res =
            unsafe { libc::pread(fd, buf.as_mut_ptr() as *mut c_void, buf.len(), offset) as c_int };
        if res == -1 {
            res = -errno();
        }

        if oneshot {
            unsafe {
                libc::close(fd);
            }
        }

        res
    }

    fn write(
        &self,
        path: &CStr,
        buf: &[u8],
        offset: libc::off_t,
        fi: Option<&mut FileInfo>,
    ) -> c_int {
        let path = self.resolve_path(path);
        log::trace!("write(path={:?})", path);

        let (fd, oneshot) = if let Some(fi) = fi {
            (fi.fh() as c_int, false)
        } else {
            let fh = unsafe { libc::open(path.as_ptr(), libc::O_WRONLY) };
            (fh, true)
        };

        let mut res =
            unsafe { libc::pwrite(fd, buf.as_ptr() as *const c_void, buf.len(), offset) as c_int };
        if res == -1 {
            res = -errno();
        }

        if oneshot {
            unsafe {
                libc::close(fd);
            }
        }

        res
    }

    fn statfs(&self, path: &CStr, stbuf: &mut libc::statvfs) -> c_int {
        let path = self.resolve_path(path);
        log::trace!("statfs(path={:?})", path);

        let res = unsafe { libc::statvfs(path.as_ptr(), stbuf) };
        if res == -1 {
            return -errno();
        }
        0
    }

    fn release(&self, path: &CStr, fi: &mut FileInfo) -> c_int {
        let path = self.resolve_path(path);
        log::trace!("release(path={:?})", path);
        unsafe {
            libc::close(fi.fh() as i32);
        }
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
