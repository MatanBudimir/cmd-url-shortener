extern crate reqwest;

use std::collections::HashMap;
use reqwest::blocking::Client;

const BASE_URI: &str = "https://tinyurl.com/create.php";

pub fn get_short_url(url: &str) -> String {

    let mut request_body = HashMap::new();
    request_body.insert("url", url);

    let client = Client::new();

    let response = client.post(BASE_URI).json(&request_body).send().unwrap();

    return response.text().unwrap();
}