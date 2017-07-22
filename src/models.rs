use std::str; // needed for from_utf8
use std::str::FromStr; // needed for FromStr trait on Channel
use rss::{Channel, Error};
use url::Url;
use diesel::sqlite::SqliteConnection;
use chrono;
use diesel::prelude::*;
use diesel::insert;
use diesel::update;
use schema::*;

#[derive(Debug, Clone, Queryable, Serialize, Deserialize, Identifiable)]
#[table_name = "subscription"]
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
    pub url:  &'a str,
}

impl Subscription {
    pub fn from_xml_feed(&self, connection: &SqliteConnection, body: Vec<u8>, as_downloaded: bool) -> Result<&str, Error> {

        let channel = match Channel::from_str(str::from_utf8(&body).unwrap()) { // parse rss into channel
            Err(e) =>  return Err(e),
            Ok(channel) => channel
        };

        match channel.last_build_date() {
            None => (),
            Some(date) => 
                if Some(date.to_string()) == self.clone().last_build_date {
                    return Ok("Already up to date")
                }
        };

        let _ = update(self)
            .set((subscription::label.eq(&channel.title()),
                 subscription::last_build_date.eq(&channel.last_build_date())))
            .execute(connection)
            .expect(&format!("Unable to find post {}", self.id));

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

            let podcast = NewPodcast { subscription_id: self.id, url: url.as_str(), filename: filename.as_str(), title: item.title().unwrap(), downloaded: as_downloaded  as i32};
            let results = podcast::table.filter(podcast::url.eq(&url.as_str())).load::<Podcast>(connection) ;
            if results.unwrap().len() == 0 {
                insert(&podcast).into(podcast::table).execute(connection).expect("Error saving");
                println!("    New podcast added: {:?}", podcast.filename);
            }
        }

        Ok("updated")
    }
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Podcast {
    pub id: i32,
    pub subscription_id: i32,
    pub url:  String,
    pub filename: String,
    pub title: String,
    pub content_text: Option<String>,
    pub downloaded: i32,
    pub downloaded_at: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime
}

#[derive(Insertable)]
#[table_name="podcast"]
pub struct NewPodcast<'a> {
    pub subscription_id: i32,
    pub url:  &'a str,
    pub filename: &'a str,
    pub title: &'a str,
    pub downloaded: i32,
}

#[derive(Debug, Queryable, Serialize, Deserialize, Insertable)]
#[table_name = "config"]
pub struct Config {
    pub key: String, 
    pub value: Option<String>,
}
