use rusqlite::Row;
use rusqlite::Connection;
use std::str; // needed for from_utf8
use std::str::FromStr; // needed for FromStr trait on Channel
use rss::{Channel, Error};
use url::Url;
use hyper;

// derive allow use of unwrap() on Subscription
#[derive(Debug, Clone)]
pub struct Subscription {
    pub id: i64,
    pub url:  String,
    pub label: String,
    pub last_build_date: String,
}

impl Subscription {
    pub fn map(row: &Row) -> Subscription {
        Subscription {
            id: row.get(0),
            url: row.get(1),
            label: row.get(2),
            last_build_date: row.get(3)
        }
    }

    pub fn delete(&self, connection: &Connection) {
        connection.execute("delete from subscription where id = ?1", &[&self.id]).unwrap();
        connection.execute("delete from podcast where subscription_id = ?1", &[&self.id]).unwrap();
    }

    pub fn from_xml_feed(&self, connection: &Connection, body: hyper::Chunk, as_downloaded: bool) -> Result<&str, Error> {
        let mut previous_insert_rowid: i64 = 0;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Podcast {
    pub id: i64,
    pub subscription_id: i64,
    pub url:  String,
    pub filename: String,
    pub title: String,
    pub content_text: String,
}

impl Podcast {
    pub fn map(row: &Row) -> Podcast {
        Podcast {
            id: row.get(0),
            subscription_id: row.get(1),
            url: row.get(2),
            filename: row.get(3),
            title: row.get(4),
            content_text: row.get(5)
        }
    }
}
