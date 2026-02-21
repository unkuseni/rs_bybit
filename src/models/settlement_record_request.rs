use serde::{Deserialize, Serialize};

/// Request for getting USDC session settlement records
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SettlementRecordRequest<'a> {
    /// Product type `linear`(USDC contract)
    pub category: &'a str,
    /// Symbol name, like `BTCPERP`, uppercase only
    pub symbol: Option<&'a str>,
    /// The start timestamp (ms)
    #[serde(rename = "startTime")]
    pub start_time: Option<u64>,
    /// The end timestamp (ms)
    #[serde(rename = "endTime")]
    pub end_time: Option<u64>,
    /// Limit for data size per page. [`1`, `50`]. Default: `20`
    pub limit: Option<u32>,
    /// Cursor
    pub cursor: Option<&'a str>,
}
