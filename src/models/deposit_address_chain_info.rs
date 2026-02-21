use crate::prelude::*;

/// Deposit address chain information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DepositAddressChainInfo {
    /// Chain type
    #[serde(rename = "chainType")]
    pub chain_type: String,

    /// The address for deposit
    #[serde(rename = "addressDeposit")]
    pub address_deposit: String,

    /// Tag of deposit
    #[serde(rename = "tagDeposit")]
    pub tag_deposit: String,

    /// Chain
    pub chain: String,

    /// The deposit limit for this coin in this chain. `"-1"` means no limit
    #[serde(rename = "batchReleaseLimit")]
    pub batch_release_limit: String,

    /// The contract address of the coin. Only display last 6 characters, if there is no contract address, it shows `""`
    #[serde(rename = "contractAddress")]
    pub contract_address: String,
}
