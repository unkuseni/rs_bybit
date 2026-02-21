use crate::prelude::*;

/// Request for querying internal deposit records (off-chain)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InternalDepositRecordRequest<'a> {
    /// Internal transfer transaction ID
    #[serde(rename = "txID", skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<&'a str>,

    /// Start time (ms). Default value: 30 days before the current time
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time (ms). Default value: current time
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Coin name: for example, BTC. Default value: all
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<&'a str>,

    /// Cursor, used for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<&'a str>,

    /// Number of items per page, [`1`, `50`]. Default value: 50
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}
