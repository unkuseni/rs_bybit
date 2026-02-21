use crate::prelude::*;

/// Internal transfer records response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InternalTransferRecordsResponse {
    /// List of internal transfer records
    pub list: Vec<InternalTransferRecord>,
    /// Next page cursor
    #[serde(rename = "nextPageCursor")]
    pub next_page_cursor: String,
}

/// Internal transfer record item
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InternalTransferRecord {
    /// Transfer ID
    #[serde(rename = "transferId")]
    pub transfer_id: String,
    /// Transferred coin
    pub coin: String,
    /// Transferred amount
    pub amount: String,
    /// From account type
    #[serde(rename = "fromAccountType")]
    pub from_account_type: String,
    /// To account type
    #[serde(rename = "toAccountType")]
    pub to_account_type: String,
    /// Transfer created timestamp (ms)
    pub timestamp: String,
    /// Transfer status
    pub status: String,
}
