mod common;

use cielo_rs_sdk::{api, CieloApi};
use std::env;

/// Test to ensure the CieloApi can fetch feed data correctly.
#[tokio::test]
async fn test_cielo_api_request() {
    common::setup();

    // Retrieve the API key from the environment variables.
    let api_key = env::var("CIELO_API_KEY").expect("CIELO_API_KEY must be set");

    // Initialize the CieloApi with the provided API key.
    let cielo_api = CieloApi::new(&api_key, None, None, None).unwrap();

    // Fetch feed data using the `get_feed` method in `CieloApi`.
    let response = cielo_api.get_feed(api::feed::Filters::default()).await;

    // Assert that the feed data was fetched successfully.
    assert!(
        response.is_ok(),
        "Failed to fetch feed: {:?}",
        response.err()
    );
}
