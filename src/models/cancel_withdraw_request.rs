use crate::prelude::*;

/// Request for canceling a withdrawal
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CancelWithdrawRequest<'a> {
    /// Withdrawal ID
    pub id: &'a str,
}
