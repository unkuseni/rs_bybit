use crate::prelude::*;

/// Universal transfer response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UniversalTransferResponse {
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
