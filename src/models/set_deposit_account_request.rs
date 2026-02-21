use crate::prelude::*;

/// Request for setting deposit account
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetDepositAccountRequest<'a> {
    /// Account type `UNIFIED`, `FUND`
    #[serde(rename = "accountType")]
    pub account_type: &'a str,
}
