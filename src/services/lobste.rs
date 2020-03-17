extern crate hyper;

use bytes::buf::BufExt as _;
use hyper::body::*;
use hyper::Client;
use hyper_tls::HttpsConnector;
use serde_json::from_reader;
use tokio::io::{stdout, AsyncWriteExt as _};

use crate::config::LOBSTE_HOT;
use crate::config::LOBSTE_NEW;

use crate::models::lobste;

pub enum LobsteAPI {
    HOTTEST,
    NEWEST,
}

impl LobsteAPI {
    fn value(&self) -> &str {
        use LobsteAPI::*;
        match *self {
            HOTTEST => LOBSTE_HOT,
            NEWEST => LOBSTE_NEW,
        }
    }
}
pub async fn fetch_lobste(
    api: LobsteAPI,
) -> Result<Vec<lobste::LobsterArticle>, Box<dyn std::error::Error + Send + Sync>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let url = api.value().parse().unwrap();
    let res = client.get(url).await?;
    let body = hyper::body::aggregate(res).await?;

    let articles = from_reader(body.reader())?;

    Ok(articles)
}
