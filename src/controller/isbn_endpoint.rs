use rocket::get;
use super::common::{RequestMode, RequestClient};
use isbn::{Isbn, IsbnError};

#[get("/isbn?<isbn>&<mode>")]
pub async fn isbn_handler(isbn: String, mode: Option<RequestMode>) -> String {
    let mut isbn_client = RequestClient::new();

    // Sets request mode if present
    if mode.is_some() {
        isbn_client.set_mode(mode.unwrap());
    }

    // Parse ISBN
    let parsed_isbn: Result<Isbn, IsbnError> = isbn.parse();
    if parsed_isbn.is_err() {
        return String::from("Failed to Parse ISBN");
    }

    isbn_client.query_isbn(parsed_isbn.unwrap())
        .await
}