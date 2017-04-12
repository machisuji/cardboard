# cardboard

[![travis ci](https://travis-ci.org/machisuji/cardboard.svg?branch=master)](https://travis-ci.org/machisuji/cardboard)

`cardboard` is a decentralized, kanban-style collaboration tool which lets you organize tasks
into customizable cards based on plain, versioned text files.

![Image of Cardboard](https://github.com/machisuji/cardboard/raw/master/doc/cardboard.png)

## Usage

When running `cardboard` for the first time it will initialize a data
repository with example data in `./.cardboard`.

Some notes:

* Cards' content has to start with a markdown title (`# Some title`).
* A card's title will be used to derive a file name for it (`some_title.md`).

Cardboard is for developers and with that as an excuse it leaves a lot of the dirty
work to the users. Here's a bunch of things you will have to do yourself.

### Configuring boards

Edit `.cardboard/config.yml` to configure the existing boards.

### Adding a remote

If you actually want to share your work with others you will have to add a remote.

```
cd .cardboard
git add remote origin git@github.com:machisuji/cardboard-example.git
```

## Development

1. Install `rust` including `cargo` as described [here](https://www.rust-lang.org/en-US/install.html).
2. Run it using `cargo run`. You can suppress the opening of a browser through
`cargo run -- -q`.

### Build

Built using cargo.

```
cargo build --release
```

Whereupon the executable will be created at `target/release/cardboard`.

### Building on Windows

On Windows the build has the following prerequisites:

* [OpenSSL](https://github.com/sfackler/rust-openssl#windows)
* [cmake](https://cmake.org/download/)

## DISCLAIMER

I don't know what I'm doing. This is an exercise in learning both Rust and about
the internals of git. Also you may have noticed that I'm not much of a web
designer.
