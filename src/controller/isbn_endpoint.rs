use rocket::tokio::task::spawn_blocking;
use rocket::{get, serde::json::Json};
use isbn::{Isbn, IsbnError};

use crate::repository::Book;
use crate::common::{RequestMode, RequestClient};
use crate::service::thai_isbn;

#[get("/isbn?<isbn>&<mode>")]
pub async fn isbn_handler(isbn: String, mode: Option<RequestMode>) -> Json<Book> {
    spawn_blocking(move || { isbn_handler_private(isbn, mode) }).await
        .unwrap()
}

fn isbn_handler_private(isbn: String, mode: Option<RequestMode>) -> Json<Book> {
    let mut isbn_client = RequestClient::new();

    // Sets request mode if present
    if mode.is_some() {
        isbn_client.set_mode(mode.unwrap());
    }

    // Parse ISBN
    let parsed_isbn: Result<Isbn, IsbnError> = isbn.parse();
    if parsed_isbn.is_err() {
        unimplemented!("To implement error handling");
    }

    // Tells the isbn request client to send a request and returns a JSON to the user
    Json(isbn_client.query_isbn(parsed_isbn.unwrap()))
}