use crate::{
    common::{ConnectionInfo, Ino},
    file::Entry,
    ops::{OperationResult, Operations},
};
use libc::stat;
use std::ffi::{CStr, CString};

pub trait FSOperations {
    fn init(&mut self, conn: &mut ConnectionInfo);
    fn getattr(&mut self, path: &CStr, fh: Option<u64>) -> OperationResult<stat>;
}

#[derive(Debug)]
struct Resolver(());

impl Resolver {
    /// Look up an inode by name and get its attribute.
    fn lookup(&mut self, parent: Ino, name: &CStr) -> Option<Ino> {
        None
    }

    /// Get the path of specified inode.
    fn get_path(&self, ino: Ino) -> Option<CString> {
        None
    }
}

#[derive(Debug)]
pub struct FS<T: FSOperations> {
    resolver: Resolver,
    attr_timeout: f64,
    ops: T,
}

impl<T: FSOperations> Operations for FS<T> {
    fn init(&mut self, conn: &mut ConnectionInfo) {
        self.ops.init(conn);
    }

    fn lookup(&mut self, parent: Ino, name: &CStr) -> OperationResult<Entry> {
        self.resolver
            .lookup(parent, name)
            .map(|ino| {
                let mut e = Entry::new();
                e.ino(ino);
                e
            })
            .ok_or_else(|| libc::ENOENT)
    }

    fn getattr(&mut self, ino: Ino, fh: Option<u64>) -> OperationResult<(stat, f64)> {
        let path = self.resolver.get_path(ino).ok_or_else(|| libc::ENOENT)?;
        self.ops
            .getattr(&*path, fh)
            .map(|attr| (attr, self.attr_timeout))
    }
}
