Goal: learning Rust language
# Rusty

Command line application for managing podcast, with sqllite backend. 

## Features

- rusty add \<url\> \[\<label\>\]: add podcast by its url with optional label
- rusty del \<id\>: delete podcast by its id
- rusty populate \<id\>: get podcasts list without downloading
- rusty download-dir \<path\>: set directory path where podcast will be downloaded
- rusty healthcheck: checkup
  - Database exists
  - Download directory exists
  - Podcast urls are reachable
- rusty get: get new podcast

## Database tables

Put database into ~/.config/rusty/rusty.sqllite3

- conf(key, value): configuration 
- podcast(id, url, label, created_at): podcast list
- downloaded(id, podcast_id, url, downloaded_at): list of podcast already downloaded

## Other features

- Build debian package
- Crossbuilding arm version
 
## Third party crate

- [sqllite](https://github.com/dckc/rust-sqlite3)