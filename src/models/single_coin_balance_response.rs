use crate::prelude::*;

/// Single coin balance response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SingleCoinBalanceResponse {
    /// Account type
    #[serde(rename = "accountType")]
    pub account_type: String,
    /// Biz type
    #[serde(rename = "bizType")]
    pub biz_type: i32,
    /// Account ID
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// UID
    #[serde(rename = "memberId")]
    pub member_id: String,
    /// Balance information
    pub balance: CoinBalance,
}

/// Coin balance details
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CoinBalance {
    /// Coin
    pub coin: String,
    /// Wallet balance
    #[serde(rename = "walletBalance")]
    pub wallet_balance: String,
    /// Transferable balance
    #[serde(rename = "transferBalance")]
    pub transfer_balance: String,
    /// Bonus
    pub bonus: String,
    /// Safe amount to transfer. Keep "" if not query
    #[serde(rename = "transferSafeAmount")]
    pub transfer_safe_amount: String,
    /// Transferable amount for ins loan account. Keep "" if not query
    #[serde(rename = "ltvTransferSafeAmount")]
    pub ltv_transfer_safe_amount: String,
}
