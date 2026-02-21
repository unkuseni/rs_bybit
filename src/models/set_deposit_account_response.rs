use crate::prelude::*;

/// Set deposit account response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetDepositAccountResponse {
    /// Request result:
    /// - `1`: SUCCESS
    /// - `0`: FAIL
    pub status: i32,
}
