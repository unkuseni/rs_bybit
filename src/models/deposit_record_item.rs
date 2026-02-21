use crate::prelude::*;

/// Deposit record item (on-chain)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DepositRecordItem {
    /// Coin
    pub coin: String,

    /// Chain
    pub chain: String,

    /// Amount
    pub amount: String,

    /// Transaction ID
    #[serde(rename = "txID")]
    pub tx_id: String,

    /// Deposit status
    pub status: i32,

    /// Deposit target address
    #[serde(rename = "toAddress")]
    pub to_address: String,

    /// Tag of deposit target address
    pub tag: String,

    /// Deposit fee
    #[serde(rename = "depositFee")]
    pub deposit_fee: String,

    /// Deposit's success time
    #[serde(rename = "successAt")]
    pub success_at: String,

    /// Number of confirmation blocks
    pub confirmations: String,

    /// Transaction sequence number
    #[serde(rename = "txIndex")]
    pub tx_index: String,

    /// Hash number on the chain
    #[serde(rename = "blockHash")]
    pub block_hash: String,

    /// The deposit limit for this coin in this chain. `"-1"` means no limit
    #[serde(rename = "batchReleaseLimit")]
    pub batch_release_limit: String,

    /// The deposit type. `0`: normal deposit, `10`: the deposit reaches daily deposit limit, `20`: abnormal deposit
    #[serde(rename = "depositType")]
    pub deposit_type: String,

    /// From address of deposit, only shown when the deposit comes from on-chain and from address is unique, otherwise gives `""`
    #[serde(rename = "fromAddress")]
    pub from_address: String,

    /// This field is used for tax purposes by Bybit EU (Austria) users, declare tax id
    #[serde(rename = "taxDepositRecordsId")]
    pub tax_deposit_records_id: String,

    /// This field is used for tax purposes by Bybit EU (Austria) users
    /// - 0: No reporting required
    /// - 1: Reporting pending
    /// - 2: Reporting completed
    #[serde(rename = "taxStatus")]
    pub tax_status: i32,

    /// Unique ID
    pub id: String,
}
