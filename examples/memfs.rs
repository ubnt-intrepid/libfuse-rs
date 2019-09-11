use chrono::Local;
use libc::{c_int, c_uint, dev_t, mode_t, off_t, stat, statvfs};
use libfuse::{
    dir::DirBuf,
    file::{Entry, ReadOptions, SetAttrs, WriteOptions},
    session::Builder,
    Ino, OperationResult, Operations,
};
use std::{
    collections::hash_map::{Entry as MapEntry, HashMap},
    ffi::{CStr, CString},
    io,
    path::PathBuf,
};
use structopt::StructOpt;

const ROOT_INO: Ino = 1;

#[derive(Debug, StructOpt)]
struct Args {
    mountpoint: PathBuf,
}

fn main() -> io::Result<()> {
    let args = Args::from_args();

    let mut memfs = MemFs::new();

    let now = Local::now();
    let data: Vec<u8> = "Hello, world!\n".into();
    let data_size = data.len();
    memfs
        .insert_inode(
            ROOT_INO,
            "hello".into(),
            INode::File(File {
                data,
                attr: {
                    let mut attr: stat = unsafe { std::mem::zeroed() };
                    attr.st_nlink = 1;
                    attr.st_size = data_size as i64;
                    attr.st_ctime = now.timestamp();
                    attr.st_mtime = now.timestamp();
                    attr.st_mode = libc::S_IFREG | 0666;
                    attr.st_uid = unsafe { libc::getuid() };
                    attr.st_gid = unsafe { libc::getgid() };
                    attr
                },
            }),
        )
        .unwrap();

    let mut session = Builder::new("memfs") //
        .debug(true)
        .build(memfs)?;
    session.set_signal_handlers()?;
    session.mount(&args.mountpoint)?;
    session.run_loop()?;

    Ok(())
}

struct MemFs {
    inodes: HashMap<Ino, INode>,
    next_id: Ino,
}

impl MemFs {
    fn new() -> Self {
        let mut inodes = HashMap::new();

        let now = Local::now();
        inodes.insert(
            ROOT_INO,
            INode::Dir(Dir {
                parent: None,
                children: HashMap::new(),
                attr: {
                    let mut attr: stat = unsafe { std::mem::zeroed() };
                    attr.st_ino = ROOT_INO;
                    attr.st_nlink = 2;
                    attr.st_ctime = now.timestamp();
                    attr.st_mtime = now.timestamp();
                    attr.st_atime = now.timestamp();
                    attr.st_mode = libc::S_IFDIR | 0777;
                    attr.st_uid = unsafe { libc::getuid() };
                    attr.st_gid = unsafe { libc::getgid() };
                    attr
                },
            }),
        );

        Self {
            inodes,
            next_id: 2, // inode 1 is reserved for the root directory.
        }
    }

    fn insert_inode(
        &mut self,
        parent: Ino,
        name: String,
        mut inode: INode,
    ) -> OperationResult<Ino> {
        let parent = self.inodes.get_mut(&parent).ok_or_else(|| libc::ENOENT)?;

        let ino = self.next_id;

        let dir = parent.as_dir_mut().ok_or_else(|| libc::ENOTDIR)?;
        match dir.children.entry(name.into()) {
            MapEntry::Occupied(..) => return Err(libc::EEXIST),
            MapEntry::Vacant(entry) => {
                entry.insert(ino);
            }
        }

        match self.inodes.entry(self.next_id) {
            MapEntry::Occupied(..) => Err(libc::EEXIST),
            MapEntry::Vacant(entry) => {
                inode.attr_mut().st_ino = ino;
                entry.insert(inode);
                self.next_id += 1;
                Ok(ino)
            }
        }
    }

    fn remove_inode(&mut self, parent: Ino, name: String) -> OperationResult<()> {
        let parent = self.inodes.get_mut(&parent).ok_or_else(|| libc::ENOENT)?;
        let ino = parent
            .as_dir_mut()
            .ok_or_else(|| libc::ENOTDIR)?
            .children
            .remove(&name);

        if let Some(ino) = ino {
            self.inodes.remove(&ino);
        }

        Ok(())
    }
}

impl Operations for MemFs {
    fn lookup(&mut self, parent: Ino, name: &CStr) -> OperationResult<Entry> {
        let name = name.to_str().map_err(|_| libc::EIO)?;

        let parent = self.inodes.get(&parent).ok_or_else(|| libc::ENOENT)?;
        let parent = parent.as_dir().ok_or_else(|| libc::ENOTDIR)?;

        let child = parent.children.get(name).ok_or_else(|| libc::ENOENT)?;
        let child = self.inodes.get(child).ok_or_else(|| libc::ENOENT)?;

        let mut e = Entry::default();
        e.ino(child.attr().st_ino);
        e.attr(child.attr().clone());
        e.attr_timeout(0.0);
        e.entry_timeout(0.0);

        Ok(e)
    }

    fn mknod(
        &mut self,
        parent: Ino,
        name: &CStr,
        mode: mode_t,
        _: dev_t,
    ) -> OperationResult<Entry> {
        let name = name.to_str().map_err(|_| libc::EIO)?;
        let now = Local::now();

        match mode & libc::S_IFMT {
            libc::S_IFREG => (),
            _ => return Err(libc::ENOTSUP),
        }

        let ino = self.insert_inode(
            parent,
            name.into(),
            INode::File(File {
                data: vec![],
                attr: {
                    let mut attr: stat = unsafe { std::mem::zeroed() };
                    attr.st_ino = self.next_id;
                    attr.st_nlink = 1;
                    attr.st_ctime = now.timestamp();
                    attr.st_mtime = now.timestamp();
                    attr.st_mode = mode;
                    attr.st_uid = unsafe { libc::getuid() };
                    attr.st_gid = unsafe { libc::getgid() };
                    attr
                },
            }),
        )?;
        let inode = self.inodes.get(&ino).unwrap();

        let mut e = Entry::new();
        e.ino(inode.attr().st_ino);
        e.attr(inode.attr().clone());
        e.attr_timeout(0.0);
        e.entry_timeout(0.0);

        Ok(e)
    }

    fn mkdir(&mut self, parent: Ino, name: &CStr, mode: mode_t) -> OperationResult<Entry> {
        let name = name.to_str().map_err(|_| libc::EIO)?;
        let now = Local::now();

        let ino = self.insert_inode(
            parent,
            name.into(),
            INode::Dir(Dir {
                parent: Some(parent),
                children: HashMap::new(),
                attr: {
                    let mut attr: stat = unsafe { std::mem::zeroed() };
                    attr.st_ino = self.next_id;
                    attr.st_nlink = 1;
                    attr.st_ctime = now.timestamp();
                    attr.st_mtime = now.timestamp();
                    attr.st_mode = mode | libc::S_IFDIR;
                    attr.st_uid = unsafe { libc::getuid() };
                    attr.st_gid = unsafe { libc::getgid() };
                    attr
                },
            }),
        )?;
        let inode = self.inodes.get(&ino).unwrap();

        let mut e = Entry::new();
        e.ino(inode.attr().st_ino);
        e.attr(inode.attr().clone());
        e.attr_timeout(0.0);
        e.entry_timeout(0.0);

        Ok(e)
    }

    fn unlink(&mut self, parent: Ino, name: &CStr) -> OperationResult<()> {
        let name = name.to_str().map_err(|_| libc::EIO)?;
        self.remove_inode(parent, name.into())
    }

    fn rmdir(&mut self, parent: Ino, name: &CStr) -> OperationResult<()> {
        let name = name.to_str().map_err(|_| libc::EIO)?;
        self.remove_inode(parent, name.into())
    }

    fn rename(
        &mut self,
        oldparent: Ino,
        oldname: &CStr,
        newparent: Ino,
        newname: &CStr,
        flags: c_uint,
    ) -> OperationResult<()> {
        let noreplace = (flags as c_int & libc::RENAME_NOREPLACE) != 0;
        if (flags as c_int & libc::RENAME_EXCHANGE) != 0 {
            return Err(libc::ENOTSUP);
        }

        let oldname = oldname.to_str().map_err(|_| libc::EIO)?;
        let newname = newname.to_str().map_err(|_| libc::EIO)?;

        // check if the destination has already exist.
        if noreplace {
            let newparent = self.inodes.get(&newparent).ok_or_else(|| libc::ENOENT)?;
            let newparent = newparent.as_dir().ok_or_else(|| libc::ENOTDIR)?;
            if newparent.children.contains_key(newname) {
                return Err(libc::EEXIST);
            }
        }

        let oldparent = self
            .inodes
            .get_mut(&oldparent)
            .ok_or_else(|| libc::ENOENT)?;
        let oldparent = oldparent.as_dir_mut().ok_or_else(|| libc::ENOTDIR)?;
        let ino = oldparent
            .children
            .remove(oldname)
            .ok_or_else(|| libc::ENOENT)?;

        let oldino = {
            let newparent = self
                .inodes
                .get_mut(&newparent)
                .ok_or_else(|| libc::ENOENT)?;
            let newparent = newparent.as_dir_mut().ok_or_else(|| libc::ENOTDIR)?;
            newparent.children.insert(newname.into(), ino)
        };

        if let Some(oldino) = oldino {
            self.inodes.remove(&oldino);
        }

        Ok(())
    }

    // TODO: symlink, readlink, forget

    fn statfs(&mut self, _: Ino) -> OperationResult<statvfs> {
        let mut st: statvfs = unsafe { std::mem::zeroed() };
        st.f_files = self.inodes.len() as u64;
        Ok(st)
    }

    fn read(
        &mut self,
        ino: Ino,
        buf: &mut [u8],
        offset: off_t,
        _: &mut ReadOptions,
        _: u64,
    ) -> OperationResult<usize> {
        let file = self.inodes.get(&ino).ok_or_else(|| libc::ENOENT)?;
        let file = file.as_file().ok_or_else(|| libc::EISDIR)?;

        debug_assert!(offset >= 0);
        let offset = offset as usize;

        if offset >= file.data.len() {
            return Ok(0);
        }

        let len = std::cmp::min(file.data.len() - offset, buf.len());
        let out = &file.data[offset..offset + len];
        buf[..len].copy_from_slice(out);

        Ok(len)
    }

    fn write(
        &mut self,
        ino: Ino,
        buf: &[u8],
        offset: off_t,
        _: &mut WriteOptions,
        _: u64,
    ) -> OperationResult<usize> {
        let file = self.inodes.get_mut(&ino).ok_or_else(|| libc::ENOENT)?;
        let file = file.as_file_mut().ok_or_else(|| libc::EISDIR)?;

        debug_assert!(offset >= 0);
        let offset = offset as usize;

        file.resize_data(offset + buf.len());

        let out = &mut file.data[offset..offset + buf.len()];
        out.copy_from_slice(buf);

        Ok(buf.len())
    }

    fn readdir(
        &mut self,
        ino: Ino,
        offset: off_t,
        buf: &mut DirBuf,
        _: u64,
    ) -> OperationResult<()> {
        let dir = self.inodes.get(&ino).ok_or_else(|| libc::ENOENT)?;
        let dir = dir.as_dir().ok_or_else(|| libc::ENOTDIR)?;

        for (i, (name, ino)) in dir.dirs(ino).enumerate().skip(offset as usize) {
            let name = CString::new(name).map_err(|_| libc::EIO)?;
            let attr = match ino {
                ROOT_INO => {
                    let mut attr: stat = unsafe { std::mem::zeroed() };
                    attr.st_ino = ROOT_INO;
                    attr
                }
                ino => {
                    let inode = self.inodes.get(&ino).ok_or_else(|| libc::ENOENT)?;
                    inode.attr().clone()
                }
            };
            let off = (i + 1) as off_t;

            if buf.add(&*name, &attr, off) {
                break;
            }
        }

        Ok(())
    }

    fn getattr(&mut self, ino: Ino, _: Option<u64>) -> OperationResult<(stat, f64)> {
        let inode = self.inodes.get(&ino).ok_or_else(|| libc::ENOENT)?;
        Ok((inode.attr().clone(), 0.0))
    }

    fn setattr(
        &mut self,
        ino: Ino,
        attrs: &SetAttrs<'_>,
        _: Option<u64>,
    ) -> OperationResult<(stat, f64)> {
        let inode = self.inodes.get_mut(&ino).ok_or_else(|| libc::ENOENT)?;
        let now = Local::now();

        if let Some(mode) = attrs.mode() {
            inode.attr_mut().st_mode = mode;
        }

        if let Some(uid) = attrs.uid() {
            inode.attr_mut().st_uid = uid;
        }

        if let Some(gid) = attrs.gid() {
            inode.attr_mut().st_gid = gid;
        }

        if let Some(size) = attrs.size() {
            match inode {
                INode::Dir(..) => return Err(libc::EISDIR),
                INode::File(ref mut file) => file.resize_data(size as usize),
            }
        }

        if let Some(mut ts) = attrs.atime() {
            if ts.tv_nsec == libc::UTIME_NOW {
                ts.tv_sec = now.timestamp();
                ts.tv_nsec = now.timestamp_subsec_nanos() as i64;
            }
            inode.attr_mut().st_atime = ts.tv_sec;
            inode.attr_mut().st_atime_nsec = ts.tv_nsec;
        }

        if let Some(mut ts) = attrs.mtime() {
            if ts.tv_nsec == libc::UTIME_NOW {
                ts.tv_sec = now.timestamp();
                ts.tv_nsec = now.timestamp_subsec_nanos() as i64;
            }
            inode.attr_mut().st_mtime = ts.tv_sec;
            inode.attr_mut().st_mtime_nsec = ts.tv_nsec;
        }

        if let Some(mut ts) = attrs.ctime() {
            if ts.tv_nsec == libc::UTIME_NOW {
                ts.tv_sec = now.timestamp();
                ts.tv_nsec = now.timestamp_subsec_nanos() as i64;
            }
            inode.attr_mut().st_ctime = ts.tv_sec;
            inode.attr_mut().st_ctime_nsec = ts.tv_nsec;
        }

        Ok((inode.attr().clone(), 0.0))
    }
}

enum INode {
    File(File),
    Dir(Dir),
}

impl INode {
    fn as_file(&self) -> Option<&File> {
        match self {
            INode::File(ref file) => Some(file),
            _ => None,
        }
    }

    fn as_file_mut(&mut self) -> Option<&mut File> {
        match self {
            INode::File(ref mut file) => Some(file),
            _ => None,
        }
    }

    fn as_dir(&self) -> Option<&Dir> {
        match self {
            INode::Dir(ref dir) => Some(dir),
            _ => None,
        }
    }

    fn as_dir_mut(&mut self) -> Option<&mut Dir> {
        match self {
            INode::Dir(ref mut dir) => Some(dir),
            _ => None,
        }
    }

    fn attr(&self) -> &stat {
        match self {
            INode::File(ref file) => &file.attr,
            INode::Dir(ref dir) => &dir.attr,
        }
    }

    fn attr_mut(&mut self) -> &mut stat {
        match self {
            INode::File(ref mut file) => &mut file.attr,
            INode::Dir(ref mut dir) => &mut dir.attr,
        }
    }
}

struct File {
    data: Vec<u8>,
    attr: stat,
}

impl File {
    fn resize_data(&mut self, new_len: usize) {
        self.data.resize(new_len, 0);
        self.attr.st_size = new_len as i64;
    }
}

struct Dir {
    parent: Option<Ino>,
    children: HashMap<String, Ino>,
    attr: stat,
}

impl Dir {
    fn dirs(&self, ino: Ino) -> impl Iterator<Item = (&str, Ino)> {
        Some((".", ino))
            .into_iter()
            .chain(
                self.parent
                    .or_else(|| Some(ROOT_INO))
                    .map(|ino| ("..", ino)),
            )
            .chain(
                self.children
                    .iter()
                    .map(|(name, &ino)| (name.as_str(), ino)),
            )
    }
}
