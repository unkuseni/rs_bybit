use crate::prelude::*;

/// Request for getting transferable coin list
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransferableCoinRequest<'a> {
    /// From account type
    #[serde(rename = "fromAccountType")]
    pub from_account_type: &'a str,
    /// To account type
    #[serde(rename = "toAccountType")]
    pub to_account_type: &'a str,
}
