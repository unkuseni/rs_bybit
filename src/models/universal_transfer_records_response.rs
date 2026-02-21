use crate::prelude::*;
/// Universal transfer records response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UniversalTransferRecordsResponse {
    /// List of universal transfer records
    pub list: Vec<UniversalTransferRecord>,
    /// Next page cursor
    #[serde(rename = "nextPageCursor")]
    pub next_page_cursor: String,
}

/// Universal transfer record item
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UniversalTransferRecord {
    /// Transfer ID
    #[serde(rename = "transferId")]
    pub transfer_id: String,
    /// Transferred coin
    pub coin: String,
    /// Transferred amount
    pub amount: String,
    /// From UID
    #[serde(rename = "fromMemberId")]
    pub from_member_id: String,
    /// To UID
    #[serde(rename = "toMemberId")]
    pub to_member_id: String,
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
