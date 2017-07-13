use std::str; // needed for from_utf8
use std::str::FromStr; // needed for FromStr trait on Channel
use rss::{Channel, Error};
use url::Url;
use diesel::sqlite::SqliteConnection;
use chrono;
use diesel::prelude::*;
use schema::subscription;

// derive allow use of unwrap() on Subscription
#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct Subscription {
    pub id: i32,
    pub url:  String,
    pub label: Option<String>,
    pub last_build_date: Option<String>,
    pub created_at: chrono::NaiveDateTime
}

#[derive(Insertable)]
#[table_name="subscription"]
pub struct NewSubscription<'a> {
    // pub id: &'a i32,
    pub url:  &'a str,
    // pub label: &'a str,
    // pub last_build_date: &'a String,
    // pub created_at: &'a chrono::NaiveDateTime
}

// TODO last_insert_rowid => https://github.com/diesel-rs/diesel/issues/771
/*
impl Subscription {
    pub fn delete(&self, connection: &SqliteConnection) {
        connection.execute("delete from subscription where id = ?1", &[&self.id]).unwrap();
        connection.execute("delete from podcast where subscription_id = ?1", &[&self.id]).unwrap();
    }

    pub fn from_xml_feed(&self, connection: &SqliteConnection, body: Vec<u8>, as_downloaded: bool) -> Result<&str, Error> {
        let mut previous_insert_rowid: i32 = 0;

        let channel = match Channel::from_str(str::from_utf8(&body).unwrap()) { // parse rss into channel
            Err(e) =>  return Err(e),
            Ok(channel) => channel
        };

        // last_build_date isn't a mandatory field 
        match channel.last_build_date() {
            None => (),
            Some(date) => 
                if date == self.last_build_date {
                    return Ok("Already up to date")
                }
        };

        connection.execute("update subscription set label = ?1, last_build_date = ?2 where id = ?3", &[&channel.title(), &channel.last_build_date(), &self.id]).unwrap(); // update podcast feed name

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

            let podcast = Podcast { id: 0, subscription_id: self.id, url: url.as_str().to_string(), filename: filename, title: item.title().unwrap().to_string(), content_text: String::new()};
            connection.execute("insert or ignore into podcast (subscription_id, url, filename, downloaded, title, content_text) values (?1, ?2, ?3, ?4, ?5, ?6)", &[&podcast.subscription_id, &podcast.url, &podcast.filename, &as_downloaded, &podcast.title, &podcast.content_text]).unwrap();
            if previous_insert_rowid != connection.last_insert_rowid() {
                println!("    New podcast added: {:?}", podcast.filename);
            }
            previous_insert_rowid = connection.last_insert_rowid();
        }

        Ok("updated")
    }
}
*/

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Podcast {
    pub id: i32,
    pub subscription_id: i32,
    pub url:  String,
    pub filename: String,
    pub title: String,
    pub content_text: String,
    pub downloaded: i32,
    pub downloaded_at: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Config {
    pub key: String, 
    pub value: Option<String>,
}
