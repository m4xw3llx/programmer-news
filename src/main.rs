extern crate hyper;

use hyper::body::HttpBody as _;
use hyper::Client;
use hyper_tls::HttpsConnector;
use tokio::io::{stdout, AsyncWriteExt as _};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let uri = "http://httpbin.org/ip".parse()?;

    let mut resp = client.get(uri).await?;
    println!("Response: {}", resp.status());

    while let Some(chunk) = resp.body_mut().data().await {
        stdout().write_all(&chunk?).await?;
    }

    Ok(())
}
