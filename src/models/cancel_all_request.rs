use crate::prelude::*;

/// Parameters for canceling all open orders.
///
/// Used to construct a request to the `/v5/order/cancel-all` endpoint to cancel all open orders for a specific trading pair or category. This is useful for bots implementing risk management strategies, such as closing all positions during high volatility or system errors in perpetual futures trading.
#[derive(Clone, Default)]
pub struct CancelAllRequest<'a> {
    /// The product category (e.g., Linear, Inverse).
    ///
    /// Specifies the instrument type, such as `Linear` for USDT-margined perpetual futures. Bots must set this correctly to target the desired contract type.
    pub category: Category,

    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the specific perpetual futures contract. If specified, only orders for this symbol are canceled. Bots can leave this empty to cancel orders across all symbols in the category.
    pub symbol: &'a str,

    /// The base coin of the instrument (e.g., "BTC").
    ///
    /// Optionally filters orders by the base asset (e.g., all BTC-based perpetuals). Useful for bots managing multiple pairs of the same asset.
    pub base_coin: Option<&'a str>,

    /// The settlement coin (e.g., "USDT").
    ///
    /// Optionally filters orders by the settlement currency. For `Linear` perpetuals, this is typically "USDT". Bots can use this to narrow down cancellations to specific margin types.
    pub settle_coin: Option<&'a str>,

    /// The order filter (e.g., "Order", "StopOrder").
    ///
    /// Specifies the type of orders to cancel, such as regular orders or stop orders. Bots can use this to selectively cancel specific order types, preserving others like take-profit/stop-loss orders.
    pub order_filter: Option<&'a str>,

    /// The stop order type (e.g., "StopLoss", "TakeProfit").
    ///
    /// Optionally filters by specific stop order types. Useful for bots managing complex strategies with multiple conditional orders in perpetual futures.
    pub stop_order_type: Option<&'a str>,
}

impl<'a> CancelAllRequest<'a> {
    /// Creates a default Cancelall request.
    ///
    /// Returns a request with `category` set to `Linear` and `symbol` set to `"BTCUSDT"`. Suitable for testing or quick cancellations but should be customized for production to match specific trading needs.
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Self {
        Self {
            category: Category::Linear,
            symbol: "BTCUSDT",
            base_coin: None,
            settle_coin: None,
            order_filter: None,
            stop_order_type: None,
        }
    }
    /// Constructs a new Cancelall request with specified parameters.
    ///
    /// Allows full customization of the cancellation request. Bots should use this to specify the exact symbol, category, and filters to align with their risk management or trading strategy.
    pub fn new(
        category: Category,
        symbol: &'a str,
        base_coin: Option<&'a str>,
        settle_coin: Option<&'a str>,
        order_filter: Option<&'a str>,
        stop_order_type: Option<&'a str>,
    ) -> Self {
        Self {
            category,
            symbol,
            base_coin,
            settle_coin,
            order_filter,
            stop_order_type,
        }
    }
}
