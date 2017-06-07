Goal: learning Rust language

# Rusty

Command line application for managing podcast feeds, with sqlite backend. 

## Features

- rusty subscribe \[-d\] \<url\>: subscribe to a podcast feed by its url 
- rusty unsubscribe \<id\>: unsubscribe to a podcast feed by its id
- rusty list: get podcast feed list
- rusty update \[-d\] \[\<id\>\]: get podcasts list without downloading, if an id is set update only this feed
- rusty pending: get list of podcast not downloaded
- rusty download-dir \<path\>: set directory path where podcast will be downloaded
- rusty download \[\<id\>\]: get all pending podcasts, or one by its id 

- -d: set new podcast as downloaded

### V 0.4

- rusty healthcheck: checkup
  - Database exists
  - Download directory exists
  - Podcast urls are reachable

## Notes on building

For sqlite3 packages needed by rusqlite([explanation](https://github.com/jgallagher/rusqlite#notes-on-building-rusqlite-and-libsqlite3-sys)):

    sudo apt install pkg-config libsqlite3-dev

and if everything goes fine:

    pkg-config --list-all|grep sql 
    sqlite3                        SQLite - SQL database engine

And openssl support for `extern crate hyper_native_tls`:

    sudo apt install libssl-dev

And finally for cross-compiling see `cross-compiling.md` in `doc/` folder

There are functionnal examples into Makefile 

## Third party crate

- [sqllite](https://github.com/dckc/rust-sqlite3): sqlite3 wrapper
- [command line parsing](https://github.com/kbknapp/clap-rs): argument parser
- [rust-url](https://github.com/servo/rust-url): url handling
- [hyper](https://hyper.rs/hyper/v0.10.9/hyper/): http client

## Rust Documentation

- [Cargo](http://doc.crates.io/guide.html)
- [Taking Rust everywhere with rustup](https://blog.rust-lang.org/2016/05/13/rustup.html)
- [sqlite3](http://www.madmode.com/rust-sqlite3/sqlite3/index.html)
- [Rust by examples](http://rustbyexample.com/index.html)
- [Piston engine](https://github.com/PistonDevelopers/Piston-Tutorials/tree/master/getting-started)
- [Install Rust](https://www.rust-lang.org/fr/install.html)

    cargo init --bin
    cargo run
