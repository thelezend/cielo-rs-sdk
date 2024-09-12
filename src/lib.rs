/*!
# Cielo Rust SDK

Cielo Rust SDK is a Rust library for interacting with the [Cielo API](https://developer.cielo.finance/reference/getfeed). It provides a convenient way to fetch and filter feed data from the Cielo platform.

<div class="warning">
This is not an official SDK. It is a personal project for learning Rust and API development. The included endpoints are primarily for personal use in my own projects and may not cover all features of the Cielo API.
</div>

## Features

- Configurable retry strategy for API requests.
- Fetch feed data with various filters.

## Configuration

You can configure the retry strategy by providing optional parameters when creating the [`CieloApi`] instance:

- `min_retry_interval`: Minimum retry interval in milliseconds.
- `max_retry_interval`: Maximum retry interval in milliseconds.
- `max_retries`: Maximum number of retries.

## Usage

<div class="warning">
The feed data is linked to your account, so the filters you set will be applied to your existing feed data. This means that if you filter by a wallet address, the wallet address should already be in your watchlist for this to work.
</div>

### Creating a CieloApi Instance

```
use cielo_rs_sdk::CieloApi;

let api_key = "your_api_key";
let client = CieloApi::new(api_key, None, None, None)
    .expect("Failed to create CieloApi");
```

### Fetching Feed Data

```no_run
# use cielo_rs_sdk::{CieloApi, api};
# #[tokio::main]
# async fn main() {
# let api_key = "your_api_key";
# let cielo_api = CieloApi::new(api_key, None, None, None).unwrap();
let filters = api::feed::Filters {
    wallet: Some("your_wallet_address".to_string()),
    limit: Some(10),
    list_id: None,
    chains: Some(vec!["solana".to_string()]),
    tx_types: Some(vec![api::feed::TxType::Swap]),
    tokens: None,
    min_usd: Some(100),
    new_trades: Some(true),
    start_from: None,
    from_timestamp: None,
    to_timestamp: None,
    include_market_cap: Some(true),
};
let feed = cielo_api.get_feed(filters).await.unwrap();
# }
*/

#![warn(
    missing_docs,
    rustdoc::unescaped_backticks,
    clippy::missing_errors_doc,
    clippy::missing_docs_in_private_items
)]

mod constants;
mod error;
mod reqwest_ext;

pub mod api;
pub mod models;

// Re-export the CieloApi struct
pub use api::CieloApi;
pub use error::Error;
