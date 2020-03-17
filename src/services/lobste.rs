extern crate hyper;

use hyper::body::HttpBody as _;
use hyper::Client;
use hyper_tls::HttpsConnector;

mod lobste_service {

    // async fn fetch_hottest_lobste() -> Result<()> {
    //     let https = HttpsConnector::new();
    //     let client = Client::builder().build::<_, hyper::Body>(https);
    // }
}
