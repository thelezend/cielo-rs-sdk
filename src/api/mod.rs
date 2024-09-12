//! The `api` module provides functionality for interacting with the Cielo API.
//!
//! This module contains the main `CieloApi` struct and its implementation,
//! as well as submodules for specific API endpoints.
//!
//! # Examples
//!
//! ```
//! use cielo_rs_sdk::CieloApi;
//!
//! let api = CieloApi::new("your_api_key", Some(100), Some(1000), Some(3)).unwrap();
//! ```
//!
pub mod feed;

use std::time::Duration;

use reqwest::header;

use crate::{constants, reqwest_ext::get_retry_strategy};

#[derive(Debug, Clone)]
/// Represents the Cielo API client.
///
/// This struct allows you to create a new client instance and interact with the Cielo API.
/// It includes a [`ClientWithMiddleware`] for handling HTTP requests
///
/// [`ClientWithMiddleware`]: https://docs.rs/reqwest-middleware/latest/reqwest_middleware/struct.ClientWithMiddleware.html
pub struct CieloApi {
    /// The HTTP client with middleware for handling requests and retries.
    client: reqwest_middleware::ClientWithMiddleware,
    // default_params: HashMap<String, String>,
}

impl CieloApi {
    /// Creates a new instance of CieloApi.
    ///
    /// This function initializes a new CieloApi client with the provided API key and optional retry parameters.
    ///
    /// # Arguments
    ///
    /// * `api_key` - A string slice that holds the API key.
    /// * `min_retry_interval` - An optional minimum retry interval in milliseconds.
    /// * `max_retry_interval` - An optional maximum retry interval in milliseconds.
    /// * `max_retries` - An optional maximum number of retries.
    ///
    /// # Errors
    ///
    /// This function returns a `crate::Error` if the client cannot be built.
    ///
    /// # Examples
    ///
    /// ```
    /// use cielo_rs_sdk::CieloApi;
    ///
    /// let api = CieloApi::new("your_api_key", Some(100), Some(1000), Some(3)).unwrap();
    /// ```
    pub fn new(
        api_key: &str,
        min_retry_interval: Option<u64>,
        max_retry_interval: Option<u64>,
        max_retries: Option<u32>,
    ) -> Result<Self, crate::Error> {
        let mut headers = header::HeaderMap::new();

        let mut auth_value = header::HeaderValue::from_str(api_key).unwrap();
        auth_value.set_sensitive(true);
        headers.insert("X-API-KEY", auth_value);

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(10))
            .build()?;

        let retry_s = get_retry_strategy(
            min_retry_interval.unwrap_or(constants::MIN_RETRY_INTERVAL),
            max_retry_interval.unwrap_or(constants::MAX_RETRY_INTERVAL),
            max_retries.unwrap_or(constants::MAX_RETRIES),
        );

        let client = reqwest_middleware::ClientBuilder::new(client)
            .with(retry_s)
            .build();

        Ok(Self { client })
    }
}
