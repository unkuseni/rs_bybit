use crate::prelude::*;

/// Represents a request to amend an existing order on Bybit.
///
/// This struct defines parameters for modifying an order’s attributes, such as price,
/// quantity, or TP/SL. In perpetual futures, amending orders is essential for adapting
/// to market movements (e.g., tightening stop-losses). Bots must ensure the `order_id`
/// or `order_link_id` is valid and that changes comply with Bybit’s constraints (e.g.,
/// minimum quantity, price tick size) to avoid rejections.
#[derive(Clone, Default, Serialize)]
pub struct AmendOrderRequest<'a> {
    /// The category of the trading product (e.g., Spot, Linear).
    ///
    /// Specifies the market for the order being amended. Bots must match this to the
    /// original order’s category to avoid errors. In perpetual futures, `Linear` is
    /// typical for USDT-margined contracts.
    pub category: Category,

    /// The trading pair symbol, e.g., "BTCUSDT".
    ///
    /// Identifies the asset pair of the order. Must match the original order’s symbol.
    /// Bots should validate symbol consistency to prevent API errors in perpetual futures
    /// trading.
    pub symbol: Cow<'a, str>,

    /// The unique identifier of the order to amend.
    ///
    /// Provided by Bybit when the order was created. Bots must include either `order_id`
    /// or `order_link_id` to identify the order. In perpetual futures, accurate order
    /// identification is critical for amending the correct position.
    pub order_id: Option<Cow<'a, str>>,

    /// The user-defined identifier of the order to amend.
    ///
    /// Allows bots to reference orders using their custom ID. Useful for tracking
    /// multiple orders in perpetual futures strategies. Either `order_id` or
    /// `order_link_id` must be provided.
    pub order_link_id: Option<Cow<'a, str>>,

    /// The implied volatility for options orders (optional).
    ///
    /// Used for options trading, not perpetual futures. Bots can ignore this unless
    /// amending options orders, where accurate volatility is needed for pricing.
    pub order_iv: Option<f64>,

    /// The new trigger price for conditional orders.
    ///
    /// Updates the price at which a conditional order (e.g., stop-loss) activates.
    /// In perpetual futures, bots use this to adjust risk management dynamically based
    /// on market conditions. Ensure the price aligns with symbol constraints.
    pub trigger_price: Option<f64>,

    /// The new quantity of the order.
    ///
    /// Updates the order size. In perpetual futures, bots must ensure the new quantity
    /// is valid (e.g., within position limits, sufficient margin) to avoid rejections.
    /// Over-sizing can increase liquidation risk.
    pub qty: f64,

    /// The new price for limit orders.
    ///
    /// Updates the target price for a limit order. Bots should validate the price against
    /// the symbol’s tick size and market conditions to ensure execution feasibility in
    /// volatile perpetual futures markets.
    pub price: Option<f64>,

    /// The new take-profit/stop-loss mode (e.g., "Full", "Partial").
    ///
    /// Updates whether TP/SL applies to the entire position or a portion. Bots should
    /// adjust this based on exit strategy; "Full" simplifies risk management, while
    /// "Partial" suits scaling out in perpetual futures.
    pub tpsl_mode: Option<Cow<'a, str>>,

    /// The new take-profit price.
    ///
    /// Updates the price to lock in profits. Bots should set this based on updated market
    /// analysis, ensuring it’s realistic given volatility in perpetual futures.
    pub take_profit: Option<f64>,

    /// The new stop-loss price.
    ///
    /// Updates the price to limit losses. Critical for risk management in perpetual
    /// futures, bots should adjust SL dynamically to protect against adverse price
    /// movements while avoiding premature exits.
    pub stop_loss: Option<f64>,

    /// The new trigger price type for take-profit.
    ///
    /// Updates the price metric (e.g., "MarkPrice") for TP triggering. Bots should choose
    /// a reliable metric to ensure consistent execution in perpetual futures markets.
    pub tp_trigger_by: Option<Cow<'a, str>>,

    /// The new trigger price type for stop-loss.
    ///
    /// Updates the price metric for SL triggering. Similar to `tp_trigger_by`, bots
    /// should select a stable trigger type to protect against manipulation in perpetual
    /// futures.
    pub sl_trigger_by: Option<Cow<'a, str>>,

    /// The new trigger price type for conditional orders.
    ///
    /// Updates the price metric for triggering conditional orders. Bots should ensure
    /// consistency with the order’s strategy and market conditions in perpetual futures.
    pub trigger_by: Option<Cow<'a, str>>,

    /// The new limit price for take-profit orders.
    ///
    /// Updates the exact price for limit-based TP orders. Bots must balance precision
    /// with execution likelihood in volatile perpetual futures markets.
    pub tp_limit_price: Option<f64>,

    /// The new limit price for stop-loss orders.
    ///
    /// Updates the exact price for limit-based SL orders. Bots should ensure the price
    /// is achievable to effectively limit losses in perpetual futures.
    pub sl_limit_price: Option<f64>,
}

impl<'a> AmendOrderRequest<'a> {
    /// Creates a default `AmendOrderRequest` with predefined values.
    ///
    /// Initializes an amendment request with common defaults. Bots should modify
    /// necessary fields before submission to align with the target order’s attributes.
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Self {
        Self {
            category: Category::Linear,
            symbol: Cow::Borrowed("BTCUSDT"),
            order_id: None,
            order_link_id: None,
            order_iv: None,
            trigger_price: None,
            qty: 0.00,
            price: None,
            tpsl_mode: None,
            take_profit: None,
            stop_loss: None,
            tp_trigger_by: None,
            sl_trigger_by: None,
            trigger_by: None,
            tp_limit_price: None,
            sl_limit_price: None,
        }
    }
    /// Creates a custom `AmendOrderRequest` with specified parameters.
    ///
    /// Allows bots to tailor an amendment request fully. Ensure all parameters are valid
    /// and consistent with the original order to avoid API errors in perpetual futures
    /// trading.
    #[allow(clippy::too_many_arguments)]
    pub fn custom(
        category: Category,
        symbol: &'a str,
        order_id: Option<&'a str>,
        order_link_id: Option<&'a str>,
        order_iv: Option<f64>,
        trigger_price: Option<f64>,
        qty: f64,
        price: Option<f64>,
        tpsl_mode: Option<&'a str>,
        take_profit: Option<f64>,
        stop_loss: Option<f64>,
        tp_trigger_by: Option<&'a str>,
        sl_trigger_by: Option<&'a str>,
        trigger_by: Option<&'a str>,
        tp_limit_price: Option<f64>,
        sl_limit_price: Option<f64>,
    ) -> Self {
        Self {
            category,
            symbol: Cow::Borrowed(symbol),
            order_id: order_id.map(Cow::Borrowed),
            order_link_id: order_link_id.map(Cow::Borrowed),
            order_iv,
            trigger_price,
            qty,
            price,
            tpsl_mode: tpsl_mode.map(Cow::Borrowed),
            take_profit,
            stop_loss,
            tp_trigger_by: tp_trigger_by.map(Cow::Borrowed),
            sl_trigger_by: sl_trigger_by.map(Cow::Borrowed),
            trigger_by: trigger_by.map(Cow::Borrowed),
            tp_limit_price,
            sl_limit_price,
        }
    }
}
