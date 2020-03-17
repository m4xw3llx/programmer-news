extern crate chrono;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct Auther {
    pub avatar: String,
    pub name: String,
    pub create_time: DateTime<Utc>,
    pub git_url: String,
}
