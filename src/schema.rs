use rusqlite::Row;
use url::ParseError;
use hyper::Url;
use hyper::client::IntoUrl;
use rusqlite::Connection;

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
}

// Impl trait from hyper https://hyper.rs/hyper/v0.10.9/hyper/client/trait.IntoUrl.html
// Used in client.get()
impl IntoUrl for Subscription {
    fn into_url(self) -> Result<Url, ParseError> {
        Url::parse(&self.url)
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
