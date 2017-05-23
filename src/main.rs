#[macro_use] extern crate clap;
extern crate url;

extern crate rusqlite;

mod schema;

use std::process;
use clap::{App, ArgMatches};
use schema::*;
use url::Url;
use rusqlite::Connection;

fn main() {
    // Sqlite database
    let database_url = "/tmp/rusty.sqlite3";
    let connection = match Connection::open(database_url) {
        Ok(connection) => connection,
        Err(e) =>{
            println!("Error: {} {}", e, database_url);
            process::exit(1)
        },
    };
    init(&connection);

    // Clap
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand() {
        ("subscribe", Some(sub_matches)) => subscribe(sub_matches, &connection),
        ("unsubscribe", Some(sub_matches)) => unsubscribe(sub_matches, &connection),
        ("list", Some(_)) => list(&connection),
        ("", None) => println!("No subcommand was used"),
        _ => println!("No!"),
    }
}

fn init(connection: &Connection) {
    connection.execute(include_str!("sql/init.sql"), &[]).unwrap();
}

fn subscribe(args: &ArgMatches, connection: &Connection) {
    match Url::parse(args.value_of("url").unwrap()) {
        Ok(url) => {
            let podcast = Podcast { id: 0, url: url.as_str().to_string(), label: args.value_of("label").unwrap_or("").to_string()};
            connection.execute("insert into podcast (url, label) values (?1, ?2)", &[&podcast.url, &podcast.label]).unwrap();
            println!("Subscribed to: {}", podcast.url);
        }
        Err(e) => println!("Could not parse url '{}' {}", args.value_of("url").unwrap(), e),
    }
}

fn unsubscribe(args: &ArgMatches, connection: &Connection) {
    let id = args.value_of("id").unwrap();

    let mut stmt = connection.prepare("select id, url, label from podcast where id = ?1").unwrap();
    let mut rows = stmt.query_map(&[&id], Podcast::map).unwrap();
    match rows.next() {
        Some(row) => {
            let podcast = row.unwrap();
            connection.execute("delete from podcast where id = ?1", &[&id]).unwrap();
            println!("Unsubscribed from: {}", podcast.url);
        },
        None => println!("Podcast doesn't exist"),
    }
}

fn list(connection: &Connection) {
    let mut stmt = connection.prepare("select id, url, label from podcast").unwrap();

    let rows = stmt.query_map(&[], Podcast::map).unwrap();
    for row in rows {
        let podcast = row.unwrap();
        println!("- {}: {}", podcast.id, podcast.url);
    }
}

