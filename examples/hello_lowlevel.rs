use libc::{c_int, off_t, stat};
use libfuse::lowlevel::{Ino, OperationResult, Operations, Session};
use libfuse_sys::{
    fuse_entry_param, //
    fuse_file_info,
};
use std::{env, ffi::CStr, mem, path::PathBuf};

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
    fn lookup(&mut self, parent: Ino, name: &CStr) -> OperationResult<fuse_entry_param> {
        if parent != 1 {
            return Err(libc::ENOENT);
        }

        match name.to_str() {
            Ok(HELLO_NAME) => (),
            _ => return Err(libc::ENOENT),
        }

        let mut e = unsafe { mem::zeroed::<fuse_entry_param>() };
        e.ino = 2;
        e.attr_timeout = 1.0;
        e.entry_timeout = 1.0;
        let _ = hello_stat(e.ino, &mut e.attr);
        Ok(e)
    }

    fn getattr(&mut self, ino: Ino, _: *mut fuse_file_info) -> OperationResult<(stat, f64)> {
        let mut stbuf = unsafe { mem::zeroed::<stat>() };
        match hello_stat(ino, &mut stbuf) {
            Ok(()) => Ok((stbuf, 1.0)),
            Err(_) => Err(libc::ENOENT),
        }
    }

    fn open(&mut self, ino: Ino, fi: &mut fuse_file_info) -> OperationResult<()> {
        match (ino, fi.flags & libc::O_ACCMODE) {
            (2, libc::O_RDONLY) => Ok(()),
            (2, _) => Err(libc::EACCES),
            _ => Err(libc::EISDIR),
        }
    }

    fn read(
        &mut self,
        ino: Ino,
        buf: &mut [u8],
        off: off_t,
        _: *mut fuse_file_info,
    ) -> OperationResult<usize> {
        debug_assert!(ino == 2);
        debug_assert!(off >= 0);
        let off = off as usize;

        if off > HELLO_STR.len() {
            return Ok(0);
        }

        let to_be_read = std::cmp::min(buf.len(), HELLO_STR.len() - off);

        let src = HELLO_STR[off..off + to_be_read].as_bytes();
        buf[..to_be_read].copy_from_slice(src);

        Ok(src.len())
    }

    fn readdir(
        &mut self,
        ino: Ino,
        buf: &mut libfuse::lowlevel::DirBuf<'_>,
        _: *mut fuse_file_info,
    ) -> OperationResult<()> {
        if ino != 1 {
            return Err(libc::ENOTDIR);
        }

        buf.add(".", 1);
        buf.add("..", 1);
        buf.add(HELLO_NAME, 2);

        Ok(())
    }
}

fn hello_stat(ino: Ino, stbuf: &mut stat) -> Result<(), c_int> {
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
