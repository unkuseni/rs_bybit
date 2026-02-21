use serde::{Deserialize, Serialize};

/// Represents the result of a pre-check order request from Bybit.
///
/// This struct captures the margin calculation results before and after placing an order,
/// allowing bots to assess the impact on initial margin requirement (IMR) and maintenance
/// margin requirement (MMR). The values are expressed in basis points (e.g., 30 means 0.30%).
/// Bots should use this to validate order feasibility and manage risk in perpetual futures trading.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PreCheckResult {
    /// Unique order identifier assigned by Bybit.
    ///
    /// This ID can be used to reference the order in subsequent API calls. Bots should
    /// store this for tracking and reconciliation purposes.
    pub order_id: String,

    /// User-defined custom order identifier.
    ///
    /// Allows bots to tag orders with a custom ID for internal tracking and management.
    /// This is particularly useful for correlating pre-check results with actual order placement.
    pub order_link_id: String,

    /// Initial margin rate before placing the order, expressed in basis points (1/10000).
    ///
    /// For example, a value of 30 represents an IMR of 30/10000 = 0.30%. Bots should
    /// compare this with `post_imr_e4` to understand how the order affects margin requirements.
    pub pre_imr_e4: i64,

    /// Maintenance margin rate before placing the order, expressed in basis points (1/10000).
    ///
    /// For example, a value of 21 represents an MMR of 21/10000 = 0.21%. This indicates
    /// the minimum margin required to maintain the position before order placement.
    pub pre_mmr_e4: i64,

    /// Initial margin rate after placing the order, expressed in basis points (1/10000).
    ///
    /// This calculated value shows the projected IMR if the order is executed. Bots should
    /// ensure this remains within acceptable limits to avoid margin calls in perpetual futures.
    pub post_imr_e4: i64,

    /// Maintenance margin rate after placing the order, expressed in basis points (1/10000).
    ///
    /// This calculated value shows the projected MMR if the order is executed. Bots should
    /// monitor this to prevent liquidation risks in volatile perpetual futures markets.
    pub post_mmr_e4: i64,
}
