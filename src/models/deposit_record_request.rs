use crate::prelude::*;

/// Request for querying deposit records (on-chain)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DepositRecordRequest<'a> {
    /// Internal ID: Can be used to uniquely identify and filter the deposit.
    /// When combined with other parameters, this field takes the highest priority
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'a str>,

    /// Transaction ID: Please note that data generated before Jan 1, 2024 cannot be queried using txID
    #[serde(rename = "txID", skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<&'a str>,

    /// Coin, uppercase only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<&'a str>,

    /// The start timestamp (ms)
    /// Note: the query logic is actually effective based on **second** level
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// The end timestamp (ms)
    /// Note: the query logic is actually effective based on **second** level
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Limit for data size per page. [`1`, `50`]. Default: `50`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Cursor. Use the `nextPageCursor` token from the response to retrieve the next page of the result set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<&'a str>,
}
