# `libfuse-rs`

[![pipeline status](https://gitlab.com/ubnt-intrepid/libfuse-rs/badges/master/pipeline.svg)](https://gitlab.com/ubnt-intrepid/libfuse-rs/commits/master)

## Overview

This library provides a Rust binding for [`libfuse`], the user-side reference implemenation of FUSE (Filesystem in Userspace).

Unlike [`rust-fuse`], this library uses the protocol-level implementation from `libfuse` *as is*.

Note that the library is now on experimental stage and not suitable for production use.
The following features has not been supported yet:

* Some operations (e.g. `ioctl`, `poll`)
* Splice (vectored) read
* Multithreaded event loop

## Example

```rust
use libfuse::{
    Ino,
    Operations,
    OperationResult,
    file::Entry,
    session::Builder,
};
use std::{ffi::CStr, io};

fn main() -> io::Result<()> {
    let fs = MyFS {};
    
    let mut session = Session::builder()
        .build(fs)?;

    session.set_signal_handlers()?;
    session.mount("/path/to/mountpoint")?;
    session.run_loop()?;
}

struct MyFS {}

impl Operations for Ops {
    fn lookup(&mut self, ino: Ino, name: &CStr) -> OperationResult<Entry> {
        ...
    }
}
```

## License

This library is licensed under either of

* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

<!-- links -->

[`libfuse`]: https://github.com/libfuse/libfuse
[`rust-fuse`]: https://github.com/zargony/rust-fuse