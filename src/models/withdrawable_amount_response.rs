use crate::prelude::*;

/// Withdrawable amount response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WithdrawableAmountResponse {
    /// The frozen amount due to risk, in USD
    #[serde(rename = "limitAmountUsd")]
    pub limit_amount_usd: String,
    /// Withdrawable amount by account type
    #[serde(rename = "withdrawableAmount")]
    pub withdrawable_amount: WithdrawableAmountDetails,
}

/// Withdrawable amount details by account type
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WithdrawableAmountDetails {
    /// Spot wallet, it is not returned if spot wallet is removed
    #[serde(rename = "SPOT")]
    pub spot: Option<WalletWithdrawableAmount>,
    /// Funding wallet
    #[serde(rename = "FUND")]
    pub fund: Option<WalletWithdrawableAmount>,
    /// Unified wallet
    #[serde(rename = "UTA")]
    pub uta: Option<WalletWithdrawableAmount>,
}

/// Wallet withdrawable amount details
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WalletWithdrawableAmount {
    /// Coin name
    pub coin: String,
    /// Amount that can be withdrawn
    #[serde(rename = "withdrawableAmount")]
    pub withdrawable_amount: String,
    /// Available balance
    #[serde(rename = "availableBalance")]
    pub available_balance: String,
}
