Code explanation 

- [Official Documentation](https://doc.rust-lang.org/book/)
- put extern crate only in main.rs 

## Include another file

If you want to include database.rs file

    mod database;

And if you want to use Db struct from that file 

    use database::Db;

## Object management 

    pub struct Db {
        conn: Connection
    }

    impl Db {
        // Associatied functions (without self, 'static method') https://doc.rust-lang.org/book/method-syntax.html#chaining-method-calls
        pub fn new(path: String) -> Db {
            Db {
                conn: Connection::open(path).unwrap(),
            }
        }

        // Chaining method with &self
        pub fn print(&self) {
            println!("paf");
        }
    }

## Clap, command line argument parser

[github](https://github.com/kbknapp/clap-rs/)

Cargo dependencies

    clap = { version = "2.19", features = ["yaml"]}

Include, macro_use for load_yaml! macro

    #[macro_use] extern crate clap;
    use clap::{App, ArgMatches};

Load yaml file containing rules 

    let yaml = load_yaml!("cli.yml");

Get matches based on yaml rules

    let matches = App::from_yaml(yaml).get_matches();

Routing based on matching rules 

    match matches.subcommand() {
        ("add", Some(sub_matches)) => add(sub_matches, &_log),
        ("", None) => println!("No subcommand was used"),
        _ => println!("No!"),
    }

Extracting value from submatches

    fn add(args: &ArgMatches, logger: &slog::Logger) {
        let podcast = Podcast { id: 0, url: args.value_of("url").unwrap().to_string(), label: args.value_of("label").unwrap_or("").to_string()};

## Slog, logging

- [Documentation](https://docs.rs/slog/2.0.5/slog/)

Cargo dependencies

    slog = "2.0.5"
    slog-term = "2.0.1"
    slog-async = "2.0.1"

Include

    #[macro_use] extern crate slog;
    extern crate slog_term;
    extern crate slog_async;
    use slog::Drain;

Initialisation

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let _log = slog::Logger::root(drain, o!());

Usage, where logger is &slog::Logger

    trace!(logger, "Adding podcast: {}", podcast.url;);
    info!(logger, "Podcast added: {}", podcast.url;);

## Rusqlite for sqlite3

- [Documentation](https://jgallagher.github.io/rusqlite/rusqlite/index.html)

Cargo dependencies

    rusqlite = "0.11.0"   

Include

    extern crate rusqlite;

