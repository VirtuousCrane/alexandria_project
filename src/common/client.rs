use std::sync::{Mutex, Arc};

use reqwest::{Client, Url};
use rocket::FromFormField;
use scraper::Html;
use isbn::Isbn;

use crate::repository::Book;

use super::service::thai_isbn;

const THAI_ISBN_BASE_URL: &str = "https://e-service.nlt.go.th/ISBNReq/ListSearchPub?KeywordTypeKey=4";

pub struct RequestClient {
    reqwest_client: Client,
    mode: Arc<Mutex<Option<RequestMode>>>,
}

#[derive(FromFormField)]
#[derive(Clone)]
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

unsafe impl Send for RequestClient {}
unsafe impl Sync for RequestClient {}

impl RequestClient {

    /// Creates a new RequestClient
    pub fn new() -> Self {
        RequestClient {
            reqwest_client: Client::new(),
            mode: Arc::new(Mutex::new(None)),
        }
    }

    /// Sets the request mode
    pub fn set_mode(&self, mode: Option<RequestMode>) {
        let mode_clone = Arc::clone(&self.mode);
        let mut mode_guard = mode_clone.lock()
            .expect("Locked wrapped data");
        *mode_guard = mode;
    }

    /// Sends a request
    pub async fn send_request(&self, uri: Url) -> String {
        let res = self.reqwest_client.get(uri)
            .send()
            .await
            .expect("Error sending request");
        let content = res.text()
            .await
            .expect("Failed to parse request to text");

        content
    }

    /// Sends a request to httpbin.org/ip
    pub async fn ip(&self) -> String {
        let uri = Url::parse("http://www.httpbin.org/ip").unwrap();
        self.send_request(uri).await
    }

    /// Queries an ISBN
    pub async fn query_isbn(&self, isbn: Isbn) -> Book {
        // Hyphenates ISBN
        let parsed_isbn = isbn
            .hyphenate()
            .unwrap();

        // Checks Request Mode
        let mode_clone = Arc::clone(&self.mode);
        let mode_obj = (*mode_clone.lock().unwrap()).clone();
        if mode_obj.is_none() {
            unimplemented!("To implement Error handling");
        }

        //let mode = self.mode.as_ref().unwrap();
        match mode_obj.unwrap() {
            RequestMode::THAI_ISBN => {
                // Sending the first request
                let uri = Url::parse_with_params(
                    THAI_ISBN_BASE_URL,
                    &[("Keyword", parsed_isbn.as_str())])
                    .expect("Failed to parse URL");
                let mut document = self.send_request(uri).await;
                
                // Getting the book info URL and send a new request
                let book_info_url = thai_isbn::get_book_info_url(&document);
                document = self.send_request(book_info_url).await;

                // Parsing the Html document into a Book object
                let html_doc = Html::parse_document(&document);
                let book = thai_isbn::parse_document(&html_doc);

                book
            },
            _ => unimplemented!("To implement other endpoints")
        }
    }

}