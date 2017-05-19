use rusqlite::{ Connection, MappedRows, Row };
use schema::*;

pub struct Db {
    conn: Connection
}

impl Db {
    // Associatied functions (without self, 'static method') https://doc.rust-lang.org/book/method-syntax.html#chaining-method-calls
    pub fn new(path: String) -> Result<Db, String> {
        let connection = match Connection::open(path) {
            Ok(connection) => connection,
            Err(err) => return Err(err.to_string()),
        };

        connection.execute("create table if not exists podcast (
                    id integer primary key autoincrement,
                    url text not null,
                    label text not null, 
                    created_at timestamp default current_timestamp)", &[]).unwrap();

        Ok(Db {
            conn: connection,
        })
    }

    // Chaining method with &self
    pub fn subscribe_to_podcast(&self, podcast: &Podcast) {
        self.conn.execute("insert into podcast (url, label)
                    values (?1, ?2)",
                    &[&podcast.url, &podcast.label]).unwrap();
    }

    //pub fn podcast_list(&self) -> MappedRows<fn(&Row) -> Podcast> {
    pub fn podcast_list(&self) -> Result {
        let mut stmt = self.conn.prepare("select id, url, label from podcast").unwrap();
        let mut rows = stmt.query(&[]);

        return rows;

        let podcasts = stmt.query_map(&[], |row| {
            Podcast {
                id: row.get(0),
                url: row.get(1),
                label: row.get(2)
            }
        }).unwrap();

        //return podcasts; 
        for podcast in podcasts {
            match podcast {
                Ok(podcast) => println!("{}", podcast.url),
                Err(e) => {},
            }
        };
    }

        /*
        */
}
