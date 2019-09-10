# `libfuse-rs`

[![pipeline status](https://gitlab.com/ubnt-intrepid/libfuse-rs/badges/master/pipeline.svg)](https://gitlab.com/ubnt-intrepid/libfuse-rs/commits/master)

[`libfuse`] bindings for Rust.

Unlike [`rust-fuse`], this library uses the protocol-level implementation from `libfuse` *as is*.

## Usage

```rust
use libfuse::{Operations, OperationResult};

fn main() {
    let ops = Ops;
    
    let session = Session::builder()
        .build(ops)
        .unwrap();
}

struct Ops;

impl Operations for Ops {
    type File = ();
    type Dir = ();
    
    ...
}
```

## Status

Experimental

## Author

Yusuke Sasaki

## License

[MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE)

<!-- links -->

[`libfuse`]: https://github.com/libfuse/libfuse
[`rust-fuse`]: https://github.com/zargony/rust-fuse