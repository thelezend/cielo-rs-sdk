//! This module provides functionality for interacting with the Cielo feed API.
//!
//! It includes structures and methods for querying and filtering feed data.

use crate::{constants, models};
use strum_macros::Display;

use super::CieloApi;

/// Filters for querying the feed.
#[derive(Debug, Clone, Default)]
pub struct Filters {
    /// Filter the feed by a specific wallet address.
    pub wallet: Option<String>,
    /// Limit the number of transactions returned in the feed. The maximum limit is 100.
    pub limit: Option<u32>,
    /// Filter transactions by a specific List ID.
    pub list_id: Option<u64>,
    /// Filter transactions by specific blockchain chains (e.g., ethereum).
    pub chains: Option<Vec<String>>,
    /// Filter transactions by types (e.g., swap, nft_trade).
    pub tx_types: Option<Vec<TxType>>,
    /// Filter transactions by specific tokens, identified by either their address or symbol.
    pub tokens: Option<Vec<String>>,
    /// Set a minimum USD value for transactions. Default is 0.
    pub min_usd: Option<u64>,
    /// Filter transactions by new trades.
    pub new_trades: Option<bool>,
    /// Set value from response 'paging.next_object_id' to get the next page.
    pub start_from: Option<String>,
    /// Filter transactions from a specific UNIX timestamp.
    pub from_timestamp: Option<u64>,
    /// Filter transactions to a specific UNIX timestamp.
    pub to_timestamp: Option<u64>,
    /// Include marketcap in the response.
    pub include_market_cap: Option<bool>,
}

/// Transaction types for filtering the feed.
///
/// This enum represents various types of transactions that can be used to filter the feed.
/// Each variant corresponds to a specific type of transaction in the Cielo ecosystem.
#[derive(Display, Debug, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum TxType {
    /// Bridge transaction between different chains or networks
    Bridge,
    /// Creation of a new smart contract
    ContractCreation,
    /// Interaction with an existing smart contract
    ContractInteraction,
    /// Flash loan transaction
    Flashloan,
    /// Lending or borrowing transaction
    Lending,
    /// Liquidity pool-related transaction
    Lp,
    /// NFT lending transaction
    NftLending,
    /// NFT liquidation transaction
    NftLiquidation,
    /// Minting of a new NFT
    NftMint,
    /// NFT sweep (bulk purchase) transaction
    NftSweep,
    /// NFT trade transaction
    NftTrade,
    /// NFT transfer transaction
    NftTransfer,
    /// Option-related transaction
    Option,
    /// Perpetual contract transaction
    Perp,
    /// Reward or yield farming transaction
    Reward,
    /// Staking transaction
    Staking,
    /// SudoPool-related transaction
    SudoPool,
    /// Token swap transaction
    Swap,
    /// Simple transfer of tokens or cryptocurrency
    Transfer,
    /// Wrapping or unwrapping of tokens (e.g., ETH to WETH)
    Wrap,
}

impl CieloApi {
    /// Fetches the feed based on the provided filters.
    ///
    /// This function sends a request to the feed endpoint with the specified filters and returns a list of feed items.
    ///
    /// # Arguments
    ///
    /// * `filters` - A Filters struct containing various filter options.
    ///
    /// # Errors
    ///
    /// This function returns a `crate::Error` if the request fails or the response status is not 200 OK.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use cielo_rs_sdk::{CieloApi, api};
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let api_key = "your_api_key";
    /// # let cielo_api = CieloApi::new(api_key, None, None, None).unwrap();
    ///     let filters = api::feed::Filters {
    ///         wallet: Some("your_wallet_address".to_string()),
    ///         limit: Some(10),
    ///         list_id: None,
    ///         chains: Some(vec!["solana".to_string()]),
    ///         tx_types: Some(vec![api::feed::TxType::Swap]),
    ///         tokens: None,
    ///         min_usd: Some(100),
    ///         new_trades: Some(true),
    ///         start_from: None,
    ///         from_timestamp: None,
    ///         to_timestamp: None,
    ///         include_market_cap: Some(true),
    ///     };
    ///     let feed = cielo_api.get_feed(filters).await.unwrap();
    /// # }
    /// ```
    pub async fn get_feed(
        &self,
        filters: Filters,
    ) -> Result<Vec<models::feed::Item>, crate::Error> {
        let url = format!("{}feed", constants::URL);

        let mut request = self.client.get(url);

        // Apply filters to the request
        if let Some(wallet) = filters.wallet {
            request = request.query(&[("wallet", wallet)]);
        }
        if let Some(limit) = filters.limit {
            request = request.query(&[("limit", limit.to_string())]);
        }
        if let Some(list_id) = filters.list_id {
            request = request.query(&[("list", list_id.to_string())]);
        }
        if let Some(chains) = filters.chains {
            request = request.query(&[("chains", chains.join(","))]);
        }
        if let Some(tx_types) = filters.tx_types {
            let tx_types_str: Vec<String> = tx_types.iter().map(|tx| tx.to_string()).collect();
            request = request.query(&[("txTypes", tx_types_str.join(","))]);
        }
        if let Some(tokens) = filters.tokens {
            request = request.query(&[("tokens", tokens.join(","))]);
        }
        if let Some(min_usd) = filters.min_usd {
            request = request.query(&[("minUSD", min_usd.to_string())]);
        }
        if let Some(new_trades) = filters.new_trades {
            request = request.query(&[("newTrades", new_trades.to_string())]);
        }
        if let Some(start_from) = filters.start_from {
            request = request.query(&[("startFrom", start_from)]);
        }
        if let Some(from_timestamp) = filters.from_timestamp {
            request = request.query(&[("fromTimestamp", from_timestamp.to_string())]);
        }
        if let Some(to_timestamp) = filters.to_timestamp {
            request = request.query(&[("toTimestamp", to_timestamp.to_string())]);
        }

        let response = request.send().await?;

        // Check if the response status is not 200 OK
        if !response.status().is_success() {
            return Err(crate::Error::StatusNot200(response.text().await?));
        }

        // Parse the response JSON into the expected structure
        let response_items = response
            .json::<models::Response<models::feed::Item>>()
            .await?
            .data
            .items;
        Ok(response_items)
    }
}
