use crate::prelude::*;

/// Parameters for setting trading stop conditions (take-profit/stop-loss/trailing-stop).
///
/// Used to construct a request to the `/v5/position/trading-stop` endpoint to set take-profit,
/// stop-loss, and trailing-stop conditions for a position. Bots use this to implement automated
/// exit strategies and manage risk in perpetual futures trading. This struct supports both
/// full and partial take-profit/stop-loss modes, allowing precise control over position exits.
///
/// # Bybit API Reference
/// According to the Bybit V5 API documentation:
/// - New version of TP/SL function supports both holding entire position TP/SL orders and
///   holding partial position TP/SL orders.
/// - Full position TP/SL orders: This API can be used to modify parameters of existing TP/SL orders.
/// - Partial position TP/SL orders: This API can only add partial position TP/SL orders.
/// - Under the new version of TP/SL function, when calling this API to perform one-sided
///   take profit or stop loss modification on existing TP/SL orders on the holding position,
///   it will cause the paired tp/sl orders to lose binding relationship.
#[derive(Clone, Default)]
pub struct TradingStopRequest<'a> {
    /// The product category (e.g., Linear, Inverse).
    ///
    /// Specifies the instrument type, such as `Linear` for USDT-margined perpetual futures.
    /// Bots must set this correctly to target the desired contract type.
    pub category: Category,

    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the perpetual futures contract for which trading stops are being set.
    /// Bots must specify a valid symbol to ensure the request targets the correct position.
    pub symbol: Cow<'a, str>,

    /// The take-profit/stop-loss mode (e.g., "Full", "Partial").
    ///
    /// Specifies whether the trading stops apply to the entire position (`Full`) or a portion (`Partial`).
    /// Bots use this to implement granular exit strategies, such as scaling out of positions.
    /// This parameter is required according to the Bybit API documentation.
    pub tpsl_mode: Cow<'a, str>,

    /// The position index (e.g., 0 for one-way mode, 1 or 2 for hedge mode).
    ///
    /// Used to identify positions in different position modes.
    /// - `0`: one-way mode
    /// - `1`: hedge-mode Buy side
    /// - `2`: hedge-mode Sell side
    /// This parameter is required according to the Bybit API documentation.
    pub position_idx: i32,

    /// The take-profit price (optional).
    ///
    /// The price at which the position will close for a profit. Bots use this to lock in gains
    /// automatically, typically based on technical indicators or predefined targets.
    /// Cannot be less than 0, 0 means cancel TP.
    pub take_profit: Option<f64>,

    /// The stop-loss price (optional).
    ///
    /// The price at which the position will close to limit losses. Bots use this to manage
    /// downside risk, protecting capital during adverse market movements.
    /// Cannot be less than 0, 0 means cancel SL.
    pub stop_loss: Option<f64>,

    /// The trailing stop by price distance (optional).
    ///
    /// Trailing stop by price distance. Cannot be less than 0, 0 means cancel TS.
    /// Bots use this to implement dynamic stop-loss strategies that follow favorable price movements.
    pub trailing_stop: Option<f64>,

    /// The trigger type for take-profit (optional).
    ///
    /// Specifies the price type used to trigger the take-profit order (e.g., `LastPrice`,
    /// `MarkPrice`, `IndexPrice`). Bots should choose a trigger type that aligns with their
    /// strategy to balance responsiveness and stability.
    pub tp_trigger_by: Option<Cow<'a, str>>,

    /// The trigger type for stop-loss (optional).
    ///
    /// Specifies the price type used to trigger the stop-loss order. Bots should select a
    /// trigger type that minimizes slippage while ensuring timely execution in volatile markets.
    pub sl_trigger_by: Option<Cow<'a, str>>,

    /// The trailing stop trigger price (optional).
    ///
    /// Trailing stop will be triggered when this price is reached **only**.
    /// Bots use this to set an activation price for trailing stops, ensuring they only
    /// become active after a certain profit level is reached.
    pub active_price: Option<f64>,

    /// The size of the take-profit order (optional).
    ///
    /// The quantity of the position to close when the take-profit is triggered, used in
    /// TP/SL partial mode. Note: the value of tp_size and sl_size must equal.
    /// Bots use this to scale out of positions incrementally, optimizing profit capture.
    pub tp_size: Option<f64>,

    /// The size of the stop-loss order (optional).
    ///
    /// The quantity of the position to close when the stop-loss is triggered, used in
    /// TP/SL partial mode. Note: the value of tp_size and sl_size must equal.
    /// Bots use this to limit losses on specific portions of a position.
    pub sl_size: Option<f64>,

    /// The limit price for the take-profit order (optional).
    ///
    /// The specific price for a `Limit` take-profit order. Only works when tpslMode=Partial
    /// and tpOrderType=Limit. Bots use this to ensure the take-profit order executes at a
    /// favorable price, avoiding slippage in stable markets.
    pub tp_limit_price: Option<f64>,

    /// The limit price for the stop-loss order (optional).
    ///
    /// The specific price for a `Limit` stop-loss order. Only works when tpslMode=Partial
    /// and slOrderType=Limit. Bots use this cautiously, as limit orders may not execute
    /// in fast-moving markets, increasing risk exposure.
    pub sl_limit_price: Option<f64>,

    /// The order type when take profit is triggered (optional).
    ///
    /// The order type when take profit is triggered. `Market` (default), `Limit`.
    /// For tpslMode=Full, it only supports tpOrderType="Market".
    /// Bots can use `Limit` orders to target specific exit prices, reducing slippage.
    pub tp_order_type: Option<OrderType>,

    /// The order type when stop loss is triggered (optional).
    ///
    /// The order type when stop loss is triggered. `Market` (default), `Limit`.
    /// For tpslMode=Full, it only supports slOrderType="Market".
    /// Bots typically use `Market` orders for stop-losses to ensure execution.
    pub sl_order_type: Option<OrderType>,
}

impl<'a> TradingStopRequest<'a> {
    /// Constructs a new TradingStop request with specified parameters.
    ///
    /// Allows full customization of the trading stop request. Bots should use this to define
    /// the exact symbol, category, and trading stop parameters to align with their risk
    /// management strategy.
    ///
    /// # Arguments
    /// * `category` - The product category (Linear, Inverse)
    /// * `symbol` - The trading pair symbol (e.g., "BTCUSDT")
    /// * `tpsl_mode` - The TP/SL mode ("Full" or "Partial")
    /// * `position_idx` - The position index (0: one-way, 1: hedge buy, 2: hedge sell)
    /// * `take_profit` - The take-profit price (optional, 0 means cancel)
    /// * `stop_loss` - The stop-loss price (optional, 0 means cancel)
    /// * `trailing_stop` - The trailing stop distance (optional, 0 means cancel)
    /// * `tp_trigger_by` - The trigger type for take-profit (optional)
    /// * `sl_trigger_by` - The trigger type for stop-loss (optional)
    /// * `active_price` - The trailing stop trigger price (optional)
    /// * `tp_size` - The take-profit size for partial mode (optional)
    /// * `sl_size` - The stop-loss size for partial mode (optional)
    /// * `tp_limit_price` - The limit price for take-profit (optional)
    /// * `sl_limit_price` - The limit price for stop-loss (optional)
    /// * `tp_order_type` - The order type for take-profit (optional)
    /// * `sl_order_type` - The order type for stop-loss (optional)
    pub fn new(
        category: Category,
        symbol: &'a str,
        tpsl_mode: &'a str,
        position_idx: i32,
        take_profit: Option<f64>,
        stop_loss: Option<f64>,
        trailing_stop: Option<f64>,
        tp_trigger_by: Option<&'a str>,
        sl_trigger_by: Option<&'a str>,
        active_price: Option<f64>,
        tp_size: Option<f64>,
        sl_size: Option<f64>,
        tp_limit_price: Option<f64>,
        sl_limit_price: Option<f64>,
        tp_order_type: Option<OrderType>,
        sl_order_type: Option<OrderType>,
    ) -> Self {
        Self {
            category,
            symbol: Cow::Borrowed(symbol),
            tpsl_mode: Cow::Borrowed(tpsl_mode),
            position_idx,
            take_profit,
            stop_loss,
            trailing_stop,
            tp_trigger_by: tp_trigger_by.map(Cow::Borrowed),
            sl_trigger_by: sl_trigger_by.map(Cow::Borrowed),
            active_price,
            tp_size,
            sl_size,
            tp_limit_price,
            sl_limit_price,
            tp_order_type,
            sl_order_type,
        }
    }
    /// Creates a default TradingStop request.
    ///
    /// Returns a request with `category` set to `Linear`, `symbol` set to `"BTCUSDT"`,
    /// `tpsl_mode` set to `"Full"`, `position_idx` set to `0` (one-way mode),
    /// and all other fields unset. Suitable for testing but should be customized for
    /// production to match specific trading needs.
    pub fn default() -> TradingStopRequest<'a> {
        TradingStopRequest::new(
            Category::Linear,
            "BTCUSDT",
            "Full",
            0,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
    }

    /// Validates the request parameters according to Bybit API constraints.
    ///
    /// Bots should call this method before sending the request to ensure compliance
    /// with API limits and requirements.
    ///
    /// # Returns
    /// * `Ok(())` if validation passes
    /// * `Err(String)` with error message if validation fails
    pub fn validate(&self) -> Result<(), String> {
        // Validate tpsl_mode
        if self.tpsl_mode != "Full" && self.tpsl_mode != "Partial" {
            return Err("tpsl_mode must be either 'Full' or 'Partial'".to_string());
        }

        // Validate position_idx
        if self.position_idx < 0 || self.position_idx > 2 {
            return Err("position_idx must be 0, 1, or 2".to_string());
        }

        // Validate take_profit and stop_loss are not negative
        if let Some(tp) = self.take_profit {
            if tp < 0.0 {
                return Err("take_profit cannot be negative".to_string());
            }
        }

        if let Some(sl) = self.stop_loss {
            if sl < 0.0 {
                return Err("stop_loss cannot be negative".to_string());
            }
        }

        if let Some(ts) = self.trailing_stop {
            if ts < 0.0 {
                return Err("trailing_stop cannot be negative".to_string());
            }
        }

        // Validate tp_size and sl_size for partial mode
        if self.tpsl_mode == "Partial" {
            if let (Some(tp_size), Some(sl_size)) = (self.tp_size, self.sl_size) {
                if tp_size != sl_size {
                    return Err("tp_size and sl_size must be equal in partial mode".to_string());
                }
            } else if self.tp_size.is_some() || self.sl_size.is_some() {
                return Err("both tp_size and sl_size must be provided in partial mode".to_string());
            }
        }

        // Validate order types for full mode
        if self.tpsl_mode == "Full" {
            if let Some(tp_order_type) = &self.tp_order_type {
                if tp_order_type != &OrderType::Market {
                    return Err("tp_order_type must be 'Market' for full mode".to_string());
                }
            }
            if let Some(sl_order_type) = &self.sl_order_type {
                if sl_order_type != &OrderType::Market {
                    return Err("sl_order_type must be 'Market' for full mode".to_string());
                }
            }
        }

        // Validate limit prices are only used with limit order types
        if self.tp_limit_price.is_some() && self.tp_order_type != Some(OrderType::Limit) {
            return Err("tp_limit_price requires tp_order_type to be 'Limit'".to_string());
        }

        if self.sl_limit_price.is_some() && self.sl_order_type != Some(OrderType::Limit) {
            return Err("sl_limit_price requires sl_order_type to be 'Limit'".to_string());
        }

        // Validate limit prices are only used in partial mode
        if (self.tp_limit_price.is_some() || self.sl_limit_price.is_some())
            && self.tpsl_mode != "Partial"
        {
            return Err("limit prices can only be used in partial mode".to_string());
        }

        Ok(())
    }
}
