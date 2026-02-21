use crate::prelude::*;

/// Internal transfer response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InternalTransferResponse {
    /// UUID
    #[serde(rename = "transferId")]
    pub transfer_id: String,
    /// Transfer status
    /// - `STATUS_UNKNOWN`
    /// - `SUCCESS`
    /// - `PENDING`
    /// - `FAILED`
    pub status: String,
}
