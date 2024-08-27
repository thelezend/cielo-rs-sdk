mod common;

use std::env;
use cielo_rs_sdk::CieloApi;

/// Test to ensure CieloApi is initialized correctly.
#[tokio::test]
async fn test_cielo_api_initialization() {
    common::setup();

    // Retrieve the API key from the environment variables.
    let api_key = env::var("CIELO_API_KEY").expect("CIELO_API_KEY must be set");

    // Initialize the CieloApi with the provided API key.
    let cielo_api = CieloApi::new(&api_key, None, None, None);
    
    // Assert that the CieloApi was initialized successfully.
    assert!(
        cielo_api.is_ok(),
        "Failed to initialize CieloApi: {:?}",
        cielo_api.err()
    );
}
