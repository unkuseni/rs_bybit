use crate::prelude::*;

/// Request for querying master deposit address
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MasterDepositAddressRequest<'a> {
    /// Coin, uppercase only
    pub coin: &'a str,

    /// Please use the value of `>> chain` from coin-info endpoint
    #[serde(rename = "chainType", skip_serializing_if = "Option::is_none")]
    pub chain_type: Option<&'a str>,
}
