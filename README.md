# `libfuse-rs`

## Overview

This library provides a Rust binding for [`libfuse`], the user-side reference implemenation of FUSE (Filesystem in Userspace).

Unlike [`rust-fuse`], this library uses the protocol-level implementation from `libfuse` *as is*.

Note that the library is now on experimental stage and not suitable for production use.
The following features has not been supported yet:

* Some operations (e.g. `ioctl`, `poll`)
* Splice (vectored) read
* Multithreaded event loop

## License

This library is licensed under either of

* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

<!-- links -->

[`libfuse`]: https://github.com/libfuse/libfuse
[`rust-fuse`]: https://github.com/zargony/rust-fuse