use cielo_rs_sdk::{api, CieloApi};

mod common;

#[tokio::main]
async fn main() {
    common::setup();

    // Retrieve the API key from the environment variables.
    let api_key = std::env::var("CIELO_API_KEY").expect("CIELO_API_KEY must be set");

    // Initialize the CieloApi with the provided API key.
    let cielo_api = CieloApi::new(&api_key, None, None, None).unwrap();

    // Fetch feed data using the `get_feed` method in `CieloApi`.
    let response_items = cielo_api
        .get_feed(api::feed::Filters::default())
        .await
        .unwrap();

    println!("{:#?}", response_items);
}
