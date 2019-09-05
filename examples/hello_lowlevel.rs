use libc::{c_char, c_int, off_t, stat};
use libfuse_sys::*;
use std::{
    ffi::{CStr, CString},
    mem, ptr,
};

const hello_str: &str = "Hello World!\n";
const hello_name: &str = "hello";

fn main() {
    std::process::exit({
        let args: Vec<CString> = std::env::args()
            .map(CString::new)
            .collect::<Result<_, _>>()
            .unwrap();
        let c_args: Vec<*const c_char> = args.iter().map(|s| s.as_ptr()).collect();
        unsafe { c_main(c_args.len() as c_int, c_args.as_ptr()) }
    })
}

unsafe fn c_main(argc: c_int, argv: *const *const c_char) -> c_int {
    let mut ops = mem::zeroed::<fuse_lowlevel_ops>();
    ops.getattr = Some(hello_ll_getattr);
    ops.lookup = Some(hello_ll_lookup);
    ops.readdir = Some(hello_ll_readdir);
    ops.open = Some(hello_ll_open);
    ops.read = Some(hello_ll_read);

    let mut args = fuse_args {
        argc: argc,
        argv: argv as *mut *mut c_char,
        allocated: 0,
    };

    let mut opts = mem::zeroed::<fuse_cmdline_opts>();

    if fuse_parse_cmdline(&mut args, &mut opts) != 0 {
        return 1;
    }

    if opts.show_help != 0 {
        let fsname = CStr::from_ptr(*argv.offset(0));
        println!("usage: {:?} [options] <mountpoint>", fsname);
        println!();
        fuse_cmdline_help();
        fuse_lowlevel_help();
        libc::free(opts.mountpoint as *mut _);
        fuse_opt_free_args(&mut args);
        return 0;
    }

    if opts.show_version != 0 {
        let pkgversion = CStr::from_ptr(fuse_pkgversion());
        println!("FUSE library version {:?}", pkgversion,);
        fuse_lowlevel_version();
        libc::free(opts.mountpoint as *mut _);
        fuse_opt_free_args(&mut args);
        return 0;
    }

    if opts.mountpoint.is_null() {
        let fsname = CStr::from_ptr(*argv.offset(0));
        println!("usage: {:?} [options] <mountpoint>", fsname);
        println!("       {:?} --help\n", fsname);
        libc::free(opts.mountpoint as *mut _);
        fuse_opt_free_args(&mut args);
        return 1;
    }

    let se = fuse_session_new(&mut args, &ops, mem::size_of_val(&ops), ptr::null_mut());
    if se.is_null() {
        libc::free(opts.mountpoint as *mut _);
        fuse_opt_free_args(&mut args);
        return 1;
    }

    if fuse_set_signal_handlers(se) != 0 {
        fuse_session_destroy(se);
        libc::free(opts.mountpoint as *mut _);
        fuse_opt_free_args(&mut args);
        return 1;
    }

    if fuse_session_mount(se, opts.mountpoint) != 0 {
        fuse_remove_signal_handlers(se);
        fuse_session_destroy(se);
        libc::free(opts.mountpoint as *mut _);
        fuse_opt_free_args(&mut args);
        return 1;
    }

    fuse_daemonize(opts.foreground);

    // Block until ctrl+c or fusermount -u
    let ret = if opts.singlethread != 0 {
        fuse_session_loop(se)
    } else {
        fuse_session_loop_mt_31(se, opts.clone_fd)
    };

    fuse_session_unmount(se);
    fuse_remove_signal_handlers(se);
    fuse_session_destroy(se);
    libc::free(opts.mountpoint as *mut _);
    fuse_opt_free_args(&mut args);

    if ret != 0 {
        1
    } else {
        0
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
            stbuf.st_size = hello_str.len() as i64;
        }
        _ => return Err(libc::ENOENT),
    }
    Ok(())
}

unsafe fn reply_buf_limited(
    req: fuse_req_t,
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

unsafe extern "C" fn hello_ll_getattr(req: fuse_req_t, ino: fuse_ino_t, _: *mut fuse_file_info) {
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

unsafe extern "C" fn hello_ll_lookup(req: fuse_req_t, parent: fuse_ino_t, name: *const c_char) {
    let name = CStr::from_ptr(name);

    if parent != 1 {
        fuse_reply_err(req, libc::ENOENT);
        return;
    }

    match name.to_str() {
        Ok(s) if s == hello_name => (),
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

#[repr(C)]
struct DirBuf {
    p: *mut c_char,
    size: usize,
}

impl DirBuf {
    unsafe fn add(&mut self, req: fuse_req_t, name: &str, ino: fuse_ino_t) {
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

unsafe extern "C" fn hello_ll_readdir(
    req: fuse_req_t,
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
    b.add(req, hello_name, 2);

    debug_assert!(off >= 0);
    reply_buf_limited(req, b.p, b.size, off, size);

    fuse_reply_err(req, libc::ENOSYS);
}

unsafe extern "C" fn hello_ll_open(req: fuse_req_t, ino: fuse_ino_t, fi: *mut fuse_file_info) {
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

unsafe extern "C" fn hello_ll_read(
    req: fuse_req_t,
    ino: fuse_ino_t,
    size: usize,
    off: off_t,
    _: *mut fuse_file_info,
) {
    debug_assert!(ino == 2);
    debug_assert!(off >= 0);

    let hello_cstr = CString::new(hello_str).unwrap();
    reply_buf_limited(req, hello_cstr.as_ptr(), hello_str.len(), off, size);
}
