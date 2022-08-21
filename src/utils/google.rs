#![allow(unused)]
extern crate percent_encoding;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_google_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    format!("https://google.com/search?q={}", encoded_query)
}

#[cfg(test)]
mod tests {
    use crate::utils::google::construct_google_search_url;

    #[test]
    fn test_construct_google_search_url() {
        let fake_url = "hello";
        assert_eq!(
            construct_google_search_url(fake_url),
            String::from("https://google.com/search?q=hello")
        )
    }

    fn test_construct_google_search_url_with_encoding() {
        let fake_url = "foo <bar>";
        assert_eq!(
            construct_google_search_url(fake_url),
            String::from("https://google.com/search?q=foo <bar>")
        )
    }
}
