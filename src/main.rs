#[macro_use] extern crate clap;
extern crate rusqlite;
#[macro_use] extern crate slog;
extern crate slog_term;
extern crate slog_async;
extern crate url;

mod database;
mod podcast;

use std::process;
use clap::{App, ArgMatches};
use podcast::Podcast;
use database::Db;
use slog::Drain;
use url::Url;

fn main() {
    // Config
    let database_path: &str = "/tmp/rusty.sqlite3";

    // Slog
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let _log = slog::Logger::root(drain, o!());

    // Clap
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let database = match Db::new(database_path.to_string()) {
        Ok(database) => database,
        Err(e) => {
            println!("Error: {} {}", e, database_path);
            process::exit(1)
        },
    };

    match matches.subcommand() {
        ("subscribe", Some(sub_matches)) => subscribe(sub_matches, &database, &_log),
        ("", None) => println!("No subcommand was used"),
        _ => println!("No!"),
    }
}

fn subscribe(args: &ArgMatches, database: &database::Db, logger: &slog::Logger) {
    match Url::parse(args.value_of("url").unwrap()) {
        Ok(url) => {
            let podcast = Podcast { id: 0, url: url.as_str().to_string(), label: args.value_of("label").unwrap_or("").to_string()};
            database.subscribe_to_podcast(&podcast);
            info!(logger, "Subscribed to: {}", podcast.url;);
        }
        Err(e) => error!(logger, "Could not parse url '{}' {}", args.value_of("url").unwrap(), e),
    }
}
