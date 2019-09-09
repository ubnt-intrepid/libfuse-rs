use libc::{c_int, off_t, stat};
use libfuse::{
    dir::DirBuf, file::Entry, DirOperations, FileOperations, Ino, OperationResult, Operations,
    Session,
};
use libfuse_sys::fuse_file_info;
use std::{
    env,
    ffi::{CStr, CString},
    mem,
    path::PathBuf,
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
    type File = HelloFile;
    type Dir = HelloDir;

    fn lookup(&mut self, parent: Ino, name: &CStr) -> OperationResult<Entry> {
        if parent != 1 {
            return Err(libc::ENOENT);
        }

        match name.to_str() {
            Ok(HELLO_NAME) => (),
            _ => return Err(libc::ENOENT),
        }

        let e = Entry::new(2)
            .attr(hello_stat(2)?)
            .attr_timeout(1.0)
            .entry_timeout(1.0);
        Ok(e)
    }

    fn getattr(&mut self, ino: Ino) -> OperationResult<(stat, f64)> {
        match hello_stat(ino) {
            Ok(stat) => Ok((stat, 1.0)),
            Err(_) => Err(libc::ENOENT),
        }
    }

    fn open(&mut self, ino: Ino, fi: &mut fuse_file_info) -> OperationResult<Self::File> {
        match (ino, fi.flags & libc::O_ACCMODE) {
            (2, libc::O_RDONLY) => Ok(HelloFile),
            (2, _) => Err(libc::EACCES),
            _ => Err(libc::EISDIR),
        }
    }

    fn opendir(&mut self, _: Ino, _: &mut fuse_file_info) -> OperationResult<Self::Dir> {
        Ok(HelloDir)
    }
}

struct HelloFile;

impl FileOperations for HelloFile {
    type Ops = Hello;

    fn read(
        &mut self,
        _: &mut Self::Ops,
        ino: Ino,
        buf: &mut [u8],
        off: off_t,
        _: &mut fuse_file_info,
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
}

struct HelloDir;

impl DirOperations for HelloDir {
    type Ops = Hello;

    fn readdir(
        &mut self,
        _: &mut Self::Ops,
        ino: Ino,
        offset: off_t,
        buf: &mut DirBuf<'_>,
    ) -> OperationResult<()> {
        if ino != 1 {
            return Err(libc::ENOTDIR);
        }

        if offset == 0 {
            let name = CString::new(HELLO_NAME).expect("valid filename");
            let attr = hello_stat(2)?;
            let hello_offset = 1;
            buf.add(&*name, &attr, hello_offset);
        }

        Ok(())
    }
}

fn hello_stat(ino: Ino) -> Result<stat, c_int> {
    let mut stbuf = unsafe { mem::zeroed::<stat>() };
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
    Ok(stbuf)
}
