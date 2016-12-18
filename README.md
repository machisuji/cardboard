# cardboard

[![travis ci](https://travis-ci.org/machisuji/cardboard.svg?branch=master)](https://travis-ci.org/machisuji/cardboard)

`cardboard` is a decentralized, kanban-style collaboration tool which lets you organize tasks
into customizable cards based on plain, versioned text files.

## Build

Built using cargo.

```
cargo build --release
```

Whereupon the executable will be created at `target/release/cardboard`.

### Building on Windows

On Windows the build has the following prerequisites:

* [OpenSSL](https://github.com/sfackler/rust-openssl#windows)
* [cmake](https://cmake.org/download/)