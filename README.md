Goal: learning Rust language
# Rusty
Command line application for managing podcast feeds, with sqllite backend. 

## Features

### V 0.1

- rusty subscribe \<url\> \[\<label\>\]: subscribe to a podcast feed by its url with optional label
- rusty unsubscribe \<id\>: unsubscribe to a podcast feed by its id
- rusty list: get podcast feed list
- rusty update \[\<id\>\]: get podcasts list without downloading, if an id is set update only this feed
- rusty populate \[\<id\>\]: get podcasts list without downloading and mark them as downloaded, if an id is set populate only this feed
- rusty pending: get list of podcast not downloaded

### V 0.3

- rusty download-dir \<path\>: set directory path where podcast will be downloaded
- rusty download \[\<id\>\]: get all pending podcasts, or one by its id 

### V 0.4

- rusty healthcheck: checkup
  - Database exists
  - Download directory exists
  - Podcast urls are reachable

## Database tables

Put database into ~/.config/rusty/rusty.sqllite3

- confg(key, value): configuration 
- subscription(id, url, label, created_at): feed list
- podcast(id, podcast_id, url, downloaded, downloaded_at): list of podcast 

## Other features

- Build debian package
- Crossbuilding arm version
- Optimize binary
 
## Third party crate

- [sqllite](https://github.com/dckc/rust-sqlite3): sqlite3 wrapper
- [command line parsing](https://github.com/kbknapp/clap-rs): argument parser
- [rust-url](https://github.com/servo/rust-url): url handling
- [hyper](https://hyper.rs/hyper/v0.10.9/hyper/): http client

## Dependencies 

    sudo apt install libsqlite3-dev

## Rust Documentation

- [Cargo](http://doc.crates.io/guide.html)
- [Taking Rust everywhere with rustup](https://blog.rust-lang.org/2016/05/13/rustup.html)
- [sqlite3](http://www.madmode.com/rust-sqlite3/sqlite3/index.html)
- [Rust by examples](http://rustbyexample.com/index.html)
- [Piston engine](https://github.com/PistonDevelopers/Piston-Tutorials/tree/master/getting-started)
- [Install Rust](https://www.rust-lang.org/fr/install.html)

    cargo init --bin
    cargo run
