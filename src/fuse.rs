use crate::ops::Operations;
use libc::{c_char, c_void};
use libfuse_sys::fuse_operations;
use std::{ffi::CString, mem};

#[derive(Debug)]
pub struct Fuse {
    fsname: String,
    foreground: bool,
    threaded: bool,
    mountopts: Vec<String>,
}

impl Fuse {
    pub fn new(fsname: impl Into<String>) -> Self {
        Self {
            fsname: fsname.into(),
            foreground: false,
            threaded: true,
            mountopts: vec![],
        }
    }

    pub fn foreground(mut self, foreground: bool) -> Self {
        self.foreground = foreground;
        self
    }

    pub fn threaded(mut self, threaded: bool) -> Self {
        self.threaded = threaded;
        self
    }

    pub fn mountopts(mut self, opts: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.mountopts.extend(opts.into_iter().map(Into::into));
        self
    }

    pub fn mount<F: Operations>(self, mountpoint: impl AsRef<[u8]>, operations: F) -> ! {
        let Self {
            fsname,
            mountopts,
            foreground,
            threaded,
        } = self;

        let mut args = vec![
            CString::new(fsname).unwrap(),
            CString::new(mountpoint.as_ref()).unwrap(),
        ];

        if !mountopts.is_empty() {
            let mut s = String::from("-o");
            for (i, opt) in mountopts.into_iter().enumerate() {
                if i > 0 {
                    s.push(',');
                }
                s.push_str(&opt);
            }
            args.push(CString::new(s).unwrap());
        }

        if foreground {
            args.push(CString::new("-f").unwrap());
        }

        if !threaded {
            args.push(CString::new("-s").unwrap());
        }
        log::debug!("fuse arguments={:?}", args);

        let mut c_args: Vec<*const c_char> = args.iter().map(|a| a.as_ptr()).collect();

        let ops = crate::ops::make_fuse_operations::<F>();

        std::process::exit(unsafe {
            libfuse_sys::fuse_main_real(
                c_args.len() as i32,
                c_args.as_mut_ptr() as *mut *mut c_char,
                &ops,
                mem::size_of::<fuse_operations>(),
                Box::into_raw(Box::new(operations)) as *mut c_void,
            )
        })
    }
}
