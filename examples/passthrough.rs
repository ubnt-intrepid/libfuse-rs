use libc::{
    c_char, c_int, c_uint, c_void, dev_t, gid_t, mode_t, off_t, stat, statvfs, timespec, uid_t,
};
use libfuse::{
    dir::{FillDir, ReadDirFlags},
    Config, ConnInfo, FileInfo, Fuse, Operations, Result,
};
use nix::errno::errno;
use std::{
    borrow::Cow,
    env,
    ffi::{CStr, CString},
    mem,
    os::unix::io::{AsRawFd, RawFd},
    ptr,
};

fn main() {
    env::set_var("RUST_LOG", "trace");
    pretty_env_logger::init();

    unsafe {
        libc::umask(0);
    }

    Fuse::new("passthrough") //
        .foreground(true)
        .threaded(false)
        .mount(
            "/tmp/mountpoint",
            Filesystem {
                source: env::current_dir().unwrap(),
            },
        )
}

enum Fd {
    Opened(RawFd),
    Temp(RawFd),
}

impl AsRawFd for Fd {
    fn as_raw_fd(&self) -> RawFd {
        match *self {
            Fd::Opened(fd) => fd,
            Fd::Temp(fd) => fd,
        }
    }
}

impl Drop for Fd {
    fn drop(&mut self) {
        if let Fd::Temp(fd) = *self {
            unsafe {
                libc::close(fd);
            }
        }
    }
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

impl Operations for Filesystem {
    fn init(&mut self, _conn: &mut ConnInfo, cfg: &mut Config) {
        log::trace!("init()");
        cfg.use_ino(1);
        cfg.entry_timeout(0.0);
        cfg.attr_timeout(0.0);
        cfg.negative_timeout(0.0);
    }

    fn destroy(&mut self) {
        log::trace!("destroy()");
    }

    fn getattr(&self, path: &CStr, stbuf: &mut stat, _fi: Option<&mut FileInfo>) -> Result<()> {
        let path = self.resolve_path(path);
        log::trace!("getattr(path={:?})", path);

        let res = unsafe { libc::lstat(path.as_ptr(), stbuf) };
        if res == -1 {
            return Err(errno());
        }

        Ok(())
    }

    fn access(&self, path: &CStr, mask: c_int) -> Result<()> {
        let path = self.resolve_path(path);
        log::trace!("access(path={:?})", path);

        let res = unsafe { libc::access(path.as_ptr(), mask) };
        if res == -1 {
            return Err(errno());
        }

        Ok(())
    }

    fn readlink(&self, path: &CStr, buf: &mut [u8]) -> Result<()> {
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
            return Err(errno());
        }
        buf[res as usize] = 0;

        Ok(())
    }

    fn readdir(
        &self,
        path: &CStr,
        filler: &mut FillDir,
        _offset: off_t,
        _fi: Option<&mut FileInfo>,
        _flags: ReadDirFlags,
    ) -> Result<()> {
        let path = self.resolve_path(path);
        log::trace!("readdir(path={:?})", path);

        let dp = unsafe {
            let dp = libc::opendir(path.as_ptr());
            if dp == ptr::null_mut() {
                return Err(errno());
            }
            &mut *dp
        };

        loop {
            let de = unsafe {
                let de = libc::readdir(dp);
                if de == ptr::null_mut() {
                    break;
                }
                &mut *de
            };

            let mut st: libc::stat = unsafe { mem::zeroed() };
            st.st_ino = de.d_ino;
            st.st_mode = (de.d_type as c_uint) << 12;

            let name =
                unsafe { CStr::from_bytes_with_nul_unchecked(mem::transmute(&de.d_name[..])) };
            if filler.add(name, Some(&st), 0, Default::default()) {
                break;
            }
        }

        unsafe { libc::closedir(dp) };

        Ok(())
    }

    fn mknod(&self, path: &CStr, mode: mode_t, rdev: dev_t) -> Result<()> {
        let path = self.resolve_path(path);
        log::trace!("mknod(path={:?}, mode={}, rdev={})", path, mode, rdev);

        let res = unsafe { mknod_wrapper(libc::AT_FDCWD, &path, None, mode, rdev) };
        if res == -1 {
            return Err(errno());
        }
        Ok(())
    }

    fn mkdir(&self, path: &CStr, mode: mode_t) -> Result<()> {
        let path = self.resolve_path(path);
        log::trace!("mkdir(path={:?}, mode={})", path, mode);

        let res = unsafe { libc::mkdir(path.as_ptr(), mode) };
        if res == -1 {
            return Err(errno());
        }

        Ok(())
    }

    fn unlink(&self, path: &CStr) -> Result<()> {
        let path = self.resolve_path(path);
        log::trace!("unlink(path={:?})", path);

        let res = unsafe { libc::unlink(path.as_ptr()) };
        if res == -1 {
            return Err(errno());
        }

        Ok(())
    }

    fn rmdir(&self, path: &CStr) -> Result<()> {
        let path = self.resolve_path(path);
        log::trace!("open(path={:?})", path);

        let res = unsafe { libc::rmdir(path.as_ptr()) };
        if res == -1 {
            return Err(errno());
        }

        Ok(())
    }

    fn symlink(&self, path_from: &CStr, path_to: &CStr) -> Result<()> {
        let path_to = self.resolve_path(path_to);
        log::trace!("symlink(from={:?}, to={:?})", path_from, path_to);

        let res = unsafe { libc::symlink(path_from.as_ptr(), path_to.as_ptr()) };
        if res == -1 {
            return Err(errno());
        }

        Ok(())
    }

    fn rename(&self, path_from: &CStr, path_to: &CStr, flags: c_uint) -> Result<()> {
        let path_to = self.resolve_path(path_to);
        log::trace!(
            "rename(from={:?}, to={:?}, flags={})",
            path_from,
            path_to,
            flags
        );

        if flags != 0 {
            return Err(libc::EINVAL);
        }

        let res = unsafe { libc::rename(path_from.as_ptr(), path_to.as_ptr()) };
        if res == -1 {
            return Err(errno());
        }

        Ok(())
    }

    fn link(&self, path_from: &CStr, path_to: &CStr) -> Result<()> {
        let path_to = self.resolve_path(path_to);
        log::trace!("link(from={:?}, to={:?})", path_from, path_to);

        let res = unsafe { libc::link(path_from.as_ptr(), path_to.as_ptr()) };
        if res == -1 {
            return Err(errno());
        }

        Ok(())
    }

    fn chmod(&self, path: &CStr, mode: mode_t, _fi: Option<&mut FileInfo>) -> Result<()> {
        let path = self.resolve_path(path);
        log::trace!("chmod(path={:?}, mode={})", path, mode);

        let res = unsafe { libc::chmod(path.as_ptr(), mode) };
        if res == -1 {
            return Err(errno());
        }

        Ok(())
    }

    fn chown(&self, path: &CStr, uid: uid_t, gid: gid_t, _fi: Option<&mut FileInfo>) -> Result<()> {
        let path = self.resolve_path(path);
        log::trace!("chown(path={:?}, uid={}, gid={})", path, uid, gid);

        let res = unsafe { libc::lchown(path.as_ptr(), uid, gid) };
        if res == -1 {
            return Err(errno());
        }

        Ok(())
    }

    fn truncate(&self, path: &CStr, size: off_t, fi: Option<&mut FileInfo>) -> Result<()> {
        let path = self.resolve_path(path);
        log::trace!("truncate(path={:?}, size={})", path, size);

        let res = if let Some(fi) = fi {
            unsafe { libc::ftruncate(fi.fh() as i32, size) }
        } else {
            unsafe { libc::truncate(path.as_ptr(), size) }
        };
        if res == -1 {
            return Err(errno());
        }

        Ok(())
    }

    fn create(&self, path: &CStr, mode: mode_t, fi: &mut FileInfo) -> Result<()> {
        let path = self.resolve_path(path);
        log::trace!("create(path={:?}, mode={})", path, mode);

        let res = unsafe { libc::open(path.as_ptr(), fi.flags(), mode) };
        if res == -1 {
            return Err(errno());
        }

        *fi.fh_mut() = res as u64;

        Ok(())
    }

    fn open(&self, path: &CStr, fi: &mut FileInfo) -> Result<()> {
        let path = self.resolve_path(path);
        log::trace!("open(path={:?})", path);

        let res = unsafe { libc::open(path.as_ptr(), fi.flags()) };
        if res == -1 {
            return Err(errno());
        }

        *fi.fh_mut() = res as u64;

        Ok(())
    }

    fn read(
        &self,
        path: &CStr,
        buf: &mut [u8],
        offset: off_t,
        fi: Option<&mut FileInfo>,
    ) -> Result<()> {
        let path = self.resolve_path(path);
        log::trace!("read(path={:?})", path);

        let fd = if let Some(fi) = fi {
            Fd::Opened(fi.fh() as c_int)
        } else {
            let fd = unsafe { libc::open(path.as_ptr(), libc::O_RDONLY) };
            if fd == -1 {
                return Err(errno());
            }
            Fd::Temp(fd)
        };

        let res = unsafe {
            libc::pread(
                fd.as_raw_fd(),
                buf.as_mut_ptr() as *mut c_void,
                buf.len(),
                offset,
            ) as c_int
        };
        if res == -1 {
            return Err(errno());
        }

        Ok(())
    }

    fn write(
        &self,
        path: &CStr,
        buf: &[u8],
        offset: off_t,
        fi: Option<&mut FileInfo>,
    ) -> Result<()> {
        let path = self.resolve_path(path);
        log::trace!("write(path={:?})", path);

        let fd = if let Some(fi) = fi {
            Fd::Opened(fi.fh() as c_int)
        } else {
            let fd = unsafe { libc::open(path.as_ptr(), libc::O_WRONLY) };
            if fd == -1 {
                return Err(errno());
            }
            Fd::Temp(fd)
        };

        let res = unsafe {
            libc::pwrite(
                fd.as_raw_fd(),
                buf.as_ptr() as *const c_void,
                buf.len(),
                offset,
            ) as c_int
        };
        if res == -1 {
            return Err(errno());
        }

        Ok(())
    }

    fn statfs(&self, path: &CStr, stbuf: &mut statvfs) -> Result<()> {
        let path = self.resolve_path(path);
        log::trace!("statfs(path={:?})", path);

        let res = unsafe { libc::statvfs(path.as_ptr(), stbuf) };
        if res == -1 {
            return Err(errno());
        }

        Ok(())
    }

    fn release(&self, path: &CStr, fi: &mut FileInfo) -> Result<()> {
        let path = self.resolve_path(path);
        log::trace!("release(path={:?})", path);

        if fi.fh() != 0 {
            unsafe {
                libc::close(fi.fh() as i32);
            }
            *fi.fh_mut() = 0;
        }

        Ok(())
    }

    fn utimens(&self, path: &CStr, ts: &[timespec; 2], fi: Option<&mut FileInfo>) -> Result<()> {
        let path = self.resolve_path(path);
        log::trace!("utimens(path={:?})", path);

        let res = if let Some(fi) = fi {
            unsafe { libc::futimens(fi.fh() as c_int, ts.as_ptr()) }
        } else {
            unsafe { libc::utimensat(0, path.as_ptr(), ts.as_ptr(), libc::AT_SYMLINK_NOFOLLOW) }
        };
        if res == -1 {
            return Err(errno());
        }

        Ok(())
    }

    fn fsync(&self, _path: &CStr, _isdatasync: c_int, _fi: Option<&mut FileInfo>) -> Result<()> {
        // Just a stub.
        // This method is optional and can safely be left unimplemented.
        Ok(())
    }

    fn fallocate(
        &self,
        path: &CStr,
        mode: c_int,
        offset: off_t,
        length: off_t,
        fi: Option<&mut FileInfo>,
    ) -> Result<()> {
        let path = self.resolve_path(path);
        log::trace!(
            "fallocate(path={:?}, mode={}, offset={}, length={})",
            path,
            mode,
            offset,
            length
        );

        if mode != 0 {
            return Err(libc::EOPNOTSUPP);
        }

        let fd = if let Some(fi) = fi {
            Fd::Opened(fi.fh() as c_int)
        } else {
            let fd = unsafe { libc::open(path.as_ptr(), libc::O_WRONLY) };
            if fd == -1 {
                return Err(errno());
            }
            Fd::Temp(fd)
        };

        let res = unsafe { libc::posix_fallocate(fd.as_raw_fd(), offset, length) };
        if res != 0 {
            // posix_allocate does not set errno.
            return Err(-res);
        }

        Ok(())
    }

    fn setxattr(&self, path: &CStr, name: &CStr, value: &[u8], flags: c_int) -> Result<()> {
        let path = self.resolve_path(path);
        log::trace!(
            "setxattr(path={:?}, name={:?}, value=[..;{}], flags={})",
            path,
            name,
            value.len(),
            flags
        );

        let res = unsafe {
            libc::lsetxattr(
                path.as_ptr(),
                name.as_ptr(),
                value.as_ptr() as *const c_void,
                value.len(),
                flags,
            )
        };
        if res == -1 {
            return Err(errno());
        }

        Ok(())
    }

    fn getxattr(&self, path: &CStr, name: &CStr, value: &mut [u8]) -> Result<()> {
        let path = self.resolve_path(path);
        log::trace!("getxattr(path={:?}, name={:?})", path, name);

        let res = unsafe {
            libc::lgetxattr(
                path.as_ptr(),
                name.as_ptr(),
                value.as_mut_ptr() as *mut c_void,
                value.len(),
            )
        };
        if res == -1 {
            return Err(errno());
        }

        Ok(())
    }

    fn listxattr(&self, path: &CStr, list: &mut [u8]) -> Result<()> {
        let path = self.resolve_path(path);
        log::trace!("listxattr(path={:?})", path,);

        let res = unsafe {
            libc::llistxattr(path.as_ptr(), list.as_mut_ptr() as *mut c_char, list.len())
        };
        if res == -1 {
            return Err(errno());
        }

        Ok(())
    }

    fn removexattr(&self, path: &CStr, name: &CStr) -> Result<()> {
        let path = self.resolve_path(path);
        log::trace!("removexattr(path={:?}, name={:?})", path, name);

        let res = unsafe { libc::lremovexattr(path.as_ptr(), name.as_ptr()) };
        if res == -1 {
            return Err(errno());
        }

        Ok(())
    }

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
        let path_in = self.resolve_path(path_in);
        let path_out = self.resolve_path(path_out);
        log::trace!("copy_file_range(path_in={:?}, offset_in={}, path_out={:?}, offset_out={}, len={}, flags={})", path_in, offset_in, path_out, offset_out, len, flags);

        let fd_in = if let Some(fi) = fi_in {
            Fd::Opened(fi.fh() as c_int)
        } else {
            let fd = unsafe { libc::open(path_in.as_ptr(), libc::O_RDONLY) };
            if fd == -1 {
                return Err(errno());
            }
            Fd::Temp(fd)
        };

        let fd_out = if let Some(fi) = fi_out {
            Fd::Opened(fi.fh() as c_int)
        } else {
            let fd = unsafe { libc::open(path_out.as_ptr(), libc::O_WRONLY) };
            if fd == -1 {
                return Err(errno());
            }
            Fd::Temp(fd)
        };

        let res = unsafe {
            libc::syscall(
                libc::SYS_copy_file_range,
                fd_in.as_raw_fd(),
                &offset_in,
                fd_out.as_raw_fd(),
                &offset_out,
                len,
                flags,
            )
        };
        if res == -1 {
            return Err(errno());
        }

        Ok(res as isize)
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
