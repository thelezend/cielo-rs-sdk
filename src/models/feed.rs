use serde::{Deserialize, Serialize};

/// Represents an item in the feed.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Item {
    /// A swap transaction.
    Swap(Swap),
}

/// Represents a swap transaction.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Swap {
    /// Wallet address.
    pub wallet: String,
    /// Label for the wallet.
    pub wallet_label: String,
    /// Transaction hash.
    pub tx_hash: String,
    /// Type of the transaction.
    pub tx_type: String,
    /// Blockchain chain.
    pub chain: String,
    /// Index of the item.
    pub index: u32,
    /// Timestamp of the transaction.
    pub timestamp: u64,
    /// Block number.
    pub block: u64,
    /// Decentralized exchange name.
    pub dex: Option<String>,
    /// Sender address.
    pub from: String,
    /// Receiver address.
    pub to: String,
    /// Address of the first token.
    pub token0_address: Option<String>,
    /// Amount of the first token.
    pub token0_amount: Option<f64>,
    /// USD value of the first token amount.
    pub token0_amount_usd: Option<f64>,
    /// USD price of the first token.
    pub token0_price_usd: Option<f64>,
    /// Name of the first token.
    pub token0_name: Option<String>,
    /// Symbol of the first token.
    pub token0_symbol: Option<String>,
    /// Address of the second token.
    pub token1_address: Option<String>,
    /// Amount of the second token.
    pub token1_amount: Option<f64>,
    /// USD value of the second token amount.
    pub token1_amount_usd: Option<f64>,
    /// USD price of the second token.
    pub token1_price_usd: Option<f64>,
    /// Name of the second token.
    pub token1_name: Option<String>,
    /// Symbol of the second token.
    pub token1_symbol: Option<String>,
    /// Indicates if this is the first interaction.
    pub first_interaction: Option<bool>,
}
