use serde::{Deserialize, Serialize};

/// Chain information for a coin
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CoinChainInfo {
    /// Chain
    pub chain: String,
    /// Chain type
    #[serde(rename = "chainType")]
    pub chain_type: String,
    /// Number of confirmations for deposit
    pub confirmation: String,
    /// Withdraw fee. If empty, coin does not support withdrawal
    #[serde(rename = "withdrawFee")]
    pub withdraw_fee: String,
    /// Minimum deposit
    #[serde(rename = "depositMin")]
    pub deposit_min: String,
    /// Minimum withdraw
    #[serde(rename = "withdrawMin")]
    pub withdraw_min: String,
    /// The precision of withdraw or deposit
    #[serde(rename = "minAccuracy")]
    pub min_accuracy: String,
    /// The chain status of deposit. `0`: suspend. `1`: normal
    #[serde(rename = "chainDeposit")]
    pub chain_deposit: String,
    /// The chain status of withdraw. `0`: suspend. `1`: normal
    #[serde(rename = "chainWithdraw")]
    pub chain_withdraw: String,
    /// The withdraw fee percentage. It is a real figure, e.g., 0.022 means 2.2%
    #[serde(rename = "withdrawPercentageFee")]
    pub withdraw_percentage_fee: String,
    /// Contract address. `""` means no contract address
    #[serde(rename = "contractAddress")]
    pub contract_address: String,
    /// Number of security confirmations
    #[serde(rename = "safeConfirmNumber")]
    pub safe_confirm_number: String,
}
