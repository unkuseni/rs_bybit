use crate::prelude::*;

/// Internal deposit record item (off-chain)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InternalDepositRecordItem {
    /// ID
    pub id: String,

    /// `1`: Internal deposit
    #[serde(rename = "type")]
    pub record_type: i32,

    /// Deposit coin
    pub coin: String,

    /// Deposit amount
    pub amount: String,

    /// Status:
    /// - 1 = Processing
    /// - 2 = Success
    /// - 3 = Deposit failed
    pub status: i32,

    /// Email address or phone number
    pub address: String,

    /// Deposit created timestamp
    #[serde(rename = "createdTime")]
    pub created_time: String,

    /// Sender UID
    #[serde(rename = "fromMemberId")]
    pub from_member_id: String,

    /// Internal transfer transaction ID
    #[serde(rename = "txID")]
    pub tx_id: String,

    /// This field is used for tax purposes by Bybit EU (Austria) users, declare tax id
    #[serde(rename = "taxDepositRecordsId")]
    pub tax_deposit_records_id: String,

    /// This field is used for tax purposes by Bybit EU (Austria) users
    /// - 0: No reporting required
    /// - 1: Reporting pending
    /// - 2: Reporting completed
    #[serde(rename = "taxStatus")]
    pub tax_status: i32,
}
