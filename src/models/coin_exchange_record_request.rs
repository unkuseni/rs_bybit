use serde::{Deserialize, Serialize};

/// Request for getting coin exchange records
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CoinExchangeRecordRequest<'a> {
    /// The currency to convert from, uppercase only. e.g,`BTC`
    #[serde(rename = "fromCoin")]
    pub from_coin: Option<&'a str>,
    /// The currency to convert to, uppercase only. e.g,`USDT`
    #[serde(rename = "toCoin")]
    pub to_coin: Option<&'a str>,
    /// Limit for data size per page. [`1`, `50`]. Default: `10`
    pub limit: Option<u32>,
    /// Cursor
    pub cursor: Option<&'a str>,
}
