extern crate hyper;

mod config;
mod models;
mod services;

#[tokio::main]
async fn main() {
    use services::lobste::LobsteAPI::*;

    let result = services::lobste::fetch_lobste(HOTTEST).await;
    match result {
        Ok(articles) => println!("Success: {:?}", articles),
        Err(error) => println!("Main execute error is {:?}", error),
    };
}
