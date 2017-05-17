#[macro_use] extern crate clap;
extern crate rusqlite;
#[macro_use] extern crate slog;
extern crate slog_term;
extern crate slog_async;

mod database;
mod podcast;

use clap::{App, ArgMatches};
use podcast::Podcast;
use database::Db;
use slog::Drain;

fn main() {
    // Slog
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let _log = slog::Logger::root(drain, o!());

    // Clap
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let database = Db::new("/tmp/rusty.sqlite3".to_string());
    database.print();

    match matches.subcommand() {
        ("add", Some(sub_matches)) => add(sub_matches, &_log),
        ("", None) => println!("No subcommand was used"),
        _ => println!("No!"),
    }
}

fn add(args: &ArgMatches, logger: &slog::Logger) {
    let podcast = Podcast { id: 0, url: args.value_of("url").unwrap().to_string(), label: args.value_of("label").unwrap_or("").to_string()};
    trace!(logger, "Adding podcast: {}", podcast.url;);
    info!(logger, "Podcast added: {}", podcast.url;);
}
