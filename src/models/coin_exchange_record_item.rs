use serde::{Deserialize, Serialize};

/// Coin exchange record item
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CoinExchangeRecordItem {
    /// The currency to convert from
    #[serde(rename = "fromCoin")]
    pub from_coin: String,
    /// The amount to convert from
    #[serde(rename = "fromAmount")]
    pub from_amount: String,
    /// The currency to convert to
    #[serde(rename = "toCoin")]
    pub to_coin: String,
    /// The amount to convert to
    #[serde(rename = "toAmount")]
    pub to_amount: String,
    /// Exchange rate
    #[serde(rename = "exchangeRate")]
    pub exchange_rate: String,
    /// Exchange created timestamp (sec)
    #[serde(rename = "createdTime")]
    pub created_time: String,
    /// Exchange transaction ID
    #[serde(rename = "exchangeTxId")]
    pub exchange_tx_id: String,
}
