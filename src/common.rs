use std::net::TcpListener;
use std::process;
use rusqlite::Connection;
use schema::*;

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

pub fn get_pending_podcasts(connection: &Connection) -> Option<Vec<Podcast>>  {
    let mut stmt = connection.prepare("select id, subscription_id, url, filename, title, content_text from podcast where downloaded = 0").unwrap();
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
