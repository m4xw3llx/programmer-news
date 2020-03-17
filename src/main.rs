extern crate hyper;

mod config;
mod models;
mod services;

#[tokio::main]
async fn main() {
    use services::lobste::LobsteAPI::*;

    let result = services::lobste::fetch_lobste(HOTTEST).await;
    match result {
        Ok(_) => println!("Success"),
        Err(error) => println!("Main execute error is {:?}", error),
    };
}
