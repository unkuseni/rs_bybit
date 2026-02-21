use crate::prelude::*;

/// Request for getting withdrawable amount
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WithdrawableAmountRequest<'a> {
    /// Coin name, uppercase only
    pub coin: &'a str,
}
