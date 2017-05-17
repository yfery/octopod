use rusqlite::Connection;

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
