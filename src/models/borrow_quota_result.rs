use serde::{Deserialize, Serialize};

/// Represents the result of a borrow quota check for spot trading from Bybit.
///
/// This struct provides information about the maximum trade quantities and amounts
/// available for spot trading, including both actual balances and borrowable amounts.
/// Bots use this to determine available trading capacity and manage position sizing
/// in spot and margin trading scenarios.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BorrowQuotaResult {
    /// Symbol name, e.g., "BTCUSDT".
    ///
    /// Identifies the trading pair for which the borrow quota is calculated.
    /// Bots should verify this matches the requested symbol to ensure data consistency.
    pub symbol: String,

    /// Transaction side: "Buy" or "Sell".
    ///
    /// Indicates whether the quota is for buying or selling the base asset.
    /// Quota values differ based on side due to different margin requirements
    /// and available balances for each direction.
    pub side: String,

    /// Maximum base coin quantity that can be traded.
    ///
    /// If spot margin trade is enabled and the symbol is a margin trading pair,
    /// this includes available balance + maximum borrowable quantity.
    /// Otherwise, it returns the actual available balance.
    /// Values are up to 4 decimal places.
    #[serde(rename = "maxTradeQty")]
    pub max_trade_qty: String,

    /// Maximum quote coin amount that can be traded.
    ///
    /// If spot margin trade is enabled and the symbol is a margin trading pair,
    /// this includes available balance + maximum borrowable amount.
    /// Otherwise, it returns the actual available balance.
    /// Values are up to 8 decimal places.
    #[serde(rename = "maxTradeAmount")]
    pub max_trade_amount: String,

    /// Actual base coin quantity available for trading (excluding borrowable amount).
    ///
    /// Regardless of spot margin switch status, this always returns the actual
    /// quantity of base coin you can trade or have available.
    /// Values are up to 4 decimal places.
    #[serde(rename = "spotMaxTradeQty")]
    pub spot_max_trade_qty: String,

    /// Actual quote coin amount available for trading (excluding borrowable amount).
    ///
    /// Regardless of spot margin switch status, this always returns the actual
    /// amount of quote coin you can trade or have available.
    /// Values are up to 8 decimal places.
    #[serde(rename = "spotMaxTradeAmount")]
    pub spot_max_trade_amount: String,

    /// The coin that would be borrowed for this trade.
    ///
    /// Indicates which currency would be borrowed if margin trading is used.
    /// For example, when buying BTCUSDT, the borrow coin would be USDT.
    #[serde(rename = "borrowCoin")]
    pub borrow_coin: String,
}
