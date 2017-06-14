#[macro_use] extern crate clap;
extern crate url;
extern crate rss; // https://github.com/rust-syndication/rss
extern crate hyper; // https://github.com/hyperium/hyper
extern crate hyper_native_tls; // https://github.com/sfackler/hyper-native-tls
extern crate curl; // https://docs.rs/curl/0.4.6/curl/easy/
extern crate rusqlite; // https://github.com/jgallagher/rusqlite
extern crate pbr; // https://a8m.github.io/pb/doc/pbr/index.html
//extern crate time; // https://doc.rust-lang.org/time/time/index.html

mod schema;
mod common;

use pbr::{ProgressBar, Units};
use std::process;
use clap::{App, ArgMatches};
use schema::*;
use url::Url;
use rusqlite::Connection;
use curl::easy::Easy;
use std::path::{Path};
use std::fs::{File, create_dir};
use std::env::home_dir;
use std::time::Duration;
use hyper::Client; // https://hyper.rs/hyper/v0.10.9/hyper/index.html
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use std::io::{Write, Read, stdin}; // needed for read_to_string trait
use std::str::FromStr; // needed for FromStr trait on Channel
use rss::Channel;

const VERSION: &'static str = env!("RUSTY_VERSION");

fn main() {
    let lock_socket = common::create_app_lock(12345); // https://rosettacode.org/wiki/Category:Rust

    // Clap
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Init Sqlite database
    let database_url: String;

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
        ("version", Some(_)) => version(),
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
            let subscription = Subscription { id: 0, url: url.as_str().to_string(), label: String::new(), last_build_date: String::new()};
            println!("Subscribing to: {}", subscription.url);

            let mut stmt = connection.prepare("select * from subscription where url = ?1").unwrap();
            if !stmt.exists(&[&subscription.url]).unwrap() {
                connection.execute("insert into subscription (url, label, last_build_date) values (?1, '', ?2)", &[&subscription.url, &subscription.last_build_date]).unwrap();
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
    let id = args.value_of("id").unwrap();

    match common::get_subscription(connection, &id) {
        Some(subscription) => {
            let mut stdin = stdin();
            let mut buffer = [0;1];
            println!("Unsubscribing from: {}", subscription.url);
            println!("Sure? [y/N]"); 
            stdin.read_exact(&mut buffer).unwrap();
            if buffer[0] == 121u8 { // 121 is ascii code for 'y'
                subscription.delete(connection);
                println!("Unsubscribed from: {}", subscription.url);
            }
        },
        None => println!("Subscription doesn't exist"),
    }
}

fn list(connection: &Connection) {
    println!("Subscriptions list:");
    match common::get_subscriptions(connection) {
        None => println!("{}", "    No subscription"),
        Some(subscriptions) => {
            for subscription in subscriptions {
                println!("    {}: {} ({})", subscription.id, subscription.label, subscription.url);
            }
        }
    }
}

fn update(args: &ArgMatches, connection: &Connection, feed_id: i64) {

    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector); // create http client with tls support

    let mut as_downloaded = 0;
    if args.is_present("as-downloaded") {
        as_downloaded = 1;
    }

    match common::get_subscriptions(connection) {
        None => println!("    No subscription to update"),
        Some(subscriptions) => for subscription in subscriptions {

            if feed_id != subscription.id && feed_id > 0 { // if an id is set, update/populate only this id
                continue;
            }

            println!("Updating {} ...", subscription.label);

            let mut res = client.get(subscription.clone()).send().unwrap(); // get query result thanks to IntoUrl trait implement for Subscription
            let mut body = String::new();
            let mut previous_insert_rowid: i64 = 0;
            res.read_to_string(&mut body).unwrap(); // extract body from query result

            let channel = match Channel::from_str(&body) { // parse rss into channel
                Err(e) => { 
                    println!("Couldn't parse rss {} ({})", subscription.url, e);
                    continue
                },
                Ok(channel) => channel
            };

            if channel.last_build_date().unwrap() == &subscription.last_build_date {
                continue
            }
            connection.execute("update subscription set label = ?1, last_build_date = ?2 where id = ?3", &[&channel.title(), &channel.last_build_date(), &subscription.id]).unwrap(); // update podcast feed name
            for item in channel.items() {
                let url = Url::parse(item.enclosure().unwrap().url()).unwrap();
                let path_segments = url.path_segments().unwrap();

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
                connection.execute("insert or ignore into podcast (subscription_id, url, filename, downloaded) values (?1, ?2, ?3, ?4)", &[&podcast.subscription_id, &podcast.url, &podcast.filename, &as_downloaded]).unwrap();
                if previous_insert_rowid != connection.last_insert_rowid() {
                    println!("    New podcast added: {:?}", podcast.filename);
                }
                previous_insert_rowid = connection.last_insert_rowid();
            }
            println!("{} updated", subscription.label);
        }
    }
}

fn pending(connection: &Connection) {
    println!("Pending list:");
    match common::get_pending_podcasts(connection) {
        None => println!("{}", "    Nothing to download"),
        Some(podcasts) => {
            for podcast in podcasts {
                println!("    {} ({})", podcast.filename, podcast.url);
            }
        }
    }
}

fn downloaddir(args: &ArgMatches, connection: &Connection) {
    match args.value_of("path") {
        None => println!("Current download dir: {}", common::getdownloaddir(connection)),
        Some(path) => {
            connection.execute("insert or replace into config (key, value) values ('downloaddir', ?1)", &[&path]).unwrap();
            println!("Download dir set to: {}", path);
        }
    }
}

fn download(connection: &Connection) {
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

    println!("Download pending podcast:");
    match common::get_pending_podcasts(connection) {
        None => println!("{}", "    Nothing to download"),
        Some(podcasts) => {
            for podcast in podcasts {
                let temp = common::getdownloaddir(connection) + podcast.filename.as_str();
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
    }
}

fn version() {
    println!("Rusty {}", VERSION);
}
