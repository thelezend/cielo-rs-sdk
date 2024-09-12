//! This module contains the data structures for the Cielo feed endpoint response.

use serde::{Deserialize, Serialize};

/// Represents an item in the feed.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Item {
    /// A swap transaction.
    Swap(Swap),
    /// A liquidity pool (LP) transaction.
    Lp(Lp),
    /// A transfer transaction.
    Transfer(Transfer),
    /// A lending transaction.
    Lending(Lending),
    /// An NFT minting transaction.
    NftMint(NftMint),
    /// An NFT trading transaction.
    NftTrade(NftTrade),
    /// An NFT transfer transaction.
    NftTransfer(NftTransfer),
    /// An NFT lending transaction.
    NftLending(NftLending),
    /// A bridge transaction.
    Bridge(Bridge),
    /// A contract interaction.
    ContractInteraction(ContractInteraction),
    /// A wrap transaction.
    Wrap(Wrap),
    /// A Sudo Pool transaction.
    SudoPool(SudoPool),
    /// A reward transaction.
    Reward(Reward),
    /// A staking transaction.
    Staking(Staking),
    /// A perpetual transaction.
    Perp(Perp),
    /// A flashloan transaction.
    Flashloan(Flashloan),
    /// A contract creation transaction.
    ContractCreation(ContractCreation),
    /// An NFT liquidation transaction.
    NftLiquidation(NftLiquidation),
    /// An option event transaction.
    Option(OptionType),
    /// An NFT sweep transaction.
    NftSweep(NftSweep),
}

/// Represents a swap transaction.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Swap {
    /// The wallet address that initiated the swap transaction.
    pub wallet: String,
    /// A readable label for the wallet address.
    pub wallet_label: String,
    /// The unique hash identifier of the swap transaction.
    pub tx_hash: String,
    /// Specifies the type of transaction (e.g., token swap).
    pub tx_type: String,
    /// The blockchain network where the swap transaction occurred.
    pub chain: String,
    /// A numerical index or identifier for the transaction.
    pub index: u32,
    /// The timestamp marking when the transaction was executed.
    pub timestamp: u64,
    /// The block number on the blockchain where this transaction is recorded.
    pub block: u64,
    /// The decentralized exchange where the swap transaction took place.
    pub dex: String,
    /// The originating wallet address of the transaction.
    pub from: String,
    /// The destination wallet address of the transaction.
    pub to: String,
    /// The address of the first token involved in the swap.
    pub token0_address: String,
    /// The amount of the first token involved in the swap.
    pub token0_amount: f64,
    /// The USD value of the first token amount at the time of the swap.
    pub token0_amount_usd: f64,
    /// The name of the first token involved in the swap.
    pub token0_name: String,
    /// The price of the first token in USD.
    pub token0_price_usd: f64,
    /// The symbol of the first token involved in the swap.
    pub token0_symbol: String,
    /// A link to the icon of the token involved in the transaction.
    pub token0_icon_link: String,
    /// The address of the second token involved in the swap.
    pub token1_address: String,
    /// The amount of the second token involved in the swap.
    pub token1_amount: f64,
    /// The USD value of the second token amount at the time of the swap.
    pub token1_amount_usd: f64,
    /// The name of the second token involved in the swap.
    pub token1_name: String,
    /// The price of the second token in USD.
    pub token1_price_usd: f64,
    /// The symbol of the second token involved in the swap.
    pub token1_symbol: String,
    /// A link to the icon of the second token involved in the transaction.
    pub token1_icon_link: String,
    /// Indicates if this was the first interaction between involved wallet addresses.
    pub first_interaction: bool,
    /// Market cap and liquidity details of the tokens involved.
    pub token_market_cap: Option<TokenMarketCap>,
}

/// Represents the market cap and liquidity details of a token.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenMarketCap {
    /// The address of the token.
    pub token_address: String,
    /// The market capitalization of the token.
    pub market_cap: f64,
    /// The liquidity of the token.
    pub liquidity: f64,
}

/// Represents a liquidity pool (LP) transaction.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Lp {
    /// The wallet address participating in the LP transaction.
    pub wallet: String,
    /// A human-readable label or name associated with the wallet, such as an ENS name.
    pub wallet_label: String,
    /// The unique transaction hash identifying this specific LP transaction.
    pub tx_hash: String,
    /// Indicates the type of transaction, in this case, liquidity pool (LP) related.
    pub tx_type: String,
    /// The blockchain network (e.g., Ethereum, Optimism) where this transaction takes place.
    pub chain: String,
    /// A numerical index or identifier for the transaction.
    pub index: u32,
    /// The timestamp marking when the transaction was executed.
    pub timestamp: u64,
    /// The block number on the blockchain where this transaction is recorded.
    pub block: u64,
    /// The name of the decentralized exchange where the LP transaction occurred.
    pub dex: String,
    /// The originating wallet address for the transaction.
    pub from: String,
    /// Specifies the nature of the LP transaction, such as 'add' or 'remove'.
    pub r#type: String,
    /// The address of the first token involved in the LP transaction.
    pub token0_address: String,
    /// The amount of the first token involved in the transaction.
    pub token0_amount: f64,
    /// The USD value of the first token amount at the time of the transaction.
    pub token0_amount_usd: f64,
    /// The name of the first token involved in the transaction.
    pub token0_name: String,
    /// The price of the first token in USD.
    pub token0_price_usd: f64,
    /// The symbol of the first token involved in the transaction.
    pub token0_symbol: String,
    /// A link to the icon of the token involved in the transaction.
    pub token0_icon_link: String,
    /// The address of the second token involved in the LP transaction.
    pub token1_address: String,
    /// The amount of the second token involved in the transaction.
    pub token1_amount: f64,
    /// The USD value of the second token amount at the time of the transaction.
    pub token1_amount_usd: f64,
    /// The name of the second token involved in the transaction.
    pub token1_name: String,
    /// The price of the second token in USD.
    pub token1_price_usd: f64,
    /// The symbol of the second token involved in the transaction.
    pub token1_symbol: String,
    /// A link to the icon of the second token involved in the transaction.
    pub token1_icon_link: String,
    /// Indicates the lower bound of the price range for the LP position, relevant in certain types of liquidity pools.
    pub lower_bound: f64,
    /// Indicates the upper bound of the price range for the LP position, relevant in certain types of liquidity pools.
    pub upper_bound: f64,
}

/// Represents a transfer transaction.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transfer {
    /// The wallet address involved in the transfer transaction.
    pub wallet: String,
    /// A readable label for the wallet address.
    pub wallet_label: String,
    /// The unique hash identifier of the transfer transaction.
    pub tx_hash: String,
    /// Specifies the type of transaction, in this case, a token transfer.
    pub tx_type: String,
    /// The blockchain network where the transfer transaction occurred.
    pub chain: String,
    /// A numerical index or identifier for the transaction.
    pub index: u32,
    /// The timestamp marking when the transaction was executed.
    pub timestamp: u64,
    /// The block number on the blockchain where this transaction is recorded.
    pub block: u64,
    /// The originating wallet address of the transaction.
    pub from: String,
    /// The destination wallet address of the transaction.
    pub to: String,
    /// A readable version of the 'from' wallet address.
    pub from_label: String,
    /// A readable version of the 'to' wallet address.
    pub to_label: String,
    /// The USD value of the amount transferred in the transaction.
    pub amount_usd: f64,
    /// The blockchain address of the contract under which the token is registered.
    pub contract_address: String,
    /// The name of the token being transferred.
    pub name: String,
    /// The symbol of the token being transferred.
    pub symbol: String,
    /// The price of the token in USD at the time of the transaction.
    pub token_price_usd: f64,
    /// Indicates the contract standard of the token, such as ERC20.
    pub r#type: String,
    /// A URL link to the token's icon image.
    pub token_icon_link: String,
    /// Contains the market capitalization and liquidity details of the tokens involved in the swap transaction.
    pub token_market_cap: Option<TokenMarketCap>,
}

/// Represents a lending transaction.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Lending {
    /// The wallet address associated with the transaction.
    pub wallet: String,
    /// A more readable, shortened version of the wallet address.
    pub wallet_label: String,
    /// The unique identifier of the transaction, represented as a hash.
    pub tx_hash: String,
    /// Specifies the type of DeFi transaction, such as lending, borrowing, etc.
    pub tx_type: String,
    /// Indicates the blockchain network on which the transaction occurred.
    pub chain: String,
    /// A sequential index or identifier for the transaction within a particular dataset or list.
    pub index: u32,
    /// The timestamp marking when the transaction was executed.
    pub timestamp: u64,
    /// The block number in the blockchain where this transaction is recorded.
    pub block: u64,
    /// The originating wallet address for the transaction.
    pub from: String,
    /// A readable, abbreviated form of the 'from' wallet address.
    pub from_label: String,
    /// Describes the action taken in the transaction, such as 'Repaid' in a lending scenario.
    pub action: String,
    /// The smart contract address involved in the transaction.
    pub address: String,
    /// The amount of the asset involved in the transaction.
    pub amount: f64,
    /// The equivalent value of the transaction amount in USD.
    pub amount_usd: f64,
    /// The decentralized exchange or platform where the transaction occurred, such as AaveV2.
    pub dex: String,
    /// A metric specific to lending platforms, indicating the health of the loan or position.
    pub health_factor: f64,
    /// The name of the asset involved in the transaction.
    pub name: String,
    /// The DeFi platform associated with the transaction, like AaveV3.
    pub platform: String,
    /// The price of the asset in USD at the time of the transaction.
    pub price_usd: f64,
    /// The symbol of the asset involved in the transaction.
    pub symbol: String,
    /// A URL link to the icon image of the token involved in the transaction.
    pub token_icon_link: String,
}

/// Represents an NFT minting transaction.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NftMint {
    /// The wallet address involved in the NFT minting transaction.
    pub wallet: String,
    /// A more readable label or identifier for the wallet address.
    pub wallet_label: String,
    /// The unique hash identifier of the NFT minting transaction.
    pub tx_hash: String,
    /// Specifies the type of transaction, in this case, NFT minting.
    pub tx_type: String,
    /// The blockchain network on which the minting transaction was conducted.
    pub chain: String,
    /// A numerical index or identifier for the transaction.
    pub index: u32,
    /// The timestamp marking when the transaction was executed.
    pub timestamp: u64,
    /// The block number on the blockchain where this transaction is recorded.
    pub block: u64,
    /// The originating wallet address of the transaction.
    pub from: String,
    /// The destination wallet address of the transaction.
    pub to: String,
    /// A readable version of the 'from' wallet address.
    pub from_label: String,
    /// A readable version of the 'to' wallet address.
    pub to_label: String,
    /// A thumbnail image URL of the NFT involved in the transaction.
    pub thumbnail: String,
    /// A full image URL of the NFT.
    pub image: String,
    /// The number of items of the NFT minted in the transaction.
    pub amount: f64,
    /// The blockchain address of the contract under which the NFT is minted.
    pub contract_address: String,
    /// The type of contract used for the NFT, such as ERC721.
    pub contract_type: String,
    /// The transaction fee incurred for minting the NFT.
    pub fee: f64,
    /// The name of the NFT minted.
    pub nft_name: String,
    /// The symbol associated with the NFT.
    pub nft_symbol: String,
    /// The unique token ID of the minted NFT.
    pub nft_token_id: String,
    /// The symbol of the currency used in the transaction (e.g., ETH, MATIC).
    pub currency_symbol: String,
    /// Indicates the contract standard of the NFT, such as ERC721.
    pub r#type: String,
    /// The value of the transaction. For minting, this is often zero since the NFT is being created.
    pub value: f64,
    /// The equivalent USD value of the transaction.
    pub value_usd: f64,
}

/// Represents an NFT trading transaction.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NftTrade {
    /// The wallet address involved in the NFT trading transaction.
    pub wallet: String,
    /// A readable label for the wallet address.
    pub wallet_label: String,
    /// The unique hash identifier of the NFT trading transaction.
    pub tx_hash: String,
    /// Specifies the type of transaction, in this case, NFT trading.
    pub tx_type: String,
    /// The blockchain network where the trading transaction occurred.
    pub chain: String,
    /// A numerical index or identifier for the transaction.
    pub index: u32,
    /// The timestamp marking when the transaction was executed.
    pub timestamp: u64,
    /// The block number on the blockchain where this transaction is recorded.
    pub block: u64,
    /// The originating wallet address of the transaction.
    pub from: String,
    /// The destination wallet address of the transaction.
    pub to: String,
    /// A thumbnail image URL of the NFT involved in the transaction.
    pub thumbnail: String,
    /// A full image URL of the NFT.
    pub image: String,
    /// Describes the action taken in the NFT trade, such as 'buy' or 'sell'.
    pub action: String,
    /// The blockchain contract address associated with the NFT.
    pub contract: String,
    /// The marketplace where the NFT trade occurred, such as OpenSea.
    pub marketplace: String,
    /// The blockchain address of the NFT involved in the trade.
    pub nft_address: String,
    /// The name of the NFT traded.
    pub nft_name: String,
    /// The symbol associated with the NFT.
    pub nft_symbol: String,
    /// The unique token ID of the NFT involved in the trade.
    pub nft_token_id: String,
    /// The price at which the NFT was traded.
    pub price: f64,
    /// The equivalent USD value of the NFT trade.
    pub price_usd: f64,
    /// The profit earned from the trade. This may be zero in some transactions.
    pub profit: f64,
    /// The symbol of the currency used in the trade, such as WETH or ETH.
    pub currency_symbol: String,
    /// The wallet address of the buyer in the trade.
    pub buyer: String,
    /// The wallet address of the seller in the trade.
    pub seller: String,
    /// The token type used in the transaction.
    pub token: String,
    /// Indicates whether this was the first interaction between the buyer and seller.
    pub first_interaction: bool,
    /// Specifies if the transaction involved a bid being accepted.
    pub bid_accepted: bool,
}

/// Represents an NFT transfer transaction.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NftTransfer {
    /// The wallet address involved in the NFT transfer transaction.
    pub wallet: String,
    /// A readable label for the wallet address.
    pub wallet_label: String,
    /// The unique hash identifier of the NFT transfer transaction.
    pub tx_hash: String,
    /// Specifies the type of transaction, in this case, NFT transfer.
    pub tx_type: String,
    /// The blockchain network where the transfer transaction occurred.
    pub chain: String,
    /// A numerical index or identifier for the transaction.
    pub index: u32,
    /// The timestamp marking when the transaction was executed.
    pub timestamp: u64,
    /// The block number on the blockchain where this transaction is recorded.
    pub block: u64,
    /// The originating wallet address of the transaction.
    pub from: String,
    /// The destination wallet address of the transaction.
    pub to: String,
    /// A readable version of the 'from' wallet address.
    pub from_label: String,
    /// A readable version of the 'to' wallet address.
    pub to_label: String,
    /// A thumbnail image URL of the NFT involved in the transaction.
    pub thumbnail: String,
    /// A full image URL of the NFT.
    pub image: String,
    /// The blockchain address of the contract under which the NFT is registered.
    pub contract_address: String,
    /// The type of contract used for the NFT, such as ERC721.
    pub contract_type: String,
    /// The transaction fee incurred for the transfer of the NFT.
    pub fee: f64,
    /// The name of the NFT being transferred.
    pub nft_name: String,
    /// The symbol associated with the NFT.
    pub nft_symbol: String,
    /// The unique token ID of the NFT involved in the transfer.
    pub nft_token_id: String,
    /// Indicates the contract standard of the NFT, such as ERC721.
    pub r#type: String,
    /// The value of the NFT at the time of the transfer, typically in the native cryptocurrency of the blockchain.
    pub value: f64,
}

/// Represents an NFT lending transaction.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NftLending {
    /// The wallet address involved in the NFT lending transaction.
    pub wallet: String,
    /// A more readable label or identifier for the wallet address.
    pub wallet_label: String,
    /// The unique hash identifier of the NFT lending transaction.
    pub tx_hash: String,
    /// Specifies the type of transaction, in this case, NFT lending.
    pub tx_type: String,
    /// The blockchain network on which the transaction was conducted.
    pub chain: String,
    /// A numerical index or identifier for the transaction.
    pub index: u32,
    /// The timestamp marking when the transaction was executed.
    pub timestamp: u64,
    /// The block number on the blockchain where this transaction is recorded.
    pub block: u64,
    /// The originating wallet address of the transaction.
    pub from: String,
    /// The destination wallet address of the transaction.
    pub to: String,
    /// A readable version of the 'from' wallet address.
    pub from_label: String,
    /// A readable version of the 'to' wallet address.
    pub to_label: String,
    /// A thumbnail image URL of the NFT involved in the transaction.
    pub thumbnail: String,
    /// A full image URL of the NFT.
    pub image: String,
    /// Describes the action taken in the NFT lending transaction, such as 'lend'.
    pub action: String,
    /// The blockchain address of the currency used in the transaction.
    pub currency_address: String,
    /// The symbol of the currency used in the transaction.
    pub currency_symbol: String,
    /// The interest rate applied in the NFT lending transaction.
    pub interest: f64,
    /// The blockchain address of the NFT involved in the transaction.
    pub nft_address: String,
    /// The name of the NFT.
    pub nft_name: String,
    /// The symbol of the NFT.
    pub nft_symbol: String,
    /// The platform or service through which the NFT lending transaction was conducted.
    pub platform: String,
    /// The unique identifier for the specific NFT within its collection.
    pub nft_token_id: String,
    /// The price at which the NFT was lent or transacted.
    pub price: f64,
    /// The equivalent USD value of the transaction price.
    pub price_usd: f64,
    /// Specifies the terms of the NFT lending agreement.
    pub terms: f64,
    /// Indicates whether the transaction involved refinancing of the NFT.
    pub refinance: bool,
}

/// Represents a bridge transaction.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bridge {
    /// The wallet address involved in the transaction.
    pub wallet: String,
    /// A shortened, more readable version of the wallet address.
    pub wallet_label: String,
    /// The unique hash identifier of the transaction.
    pub tx_hash: String,
    /// The type of transaction.
    pub tx_type: String,
    /// The blockchain on which the transaction occurred, like 'ethereum'.
    pub chain: String,
    /// A numeric index or identifier for the transaction.
    pub index: u32,
    /// The timestamp when the transaction occurred.
    pub timestamp: u64,
    /// The block number on the blockchain in which the transaction is included.
    pub block: u64,
    /// The originating wallet address of the transaction.
    pub from: String,
    /// The destination wallet address of the transaction.
    pub to: String,
    /// A shortened, more readable version of the originating wallet address.
    pub from_label: String,
    /// A shortened, more readable version of the destination wallet address.
    pub to_label: String,
    /// The address of the token involved in the transaction.
    pub token_address: String,
    /// The name of the token involved in the transaction.
    pub token_name: String,
    /// The symbol of the token involved in the transaction.
    pub token_symbol: String,
    /// A link to the icon of the token involved in the transaction.
    pub token_icon_link: String,
    /// The amount of the token involved in the transaction.
    pub amount: f64,
    /// The equivalent amount in USD of the tokens involved in the transaction.
    pub amount_usd: f64,
    /// The originating chain of the transaction, in cases of cross-chain activities.
    pub from_chain: String,
    /// The destination chain of the transaction, in cases of cross-chain activities.
    pub to_chain: String,
    /// The platform or service used for the transaction.
    pub platform: String,
    /// The price of the token at the time of the transaction.
    pub price: f64,
    /// Specifies the nature of the transaction, like 'withdraw', 'deposit', etc.
    pub r#type: String,
}

/// Represents a contract interaction.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContractInteraction {
    /// The wallet address participating in the contract interaction.
    pub wallet: String,
    /// A human-readable label or name associated with the wallet, such as an ENS name.
    pub wallet_label: String,
    /// The unique transaction hash identifying this specific contract interaction.
    pub tx_hash: String,
    /// Indicates the type of transaction.
    pub tx_type: String,
    /// The blockchain network (e.g., Ethereum, Optimism) where this transaction takes place.
    pub chain: String,
    /// A numerical index or identifier for the transaction.
    pub index: u32,
    /// The timestamp marking when the transaction was executed.
    pub timestamp: u64,
    /// The block number on the blockchain where this transaction is recorded.
    pub block: u64,
    /// The originating wallet address for the transaction.
    pub from: String,
    /// The destination wallet address for the transaction.
    pub to: String,
    /// The address of the smart contract involved in the interaction.
    pub contract_address: String,
    /// A human-readable label or name associated with the smart contract.
    pub contract_label: String,
}

/// Represents a wrap transaction.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Wrap {
    /// The wallet address participating in the wrap transaction.
    pub wallet: String,
    /// A human-readable label or name associated with the wallet, such as an ENS name.
    pub wallet_label: String,
    /// The unique transaction hash identifying this specific wrap transaction.
    pub tx_hash: String,
    /// Indicates the type of transaction, in this case, wrap related.
    pub tx_type: String,
    /// The blockchain network (e.g., Ethereum, Optimism) where this transaction takes place.
    pub chain: String,
    /// A numerical index or identifier for the transaction.
    pub index: u32,
    /// The timestamp marking when the transaction was executed.
    pub timestamp: u64,
    /// The block number on the blockchain where this transaction is recorded.
    pub block: u64,
    /// The decentralized exchange where the wrap transaction occurred.
    pub dex: String,
    /// The originating wallet address for the transaction.
    pub from: String,
    /// The destination wallet address for the transaction.
    pub to: String,
    /// The action describing the wrap process (e.g., wrap or unwrap).
    pub action: String,
    /// The amount of tokens wrapped in the transaction.
    pub amount: f64,
    /// The equivalent amount in USD of the wrapped tokens.
    pub amount_usd: f64,
    /// The address of the smart contract involved in the interaction.
    pub contract_address: String,
    /// The name of the token wrapped in the transaction.
    pub name: String,
    /// The symbol of the token wrapped in the transaction.
    pub symbol: String,
    /// The price of the token in USD at the time of the transaction.
    pub token_price_usd: f64,
    /// The type of token wrapped in the transaction.
    pub token_type: String,
    /// A link to the icon of the token involved in the transaction.
    pub token_icon_link: String,
}

/// Represents a Sudo Pool transaction.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SudoPool {
    /// The wallet address participating in the Sudo Pool transaction.
    pub wallet: String,
    /// A human-readable label or name associated with the wallet, such as an ENS name.
    pub wallet_label: String,
    /// The unique transaction hash identifying this specific Sudo Pool transaction.
    pub tx_hash: String,
    /// Indicates the type of transaction, in this case, liquidity pool (LP) related.
    pub tx_type: String,
    /// The blockchain network (e.g., Ethereum, Optimism) where this transaction takes place.
    pub chain: String,
    /// A numerical index or identifier for the transaction.
    pub index: u32,
    /// The timestamp marking when the transaction was executed.
    pub timestamp: u64,
    /// The block number on the blockchain where this transaction is recorded.
    pub block: Option<u64>,
    /// The decentralized exchange where the Sudo Pool transaction occurred.
    pub dex: Option<String>,
    /// The originating wallet address for the transaction.
    pub from: String,
    /// The address of the NFT contract involved in the interaction.
    pub nft_address: String,
    /// The amount of NFTs involved in the transaction.
    pub nft_amount: u32,
    /// The price of the NFT in the transaction.
    pub nft_price: f64,
    /// The symbol of the NFT in the transaction.
    pub nft_symbol: String,
    /// The destination wallet address for the transaction.
    pub to: String,
    /// The address of the first token in the LP pair.
    pub token0_address: String,
    /// The amount of the first token in the LP pair.
    pub token0_amount: f64,
    /// The equivalent amount in USD of the first token in the LP pair.
    pub token0_amount_usd: f64,
    /// The name of the first token in the LP pair.
    pub token0_name: String,
    /// The price of the first token in the LP pair in USD.
    pub token0_price_usd: f64,
    /// The symbol of the first token in the LP pair.
    pub token0_symbol: String,
    /// A link to the icon of the first token involved in the transaction.
    pub token0_icon_link: String,
}

/// Represents a reward transaction.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Reward {
    /// The wallet address participating in the reward transaction.
    pub wallet: String,
    /// A human-readable label or name associated with the wallet, such as an ENS name.
    pub wallet_label: String,
    /// The unique transaction hash identifying this specific reward transaction.
    pub tx_hash: String,
    /// Indicates the type of transaction, in this case, reward related.
    pub tx_type: String,
    /// The blockchain network (e.g., Ethereum, Optimism) where this transaction takes place.
    pub chain: String,
    /// A numerical index or identifier for the transaction.
    pub index: u32,
    /// The timestamp marking when the transaction was executed.
    pub timestamp: u64,
    /// The block number on the blockchain where this transaction is recorded.
    pub block: u64,
    /// The address of the token involved in the transaction.
    pub address: String,
    /// The amount of tokens involved in the transaction.
    pub amount: f64,
    /// The equivalent amount in USD of the tokens involved in the transaction.
    pub amount_usd: f64,
    /// The originating wallet address for the transaction.
    pub from: String,
    /// The name of the token involved in the transaction.
    pub name: String,
    /// The price of the token in USD.
    pub price_usd: f64,
    /// The symbol of the token involved in the transaction.
    pub symbol: String,
}

/// Represents a staking transaction.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Staking {
    /// The wallet address participating in the staking transaction.
    pub wallet: String,
    /// A human-readable label or name associated with the wallet, such as an ENS name.
    pub wallet_label: String,
    /// The unique transaction hash identifying this specific staking transaction.
    pub tx_hash: String,
    /// Indicates the type of transaction, in this case, staking related.
    pub tx_type: String,
    /// The blockchain network (e.g., Ethereum, Optimism) where this transaction takes place.
    pub chain: String,
    /// A numerical index or identifier for the transaction.
    pub index: u32,
    /// The timestamp marking when the transaction was executed.
    pub timestamp: u64,
    /// The block number on the blockchain where this transaction is recorded.
    pub block: u64,
    /// The originating wallet address for the transaction.
    pub from: String,
    /// The destination wallet address for the transaction.
    pub to: String,
    /// A human-readable label or name associated with the originating wallet.
    pub from_label: String,
    /// A human-readable label or name associated with the destination wallet.
    pub to_label: String,
    /// The amount of tokens staked in the transaction.
    pub amount: f64,
    /// The equivalent amount in USD of the staked tokens.
    pub amount_usd: f64,
    /// The price of the token in USD.
    pub token_price_usd: f64,
    /// The address of the smart contract involved in the interaction.
    pub contract_address: String,
    /// The symbol of the token staked in the transaction.
    pub symbol: String,
    /// The name of the token staked in the transaction.
    pub name: String,
    /// The action taken in the staking transaction (e.g., 'stake', 'unstake').
    pub action: String,
}

/// Represents a Perpetual transaction.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Perp {
    /// The wallet address participating in the Perpetual transaction.
    pub wallet: String,
    /// A human-readable label or name associated with the wallet, such as an ENS name.
    pub wallet_label: String,
    /// The unique transaction hash identifying this specific Perpetual transaction.
    pub tx_hash: String,
    /// Indicates the type of transaction, in this case, Perpetual related.
    pub tx_type: String,
    /// The blockchain network (e.g., Ethereum, Optimism) where this transaction takes place.
    pub chain: String,
    /// A numerical index or identifier for the transaction.
    pub index: u32,
    /// The timestamp marking when the transaction was executed.
    pub timestamp: u64,
    /// The block number on the blockchain where this transaction is recorded.
    pub block: u64,
    /// The action taken in the Perpetual event.
    pub action: String,
    /// The equivalent amount in USD of the tokens involved in the transaction.
    pub amount_usd: f64,
    /// The average price of the tokens involved in the transaction.
    pub average_price: f64,
    /// The address of the base token involved in the transaction.
    pub base_token_address: String,
    /// The amount of base tokens involved in the transaction.
    pub base_token_amount: f64,
    /// The symbol of the base token involved in the transaction.
    pub base_token_symbol: String,
    /// The decentralized exchange where the Perpetual transaction occurred.
    pub dex: String,
    /// The originating wallet address for the transaction.
    pub from: String,
    /// Indicates whether the transaction was a liquidation.
    pub liquidation: bool,
    /// The price at which the liquidation occurred.
    pub liquidation_price: f64,
    /// The destination wallet address for the transaction.
    pub to: String,
    /// The direction of the trade in the Perpetual transaction (e.g., 'long', 'short').
    pub trade_direction: String,
    /// Additional details about the Perpetual transaction.
    pub perp_details: String,
    /// The address of the first token in the LP pair.
    pub token0_address: String,
    /// The amount of the first token in the LP pair.
    pub token0_amount: f64,
    /// The equivalent amount in USD of the first token in the LP pair.
    pub token0_amount_usd: f64,
    /// The name of the first token in the LP pair.
    pub token0_name: String,
    /// The price of the first token in the LP pair in USD.
    pub token0_price_usd: f64,
    /// The symbol of the first token in the LP pair.
    pub token0_symbol: String,
    /// A link to the icon of the first token involved in the transaction.
    pub token0_icon_link: String,
    /// The address of the second token in the LP pair.
    pub token1_address: String,
    /// The amount of the second token in the LP pair.
    pub token1_amount: f64,
    /// The equivalent amount in USD of the second token in the LP pair.
    pub token1_amount_usd: f64,
    /// The name of the second token in the LP pair.
    pub token1_name: String,
    /// The price of the second token in the LP pair in USD.
    pub token1_price_usd: f64,
    /// The symbol of the second token in the LP pair.
    pub token1_symbol: String,
    /// A link to the icon of the second token involved in the transaction.
    pub token1_icon_link: String,
    /// The realized profit and loss of the Perpetual transaction.
    pub realized_pnl: f64,
    /// Indicates whether the Perpetual transaction involves an NFT.
    pub is_nft_perp: bool,
    /// The size of the position in the Perpetual transaction.
    pub position_size: Option<f64>,
    /// The equivalent amount in USD of the position size.
    pub position_size_usd: Option<f64>,
    /// The leverage used in the Perpetual transaction.
    pub leverage: Option<f64>,
    /// The unrealized profit and loss of the Perpetual transaction.
    pub unrealized_pnl: Option<f64>,
}

/// Represents a flashloan transaction.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Flashloan {
    /// The wallet address participating in the flashloan transaction.
    pub wallet: String,
    /// A human-readable label or name associated with the wallet, such as an ENS name.
    pub wallet_label: String,
    /// The unique transaction hash identifying this specific flashloan transaction.
    pub tx_hash: String,
    /// Indicates the type of transaction, in this case, flashloan related.
    pub tx_type: String,
    /// The blockchain network (e.g., Ethereum, Optimism) where this transaction takes place.
    pub chain: String,
    /// A numerical index or identifier for the transaction.
    pub index: u32,
    /// The timestamp marking when the transaction was executed.
    pub timestamp: u64,
    /// The block number on the blockchain where this transaction is recorded.
    pub block: u64,
    /// The address of the token involved in the transaction.
    pub address: String,
    /// The amount of tokens involved in the transaction.
    pub amount: f64,
    /// The equivalent amount in USD of the tokens involved in the transaction.
    pub amount_usd: f64,
    /// The decentralized exchange (DEX) where the flashloan transaction took place.
    pub dex: String,
    /// The originating wallet address for the transaction.
    pub from: String,
    /// The health factor of the wallet after the flashloan transaction.
    pub health_factor: f64,
    /// The name of the token involved in the transaction.
    pub name: String,
    /// The platform where the flashloan transaction took place.
    pub platform: String,
    /// The price of the token in USD.
    pub price_usd: f64,
    /// The symbol of the token involved in the transaction.
    pub symbol: String,
    /// A link to the icon of the token involved in the transaction.
    pub token_icon_link: String,
}

/// Represents a contract creation transaction.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContractCreation {
    /// The wallet address participating in the contract creation transaction.
    pub wallet: String,
    /// A human-readable label or name associated with the wallet, such as an ENS name.
    pub wallet_label: String,
    /// The unique transaction hash identifying this specific contract creation transaction.
    pub tx_hash: String,
    /// Indicates the type of transaction, in this case, contract creation.
    pub tx_type: String,
    /// The blockchain network (e.g., Ethereum, Optimism) where this transaction takes place.
    pub chain: String,
    /// A numerical index or identifier for the transaction.
    pub index: u32,
    /// The timestamp marking when the transaction was executed.
    pub timestamp: u64,
    /// The block number on the blockchain where this transaction is recorded.
    pub block: u64,
    /// The equivalent amount in USD of the wrapped tokens.
    pub amount_usd: f64,
    /// The address of the smart contract involved in the interaction.
    pub contract_address: String,
    /// The originating wallet address for the transaction.
    pub from: String,
    /// A human-readable label or name associated with the originating wallet.
    pub from_label: String,
}

/// Represents an NFT liquidation transaction.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NftLiquidation {
    /// The wallet address participating in the NFT liquidation transaction.
    pub wallet: String,
    /// A human-readable label or name associated with the wallet, such as an ENS name.
    pub wallet_label: String,
    /// The unique transaction hash identifying this specific NFT liquidation transaction.
    pub tx_hash: String,
    /// Indicates the type of transaction, in this case, NFT liquidation related.
    pub tx_type: String,
    /// The blockchain network (e.g., Ethereum, Optimism) where this transaction takes place.
    pub chain: String,
    /// A numerical index or identifier for the transaction.
    pub index: u32,
    /// The timestamp marking when the transaction was executed.
    pub timestamp: u64,
    /// The block number on the blockchain where this transaction is recorded.
    pub block: u64,
    /// The address of the NFT contract involved in the interaction.
    pub contract_address: String,
    /// The address of the currency involved in the transaction.
    pub currency_address: String,
    /// The symbol of the currency involved in the transaction.
    pub currency_symbol: String,
    /// The decentralized exchange where the NFT liquidation transaction occurred.
    pub dex: String,
    /// The originating wallet address for the transaction.
    pub from: String,
    /// The address of the NFT contract involved in the interaction.
    pub nft_address: String,
    /// The name of the NFT in the transaction.
    pub nft_name: String,
    /// The symbol of the NFT in the transaction.
    pub nft_symbol: String,
    /// The platform where the NFT liquidation transaction took place.
    pub platform: String,
    /// The price of the NFT in the transaction.
    pub price: f64,
    /// The price of the NFT in USD.
    pub price_usd: f64,
    /// The destination wallet address for the transaction.
    pub to: String,
    /// The unique identifier of the NFT in the transaction.
    pub token_id: String,
}

/// Represents an option event transaction.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OptionType {
    /// The wallet address participating in the option event.
    pub wallet: String,
    /// A human-readable label or name associated with the wallet, such as an ENS name.
    pub wallet_label: String,
    /// The unique transaction hash identifying this specific option event.
    pub tx_hash: String,
    /// Indicates the type of transaction, in this case, option-related.
    pub tx_type: String,
    /// The blockchain network (e.g., Ethereum, Optimism) where this transaction takes place.
    pub chain: String,
    /// A numerical index or identifier for the transaction.
    pub index: u32,
    /// The timestamp marking when the transaction was executed.
    pub timestamp: u64,
    /// The block number on the blockchain where this transaction is recorded.
    pub block: u64,
    /// The action taken in the option event.
    pub action: String,
    /// The amount of tokens involved in the transaction.
    pub amount: f64,
    /// The asset involved in the option event.
    pub asset: String,
    /// The decentralized exchange (DEX) where the option event took place.
    pub dex: String,
    /// The direction of the option event (e.g., call or put).
    pub direction: String,
    /// The expiry date of the option.
    pub expiry: String,
    /// The originating wallet address for the transaction.
    pub from: String,
    /// The price of the option in USD.
    pub option_price_usd: f64,
    /// The status of the option position.
    pub position_status: String,
    /// The spot price of the asset in USD.
    pub spot_price_usd: f64,
    /// The status of the option event.
    pub status: String,
    /// The strike price of the option in USD.
    pub strike_price_usd: f64,
    /// The destination wallet address for the transaction.
    pub to: String,
    /// The type of option event (e.g., exercise, sell).
    pub r#type: String,
}

/// Represents an NFT sweep transaction.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NftSweep {
    /// The wallet address involved in the NFT sweep transaction.
    pub wallet: String,
    /// A readable label for the wallet address.
    pub wallet_label: String,
    /// The unique hash identifier of the NFT sweep transaction.
    pub tx_hash: String,
    /// Specifies the type of transaction, in this case, NFT sweep related.
    pub tx_type: String,
    /// The blockchain network where the sweep transaction occurred.
    pub chain: String,
    /// A numerical index or identifier for the transaction.
    pub index: u32,
    /// The timestamp marking when the transaction was executed.
    pub timestamp: u64,
    /// The block number on the blockchain where this transaction is recorded.
    pub block: u64,
    /// The originating wallet address of the transaction.
    pub from: String,
    /// The destination wallet address of the transaction.
    pub to: String,
    /// A thumbnail image URL of the NFT involved in the transaction.
    pub thumbnail: String,
    /// A full image URL of the NFT.
    pub image: String,
    /// Describes the action taken in the NFT sweep, such as 'buy' or 'sell'.
    pub action: String,
    /// The blockchain contract address associated with the NFT.
    pub contract: String,
    /// The marketplace where the NFT sweep occurred, such as OpenSea.
    pub marketplace: String,
    /// The blockchain address of the NFT involved in the sweep.
    pub nft_address: String,
    /// The name of the NFT swept.
    pub nft_name: String,
    /// The symbol associated with the NFT.
    pub nft_symbol: String,
    /// The unique token ID of the NFT involved in the sweep.
    pub nft_token_id: String,
    /// The price at which the NFT was traded.
    pub price: f64,
    /// The equivalent USD value of the NFT sweep.
    pub price_usd: f64,
    /// The profit earned from the trade. This may be zero in some transactions.
    pub profit: f64,
    /// The symbol of the currency used in the trade, such as WETH or ETH.
    pub currency_symbol: String,
    /// The wallet address of the buyer in the trade.
    pub buyer: String,
    /// The wallet address of the seller in the trade.
    pub seller: String,
    /// The token type used in the transaction.
    pub token: String,
    /// Indicates whether this was the first interaction between the buyer and seller.
    pub first_interaction: bool,
    /// Specifies if the transaction involved a bid being accepted.
    pub bid_accepted: bool,
}
