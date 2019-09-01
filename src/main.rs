mod fuse;

use fuse::sys;
use std::{
    env, ffi, mem,
    os::raw::{c_char, c_int, c_uint, c_void},
    ptr,
};

fn main() {
    env::set_var("RUST_LOG", "trace");
    pretty_env_logger::init();

    unsafe {
        sys::umask(0);
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
    unsafe fn resolve_path(&self, path: *const c_char) -> ffi::CString {
        let path = ffi::CStr::from_ptr(path);
        let path = path.to_string_lossy();
        let path = if path == "/" {
            std::borrow::Cow::Borrowed(&*self.source)
        } else {
            std::borrow::Cow::Owned(self.source.join(&*path.trim_start_matches("/")))
        };

        ffi::CString::from_vec_unchecked(path.to_string_lossy().into_owned().into())
    }
}

impl fuse::FS for Filesystem {
    unsafe fn init(&self, _conn: *mut sys::fuse_conn_info, cfg: *mut sys::fuse_config) {
        log::trace!("called passthrough_init()");

        let cfg = &mut *cfg;

        cfg.use_ino = 1;

        cfg.entry_timeout = 0.0;
        cfg.attr_timeout = 0.0;
        cfg.negative_timeout = 0.0;
    }

    unsafe fn getattr(
        &self,
        path: *const c_char,
        stbuf: *mut sys::stat,
        _fi: *mut sys::fuse_file_info,
    ) -> c_int {
        let path = self.resolve_path(path);
        log::trace!("called passthrough_getattr(path={:?})", path);

        let res = sys::lstat(path.as_ptr(), stbuf);
        if res == -1 {
            return sys::errno() * -1;
        }

        0
    }

    unsafe fn readlink(&self, path: *const c_char, buf: *mut c_char, size: usize) -> c_int {
        let path = self.resolve_path(path);
        log::trace!("called passthrough_readlink(path={:?})", path);

        let res = sys::readlink(path.as_ptr(), buf, size - 1);
        if res == -1 {
            return sys::errno() * -1;
        }

        *buf.offset(res) = 0;
        0
    }

    unsafe fn readdir(
        &self,
        path: *const c_char,
        buf: *mut c_void,
        filler: sys::fuse_fill_dir_t,
        _offset: sys::off_t,
        _fi: *mut sys::fuse_file_info,
        _flags: sys::fuse_readdir_flags,
    ) -> c_int {
        let path = self.resolve_path(path);
        let filler = filler.expect("filler should not be null");
        log::trace!("called passthrough_readdir(path={:?})", path);

        let dp = sys::opendir(path.as_ptr());
        if dp == ptr::null_mut() {
            return sys::errno() * -1;
        }
        let dp = &mut *dp;

        loop {
            let de = sys::readdir(dp);
            if de == ptr::null_mut() {
                break;
            }
            let de = &mut *de;

            let mut st: sys::stat = mem::zeroed();
            st.st_ino = de.d_ino;
            st.st_mode = (de.d_type as c_uint) << 12;

            if filler(buf, de.d_name.as_ptr(), &mut st, 0, 0) != 0 {
                break;
            }
        }

        sys::closedir(dp);
        0
    }

    unsafe fn open(&self, path: *const c_char, fi: *mut sys::fuse_file_info) -> c_int {
        let path = self.resolve_path(path);
        log::trace!("called passthrough_open(path={:?})", path);

        let fi = &mut *fi;

        let res = sys::open(path.as_ptr(), fi.flags);
        if res == -1 {
            return sys::errno() * -1;
        }

        fi.fh = res as u64;
        0
    }

    unsafe fn read(
        &self,
        path: *const c_char,
        buf: *mut c_char,
        size: usize,
        offset: sys::off_t,
        fi: *mut sys::fuse_file_info,
    ) -> c_int {
        let path = self.resolve_path(path);
        log::trace!("called passthrough_read(path={:?})", path);

        let fd;
        if fi == ptr::null_mut() {
            fd = sys::open(path.as_ptr(), sys::O_RDONLY as i32);
        } else {
            let fi = &mut *fi;
            fd = fi.fh as c_int;
        }
        if fd == -1 {
            return sys::errno() * -1;
        }

        let mut res = sys::pread(fd, buf as *mut c_void, size, offset) as c_int;
        if res == -1 {
            res = sys::errno() * -1;
        }

        if fi == ptr::null_mut() {
            sys::close(fd);
        }

        res
    }
}
