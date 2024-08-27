pub mod feed;

use serde::{Deserialize, Serialize};

/// A generic response struct used for API responses.
///
/// # Type Parameters
///
/// * `T` - The type of the result field.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Response<T> {
    /// The status of the API response.
    pub status: String,
    /// The data field containing the response data.
    pub data: ResponseData<T>,
}

/// A struct representing the data field in the API response.
///
/// # Type Parameters
///
/// * `T` - The type of the items field.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseData<T> {
    /// A vector of items of type `T`.
    pub items: Vec<T>,
    /// The paging information for the response data.
    pub paging: Paging,
}

/// A struct representing the paging information in the API response.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Paging {
    /// The total number of rows in the current page.
    pub total_rows_in_page: u64,
    /// Indicates if there is a next page.
    pub has_next_page: bool,
    /// The cursor for the next page, if available.
    pub next_cursor: Option<String>,
}
