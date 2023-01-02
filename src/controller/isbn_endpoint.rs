use rocket::State;
use rocket::{get, serde::json::Json};
use isbn::{Isbn, IsbnError};

use crate::repository::Book;
use crate::common::{RequestMode, RequestClient};

#[get("/isbn?<isbn>&<mode>")]
pub async fn isbn_handler(isbn: String, mode: Option<RequestMode>, isbn_client: &State<RequestClient>) -> Json<Book> {
    // Parse ISBN
    let parsed_isbn: Result<Isbn, IsbnError> = isbn.parse();
    if parsed_isbn.is_err() {
        unimplemented!("To implement error handling");
    }

    // Tells the isbn request client to send a request and returns a JSON to the user
    Json(isbn_client.query_isbn(parsed_isbn.unwrap(), mode).await)
}