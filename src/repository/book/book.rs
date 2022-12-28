use isbn::Isbn;
use rocket::serde::{Serialize, self, Serializer};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Book {
    #[serde(serialize_with = "serialize_isbn")]
    pub isbn: Isbn,
    pub title: String,
    pub author: String,
    pub translator: String,
    pub illustrator: String,
    pub series: String,
    pub publisher: String,
    pub pages: i32,
    pub edition: i32,
}

/// Serializes an Isbn enum into string
fn serialize_isbn<S>(isbn: &Isbn, s: S) -> Result<S::Ok, S::Error> 
    where
        S: Serializer
{
    let isbn_str = isbn.to_string();
    s.serialize_str(&isbn_str)
}