use crate::prelude::*;

/// Response for querying withdrawal addresses
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WithdrawalAddressResponse {
    /// List of withdrawal addresses
    pub rows: Vec<WithdrawalAddress>,

    /// Cursor. Used for pagination
    #[serde(rename = "nextPageCursor")]
    pub next_page_cursor: Option<String>,
}

/// Withdrawal address information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WithdrawalAddress {
    /// Coin
    pub coin: String,

    /// Chain name
    pub chain: String,

    /// Address
    pub address: String,

    /// Address tag
    pub tag: String,

    /// Remark
    pub remark: String,

    /// Address status:
    /// `0`: Normal.
    /// `1`: New Addresses are prohibited from withdrawing coins for 24 Hours.
    pub status: i32,

    /// Address type.
    /// `0`: OnChain Address Type(Regular Address Type And Universal Address Type)
    /// `1`: Internal Transfer Address Type.
    /// `2`: Internal Transfer Address Type And OnChain Address Type
    #[serde(rename = "addressType")]
    pub address_type: i32,

    /// Whether the address has been verified or not:
    /// `0`: Unverified Address.
    /// `1`: Verified Address.
    pub verified: i32,

    /// Address create time
    #[serde(rename = "createdAt")]
    pub created_at: String,
}
