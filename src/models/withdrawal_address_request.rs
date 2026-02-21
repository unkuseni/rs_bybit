use crate::prelude::*;

/// Request for querying withdrawal addresses
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WithdrawalAddressRequest<'a> {
    /// Coin:
    /// - When passing `coin=baseCoin`, it refers to the universal addresses.
    /// - When passing a coin name, it refers to the regular address on the chain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<&'a str>,

    /// Chain name:
    /// - When only passing the chain name, it returns both regular addresses and universal addresses.
    /// - When passing the chain name and `coin=baseCoin`, it only returns the universal address corresponding to the chain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain: Option<&'a str>,

    /// Address type.
    /// `0`: OnChain Address Type(Regular Address Type and Universal Address Type).
    /// `1`: Internal Transfer Address Type(Invalid "coin" & "chain" Parameters)
    /// `2`: On chain address and internal transfer address type (Invalid "coin" & "chain" Parameters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_type: Option<i32>,

    /// Limit for data size per page. [`1`, `50`]. Default: `50`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Cursor. Use the `nextPageCursor` token from the response to retrieve the next page of the result set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<&'a str>,
}
