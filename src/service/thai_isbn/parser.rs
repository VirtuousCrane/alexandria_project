use isbn::{Isbn, IsbnError};
use reqwest::Url;
use scraper::{Html, Selector};
use std::collections::HashMap;

use super::Book;

const BOOK_ISBN_BASE_URL: &str = "https://e-service.nlt.go.th/ISBNReq/";

// Given an HTML document from the Thai ISBN Database, return a URL of the book's info
pub fn get_book_info_url(document: &str) -> Url {
    let html_doc = Html::parse_document(document);
    let tr_selector = Selector::parse("tr").unwrap();
    let a_selector = Selector::parse("a").unwrap();
    let mut result = Url::parse(BOOK_ISBN_BASE_URL)
        .expect("Failed to parse base url");

    for tr_element in html_doc.select(&tr_selector) {
        let a_element = tr_element.select(&a_selector).next();
        match a_element {
            Some(a) => {
                let href = a.value().attr("href").unwrap();
                result = result.join(href).unwrap();
            },
            _ => (),
        };
    }

    result
}

pub fn parse_document(html_doc: &Html) -> Book {
    let input_selector = Selector::parse("input").unwrap();
    let mut value_map: HashMap<&str, &str> = HashMap::new();
    for input in html_doc.select(&input_selector) {
        let input_value = input.value();
        let id = input_value.attr("id").unwrap();
        let value = input_value.attr("value");

        match value {
            Some(v) => value_map.insert(id, v),
            None => value_map.insert(id, "")
        };
    }

    let raw_isbn = String::from(*value_map.get("ISBNCode").unwrap());
    let isbn: Isbn = raw_isbn.parse()
        .expect("Failed to parse ISBN");
    
    let raw_pages = *value_map.get("NoOfPage").unwrap();
    let raw_edition = *value_map.get("EditionNote").unwrap();

    let pages = match raw_pages.parse::<i32>() {
        Ok(i) => i,
        _ => 0,
    };

    let edition = match raw_edition.parse::<i32>() {
        Ok(i) => i,
        _ => 0,
    };
    
    Book {
        isbn,
        title: String::from(*value_map.get("ReqBookTitleName").unwrap()),
        author: String::from(*value_map.get("AuthorNames").unwrap()),
        translator: String::from(*value_map.get("TranslatorNames").unwrap()),
        illustrator: String::from(*value_map.get("IllustratorNames").unwrap()),
        series: String::from(*value_map.get("SeriesName").unwrap()),
        publisher: String::from(*value_map.get("PubName").unwrap()),
        pages,
        edition
    }
}