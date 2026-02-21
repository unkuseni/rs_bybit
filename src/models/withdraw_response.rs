use crate::prelude::*;

/// Response for creating a withdrawal
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WithdrawResponse {
    /// Withdrawal ID
    pub id: String,
}
