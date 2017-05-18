use rusqlite::Connection;
use podcast::Podcast;

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
}
