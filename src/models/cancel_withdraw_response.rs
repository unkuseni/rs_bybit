use crate::prelude::*;

/// Response for canceling a withdrawal
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CancelWithdrawResponse {
    /// Status of the cancellation
    /// `0`: fail. `1`: success
    pub status: i32,
}
