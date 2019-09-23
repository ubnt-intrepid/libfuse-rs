use libc::{off_t as Offset, stat as Stat};
use libfuse::{
    file::{Entry, OpenOptions, ReadOptions, SetAttrs, WriteOptions},
    session::Builder,
    NodeId, OperationResult, Operations, ROOT_NODEID,
};
use std::{borrow::Cow, env, ffi::CStr, io, path::PathBuf};

fn main() -> io::Result<()> {
    let mountpoint = env::args()
        .nth(1)
        .map(PathBuf::from)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "missing mountpoint"))?;

    if !mountpoint.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "the mountpoint must be a regular file",
        ));
    }

    unsafe {
        libc::umask(0);
    }

    let mut session = Builder::new("null")
        .debug(true)
        .build(Null)
        .expect("failed to start fuse session");

    session.set_signal_handlers()?;
    session.mount(&mountpoint)?;
    session.run_loop()?;

    Ok(())
}

struct Null;

impl Null {
    fn root_attr(&self) -> Stat {
        let now = chrono::Local::now();
        let mut attr = unsafe { std::mem::zeroed::<Stat>() };
        attr.st_ino = ROOT_NODEID;
        attr.st_mode = libc::S_IFREG | 0o666;
        attr.st_nlink = 1;
        attr.st_uid = unsafe { libc::getuid() };
        attr.st_gid = unsafe { libc::getgid() };
        attr.st_size = 0;
        attr.st_blocks = 0;
        attr.st_atime = now.timestamp();
        attr.st_mtime = now.timestamp();
        attr.st_ctime = now.timestamp();
        attr
    }
}

impl Operations for Null {
    fn lookup(&mut self, _: NodeId, _: &CStr) -> OperationResult<Entry> {
        Err(libc::ENOENT)
    }

    fn getattr(&mut self, id: NodeId, _: Option<u64>) -> OperationResult<(Stat, f64)> {
        match id {
            ROOT_NODEID => Ok((self.root_attr(), 0.0)),
            _ => Err(libc::ENOENT),
        }
    }

    fn setattr(
        &mut self,
        id: NodeId,
        _: &SetAttrs,
        _: Option<u64>,
    ) -> OperationResult<(Stat, f64)> {
        match id {
            ROOT_NODEID => Ok((self.root_attr(), 0.0)),
            _ => Err(libc::ENOENT),
        }
    }

    fn open(&mut self, id: NodeId, _: &mut OpenOptions) -> OperationResult<u64> {
        if id != ROOT_NODEID {
            return Err(libc::ENOENT);
        }
        Ok(0)
    }

    fn read(
        &mut self,
        id: NodeId,
        _: Offset,
        _: usize,
        _: &mut ReadOptions,
        _: u64,
    ) -> OperationResult<Cow<[u8]>> {
        if id != ROOT_NODEID {
            return Err(libc::ENOENT);
        }
        Ok(Cow::Borrowed(&[]))
    }

    fn write(
        &mut self,
        id: NodeId,
        buf: &[u8],
        _: Offset,
        _: &mut WriteOptions,
        _: u64,
    ) -> OperationResult<usize> {
        if id != ROOT_NODEID {
            return Err(libc::ENOENT);
        }
        Ok(buf.len())
    }
}
