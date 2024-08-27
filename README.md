# Cielo Rust SDK

Cielo Rust SDK is a Rust library for interacting with the [Cielo API](https://developer.cielo.finance/reference/getfeed). It provides a convenient way to fetch and filter feed data from the Cielo platform.

> **IMPORTANT: This is not an official SDK. It is a personal project for learning Rust and API development. The included endpoints are primarily for personal use in my own projects and may not cover all features of the Cielo API. However, your contributions are welcome!**

## Features

- Fetch feed data with various filters
- Retry strategy for API requests
- Easy integration with async Rust applications

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
cielo-rs-sdk = { git = "https://github.com/thelezend/cielo-rs-sdk.git", branch = "main"}
```

Sometimes your `Cargo.lock` file will point to an older version of the SDK. To fix this, you can run `cargo update` command to update the SDK to the latest version.

## Usage

### Initialization

To use the Cielo Rust SDK, you need to initialize the `CieloApi` with your API key:

```rust
use cielo_rs_sdk::CieloApi;

let api_key = "your_api_key";
let cielo_api = CieloApi::new(&api_key, None, None, None).expect("Failed to initialize CieloApi");
```

### Fetching Feed Data

You can fetch feed data using the `get_feed` method with various filters:

```rust
use cielo_rs_sdk::api::feed;

let filters = feed::Filters {
    wallet: Some("your_wallet_address".to_string()),
    limit: Some(10),
    ..Default::default()
};

let feed_data = cielo_api.get_feed(filters).await?;
```

## Configuration

You can configure the retry strategy by providing optional parameters when creating the `CieloApi` instance:

- `min_retry_interval`: Minimum retry interval in milliseconds.
- `max_retry_interval`: Maximum retry interval in milliseconds.
- `max_retries`: Maximum number of retries.

Example:

```rust
let cielo_api = CieloApi::new(api_key, Some(500), Some(1000), Some(3)).expect("Failed to create CieloApi");
```

## Testing

To run the tests, set the `CIELO_API_KEY` environment variable and use cargo test:

```sh
export CIELO_API_KEY=your_api_key
cargo test
```

## Contribution

Your contributions are welcome! To contribute, please follow these steps:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature/your-feature`).
3. Make your changes.
4. Commit your changes (`git commit -m "feat: add your feature"`).
5. Push to the branch (`git push origin feature/your-feature`).
6. Create a pull request to the main branch.

Please ensure your code follows the existing style and includes tests for any new functionality.

## License

This project is licensed under the MIT License.
