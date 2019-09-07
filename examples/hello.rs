use libc::{off_t, stat};
use libfuse::{
    dir::{FillDir, ReadDirFlags},
    Config, ConnInfo, FileInfo, Fuse, Operations, Result,
};
use std::ffi::{CStr, CString};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(short = "f", long = "filename", default_value = "hello")]
    filename: String,

    #[structopt(short = "c", long = "contents", default_value = "Hello, World!\n")]
    contents: String,

    #[structopt(name = "mountpoint")]
    mountpoint: String,
}

fn main() {
    std::env::set_var("RUST_LOG", "trace");
    pretty_env_logger::init();

    let args = Args::from_args();
    let hello = Hello {
        filename: CString::new(args.filename).unwrap(),
        contents: args.contents,
    };

    Fuse::new("hello")
        .foreground(true)
        .threaded(false)
        .mount(args.mountpoint, hello)
}

struct Hello {
    filename: CString,
    contents: String,
}

impl Operations for Hello {
    fn init(&mut self, _: &mut ConnInfo, cfg: &mut Config) {
        log::trace!("Hello::init()");
        cfg.kernel_cache(true);
    }

    fn getattr(&self, path: &CStr, _: Option<&mut FileInfo>) -> Result<stat> {
        log::trace!("Hello::getattr(path={:?})", path);

        let mut stat = unsafe { std::mem::zeroed::<stat>() };
        match path.to_bytes() {
            b"/" => {
                stat.st_mode = libc::S_IFDIR | 0755;
                stat.st_nlink = 2;
            }
            path if path[1..] == *self.filename.as_bytes() => {
                stat.st_mode = libc::S_IFREG | 0444;
                stat.st_nlink = 1;
                stat.st_size = self.contents.as_bytes().len() as i64;
            }
            _ => return Err(libc::ENOENT),
        }
        Ok(stat)
    }

    fn readdir(
        &self,
        path: &CStr,
        filler: &mut FillDir,
        _: off_t,
        _: Option<&mut FileInfo>,
        _: ReadDirFlags,
    ) -> Result<()> {
        log::trace!("Hello::readdir(path={:?})", path);
        if path.to_bytes() != b"/" {
            return Err(libc::ENOENT);
        }

        filler.add(&*CString::new(".").unwrap(), None, 0, Default::default());
        filler.add(&*CString::new("..").unwrap(), None, 0, Default::default());
        filler.add(&*self.filename, None, 0, Default::default());

        Ok(())
    }

    fn open(&self, path: &CStr, fi: &mut FileInfo) -> Result<()> {
        log::trace!("Hello::open(path={:?})", path);
        if path.to_bytes()[1..] != *self.filename.to_bytes() {
            return Err(libc::ENOENT);
        }
        match fi.flags() & libc::O_ACCMODE {
            libc::O_RDONLY => Ok(()),
            _ => Err(libc::EACCES),
        }
    }

    fn read(
        &self,
        path: &CStr,
        buf: &mut [u8],
        offset: off_t,
        _: Option<&mut FileInfo>,
    ) -> Result<usize> {
        log::trace!("Hello::read(path={:?})", path);

        debug_assert!(offset >= 0);
        let offset = offset as usize;

        if path.to_bytes()[1..] != *self.filename.to_bytes() {
            return Err(libc::ENOENT);
        }

        if offset >= self.contents.len() {
            log::debug!("the content has already been read");
            return Ok(0);
        }

        let contents = self.contents[offset..].as_bytes();
        let len = std::cmp::min(buf.len(), contents.len());
        buf[..len].copy_from_slice(&contents[..len]);

        Ok(len)
    }
}
