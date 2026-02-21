use crate::prelude::*;

/// Request for getting universal transfer records
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UniversalTransferRecordsRequest<'a> {
    /// UUID. Use the one you generated in createTransfer
    #[serde(rename = "transferId")]
    pub transfer_id: Option<&'a str>,
    /// Coin, uppercase only
    pub coin: Option<&'a str>,
    /// Transfer status. `SUCCESS`,`FAILED`,`PENDING`
    pub status: Option<&'a str>,
    /// The start timestamp (ms) Note: the query logic is actually effective based on second level
    #[serde(rename = "startTime")]
    pub start_time: Option<u64>,
    /// The end timestamp (ms) Note: the query logic is actually effective based on second level
    #[serde(rename = "endTime")]
    pub end_time: Option<u64>,
    /// Limit for data size per page. [1, 50]. Default: 20
    pub limit: Option<u32>,
    /// Cursor. Use the nextPageCursor token from the response to retrieve the next page of the result set
    pub cursor: Option<&'a str>,
}
