use crate::prelude::*;

/// Request for querying sub deposit address
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubDepositAddressRequest<'a> {
    /// Coin, uppercase only
    pub coin: &'a str,

    /// Please use the value of `chain` from coin-info endpoint
    #[serde(rename = "chainType")]
    pub chain_type: &'a str,

    /// Sub user ID
    #[serde(rename = "subMemberId")]
    pub sub_member_id: &'a str,
}
