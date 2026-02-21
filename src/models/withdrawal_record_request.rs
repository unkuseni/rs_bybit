use crate::prelude::*;

/// Request for querying withdrawal records
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WithdrawalRecordRequest<'a> {
    /// Withdraw ID
    #[serde(rename = "withdrawID", skip_serializing_if = "Option::is_none")]
    pub withdraw_id: Option<&'a str>,

    /// Transaction hash ID
    #[serde(rename = "txID", skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<&'a str>,

    /// Coin, uppercase only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<&'a str>,

    /// Withdraw type.
    /// `0`(default): on chain.
    /// `1`: off chain.
    /// `2`: all
    #[serde(rename = "withdrawType", skip_serializing_if = "Option::is_none")]
    pub withdraw_type: Option<i32>,

    /// The start timestamp (ms)
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// The end timestamp (ms)
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Limit for data size per page. [`1`, `50`]. Default: `50`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Cursor. Use the `nextPageCursor` token from the response to retrieve the next page of the result set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<&'a str>,
}
