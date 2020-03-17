extern crate chrono;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

type Url = String;

#[derive(Deserialize, Serialize, Debug)]
pub struct LobsterArticle {
    pub short_id: String,
    pub short_id_url: Url,
    pub created_at: DateTime<Utc>,
    pub title: String,
    pub url: Url,
    pub score: i32,
    pub upvotes: i32,
    pub downvotes: i32,
    pub comment_count: i32,
    pub description: String,
    pub comments_url: Url,
    pub tags: Vec<String>,
}
