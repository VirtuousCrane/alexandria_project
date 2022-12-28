use reqwest::{Client, Url};
use rocket::FromFormField;
use isbn::{Isbn, IsbnError};

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
    async fn send_request(&self, uri: Url) -> String {
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
    pub async fn query_isbn(&self, isbn: Isbn) -> String {
        // Hyphenates ISBN
        let parsed_isbn = isbn
            .hyphenate()
            .unwrap();

        // Checks Request Mode
        if self.mode.is_none() {
            panic!("No mode selected");
        }

        let mode = self.mode.as_ref().unwrap();
        match mode {
            RequestMode::THAI_ISBN => {
                let uri = Url::parse_with_params(
                    THAI_ISBN_BASE_URL,
                    &[("Keyword", parsed_isbn.as_str())])
                    .expect("Failed to parse URL");
                self.send_request(uri)
                    .await
            },
            _ => String::from("NOT YET IMPLEMENTED")
        }
    }

}