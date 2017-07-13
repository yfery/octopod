#[macro_use] extern crate clap;
extern crate url;
extern crate rss; // https://github.com/rust-syndication/rss
extern crate hyper; // https://github.com/hyperium/hyper
extern crate tokio_core; 
extern crate reqwest;
extern crate futures;
extern crate mime;
extern crate curl; // https://docs.rs/curl/0.4.6/curl/easy/
// extern crate rusqlite; // https://github.com/jgallagher/rusqlite
extern crate pbr; // https://a8m.github.io/pb/doc/pbr/index.html
extern crate time; // https://doc.rust-lang.org/time/time/index.html
extern crate serde;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate serde_derive;
extern crate dotenv;
extern crate chrono;

mod schema;
mod common;
mod models;

use std::process;
use clap::{App, ArgMatches};
use schema::*;
use models::*;
use url::Url;
// use rusqlite::Connection;
use std::fs::create_dir;
use std::path::{Path};
use std::env::home_dir;
use hyper::header::ContentType;
use std::io::{Read, stdin}; // needed for read_to_string trait
use std::str;
use std::env;
use std::str::FromStr;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use chrono::prelude::*;

embed_migrations!("migrations/");
const VERSION: &'static str = env!("RUSTY_VERSION");

fn main() {
    let lock_socket = common::create_app_lock(12345); // https://rosettacode.org/wiki/Category:Rust

    // Clap
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Init Sqlite database
    let database_url: String;
    dotenv().ok();

    if matches.is_present("database") { // get path from command line 
        database_url = matches.value_of("database").unwrap().to_string();
    } else { // if env var exists use it or put database file into ~/.config/rusty/rusty.sqlite3
        let mut db_path = home_dir().expect("/tmp/").into_os_string().into_string().unwrap() + "/.config/rusty";
        if !Path::new(&db_path).exists() { // If path doesn't exist we create it
            create_dir(&db_path).unwrap();
        }
        db_path = db_path + "/rusty.sqlite3";
        database_url = env::var("DATABASE_URL").expect(db_path.as_str());
    }

    let connection = SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url));
    // without output
    // embedded_migrations::run(&connection);
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout());

    match matches.subcommand() {
        ("subscribe", Some(sub_matches)) => subscribe(sub_matches, &connection),
        ("unsubscribe", Some(sub_matches)) => unsubscribe(sub_matches, &connection),
        ("list", Some(_)) => list(&connection),
        ("update", Some(sub_matches)) => {
            let feed_id = sub_matches.value_of("id").unwrap_or("0").parse::<i32>().unwrap();
            update(sub_matches, &connection, feed_id);
        },
        ("pending", Some(_)) => pending(&connection),
        ("downloaded", Some(_)) => downloaded(&connection),
        ("download", Some(sub_matches)) => download(sub_matches, &connection),
        ("version", Some(_)) => version(),
        ("download-dir", Some(sub_matches)) => downloaddir(sub_matches, &connection),
        ("", None) => println!("No subcommand was used"),
        _ => println!("No!"),
    }

    common::remove_app_lock(lock_socket);
}

fn subscribe(args: &ArgMatches, connection: &SqliteConnection) {
    match Url::parse(args.value_of("url").unwrap()) {
        Ok(url) => {
            let subscription = NewSubscription { url: url.as_str()};
            println!("Subscribing to: {}", subscription.url);

            //TODO
            let results = subscription::table.filter(subscription::url.eq(&url.as_str())).load::<Subscription>(connection) ;
            if results.unwrap().len() == 0 {
                diesel::insert(&subscription).into(subscription::table).execute(connection).expect("Error saving");
                // connection.execute("insert into subscription (url, label, last_build_date) values (?1, '', '')", &[&subscription.url]).unwrap();
            } else {
                println!("    Feed already subscribed to");
                return
            }
            // TODO: update(args, connection, connection.last_insert_rowid()); 
            println!("Subscribed");
            return
        },
        Err(e) => println!("Could not parse url '{}' {}", args.value_of("url").unwrap(), e),
    };
}

fn unsubscribe(args: &ArgMatches, connection: &SqliteConnection) {
    let id = args.value_of("id").unwrap();


    match subscription::table.find(&id.parse::<i32>().unwrap()).first::<Subscription>(connection) {
        Ok(subscription) => {
            let mut stdin = stdin();
            let mut buffer = [0;1];
            println!("Unsubscribing from: {}", subscription.url);
            println!("Sure? [y/N]"); 
            stdin.read_exact(&mut buffer).unwrap();
            if buffer[0] == 121u8 { // 121 is ascii code for 'y'
                // TODO
                diesel::delete(subscription::table.find(&id.parse::<i32>().unwrap())).execute(connection).expect("Error deleting posts");
                // subscription.delete(connection);
                println!("Unsubscribed from: {}", subscription.url);
            }
        },
        Err(_) => println!("Subscription doesn't exist")
    }
}

fn list(connection: &SqliteConnection) {
    println!("Subscriptions list:");
    match subscription::table.load::<Subscription>(connection) {
        Err(_) => println!("{}", "    No subscription"),
        Ok(subscriptions) => {
            for subscription in subscriptions {
                println!("    {}: {:?} ({})", subscription.id, subscription.label, subscription.url);
            }
        }
    };
    }

    fn update(args: &ArgMatches, connection: &SqliteConnection, feed_id: i32) {

        match subscription::table.load::<Subscription>(connection) {
            // match common::get_subscriptions(connection) {
            Err(_) => println!("    No subscription to update"),
            Ok(subscriptions) => for subscription in subscriptions {

                if feed_id != subscription.id && feed_id > 0 { // if an id is set, update/populate only this id
                    continue;
                }

                print!("Updating {} ... ", subscription.label.unwrap());

                let mut body: Vec<u8> = Vec::new();
                let mut res = reqwest::get(&subscription.url).unwrap();
                res.read_to_end(&mut body).unwrap();

                let rss = mime::Mime::from_str("application/rss+xml").unwrap();
                match res.headers().get() {
                    Some(&ContentType(ref mime)) => {
                        // impossible to find how to use match instead of if here, because of the custom mime type
                        if mime.subtype() == mime::XML || mime.subtype() == rss.subtype() {
                            ();
                            // TODO
                            // match subscription.from_xml_feed(connection, body, args.is_present("as-downloaded")) {
                            //     Ok(message) => println!("{}", message),
                            //     Err(e) => println!("{}", e),
                            // };
                        } else {
                            println!("Unsupported mime type: {}", mime.subtype());
                        }
                    },
                    None => ()
                }
            }
        }
        }

        fn pending(connection: &SqliteConnection) {
            println!("Pending list:");
            match podcast::table.filter(podcast::downloaded.eq(0)).load::<Podcast>(connection) {
                // match common::get_pending_podcasts(connection) {
                Err(_) => println!("{}", "    Nothing to download"),
                Ok(podcasts) => {
                    for podcast in podcasts {
                        println!("    {}: {} ({})", podcast.id, podcast.filename, podcast.url);
                    }
                }
            }
            }

            fn downloaded(connection: &SqliteConnection) {
                println!("Downloaded list:");
                match podcast::table.filter(podcast::downloaded.eq(1)).load::<Podcast>(connection) {
                    Err(_) => println!("{}", "    Nothing has been downloaded"),
                    Ok(podcasts) => {
                        for podcast in podcasts {
                            println!("    {}: {} ({})", podcast.id, podcast.filename, podcast.url);
                        }
                    }
                }
            }

            fn downloaddir(args: &ArgMatches, connection: &SqliteConnection) {
                // TODO
                // match args.value_of("path") {
                //     None => println!("Current download dir: {}", common::getdownloaddir(connection)),
                //     Some(path) => {
                //         connection.execute("insert or replace into config (key, value) values ('downloaddir', ?1)", &[&path]).unwrap();
                //         println!("Download dir set to: {}", path);
                //     }
                // }
            }

            fn download(args: &ArgMatches, connection: &SqliteConnection) {
                match args.value_of("id") {
                    None => {
                        println!("Download pending podcast:");
                        match podcast::table.filter(podcast::downloaded.eq(0)).load::<Podcast>(connection) {
                            // match common::get_pending_podcasts(connection) {
                            Err(_) => println!("{}", "    Nothing to download"),
                            Ok(podcasts) => {
                                for podcast in podcasts {
                                    println!("    {}", podcast.filename);
                                    //TODO
                                    // common::download_podcast(connection, podcast);
                                }
                            }
                        }
                        }, 
                        Some(id) => {
                            let podcast = podcast::table.filter(podcast::id.eq(id.parse::<i32>().unwrap())).first::<Podcast>(connection).expect("Error loading podcasts");
                            println!(" Download: {}", podcast.filename);
                            common::download_podcast(connection, podcast);
                        }
                    }
                }

                fn version() {
                    println!("Rusty {}", VERSION);
                }
