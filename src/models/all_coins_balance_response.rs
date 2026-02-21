use crate::prelude::*;

/// All coins balance response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AllCoinsBalanceResponse {
    /// UserID
    #[serde(rename = "memberId")]
    pub member_id: String,
    /// Account type
    #[serde(rename = "accountType")]
    pub account_type: String,
    /// List of coin balances
    pub balance: Vec<CoinBalanceItem>,
}

/// Coin balance item for all coins balance response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CoinBalanceItem {
    /// Currency
    pub coin: String,
    /// Wallet balance
    #[serde(rename = "walletBalance")]
    pub wallet_balance: String,
    /// Transferable balance
    #[serde(rename = "transferBalance")]
    pub transfer_balance: String,
    /// Bonus
    pub bonus: String,
}
