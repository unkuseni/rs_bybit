use crate::prelude::*;

/// Request for getting single coin balance
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SingleCoinBalanceRequest<'a> {
    /// UID. Required when querying sub UID balance with master api key
    #[serde(rename = "memberId")]
    pub member_id: Option<&'a str>,
    /// UID. Required when querying the transferable balance between different UIDs
    #[serde(rename = "toMemberId")]
    pub to_member_id: Option<&'a str>,
    /// Account type
    #[serde(rename = "accountType")]
    pub account_type: &'a str,
    /// To account type. Required when querying the transferable balance between different account types
    #[serde(rename = "toAccountType")]
    pub to_account_type: Option<&'a str>,
    /// Coin, uppercase only
    pub coin: &'a str,
    /// 0(default): not query bonus. 1: query bonus
    #[serde(rename = "withBonus")]
    pub with_bonus: Option<u8>,
    /// Whether query delay withdraw/transfer safe amount
    /// 0(default): false, 1: true
    #[serde(rename = "withTransferSafeAmount")]
    pub with_transfer_safe_amount: Option<u8>,
    /// For OTC loan users in particular, you can check the transferable amount under risk level
    /// 0(default): false, 1: true
    /// toAccountType is mandatory
    #[serde(rename = "withLtvTransferSafeAmount")]
    pub with_ltv_transfer_safe_amount: Option<u8>,
}
