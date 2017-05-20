#[macro_use] extern crate clap;
#[macro_use] extern crate slog;
extern crate slog_term;
extern crate slog_async;
extern crate url;

extern crate rusqlite;

mod schema;

use std::process;
use clap::{App, ArgMatches};
use schema::*;
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
        ("subscribe", Some(sub_matches)) => subscribe(sub_matches, &connection, &_log),
        ("list", Some(_)) => list(&connection),
        ("", None) => println!("No subcommand was used"),
        _ => println!("No!"),
    }
}

    fn init(connection: &Connection) {

        connection.execute("create table if not exists podcast (
                    id integer primary key autoincrement,
                    url text not null,
                    label text not null, 
                    created_at timestamp default current_timestamp)", &[]).unwrap();
    }

fn subscribe(args: &ArgMatches, connection: &Connection, logger: &slog::Logger) {
    match Url::parse(args.value_of("url").unwrap()) {
        Ok(url) => {
            let podcast = Podcast { id: 0, url: url.as_str().to_string(), label: args.value_of("label").unwrap_or("").to_string()};
            connection.execute("insert into podcast (url, label)
                    values (?1, ?2)",
                    &[&podcast.url, &podcast.label]).unwrap();
            info!(logger, "Subscribed to: {}", podcast.url;);
        }
        Err(e) => error!(logger, "Could not parse url '{}' {}", args.value_of("url").unwrap(), e),
    }
}

fn list(connection: &Connection) {
        let mut stmt = connection.prepare("select id, url, label from podcast").unwrap();

        let podcasts = stmt.query_map(&[], |row| {
            Podcast {
                id: row.get(0),
                url: row.get(1),
                label: row.get(2)
            }
        }).unwrap();

        for podcast in podcasts {
            match podcast {
                Ok(podcast) => println!("{}", podcast.url),
                Err(e) => println!("{}", e) ,
            }
        };
}

