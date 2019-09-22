use libc::{c_int, off_t, stat};
use libfuse::{
    dir::DirBuf,
    file::{Entry, OpenOptions, ReadOptions},
    session::Builder,
    NodeId, OperationResult, Operations, ROOT_NODEID,
};
use std::{
    borrow::Cow,
    env,
    ffi::{CStr, CString},
    mem,
    path::PathBuf,
};

const HELLO_STR: &str = "Hello World!\n";
const HELLO_NAME: &str = "hello";
const HELLO_NODEID: NodeId = 2;

fn main() {
    let mountpoint = env::args()
        .nth(1)
        .map(PathBuf::from)
        .expect("requires the mountpoint path");

    let mut session = Builder::new("hello")
        .debug(true)
        .build(Hello)
        .expect("failed to start fuse session");

    session.set_signal_handlers().unwrap();
    session.mount(&mountpoint).unwrap();
    session.run_loop().unwrap();
}

struct Hello;

impl Operations for Hello {
    fn lookup(&mut self, parent: NodeId, name: &CStr) -> OperationResult<Entry> {
        if parent != ROOT_NODEID {
            return Err(libc::ENOENT);
        }

        match name.to_str() {
            Ok(HELLO_NAME) => (),
            _ => return Err(libc::ENOENT),
        }

        Ok(Entry {
            nodeid: HELLO_NODEID,
            attr: hello_stat(HELLO_NODEID)?,
            attr_timeout: 1.0,
            entry_timeout: 1.0,
            ..Entry::default()
        })
    }

    fn getattr(&mut self, id: NodeId, _: Option<u64>) -> OperationResult<(stat, f64)> {
        match hello_stat(id) {
            Ok(stat) => Ok((stat, 1.0)),
            Err(_) => Err(libc::ENOENT),
        }
    }

    fn open(&mut self, id: NodeId, opts: &mut OpenOptions) -> OperationResult<u64> {
        match (id, opts.flags() & libc::O_ACCMODE) {
            (HELLO_NODEID, libc::O_RDONLY) => Ok(0),
            (HELLO_NODEID, _) => Err(libc::EACCES),
            _ => Err(libc::EISDIR),
        }
    }

    fn read(
        &mut self,
        id: NodeId,
        off: off_t,
        _: usize,
        _: &mut ReadOptions<'_>,
        _: u64,
    ) -> OperationResult<Cow<'_, [u8]>> {
        debug_assert!(id == HELLO_NODEID);
        debug_assert!(off >= 0);
        let off = off as usize;

        if off > HELLO_STR.len() {
            return Ok(Cow::Borrowed(&[]));
        }

        Ok(HELLO_STR[off..].as_bytes().into())
    }

    fn readdir(
        &mut self,
        id: NodeId,
        offset: off_t,
        buf: &mut DirBuf<'_>,
        _: u64,
    ) -> OperationResult<()> {
        if id != ROOT_NODEID {
            return Err(libc::ENOTDIR);
        }

        if offset == 0 {
            let name = CString::new(HELLO_NAME).expect("valid filename");
            let attr = hello_stat(HELLO_NODEID)?;
            let hello_offset = 1;
            buf.add(&*name, &attr, hello_offset);
        }

        Ok(())
    }
}

fn hello_stat(ino: NodeId) -> Result<stat, c_int> {
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
