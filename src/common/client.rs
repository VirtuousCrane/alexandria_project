use reqwest::{blocking::Client, Url};
use rocket::FromFormField;
use scraper::Html;
use isbn::Isbn;

use crate::repository::Book;

use super::service::thai_isbn;

const THAI_ISBN_BASE_URL: &str = "https://e-service.nlt.go.th/ISBNReq/ListSearchPub?KeywordTypeKey=4";

pub struct RequestClient {
    reqwest_client: Client,
    mode: Option<RequestMode>,
}

#[derive(FromFormField)]
pub enum RequestMode {
    #[field(value = "th")]
    #[field(value = "thai")]
    THAI_ISBN,

    #[field(value = "go")]
    #[field(value = "google")]
    GOOGLE,

    #[field(value = "op")]
    #[field(value = "openlib")]
    OPEN_LIBRARY,
}

impl RequestClient {

    /// Creates a new RequestClient
    pub fn new() -> Self {
        RequestClient {
            reqwest_client: Client::new(),
            mode: None,
        }
    }

    /// Sets the request mode
    pub fn set_mode(&mut self, mode: RequestMode) {
        self.mode = Some(mode);
    }

    /// Sends a request
    fn send_request(&self, uri: Url) -> String {
        let res = self.reqwest_client.get(uri)
            .send()
            .expect("Error sending request");
        let content = res.text()
            .expect("Failed to parse request to text");

        content
    }

    /// Sends a request to httpbin.org/ip
    pub fn ip(&self) -> String {
        let uri = Url::parse("http://www.httpbin.org/ip").unwrap();
        self.send_request(uri)
    }

    /// Queries an ISBN
    pub fn query_isbn(&self, isbn: Isbn) -> Book {
        // Hyphenates ISBN
        let parsed_isbn = isbn
            .hyphenate()
            .unwrap();

        // Checks Request Mode
        if self.mode.is_none() {
            unimplemented!("To implement Error handling");
        }

        let mode = self.mode.as_ref().unwrap();
        match mode {
            RequestMode::THAI_ISBN => {
                // Sending the first request
                let uri = Url::parse_with_params(
                    THAI_ISBN_BASE_URL,
                    &[("Keyword", parsed_isbn.as_str())])
                    .expect("Failed to parse URL");
                let mut document = self.send_request(uri);
                
                // Getting the book info URL and send a new request
                let html_doc = Html::parse_document(&document);
                let book_info_url = thai_isbn::get_book_info_url(&html_doc);
                document = self.send_request(book_info_url);
                
                // Parsing the Book struct
                let html_doc = Html::parse_document(&document);
                let book = thai_isbn::parse_document(&html_doc);

                book
            },
            _ => unimplemented!("To implement other endpoints")
        }
    }

}