extern crate hyper;

mod config;
mod models;
mod services;

use services::webhook::Webhooks;

#[tokio::main]
async fn main() {
    use services::lobste::LobsteAPI::*;

    let result = services::lobste::fetch_lobste(HOTTEST).await;
    match result {
        Ok(articles) => {
            let hooks_result = services::webhook::request_web_hooks(Webhooks::LOBSTE).await;
            match hooks_result {
                Ok(_) => println!("Success"),
                Err(err) => println!("Main execute error is {:?}", err),
            };
        }
        Err(error) => println!("Main execute error is {:?}", error),
    }
}
