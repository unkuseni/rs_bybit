use serde::{Deserialize, Serialize};

/// Request for getting delivery records
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeliveryRecordRequest<'a> {
    /// Product type `inverse`(inverse futures), `linear`(USDT/USDC futures), `option`
    pub category: &'a str,
    /// Symbol name, like `BTCUSDT`, uppercase only
    pub symbol: Option<&'a str>,
    /// The start timestamp (ms)
    #[serde(rename = "startTime")]
    pub start_time: Option<u64>,
    /// The end timestamp (ms)
    #[serde(rename = "endTime")]
    pub end_time: Option<u64>,
    /// Expiry date. `25MAR22`. Default: return all
    #[serde(rename = "expDate")]
    pub exp_date: Option<&'a str>,
    /// Limit for data size per page. [`1`, `50`]. Default: `20`
    pub limit: Option<u32>,
    /// Cursor
    pub cursor: Option<&'a str>,
}
