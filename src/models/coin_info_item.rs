use serde::{Deserialize, Serialize};

use super::coin_chain_info::CoinChainInfo;

/// Coin information item
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CoinInfoItem {
    /// Coin name
    pub name: String,
    /// Coin symbol
    pub coin: String,
    /// Maximum withdraw amount per transaction
    #[serde(rename = "remainAmount")]
    pub remain_amount: String,
    /// Chain information
    pub chains: Vec<CoinChainInfo>,
}
