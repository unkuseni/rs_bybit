use crate::prelude::*;

/// Transferable coin response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransferableCoinResponse {
    /// A list of coins (as strings)
    pub list: Vec<String>,
}
