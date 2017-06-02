#[macro_use] extern crate clap;
extern crate url;
extern crate rss;
extern crate hyper;
extern crate curl; // https://docs.rs/curl/0.4.6/curl/easy/
extern crate rusqlite;

mod schema;
mod common;

use std::process;
use clap::{App, ArgMatches};
use schema::*;
use url::Url;
use rusqlite::Connection;
use curl::easy::Easy;
use std::path::{Path};
use std::fs::{File, create_dir};
use std::env::home_dir;

fn main() {
    let lock_socket = common::create_app_lock(12345); // https://rosettacode.org/wiki/Category:Rust

    // Clap
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Init Sqlite database
    let mut database_url = String::new();

    if matches.is_present("database") { // get path from command line 
        database_url = matches.value_of("database").unwrap().to_string();
    } else { // or put database file into ~/.config/rusty/rusty.sqlite3
        let db_path = home_dir().expect("/tmp/").into_os_string().into_string().unwrap() + "/.config/rusty";
        if !Path::new(&db_path).exists() { // If path doesn't exist we create it
            create_dir(&db_path).unwrap();
        }
        database_url = db_path + "/rusty.sqlite3";
    }
    let connection = match Connection::open(&database_url) {
        Ok(connection) => connection,
        Err(e) => {
            println!("Error database connection: {} {}", e, database_url);
            process::exit(1)
        }
    };
    init(&connection); // Initialize database

    match matches.subcommand() {
        ("subscribe", Some(sub_matches)) => subscribe(sub_matches, &connection),
        ("unsubscribe", Some(sub_matches)) => unsubscribe(sub_matches, &connection),
        ("list", Some(_)) => list(&connection),
        ("update", Some(sub_matches)) => {
            let feed_id = sub_matches.value_of("id").unwrap_or("0").parse::<i64>().unwrap();
            update(sub_matches, &connection, feed_id);
        },
        ("pending", Some(_)) => pending(&connection),
        ("download", Some(_)) => download(&connection),
        ("download-dir", Some(sub_matches)) => downloaddir(sub_matches, &connection),
        ("", None) => println!("No subcommand was used"),
        _ => println!("No!"),
    }

    common::remove_app_lock(lock_socket);
}

fn init(connection: &Connection) {
    // execute only the first query if there are several queries. So we need to used split()
    let split = include_str!("sql/init.sql").split(";");
    for s in split {
        if s.trim() == "" {
            continue;
        }
        connection.execute(s, &[]).unwrap(); 
    }
}

fn subscribe(args: &ArgMatches, connection: &Connection) {
    match Url::parse(args.value_of("url").unwrap()) {
        Ok(url) => {
            let subscription = Subscription { id: 0, url: url.as_str().to_string(), label: String::new()};
            println!("Subscribing to: {}", subscription.url);

            let mut stmt = connection.prepare("select * from subscription where url = ?1").unwrap();
            if !stmt.exists(&[&subscription.url]).unwrap() {
                connection.execute("insert into subscription (url, label) values (?1, '')", &[&subscription.url]).unwrap();
            } else {
                println!("    Feed already subscribed to");
                return
            }
            update(args, connection, connection.last_insert_rowid()); 
            println!("Subscribed");
        }
        Err(e) => println!("Could not parse url '{}' {}", args.value_of("url").unwrap(), e),
    }
}

fn unsubscribe(args: &ArgMatches, connection: &Connection) {
    use std::io;
    use std::io::prelude::*;
    let mut stdin = io::stdin();
    let id = args.value_of("id").unwrap();
    let mut buffer = [0;1];

    let mut stmt = connection.prepare("select id, url, label from subscription where id = ?1").unwrap();
    let mut rows = stmt.query_map(&[&id], Subscription::map).unwrap();
    match rows.next() {
        Some(row) => {
            let subscription = row.unwrap();
            println!("Unsubscribing from: {}", subscription.url);
            println!("Sure? [y/N]"); 
            stdin.read_exact(&mut buffer).unwrap();
            if buffer[0] == 121u8 { // 121 is ascii code for 'y'
                connection.execute("delete from subscription where id = ?1", &[&id]).unwrap();
                println!("Unsubscribed from: {}", subscription.url);
            }
        },
        None => println!("Subscription doesn't exist"),
    }
}

fn list(connection: &Connection) {
    let mut stmt = connection.prepare("select id, url, label from subscription").unwrap();

    println!("Subscriptions list:");
    if !stmt.exists(&[]).unwrap() {
        println!("{}", "    No subscription");
    }

    for row in stmt.query_map(&[], Subscription::map).unwrap() {
        let subscription = row.unwrap();
        println!("    {}: {} ({})", subscription.id, subscription.label, subscription.url);
    }
}

fn update(args: &ArgMatches, connection: &Connection, feed_id: i64) {
    use hyper::Client; // https://hyper.rs/hyper/v0.10.9/hyper/index.html
    use std::io::Read; // needed for read_to_string trait
    use std::str::FromStr; // needed for FromStr trait on Channel
    use rss::Channel;

    let mut stmt = connection.prepare("select id, url, label from subscription").unwrap();
    let client = Client::new(); // create http client

    let mut as_downloaded = 0;
    if args.is_present("as-downloaded") {
        as_downloaded = 1;
    }

    for row in stmt.query_map(&[], Subscription::map).unwrap() {
        let subscription = row.unwrap();
        println!("Updating {}:", subscription.label);

        if feed_id != subscription.id && feed_id > 0 { // if an id is set, update/populate only this id
            continue;
        }

        let mut res = client.get(subscription.clone()).send().unwrap(); // get query result thanks to IntoUrl trait implement for Subscription
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap(); // extract body from query result

        let channel = Channel::from_str(&body).unwrap(); // parse rss into channel
        connection.execute("update subscription set label = ?1 where id = ?2", &[&channel.title(), &subscription.id]).unwrap(); // update podcast feed name
        for item in channel.items() {
            let url = Url::parse(item.enclosure().unwrap().url()).unwrap();
            let  path_segments = url.path_segments().unwrap();

            // create filename
            let mut filename = String::new();
            for segment in path_segments {
                if segment == "enclosure.mp3" || segment == "listen.mp3" {
                    filename = filename + ".mp3";
                    break;
                }
                filename = segment.to_string();
            }

            let podcast = Podcast { id: 0, subscription_id: subscription.id, url: url.as_str().to_string(), filename: filename};
            let previous_insert_rowid = connection.last_insert_rowid();
            connection.execute("insert or ignore into podcast (subscription_id, url, filename, downloaded) values (?1, ?2, ?3, ?4)", &[&podcast.subscription_id, &podcast.url, &podcast.filename, &as_downloaded]).unwrap();
            if previous_insert_rowid != connection.last_insert_rowid() {
                println!("    New podcast added: {:?}", podcast.filename);
            }

        }
        println!("Updated");
    }
}

fn pending(connection: &Connection) {
    let mut stmt = connection.prepare("select id, subscription_id, url, filename from podcast where downloaded = 0").unwrap();

    println!("Pending list:");
    if !stmt.exists(&[]).unwrap() {
        println!("{}", "    Nothing to download");
    }
    for row in stmt.query_map(&[], Podcast::map).unwrap(){
        let podcast = row.unwrap();
        println!("    {} ({})", podcast.filename, podcast.url);
    }
}

fn downloaddir(args: &ArgMatches, connection: &Connection) {
    let path = args.value_of("path").unwrap(); 
    connection.execute("insert or replace into config (key, value) values ('downloaddir', ?1)", &[&path]).unwrap();
    println!("Download dir set to: {}", path);
}

fn download(connection: &Connection) {
    use std::io::Write;
    use std::ops::Mul;
    let mut curl = Easy::new();

    curl.progress(true).unwrap();
    curl.follow_location(true).unwrap();
    curl.progress_function( |a, b, _, _| {
        print!("    Downloading: {}%      \r", (b.mul(100_f64)/a).floor());
        true
    }).unwrap();
    let mut stmt = connection.prepare("select id, subscription_id, url, filename from podcast where downloaded = 0").unwrap();

    println!("Download pending podcast:");
    if !stmt.exists(&[]).unwrap() {
        println!("{}", "    Nothing to download");
    }
    for row in stmt.query_map(&[], Podcast::map).unwrap(){
        let podcast = row.unwrap();
        let temp = "/tmp/".to_string() + podcast.filename.as_str();
        let path = Path::new(&temp);

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}",
                               why),
            Ok(file) => file,
        };
        curl.url(&podcast.url).unwrap();
        curl.write_function( move |data| {
            Ok(file.write(data).unwrap())
        }).unwrap();
        println!("    {}", podcast.filename);
        curl.perform().unwrap();
        connection.execute("update podcast set downloaded = 1, downloaded_at = current_timestamp where id = ?1", &[&podcast.id]).unwrap();
    }
}
