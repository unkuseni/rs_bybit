use crate::prelude::*;

/// Response for querying withdrawal records
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WithdrawalRecordResponse {
    /// List of withdrawal records
    pub rows: Vec<WithdrawalRecord>,

    /// Cursor. Used for pagination
    #[serde(rename = "nextPageCursor")]
    pub next_page_cursor: Option<String>,
}

/// Withdrawal record information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WithdrawalRecord {
    /// Transaction ID. It returns `""` when withdrawal failed, withdrawal cancelled
    #[serde(rename = "txID")]
    pub tx_id: String,

    /// Coin
    pub coin: String,

    /// Chain
    pub chain: String,

    /// Amount
    pub amount: String,

    /// Withdraw fee
    #[serde(rename = "withdrawFee")]
    pub withdraw_fee: String,

    /// Withdraw status
    pub status: String,

    /// To withdrawal address. Shows the Bybit UID for internal transfers
    #[serde(rename = "toAddress")]
    pub to_address: String,

    /// Tag
    pub tag: String,

    /// Withdraw created timestamp (ms)
    #[serde(rename = "createTime")]
    pub create_time: String,

    /// Withdraw updated timestamp (ms)
    #[serde(rename = "updateTime")]
    pub update_time: String,

    /// Withdraw ID
    #[serde(rename = "withdrawId")]
    pub withdraw_id: String,

    /// Withdraw type.
    /// `0`: on chain.
    /// `1`: off chain.
    #[serde(rename = "withdrawType")]
    pub withdraw_type: i32,

    /// Fee
    pub fee: String,

    /// Tax
    pub tax: String,

    /// Tax rate
    #[serde(rename = "taxRate")]
    pub tax_rate: String,

    /// Tax type
    #[serde(rename = "taxType")]
    pub tax_type: String,
}
