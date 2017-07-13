
use std::net::TcpListener;
use std::process;
// use rusqlite::Connection;
use schema::*;
use models;
use models::*;
use curl::easy::Easy;
use std::time::Duration;
use pbr::{ProgressBar, Units};
use std::fs::{File, OpenOptions};
use std::path::{Path};
use std::io::Write; // needed for read_to_string trait
use std::fs;
use diesel::sqlite::SqliteConnection;
use diesel::*; // useful for load & find

// https://rosettacode.org/wiki/Category:Rust
pub fn create_app_lock(port: u16) -> TcpListener {
    match TcpListener::bind(("0.0.0.0", port)) {
        Ok(socket) => {
            socket
        },
        Err(_) => {
            println!("Another instance already running?");
            process::exit(1)
        }
    }
}
 
pub fn remove_app_lock(socket: TcpListener) {
    drop(socket);
}

pub fn getdownloaddir(connection: &SqliteConnection) -> String {
    let config =  config::table.find("downloaddir").first::<Config>(connection).unwrap();
    match config.value {
        Some(path) => path,
        None => "/tmp/".to_string()
    }
}

pub fn download_podcast(connection: &SqliteConnection, podcast: Podcast) {
    let mut curl = Easy::new();
    let mut pb = ProgressBar::new(100);
    pb.format("╢▌▌░╟");
    pb.set_units(Units::Bytes);
    pb.set_max_refresh_rate(Some(Duration::from_millis(100)));
    curl.progress(true).unwrap();
    curl.follow_location(true).unwrap();
    curl.progress_function( move |a, b, _, _| {
        pb.total = a as u64;
        pb.set(b as u64);
        true
    }).unwrap();

    let temp = getdownloaddir(connection) + podcast.filename.as_str() + ".downloading";
    let path = Path::new(&temp);

    if Path::new(&(getdownloaddir(connection) + podcast.filename.as_str())).exists() {
        println!("  File already downloaded");
        return 
    }

    let mut file: File;
    let mut options = OpenOptions::new();
    // We want to write to our file as well as append new data to it.
    options.write(true).append(true);
    if Path::new(path).exists() {
        let metadata = fs::metadata(path).unwrap();
        curl.resume_from(metadata.len()).unwrap();

        file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&path)
            .unwrap();

    } else {
        file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}", why),
            Ok(file) => file
        };
    }
    curl.url(&podcast.url).unwrap();
    curl.write_function( move |data| {
        Ok(file.write(data).unwrap())
    }).unwrap();

    curl.perform().unwrap();
    // TODO
    // connection.execute("update podcast set downloaded = 1, downloaded_at = current_timestamp where id = ?1", &[&podcast.id]).unwrap();
    fs::rename(getdownloaddir(connection) + podcast.filename.as_str() + ".downloading", getdownloaddir(connection) + podcast.filename.as_str()).unwrap();
}
