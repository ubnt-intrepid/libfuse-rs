use libc::{c_char, c_int, off_t, stat};
use libfuse::lowlevel::{Operations, Session};
use libfuse_sys::{
    fuse_add_direntry, //
    fuse_entry_param,
    fuse_file_info,
    fuse_ino_t,
    fuse_reply_attr,
    fuse_reply_buf,
    fuse_reply_entry,
    fuse_reply_err,
    fuse_reply_open,
    fuse_req,
};
use std::{
    env,
    ffi::{CStr, CString},
    mem,
    path::PathBuf,
    ptr,
};

const HELLO_STR: &str = "Hello World!\n";
const HELLO_NAME: &str = "hello";

fn main() {
    let mountpoint = env::args()
        .nth(1)
        .map(PathBuf::from)
        .expect("requires the mountpoint path");

    let session = Session::builder("hello")
        .debug(true)
        .set_signal_handlers(true)
        .build(Hello)
        .expect("failed to start fuse session");

    session
        .run(&mountpoint)
        .expect("error during the session loop");
}

struct Hello;

impl Operations for Hello {
    unsafe fn getattr(&mut self, req: &mut fuse_req, ino: fuse_ino_t, _: *mut fuse_file_info) {
        let mut stbuf = mem::zeroed::<stat>();
        match hello_stat(ino, &mut stbuf) {
            Ok(()) => {
                fuse_reply_attr(req, &stbuf, 1.0);
            }
            Err(_) => {
                fuse_reply_err(req, libc::ENOENT);
            }
        }
    }

    unsafe fn lookup(&mut self, req: &mut fuse_req, parent: fuse_ino_t, name: *const c_char) {
        let name = CStr::from_ptr(name);

        if parent != 1 {
            fuse_reply_err(req, libc::ENOENT);
            return;
        }

        match name.to_str() {
            Ok(s) if s == HELLO_NAME => (),
            _ => {
                fuse_reply_err(req, libc::ENOENT);
                return;
            }
        }

        let mut e = mem::zeroed::<fuse_entry_param>();
        e.ino = 2;
        e.attr_timeout = 1.0;
        e.entry_timeout = 1.0;
        let _ = hello_stat(e.ino, &mut e.attr);

        fuse_reply_entry(req, &e);
    }

    unsafe fn readdir(
        &mut self,
        req: &mut fuse_req,
        ino: fuse_ino_t,
        size: usize,
        off: off_t,
        _: *mut fuse_file_info,
    ) {
        if ino != 1 {
            fuse_reply_err(req, libc::ENOTDIR);
            return;
        }

        let mut b = DirBuf {
            p: ptr::null_mut(),
            size: 0,
        };
        b.add(req, ".", 1);
        b.add(req, "..", 1);
        b.add(req, HELLO_NAME, 2);

        debug_assert!(off >= 0);
        reply_buf_limited(req, b.p, b.size, off, size);
    }

    unsafe fn open(&mut self, req: &mut fuse_req, ino: fuse_ino_t, fi: *mut fuse_file_info) {
        debug_assert!(!fi.is_null());
        let fi = &mut *fi;

        if ino != 2 {
            fuse_reply_err(req, libc::EISDIR);
            return;
        }

        match fi.flags & libc::O_ACCMODE {
            libc::O_RDONLY => {
                fuse_reply_open(req, fi);
            }
            _ => {
                fuse_reply_err(req, libc::EACCES);
            }
        }
    }

    unsafe fn read(
        &mut self,
        req: &mut fuse_req,
        ino: fuse_ino_t,
        size: usize,
        off: off_t,
        _: *mut fuse_file_info,
    ) {
        debug_assert!(ino == 2);
        debug_assert!(off >= 0);

        let hello_cstr = CString::new(HELLO_STR).unwrap();
        reply_buf_limited(req, hello_cstr.as_ptr(), HELLO_STR.len(), off, size);
    }
}

unsafe fn hello_stat(ino: fuse_ino_t, stbuf: &mut stat) -> Result<(), c_int> {
    stbuf.st_ino = ino;
    match ino {
        1 => {
            stbuf.st_mode = libc::S_IFDIR | 0755;
            stbuf.st_nlink = 2;
        }
        2 => {
            stbuf.st_mode = libc::S_IFREG | 0444;
            stbuf.st_nlink = 1;
            stbuf.st_size = HELLO_STR.len() as i64;
        }
        _ => return Err(libc::ENOENT),
    }
    Ok(())
}

unsafe fn reply_buf_limited(
    req: &mut fuse_req,
    buf: *const c_char,
    bufsize: usize,
    off: off_t,
    maxsize: usize,
) -> c_int {
    if (off as usize) < bufsize {
        fuse_reply_buf(
            req,
            buf.offset(off as isize),
            std::cmp::min(bufsize - off as usize, maxsize),
        )
    } else {
        fuse_reply_buf(req, ptr::null(), 0)
    }
}

struct DirBuf {
    p: *mut c_char,
    size: usize,
}

impl Drop for DirBuf {
    fn drop(&mut self) {
        unsafe {
            if !self.p.is_null() {
                libc::free(self.p as *mut _);
            }
            self.p = ptr::null_mut();
        }
    }
}

impl DirBuf {
    unsafe fn add(&mut self, req: &mut fuse_req, name: &str, ino: fuse_ino_t) {
        let name = CString::new(name).unwrap();

        let oldsize = self.size;
        self.size += fuse_add_direntry(req, ptr::null_mut(), 0, name.as_ptr(), ptr::null(), 0);
        self.p = libc::realloc(self.p as *mut _, self.size) as *mut c_char;

        let mut stbuf = mem::zeroed::<stat>();
        stbuf.st_ino = ino;

        fuse_add_direntry(
            req,
            self.p.offset(oldsize as isize),
            self.size - oldsize,
            name.as_ptr(),
            &stbuf,
            self.size as i64,
        );
    }
}
