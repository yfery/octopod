use rusqlite::{Row};

// derive allow use of unwrap() on Podcast
#[derive(Debug)]
pub struct Podcast {
    pub id: i32,
    pub url:  String,
    pub label: String,
}

impl Podcast {
    pub fn map(row: &Row) -> Podcast {
        Podcast {
            id: row.get(0),
            url: row.get(1),
            label: row.get(2)
        }
    }
}
