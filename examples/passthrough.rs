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
        log::trace!("called passthrough_init()");

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
        log::trace!("called passthrough_getattr(path={:?})", path);

        let res = libc::lstat(path.as_ptr(), stbuf);
        if res == -1 {
            return -errno();
        }

        0
    }

    unsafe fn readlink(&self, path: &CStr, buf: &mut [u8]) -> c_int {
        let path = self.resolve_path(path);
        log::trace!("called passthrough_readlink(path={:?})", path);

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
        log::trace!("called passthrough_readdir(path={:?})", path);

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

    unsafe fn open(&self, path: &CStr, fi: *mut fuse::sys::fuse_file_info) -> c_int {
        let path = self.resolve_path(path);
        log::trace!("called passthrough_open(path={:?})", path);

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
        log::trace!("called passthrough_read(path={:?})", path);

        let fd;
        if fi == ptr::null_mut() {
            fd = libc::open(path.as_ptr(), libc::O_RDONLY as i32);
        } else {
            let fi = &mut *fi;
            fd = fi.fh as c_int;
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
}
