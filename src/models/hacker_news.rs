use serde::Serialize;
#[derive(Serialize)]

pub struct HackerNews {
    pub by: String,
    pub id: i32,
    pub score: i32,
    pub title: String,
    pub news_type: String,
    pub url: String,
    pub time: i64,
}
