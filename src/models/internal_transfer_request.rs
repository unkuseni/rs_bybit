use crate::prelude::*;

/// Request for creating internal transfer
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InternalTransferRequest<'a> {
    /// UUID. Please manually generate a UUID
    #[serde(rename = "transferId")]
    pub transfer_id: &'a str,
    /// Coin, uppercase only
    pub coin: &'a str,
    /// Amount
    pub amount: &'a str,
    /// From account type
    #[serde(rename = "fromAccountType")]
    pub from_account_type: &'a str,
    /// To account type
    #[serde(rename = "toAccountType")]
    pub to_account_type: &'a str,
}
