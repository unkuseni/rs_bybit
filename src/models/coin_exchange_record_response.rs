use crate::prelude::*;
use serde::{Deserialize, Serialize};

/// Coin exchange record response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CoinExchangeRecordResponse {
    /// Next page cursor
    #[serde(rename = "nextPageCursor")]
    pub next_page_cursor: String,
    /// List of coin exchange records
    #[serde(rename = "orderBody")]
    pub order_body: Vec<CoinExchangeRecordItem>,
}
