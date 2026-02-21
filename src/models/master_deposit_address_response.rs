use super::deposit_address_chain_info::DepositAddressChainInfo;
use crate::prelude::*;

/// Master deposit address response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MasterDepositAddressResponse {
    /// Coin
    pub coin: String,

    /// Array of chain information
    pub chains: Vec<DepositAddressChainInfo>,
}
