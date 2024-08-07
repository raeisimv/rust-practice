### Sample Windows Hook DLL

This project demonstrate how to create a DLL library for Windows that would be attached to another process in order to
debug on production.

### Build using Cargo

You can build the project using Cargo:

```shell
cargo build --release
```

### Build using Just

Alternatively, the `justfile` is provided to ease. Install [Just](https://crates.io/crates/just) toolchain globally:

```shell
cargo install just
```

then, to build 32-bit run:

```shell
just build-32
```

to build 64-bit run:

```shell
just build-64
```

#### License

MIT