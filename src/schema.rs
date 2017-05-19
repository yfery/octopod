// derive allow use of unwrap() on Podcast
#[derive(Debug)]
pub struct Podcast {
    pub id: i32,
    pub url:  String,
    pub label: String,
}
