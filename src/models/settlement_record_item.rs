use serde::{Deserialize, Serialize};

/// Settlement record item for USDC perpetual and futures
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SettlementRecordItem {
    /// Symbol name
    pub symbol: String,
    /// Side: `Buy`,`Sell`
    pub side: String,
    /// Position size
    pub size: String,
    /// Settlement price
    #[serde(rename = "sessionAvgPrice")]
    pub session_avg_price: String,
    /// Mark price
    #[serde(rename = "markPrice")]
    pub mark_price: String,
    /// Realised PnL
    #[serde(rename = "realisedPnl")]
    pub realised_pnl: String,
    /// Created time (ms)
    #[serde(rename = "createdTime")]
    pub created_time: String,
}
