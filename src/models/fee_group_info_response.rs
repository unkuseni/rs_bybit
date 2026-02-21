use crate::prelude::*;

/// Represents a fee rate structure for a specific level (Pro or Market Maker).
///
/// This struct contains the fee rates for a specific level within either the Pro or Market Maker category.
/// Each level has different taker fee rate, maker fee rate, and maker rebate values.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeeRateLevel {
    /// The level name (e.g., "Pro 1", "Pro 2", ..., "Pro 6" or "MM 1", "MM 2", "MM 3").
    ///
    /// Identifies the specific fee tier within the category.
    pub level: String,

    /// The taker fee rate as a string (e.g., "0.00028").
    ///
    /// The fee rate charged for taker orders (orders that remove liquidity).
    /// For Market Maker levels, this field may be empty.
    pub taker_fee_rate: String,

    /// The maker fee rate as a string (e.g., "0.0001").
    ///
    /// The fee rate charged for maker orders (orders that add liquidity).
    /// For Market Maker levels, this field may be empty.
    pub maker_fee_rate: String,

    /// The maker rebate fee rate as a string (e.g., "-0.0000075").
    ///
    /// The rebate rate for maker orders. Negative values indicate rebates (payments to the trader).
    /// For Pro levels, this field may be empty.
    pub maker_rebate: String,
}

/// Represents the fee rate structures for both Pro and Market Maker categories.
///
/// This struct contains arrays of fee rate levels for Pro-level clients and Market Maker clients.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeeRates {
    /// Array of fee rate levels for Pro-level clients.
    ///
    /// Contains 6 levels: Pro 1 through Pro 6, each with different fee rates.
    pub pro: Vec<FeeRateLevel>,

    /// Array of fee rate levels for Market Maker clients.
    ///
    /// Contains 3 levels: MM 1 through MM 3, each with different rebate rates.
    pub market_maker: Vec<FeeRateLevel>,
}

/// Represents a fee group with its associated symbols and fee rates.
///
/// Each fee group contains a set of symbols that share the same fee structure
/// and weighting factor for volume calculations.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeeGroup {
    /// The fee group name (e.g., "G1(Major Coins)").
    ///
    /// Descriptive name of the fee group.
    pub group_name: String,

    /// The group weighting factor.
    ///
    /// Used in calculating weighted maker volume:
    /// Weighted maker volume = Σ(Maker volume on pair × Group weighting factor)
    pub weighting_factor: i32,

    /// The number of symbols in this fee group.
    ///
    /// Count of symbols included in the `symbols` array.
    pub symbols_numbers: i32,

    /// Array of symbol names included in this fee group.
    ///
    /// List of trading symbols (e.g., ["BTCUSDT", "ETHUSDT", ...]).
    pub symbols: Vec<String>,

    /// Fee rate details for different client categories.
    ///
    /// Contains fee structures for Pro-level and Market Maker clients.
    pub fee_rates: FeeRates,

    /// Latest data update timestamp in milliseconds.
    ///
    /// Timestamp when the fee group data was last updated.
    pub update_time: String,
}

/// Contains the list of fee groups returned by the API.
///
/// This struct wraps the array of fee groups in the API response.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeeGroupList {
    /// List of fee group objects.
    ///
    /// Contains all fee groups (or a specific group if filtered by groupId).
    pub list: Vec<FeeGroup>,
}

/// The main response type for the fee group info API endpoint.
///
/// This is the top-level response structure returned by the `/v5/market/fee-group-info` endpoint.
pub type FeeGroupInfoResponse = BybitApiResponse<FeeGroupList>;
