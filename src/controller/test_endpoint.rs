use rocket::get;
use super::common::RequestClient;

#[get("/ping")]
pub fn ping() -> &'static str {
    "PONG"
}

#[get("/hello?<name>")]
pub fn hello(name: Option<String>) -> String {
    match name {
        Some(n) => format!("Hello, {}!", n),
        None => String::from("Hello, world!"),
    }
}

#[get("/ip")]
pub async fn get_ip() -> String {
    let isbn_client = RequestClient::new();
    isbn_client.ip().await
}