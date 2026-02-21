use crate::prelude::*;

/// Request for getting all coins balance
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AllCoinsBalanceRequest<'a> {
    /// User Id. It is required when you use master api key to check sub account coin balance
    #[serde(rename = "memberId")]
    pub member_id: Option<&'a str>,
    /// Account type
    #[serde(rename = "accountType")]
    pub account_type: &'a str,
    /// Coin name, uppercase only
    /// - Query all coins if not passed
    /// - Can query multiple coins, separated by comma. `USDT,USDC,ETH`
    /// **Note:** this field is **mandatory** for accountType=`UNIFIED`, and supports up to 10 coins each request
    pub coin: Option<&'a str>,
    /// 0(default): not query bonus. 1: query bonus
    #[serde(rename = "withBonus")]
    pub with_bonus: Option<u8>,
}
