use super::ops::{assign_ops, Operations};
use libc::{c_char, c_int};
use libfuse_sys::{
    fuse_remove_signal_handlers, //
    fuse_session,
    fuse_session_destroy,
    fuse_session_fd,
    fuse_session_loop,
    fuse_session_mount,
    fuse_session_unmount,
    fuse_set_signal_handlers,
    helpers::{fuse_ll_ops_new, fuse_session_new_wrapped},
};
use std::{
    ffi::CString,
    io,
    marker::PhantomData,
    os::unix::ffi::OsStrExt,
    os::unix::io::RawFd,
    path::{Path, PathBuf},
    ptr::NonNull,
};

#[derive(Debug)]
pub struct Builder {
    fsname: String,
    options: Vec<String>,
}

impl Builder {
    pub fn new(fsname: impl Into<String>) -> Self {
        Self {
            fsname: fsname.into(),
            options: vec![],
        }
    }

    pub fn debug(self, enabled: bool) -> Self {
        if enabled {
            self.options(vec!["-o", "debug"])
        } else {
            self
        }
    }

    pub fn options(mut self, options: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.options.extend(options.into_iter().map(Into::into));
        self
    }

    /// Build a new `Session` using the specified filesystem operations.
    pub fn build<T: Operations>(self, ops: T) -> io::Result<Session<T>> {
        let mut args = vec![CString::new(self.fsname)?];
        args.extend(
            self.options
                .into_iter()
                .map(CString::new)
                .collect::<Result<Vec<_>, _>>()?,
        );

        let c_args: Vec<*const c_char> = args.iter().map(|arg| arg.as_ptr()).collect();
        let se;
        unsafe {
            let fops = fuse_ll_ops_new();
            if fops.is_null() {
                return Err(io::Error::from_raw_os_error(libc::ENOMEM));
            }
            assign_ops(&mut *fops, &ops);
            se = fuse_session_new_wrapped(
                c_args.len() as c_int,
                c_args.as_ptr(),
                fops,
                Box::into_raw(Box::new(ops)) as *mut _,
            );
            libc::free(fops as *mut _);
        };
        if se.is_null() {
            return Err(io::ErrorKind::Other.into());
        }

        Ok(Session {
            se: unsafe { NonNull::new_unchecked(se) },
            set_signal_handlers: false,
            mountpoint: None,
            _marker: PhantomData,
        })
    }
}

/// The session for operating a filesystem.
pub struct Session<T: Operations> {
    se: NonNull<fuse_session>,
    set_signal_handlers: bool,
    mountpoint: Option<PathBuf>,
    _marker: PhantomData<T>,
}

impl<T: Operations> Session<T> {
    /// Register the signal handlers that exits this session for HUP, TERM
    /// and INT signals.
    ///
    /// This method also disables the SIGPIPE signal handling preventing to
    /// terminate the process when the connection to `/dev/fuse` is lost.
    pub fn set_signal_handlers(&mut self) -> io::Result<()> {
        if !self.set_signal_handlers {
            let res = unsafe { fuse_set_signal_handlers(self.se.as_ptr()) };
            if res == -1 {
                return Err(io::Error::last_os_error());
            }
        }
        self.set_signal_handlers = true;
        Ok(())
    }

    /// Unregister the signal handlers.
    pub fn remove_signal_handlers(&mut self) {
        if self.set_signal_handlers {
            unsafe { fuse_remove_signal_handlers(self.se.as_ptr()) };
        }
        self.set_signal_handlers = false;
    }

    /// Mount this session to the specified mountpoint.
    pub fn mount(&mut self, mountpoint: impl AsRef<Path>) -> io::Result<()> {
        let mountpoint = mountpoint.as_ref().to_path_buf();

        let c_mountpoint = CString::new(mountpoint.as_os_str().as_bytes())?;
        let res = unsafe { fuse_session_mount(self.se.as_ptr(), c_mountpoint.as_ptr()) };
        if res != 0 {
            return Err(io::Error::last_os_error());
        }

        self.mountpoint = Some(mountpoint);

        Ok(())
    }

    pub fn mountpoint(&self) -> Option<&Path> {
        self.mountpoint.as_ref().map(|path| &**path)
    }

    pub fn unmount(&mut self) {
        unsafe {
            if let Some(_) = self.mountpoint.take() {
                fuse_session_unmount(self.se.as_ptr());
            }
        }
    }

    /// Returns the *raw* file descriptor for communication with the kernel.
    pub fn raw_fd(&self) -> Option<RawFd> {
        if self.mountpoint.is_some() {
            Some(unsafe { fuse_session_fd(self.se.as_ptr()) })
        } else {
            None
        }
    }

    /// Enter a single threaded, blocking event loop.
    ///
    /// When the event loop exits as a result of receiving a signal,
    /// this method returns the code of its signal.
    pub fn run_loop(&mut self) -> io::Result<c_int> {
        if self.mountpoint.is_none() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "The session has not mounted yet.",
            ));
        }
        let res = unsafe { fuse_session_loop(self.se.as_ptr()) };
        match res {
            0 => Ok(0),
            signo if signo > 0 => Ok(signo),
            n => Err(io::Error::from_raw_os_error(-n)),
        }
    }
}

impl<T: Operations> Drop for Session<T> {
    fn drop(&mut self) {
        self.unmount();
        self.remove_signal_handlers();
        unsafe {
            fuse_session_destroy(self.se.as_ptr());
        }
    }
}
