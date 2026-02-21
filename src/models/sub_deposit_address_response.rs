use super::deposit_address_chain_info::DepositAddressChainInfo;
use crate::prelude::*;

/// Sub deposit address response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubDepositAddressResponse {
    /// Coin
    pub coin: String,

    /// Chain information (note: API returns an object, not an array for sub accounts)
    pub chains: DepositAddressChainInfo,
}
