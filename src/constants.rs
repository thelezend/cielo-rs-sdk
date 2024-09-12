//! This module contains constants used throughout the project.

/// Base URL for the Cielo feed API
pub const URL: &str = "https://feed-api.cielo.finance/api/v1/";

/// Minimum retry interval in milliseconds
pub const MIN_RETRY_INTERVAL: u64 = 500;
/// Maximum retry interval in milliseconds
pub const MAX_RETRY_INTERVAL: u64 = 1000;
/// Maximum number of retries
pub const MAX_RETRIES: u32 = 3;
