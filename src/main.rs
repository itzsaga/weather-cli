use std::env::var;
use std::env::args;

fn main() {
    let api_token = var("API_TOKEN")
        .expect("expected there to be an api token");

    let mut arg_iterator = args();
    arg_iterator.next();
    let args: String = arg_iterator.collect();

    dbg!(args);
}
