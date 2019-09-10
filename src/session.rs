use super::ops::{make_fuse_lowlevel_ops, Operations};
use libc::{c_char, c_int};
use libfuse_sys::{
    fuse_args, //
    fuse_buf,
    fuse_opt_free_args,
    fuse_remove_signal_handlers,
    fuse_session,
    fuse_session_destroy,
    fuse_session_exited,
    fuse_session_mount,
    fuse_session_new,
    fuse_session_process_buf,
    fuse_session_receive_buf,
    fuse_session_reset,
    fuse_session_unmount,
    fuse_set_signal_handlers,
};
use std::{
    ffi::CString,
    io, mem,
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
    ptr::NonNull,
};

#[derive(Debug)]
pub struct Builder {
    fsname: String,
    debug: bool,
}

impl Builder {
    pub fn new(fsname: impl Into<String>) -> Self {
        Self {
            fsname: fsname.into(),
            debug: false,
        }
    }

    pub fn debug(mut self, enabled: bool) -> Self {
        self.debug = enabled;
        self
    }

    pub fn build<T: Operations>(self, ops: T) -> io::Result<Session> {
        let mut args = vec![CString::new(self.fsname)?];
        if self.debug {
            args.push(CString::new("-d")?);
        }

        let c_args: Vec<*const c_char> = args.iter().map(|arg| arg.as_ptr()).collect();
        let mut fargs = fuse_args {
            argc: c_args.len() as c_int,
            argv: c_args.as_ptr() as *mut *mut c_char,
            allocated: 0,
        };

        let fops = make_fuse_lowlevel_ops::<T>();

        let se = unsafe {
            fuse_session_new(
                &mut fargs, //
                &fops,
                mem::size_of_val(&fops),
                Box::into_raw(Box::new(ops)) as *mut _,
            )
        };
        if se.is_null() {
            unsafe { fuse_opt_free_args(&mut fargs) };
            return Err(io::ErrorKind::Other.into());
        }

        Ok(Session {
            se: unsafe { NonNull::new_unchecked(se) },
            args: fargs,
            set_signal_handlers: false,
            mountpoint: None,
        })
    }
}

pub struct Session {
    se: NonNull<fuse_session>,
    args: fuse_args,
    set_signal_handlers: bool,
    mountpoint: Option<PathBuf>,
}

impl Session {
    pub fn builder(fsname: impl Into<String>) -> Builder {
        Builder::new(fsname)
    }

    pub fn set_signal_handlers(&mut self) -> io::Result<()> {
        if !self.set_signal_handlers {
            let res = unsafe { fuse_set_signal_handlers(self.se.as_ptr()) };
            if res == -1 {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "failed to set signal handlers",
                ));
            }
        }
        self.set_signal_handlers = true;
        Ok(())
    }

    pub fn remove_signal_handlers(&mut self) {
        if self.set_signal_handlers {
            unsafe { fuse_remove_signal_handlers(self.se.as_ptr()) };
        }
        self.set_signal_handlers = false;
    }

    pub fn mount(&mut self, mountpoint: &Path) -> io::Result<()> {
        let c_mountpoint = CString::new(mountpoint.as_os_str().as_bytes())?;
        if unsafe { fuse_session_mount(self.se.as_ptr(), c_mountpoint.as_ptr()) } != 0 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("failed to mount to the specified path: {:?}", mountpoint),
            ));
        }
        self.mountpoint = Some(mountpoint.into());
        Ok(())
    }

    pub fn unmount(&mut self) {
        unsafe {
            if let Some(_) = self.mountpoint.take() {
                fuse_session_unmount(self.se.as_ptr());
            }
        }
    }

    pub fn run_loop(&mut self) -> io::Result<()> {
        if self.mountpoint.is_none() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "The session has not mounted yet.",
            ));
        }

        // Block until ctrl+c or fusermount -u
        unsafe {
            let mut buf = Buf(mem::zeroed());
            while fuse_session_exited(self.se.as_ptr()) == 0 {
                let res = fuse_session_receive_buf(self.se.as_ptr(), &mut buf.0);
                if res == -libc::EINTR {
                    continue;
                }
                if res <= 0 {
                    break;
                }

                fuse_session_process_buf(self.se.as_ptr(), &buf.0);
            }

            fuse_session_reset(self.se.as_ptr());
        }

        Ok(())
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        self.unmount();
        self.remove_signal_handlers();
        unsafe {
            fuse_session_destroy(self.se.as_ptr());
            fuse_opt_free_args(&mut self.args);
        }
    }
}

struct Buf(fuse_buf);

impl Drop for Buf {
    fn drop(&mut self) {
        unsafe {
            if !self.0.mem.is_null() {
                libc::free(self.0.mem);
            }
        }
    }
}
