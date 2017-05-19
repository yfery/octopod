#[macro_use] extern crate clap;
#[macro_use] extern crate slog;
extern crate slog_term;
extern crate slog_async;
extern crate url;

extern crate rusqlite;

mod database;
mod schema;

use std::process;
use std::env;
use clap::{App, ArgMatches};
use schema::*;
use database::Db;
use slog::Drain;
use url::Url;
use rusqlite::Connection;

fn main() {
    // Slog
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let _log = slog::Logger::root(drain, o!());

    // Sqlite database
    let database_url = "/tmp/rusty.sqlite3";
    let database = match Db::new(database_url.to_string()) {
        Ok(database) => database,
        Err(e) => {
            println!("Error: {} {}", e, database_url);
            process::exit(1)
        },
    };

    // Clap
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand() {
        ("subscribe", Some(sub_matches)) => subscribe(sub_matches, &database, &_log),
        ("list", Some(sub_matches)) => list(sub_matches, &database, &_log),
        ("", None) => println!("No subcommand was used"),
        _ => println!("No!"),
    }
}

fn subscribe(args: &ArgMatches, connection: &database::Db, logger: &slog::Logger) {
    match Url::parse(args.value_of("url").unwrap()) {
        Ok(url) => {
            let podcast = Podcast { id: 0, url: url.as_str().to_string(), label: args.value_of("label").unwrap_or("").to_string()};
            connection.subscribe_to_podcast(&podcast);
            info!(logger, "Subscribed to: {}", podcast.url;);
        }
        Err(e) => error!(logger, "Could not parse url '{}' {}", args.value_of("url").unwrap(), e),
    }
}


fn list(args: &ArgMatches, connection: &database::Db, logger: &slog::Logger) {
    connection.podcast_list();
}

