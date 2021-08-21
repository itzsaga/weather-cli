use reqwest::blocking::Client;
use std::env::args;
use std::env::var;

fn main() {
    let api_token = var("API_TOKEN")
        .expect("expected there to be an api token");

    let mut arg_iterator = args();
    arg_iterator.next();
    let args: String = arg_iterator.collect();

    let client = Client::new();

    let response = client
        .get("https://api.waqi.info/search/")
        .query(&[("token", api_token), ("keyword", args)])
        .send()
        .expect("a successful request")
        .json::<serde_json::Value>()
        .expect("expected the body to be json");

    dbg!(response);
}
