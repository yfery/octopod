use std::net::TcpListener;
use std::process;
use rusqlite::Connection;
use schema::*;
use curl::easy::Easy;
use std::time::Duration;
use pbr::{ProgressBar, Units};
use std::fs::File;
use std::path::{Path};
use std::io::Write; // needed for read_to_string trait

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

pub fn getdownloaddir(connection: &Connection) -> String {
    let mut stmt = connection.prepare("select value from config where key = 'downloaddir'").unwrap();
    let mut rows = stmt.query(&[]).unwrap();
    while let Some(row) = rows.next() {
        return row.unwrap().get(0);
    }
    return "/tmp/".to_string();
}

pub fn get_podcast(connection: &Connection, id: i64) -> Option<Podcast>  {
    let mut stmt = connection.prepare("select id, subscription_id, url, filename, title, content_text from podcast where id = ?1").unwrap();
    let mut podcasts = stmt.query_map(&[&id], Podcast::map).unwrap();
    match podcasts.next() {
        Some(podcast) => Some(podcast.unwrap()),
        None => None
    }
}

pub fn get_podcasts(connection: &Connection, query: &str) -> Option<Vec<Podcast>>  {
    let mut stmt = connection.prepare(query).unwrap();
    if !stmt.exists(&[]).unwrap() {
        return None;
    }
    let rows = stmt.query_map(&[], Podcast::map).unwrap();

    let mut podcasts = Vec::new();
    for podcast in rows {
        podcasts.push(podcast.unwrap());
    }
    Some(podcasts)
}

pub fn get_pending_podcasts(connection: &Connection) -> Option<Vec<Podcast>>  {
    get_podcasts(connection, "select id, subscription_id, url, filename, title, content_text from podcast where downloaded = 0")
}

pub fn get_downloaded_podcasts(connection: &Connection) -> Option<Vec<Podcast>>  {
    get_podcasts(connection, "select id, subscription_id, url, filename, title, content_text from podcast where downloaded = 1")
}

pub fn get_subscriptions(connection: &Connection) -> Option<Vec<Subscription>> {
    let mut stmt = connection.prepare("select id, url, label, coalesce(last_build_date, 'Nothing') from subscription ").unwrap();
    let rows = stmt.query_map(&[], Subscription::map).unwrap();

    let mut subscriptions = Vec::new();
    for subscription in rows {
        subscriptions.push(subscription.unwrap());
    }
    Some(subscriptions)
}
pub fn get_subscription(connection: &Connection, id: &str) -> Option<Subscription> {
    let mut stmt = connection.prepare("select id, url, label, coalesce(last_build_date, 'Nothing') from subscription where id = ?1").unwrap();
    let mut subscriptions = stmt.query_map(&[&id], Subscription::map).unwrap();
    match subscriptions.next() {
        Some(subscription) => Some(subscription.unwrap()),
        None => None
    }
}

pub fn download_podcast(connection: &Connection, podcast: Podcast) {
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

    let temp = getdownloaddir(connection) + podcast.filename.as_str();
    let path = Path::new(&temp);

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };
    curl.url(&podcast.url).unwrap();
    curl.write_function( move |data| {
        Ok(file.write(data).unwrap())
    }).unwrap();

    curl.perform().unwrap();
    connection.execute("update podcast set downloaded = 1, downloaded_at = current_timestamp where id = ?1", &[&podcast.id]).unwrap();
}
