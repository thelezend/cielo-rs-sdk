use crate::{constants, models, reqwest_ext};
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
}

/// Transaction types for filtering the feed.
#[derive(Display, Debug, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum TxType {
    Bridge,
    ContractCreation,
    ContractInteraction,
    Flashloan,
    Lending,
    Lp,
    NftLending,
    NftLiquidation,
    NftMint,
    NftSweep,
    NftTrade,
    NftTransfer,
    Option,
    Perp,
    Reward,
    Staking,
    SudoPool,
    Swap,
    Transfer,
    Wrap,
}

impl CieloApi {
    /// Fetches the feed based on the provided filters.
    ///
    /// # Arguments
    ///
    /// * `filters` - A Filters struct containing various filter options.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<models::feed::Item>, reqwest_ext::Error>` - A result containing a vector of feed items or an error.
    pub async fn get_feed(
        &self,
        filters: Filters,
    ) -> Result<Vec<models::feed::Item>, reqwest_ext::Error> {
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
            return Err(reqwest_ext::Error::StatusNot200(response.text().await?));
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
