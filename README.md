Goal: learning Rust language
# Rusty
Command line application for managing podcast, with sqllite backend. 

## Features

### V 0.1

- rusty subscribe \<url\> \[\<label\>\]: subscribe to podcast by its url with optional label
- rusty unsubscribe \<id\>: unsubscribe to podcast by its id
- rusty list: get podcast list
- rusty populate \<id\>: get podcasts list without downloading and mark them as downloaded
- rusty download-dir \<path\>: set directory path where podcast will be downloaded

### V 0.2

- rusty update: update list without downloading
- rusty pending: get list of podcast not downloaded

### V 0.3

- rusty download \[\<id\>\]: get all pending podcasts, or one by its id 

### V 0.4

- rusty healthcheck: checkup
  - Database exists
  - Download directory exists
  - Podcast urls are reachable
- rusty install-cron: add command line into cron

## Database tables

Put database into ~/.config/rusty/rusty.sqllite3

- conf(key, value): configuration 
- podcast(id, url, label, created_at): podcast list
- downloaded(id, podcast_id, url, downloaded, downloaded_at): list of podcast already downloaded

## Other features

- Build debian package
- Crossbuilding arm version
- Optimize binary
 
## Third party crate

- [sqllite](https://github.com/dckc/rust-sqlite3)
- [command line parsing](https://github.com/kbknapp/clap-rs)

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
