extern crate hyper;

use hyper::body::HttpBody as _;
use hyper::Client;
use hyper_tls::HttpsConnector;
use tokio::io::{stdout, AsyncWriteExt as _};

use crate::config::LOBSTE_HOT;
use crate::config::LOBSTE_NEW;
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
pub async fn fetch_lobste(api: LobsteAPI) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let url = api.value().parse().unwrap();
    let res = client.get(url).await?;
    println!("Response: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());
    Ok(())
}
