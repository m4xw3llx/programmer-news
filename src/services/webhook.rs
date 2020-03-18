use crate::config::*;
use hyper::{header, Body, Client, Method, Request, Response};
use hyper_tls::HttpsConnector;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

pub enum Webhooks {
    LOBSTE,
}

impl Webhooks {
    fn value(&self) -> &str {
        use Webhooks::*;
        match *self {
            LOBSTE => LOBSTE_HOOK,
        }
    }
}

pub async fn request_web_hooks(hooks: Webhooks) -> Result<Response<Body>> {
    print!("begin call hooks");
    let uri = hooks.value();

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let test_content = r#"{"content": "data"}"#;
    let request = Request::builder()
        .method(Method::POST)
        .uri(uri)
        .header(header::CONTENT_TYPE, "application/json")
        .body(test_content.into())
        .unwrap();

    let web_res = client.request(request).await?;
    let body = Body::from(web_res.into_body());
    Ok(Response::new(body))
}
