use super::coin_info_item::CoinInfoItem;
use serde::{Deserialize, Serialize};

/// Coin information response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CoinInfoResponse {
    /// List of coin information
    pub rows: Vec<CoinInfoItem>,
}
