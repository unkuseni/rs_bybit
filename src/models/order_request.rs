use crate::prelude::*;

/// Represents a request to place an order on Bybit's trading platform.
///
/// The `OrderRequest` struct encapsulates all parameters needed to create an order,
/// supporting both spot and perpetual futures trading. Perpetual futures on Bybit
/// allow leveraged trading without expiration, with funding rates balancing long and
/// short positions. Bots using this struct must handle fields carefully to align with
/// trading strategies, manage risk (e.g., take-profit/stop-loss), and comply with
/// Bybit's API constraints (e.g., valid symbol, quantity precision). Incorrect
/// configurations can lead to order rebing, insufficient margin, or unintended
/// liquidations.
#[derive(Clone, Serialize)]
pub struct OrderRequest<'a> {
    /// The category of the trading product (e.g., Spot, Linear for perpetual futures).
    ///
    /// Bybit organizes trading into categories like Spot (cash trading) and Linear
    /// (USDT-margined perpetual futures). For perpetual futures, `Linear` is commonly
    /// used, supporting symbols like BTCUSDT. Bots must set this correctly to target
    /// the intended market. Misconfiguring the category can result in API errors or
    /// orders placed in the wrong market, disrupting strategy execution.
    pub category: Category,

    /// The trading pair symbol, e.g., "BTCUSDT".
    ///
    /// The symbol identifies the asset pair being traded, such as BTCUSDT for Bitcoin
    /// against USDT. In perpetual futures, symbols are typically USDT-margined (Linear).
    /// Bots must ensure the symbol is valid and supported by Bybit, as invalid symbols
    /// lead to order rejection. Additionally, each symbol has specific contract details
    /// (e.g., tick size, quantity precision), which bots should validate to avoid errors.
    pub symbol: Cow<'a, str>,

    /// Indicates if the order uses leverage (true for margin trading, false otherwise).
    ///
    /// Leverage allows traders to amplify positions with borrowed funds, common in
    /// perpetual futures. For example, 10x leverage means a $1,000 position controls
    /// $10,000 of assets. Bots must set this field based on the account's margin mode
    /// (isolated or cross) and risk tolerance. High leverage increases potential returns
    /// but also liquidation risk if prices move adversely. Bots should monitor margin
    /// levels to avoid forced liquidations.
    pub is_leverage: Option<bool>,

    /// The side of the order (Buy or Sell).
    ///
    /// Specifies whether the order is to buy (long) or sell (short). In perpetual
    /// futures, Buy orders open long positions or close short ones, while Sell orders
    /// open short positions or close long ones. Bots must align the side with their
    /// market outlook and position management strategy. Incorrect sides can lead to
    /// unintended exposure, so bots should include validation logic.
    pub side: Side,

    /// The type of order (Limit or Market).
    ///
    /// Determines how the order is executed. Limit orders specify a price, while Market
    /// orders execute at the best available price. In volatile perpetual futures markets,
    /// Market orders risk slippage, while Limit orders may not execute if the price moves
    /// away. Bots should choose based on strategy goals (e.g., speed vs. price control)
    /// and account for Bybit's fee structure (maker vs. taker fees).
    pub order_type: OrderType,

    /// The quantity of the asset to trade.
    ///
    /// Represents the size of the order, typically in the base asset (e.g., BTC for
    /// BTCUSDT). In perpetual futures, quantity must adhere to Bybit's minimum and
    /// maximum limits, which vary by symbol. Bots must validate quantity against account
    /// balance, margin requirements, and symbol constraints to avoid rejections. Overly
    /// large quantities can trigger liquidations if the market moves against the position.
    pub qty: f64,

    /// The unit for market orders (optional, used in specific cases).
    ///
    /// Some Bybit markets allow specifying quantities in alternative units (e.g., USDT
    /// for Linear contracts). This field is typically used for market orders in specific
    /// trading modes. Bots should consult Bybit's API documentation for symbol-specific
    /// requirements, as incorrect units can cause order failures. Most strategies leave
    /// this as `None` unless explicitly required.
    pub market_unit: Option<f64>,

    /// The price at which to place a limit order (optional for market orders).
    ///
    /// For Limit orders, this specifies the target price. For Market orders, it’s
    /// typically `None`. In perpetual futures, price must align with the symbol’s tick
    /// size (e.g., $0.5 for BTCUSDT). Bots should validate prices against current market
    /// conditions to ensure orders are realistic. Setting prices too far from the market
    /// can prevent execution, while tight prices risk being filled at suboptimal levels.
    pub price: Option<f64>,

    /// The direction of the trigger price (true for rising, false for falling).
    ///
    /// Used for conditional orders (e.g., stop or take-profit orders), indicating whether
    /// the trigger activates when the price rises or falls. For example, a stop-loss
    /// might trigger when the price falls below a threshold. Bots must set this correctly
    /// to align with the intended risk management strategy. Incorrect directions can lead
    /// to premature or missed triggers, impacting profitability.
    pub trigger_direction: Option<bool>,

    /// A filter to specify the order type (e.g., "tpslOrder" for take-profit/stop-loss).
    ///
    /// Bybit uses order filters to categorize special order types, such as take-profit/
    /// stop-loss (TPSL) orders. For example, setting `order_filter` to "tpslOrder"
    /// indicates the order is part of a TPSL strategy. Bots must use valid filter values
    /// as per Bybit’s API to avoid rejections. This field is crucial for automated risk
    /// management in perpetual futures, where volatility necessitates robust controls.
    pub order_filter: Option<Cow<'a, str>>,

    /// The price at which a conditional order is triggered.
    ///
    /// Specifies the price level that activates a conditional order (e.g., stop-loss or
    /// take-profit). In perpetual futures, trigger prices are critical for risk management,
    /// as they define exit points for profitable or losing positions. Bots should set
    /// trigger prices based on technical analysis or risk thresholds, ensuring they align
    /// with market volatility and symbol tick sizes to avoid invalid orders.
    pub trigger_price: Option<f64>,

    /// The price type for triggering the order (e.g., "LastPrice", "IndexPrice").
    ///
    /// Defines which price metric (e.g., last traded price, index price, or mark price)
    /// triggers a conditional order. In perpetual futures, Bybit uses different price
    /// types to account for market manipulation or volatility. For example, "MarkPrice"
    /// is less susceptible to spoofing than "LastPrice". Bots must choose the appropriate
    /// trigger type to ensure reliable execution, especially in high-frequency trading.
    pub trigger_by: Option<Cow<'a, str>>,

    /// The implied volatility for options orders (optional, not used in perpetual futures).
    ///
    /// While not typically used in perpetual futures, this field applies to options
    /// trading on Bybit, representing the expected volatility of the underlying asset.
    /// Bots trading perpetual futures can ignore this field, but those expanding to
    /// options must calculate or source accurate volatility data to set this correctly.
    pub order_iv: Option<f64>,

    /// The time-in-force policy for the order.
    ///
    /// Specifies how long the order remains active (e.g., GTC, IOC). In perpetual
    /// futures, TIF affects execution strategy and fees. For example, PostOnly orders
    /// optimize for maker fees but may not execute in fast markets. Bots should select
    /// TIF based on market conditions and strategy goals, ensuring compatibility with
    /// Bybit’s API requirements.
    pub time_in_force: Option<Cow<'a, str>>,

    /// The position index for futures (0 for one-way, 1 or 2 for hedge mode).
    ///
    /// In Bybit’s perpetual futures, position index distinguishes between one-way mode
    /// (0) and hedge mode (1 for long, 2 for short). Hedge mode allows simultaneous long
    /// and short positions on the same symbol. Bots must set this correctly based on the
    /// account’s position mode to avoid order rejections. Mismanaging position index can
    /// lead to unintended position netting or increased margin requirements.
    pub position_idx: Option<u8>,

    /// A user-defined identifier for the order.
    ///
    /// Allows bots to tag orders with a custom ID for tracking and management. In
    /// perpetual futures, where multiple orders may be active, `order_link_id` helps
    /// bots correlate API responses with specific strategies. Bots should use unique,
    /// descriptive IDs to avoid confusion and ensure robust order tracking.
    pub order_link_id: Option<Cow<'a, str>>,

    /// The take-profit price for the order.
    ///
    /// Sets the price at which a position is automatically closed to lock in profits.
    /// In perpetual futures, take-profit (TP) is essential for risk management, allowing
    /// bots to exit positions at predefined profit levels. Bots should calculate TP based
    /// on market analysis and risk-reward ratios, ensuring the price is realistic given
    /// volatility and tick size constraints.
    pub take_profit: Option<f64>,

    /// The stop-loss price for the order.
    ///
    /// Sets the price at which a position is closed to limit losses. In perpetual futures,
    /// stop-loss (SL) is critical to prevent significant drawdowns, especially with
    /// leverage. Bots should set SL based on risk tolerance and market volatility, ensuring
    /// it’s not too tight (risking premature exit) or too loose (risking large losses).
    pub stop_loss: Option<f64>,

    /// The price type for triggering take-profit (e.g., "LastPrice", "MarkPrice").
    ///
    /// Specifies which price metric triggers the take-profit order. Choosing the right
    /// trigger type is crucial in perpetual futures to avoid manipulation-driven triggers.
    /// Bots should prefer stable metrics like "MarkPrice" for reliability, especially in
    /// volatile markets.
    pub tp_trigger_by: Option<Cow<'a, str>>,

    /// The price type for triggering stop-loss (e.g., "LastPrice", "MarkPrice").
    ///
    /// Specifies which price metric triggers the stop-loss order. Similar to `tp_trigger_by`,
    /// bots should select a reliable trigger type to ensure stop-loss activates as intended,
    /// protecting against adverse price movements in perpetual futures.
    pub sl_trigger_by: Option<Cow<'a, str>>,

    /// Indicates if the order reduces an existing position (true) or opens a new one (false).
    ///
    /// In perpetual futures, `reduce_only` ensures an order only closes an existing
    /// position, preventing unintended increases in exposure. Bots must set this correctly
    /// when closing positions to avoid accidentally opening opposing positions, which could
    /// increase margin requirements or trigger liquidations.
    pub reduce_only: Option<bool>,

    /// Indicates if the order closes the position when triggered (used for stop orders).
    ///
    /// When `true`, the order closes the position upon triggering, typically used for
    /// stop-loss or take-profit orders. In perpetual futures, this helps bots manage risk
    /// by ensuring automatic position closure at predefined levels. Bots should enable
    /// this for risk-limiting orders to prevent unintended position retention.
    pub close_on_trigger: Option<bool>,

    /// The type of self-match prevention (optional, for institutional traders).
    ///
    /// Self-match prevention (SMP) prevents a trader’s own orders from matching against
    /// each other, which could manipulate market perception. In perpetual futures, this
    /// is relevant for large or institutional bots to comply with exchange rules. Most
    /// retail bots can leave this as `None` unless operating at scale.
    pub smp_type: Option<Cow<'a, str>>,

    /// Indicates if the order uses market maker protection (optional).
    ///
    /// Market maker protection (MMP) prevents rapid liquidations for market makers by
    /// adjusting order execution. In perpetual futures, this is typically used by advanced
    /// bots providing liquidity. Retail bots can usually ignore this field unless
    /// participating in Bybit’s market maker program.
    pub mmp: Option<bool>,

    /// The take-profit/stop-loss mode (e.g., "Full", "Partial").
    ///
    /// Specifies whether TP/SL applies to the entire position ("Full") or a portion
    /// ("Partial"). In perpetual futures, "Full" ensures complete position closure at
    /// TP/SL levels, while "Partial" allows scaling out. Bots should choose based on
    /// strategy; "Full" is simpler for risk management, while "Partial" suits complex
    /// exit strategies.
    pub tpsl_mode: Option<Cow<'a, str>>,

    /// The limit price for take-profit orders (for limit TP orders).
    ///
    /// Specifies the exact price for a limit-based take-profit order. In perpetual futures,
    /// this allows precise profit-taking but risks non-execution if the market doesn’t
    /// reach the price. Bots should set this based on market depth and volatility to
    /// balance execution likelihood and profitability.
    pub tp_limit_price: Option<f64>,

    /// The limit price for stop-loss orders (for limit SL orders).
    ///
    /// Specifies the exact price for a limit-based stop-loss order. Similar to
    /// `tp_limit_price`, bots must balance precision with execution risk, ensuring the
    /// price is achievable in volatile perpetual futures markets to protect against losses.
    pub sl_limit_price: Option<f64>,

    /// The order type for take-profit (e.g., "Market", "Limit").
    ///
    /// Defines whether the take-profit order executes as a Market or Limit order. Market
    /// TP ensures execution but risks slippage, while Limit TP offers price control but
    /// may not fill. Bots should choose based on market conditions and strategy priorities
    /// in perpetual futures trading.
    pub tp_order_type: Option<Cow<'a, str>>,

    /// The order type for stop-loss (e.g., "Market", "Limit").
    ///
    /// Defines whether the stop-loss order executes as a Market or Limit order. Similar
    /// to `tp_order_type`, bots must weigh execution speed against price control, as
    /// Market SL orders ensure loss limitation but may incur slippage in volatile markets.
    pub sl_order_type: Option<Cow<'a, str>>,

    /// Slippage tolerance type for market orders (TickSize or Percent).
    ///
    /// Specifies how slippage tolerance is calculated for market orders.
    /// - `TickSize`: slippage tolerance is measured in tick size units
    /// - `Percent`: slippage tolerance is measured as a percentage
    /// Not supported for take profit, stop loss, or conditional orders.
    pub slippage_tolerance_type: Option<Cow<'a, str>>,

    /// Slippage tolerance value for market orders.
    ///
    /// The value depends on `slippage_tolerance_type`:
    /// - For `TickSize`: range is [1, 10000], integer only
    /// - For `Percent`: range is [0.01, 10], up to 2 decimals
    pub slippage_tolerance: Option<f64>,

    /// BBO (Best Bid/Offer) side type for linear & inverse orders.
    ///
    /// Determines which side of the order book to use for pricing:
    /// - `Queue`: use order price on the same side as the order
    /// - `Counterparty`: use order price on the opposite side
    /// Valid for `linear` & `inverse` categories only.
    pub bbo_side_type: Option<Cow<'a, str>>,

    /// BBO (Best Bid/Offer) level for linear & inverse orders.
    ///
    /// Specifies which level of the order book to use (1-5).
    /// Valid for `linear` & `inverse` categories only.
    pub bbo_level: Option<u8>,
}

impl<'a> Default for OrderRequest<'a> {
    fn default() -> Self {
        Self {
            category: Category::Linear,
            symbol: Cow::Borrowed("BTCUSDT"),
            is_leverage: None,
            side: Side::default(),
            order_type: OrderType::Market,
            qty: 0.00,
            market_unit: None,
            price: None,
            trigger_direction: None,
            order_filter: None,
            trigger_price: None,
            trigger_by: None,
            order_iv: None,
            time_in_force: None,
            position_idx: None,
            order_link_id: None,
            take_profit: None,
            stop_loss: None,
            tp_trigger_by: None,
            sl_trigger_by: None,
            reduce_only: None,
            close_on_trigger: None,
            smp_type: None,
            mmp: None,
            tpsl_mode: None,
            tp_limit_price: None,
            sl_limit_price: None,
            tp_order_type: None,
            sl_order_type: None,
            slippage_tolerance_type: None,
            slippage_tolerance: None,
            bbo_side_type: None,
            bbo_level: None,
        }
    }
}

impl<'a> OrderRequest<'a> {
    /// Creates a custom `OrderRequest` with specified parameters.
    ///
    /// Allows bots to construct an order request with full control over all fields.
    /// Ensure all parameters comply with Bybit’s API constraints (e.g., valid symbol,
    /// quantity precision) to avoid rejections. This method is ideal for tailored
    /// strategies in perpetual futures trading.
    pub fn custom(
        category: Category,
        symbol: &'a str,
        leverage: Option<bool>,
        side: Side,
        order_type: OrderType,
        qty: f64,
        market_unit: Option<f64>,
        price: Option<f64>,
        trigger_direction: Option<bool>,
        order_filter: Option<&'a str>,
        trigger_price: Option<f64>,
        trigger_by: Option<&'a str>,
        order_iv: Option<f64>,
        time_in_force: Option<&'a str>,
        position_idx: Option<u8>,
        order_link_id: Option<&'a str>,
        take_profit: Option<f64>,
        stop_loss: Option<f64>,
        tp_trigger_by: Option<&'a str>,
        sl_trigger_by: Option<&'a str>,
        reduce_only: Option<bool>,
        close_on_trigger: Option<bool>,
        smp_type: Option<&'a str>,
        mmp: Option<bool>,
        tpsl_mode: Option<&'a str>,
        tp_limit_price: Option<f64>,
        sl_limit_price: Option<f64>,
        tp_order_type: Option<&'a str>,
        sl_order_type: Option<&'a str>,
        slippage_tolerance_type: Option<&'a str>,
        slippage_tolerance: Option<f64>,
        bbo_side_type: Option<&'a str>,
        bbo_level: Option<u8>,
    ) -> Self {
        Self {
            category,
            symbol: Cow::Borrowed(symbol),
            is_leverage: leverage,
            side,
            order_type,
            qty,
            market_unit,
            price,
            trigger_direction,
            order_filter: order_filter.map(Cow::Borrowed),
            trigger_price,
            trigger_by: trigger_by.map(Cow::Borrowed),
            order_iv,
            time_in_force: time_in_force.map(Cow::Borrowed),
            position_idx,
            order_link_id: order_link_id.map(Cow::Borrowed),
            take_profit,
            stop_loss,
            tp_trigger_by: tp_trigger_by.map(Cow::Borrowed),
            sl_trigger_by: sl_trigger_by.map(Cow::Borrowed),
            reduce_only,
            close_on_trigger,
            smp_type: smp_type.map(Cow::Borrowed),
            mmp,
            tpsl_mode: tpsl_mode.map(Cow::Borrowed),
            tp_limit_price,
            sl_limit_price,
            tp_order_type: tp_order_type.map(Cow::Borrowed),
            sl_order_type: sl_order_type.map(Cow::Borrowed),
            slippage_tolerance_type: slippage_tolerance_type.map(Cow::Borrowed),
            slippage_tolerance,
            bbo_side_type: bbo_side_type.map(Cow::Borrowed),
            bbo_level,
        }
    }
    /// Creates a spot limit order with market-based take-profit and stop-loss.
    ///
    /// Constructs a spot limit order with predefined TP/SL executed as market orders.
    /// Suitable for spot trading strategies aiming for precise entry prices with
    /// automated exits. Bots should ensure TP/SL prices are set appropriately to balance
    /// risk and reward.
    pub fn spot_limit_with_market_tpsl(
        symbol: &'a str,
        side: Side,
        qty: f64,
        price: f64,
        tp: f64,
        sl: f64,
    ) -> Self {
        Self {
            category: Category::Spot,
            symbol: Cow::Borrowed(symbol),
            side,
            order_type: OrderType::Limit,
            qty,
            price: Some(price),
            time_in_force: Some(Cow::Borrowed(TimeInForce::PostOnly.as_str())),
            take_profit: Some(tp),
            stop_loss: Some(sl),
            tp_order_type: Some(Cow::Borrowed("Market")),
            sl_order_type: Some(Cow::Borrowed("Market")),
            ..Self::default()
        }
    }
    /// Creates a spot limit order with limit-based take-profit and stop-loss.
    ///
    /// Constructs a spot limit order with TP/SL executed as limit orders for precise
    /// exit prices. Ideal for strategies prioritizing price control over execution
    /// certainty. Bots must ensure TP/SL prices are achievable given market conditions.
    pub fn spot_limit_with_limit_tpsl(
        symbol: &'a str,
        side: Side,
        qty: f64,
        price: f64,
        tp: f64,
        sl: f64,
    ) -> Self {
        Self {
            category: Category::Spot,
            symbol: Cow::Borrowed(symbol),
            side,
            order_type: OrderType::Limit,
            qty,
            price: Some(price),
            time_in_force: Some(Cow::Borrowed(TimeInForce::PostOnly.as_str())),
            take_profit: Some(tp),
            stop_loss: Some(sl),
            tp_limit_price: Some(tp),
            sl_limit_price: Some(sl),
            tp_order_type: Some(Cow::Borrowed("Limit")),
            sl_order_type: Some(Cow::Borrowed("Limit")),
            ..Self::default()
        }
    }
    /// Creates a spot post-only limit order.
    ///
    /// Constructs a spot limit order that only adds liquidity (maker order). Useful for
    /// minimizing fees but risks non-execution if the market moves away. Bots should
    /// monitor order book depth to ensure the price is competitive.
    pub fn spot_postonly(symbol: &'a str, side: Side, qty: f64, price: f64) -> Self {
        Self {
            category: Category::Spot,
            symbol: Cow::Borrowed(symbol),
            side,
            order_type: OrderType::Limit,
            qty,
            price: Some(price),
            time_in_force: Some(Cow::Borrowed(TimeInForce::PostOnly.as_str())),
            ..Self::default()
        }
    }
    /// Creates a spot take-profit/stop-loss order.
    ///
    /// Constructs a spot order specifically for TP/SL, often used to manage existing
    /// positions. The `order_filter` is set to "tpslOrder" to indicate this purpose.
    /// Bots should use this for automated risk management, ensuring the `order_link_id`
    /// is unique for tracking.
    pub fn spot_tpsl(
        symbol: &'a str,
        side: Side,
        price: f64,
        qty: f64,
        order_link_id: Option<&'a str>,
    ) -> Self {
        Self {
            category: Category::Spot,
            symbol: Cow::Borrowed(symbol),
            side,
            order_type: OrderType::Limit,
            qty,
            price: Some(price),
            time_in_force: Some(Cow::Borrowed(TimeInForce::GTC.as_str())),
            order_link_id: order_link_id.map(Cow::Borrowed),
            order_filter: Some(Cow::Borrowed("tpslOrder")),
            ..Self::default()
        }
    }
    /// Creates a spot margin order with leverage.
    ///
    /// Constructs a spot order using margin (borrowed funds). Leverage increases
    /// potential returns but also liquidation risk. Bots must monitor margin levels
    /// and ensure sufficient collateral to avoid forced liquidations.
    pub fn spot_margin(symbol: &'a str, side: Side, qty: f64, price: f64) -> Self {
        Self {
            category: Category::Spot,
            symbol: Cow::Borrowed(symbol),
            side,
            order_type: OrderType::Market,
            qty,
            price: Some(price),
            time_in_force: Some(Cow::Borrowed(TimeInForce::PostOnly.as_str())),
            is_leverage: Some(true),
            ..Self::default()
        }
    }
    /// Creates a spot market order.
    ///
    /// Constructs a spot market order for immediate execution. Ideal for strategies
    /// prioritizing speed over price control. Bots should account for slippage and
    /// taker fees, which can impact profitability in volatile markets.
    pub fn spot_market(symbol: &'a str, side: Side, qty: f64) -> Self {
        Self {
            category: Category::Spot,
            symbol: Cow::Borrowed(symbol),
            side,
            order_type: OrderType::Market,
            qty,
            time_in_force: Some(Cow::Borrowed(TimeInForce::IOC.as_str())),
            ..Self::default()
        }
    }
    /// Creates a futures limit order with market-based take-profit and stop-loss.
    ///
    /// Constructs a perpetual futures limit order with TP/SL executed as market orders.
    /// Suitable for strategies balancing precise entry with automated exits. Bots should
    /// ensure TP/SL levels align with risk management goals and market volatility.
    pub fn futures_limit_with_market_tpsl(
        symbol: &'a str,
        side: Side,
        qty: f64,
        price: f64,
        tp: f64,
        sl: f64,
    ) -> Self {
        Self {
            category: Category::Linear,
            symbol: Cow::Borrowed(symbol),
            side,
            order_type: OrderType::Limit,
            qty,
            price: Some(price),
            time_in_force: Some(Cow::Borrowed(TimeInForce::PostOnly.as_str())),
            reduce_only: Some(false),
            take_profit: Some(tp),
            stop_loss: Some(sl),
            tpsl_mode: Some(Cow::Borrowed("Full")),
            tp_order_type: Some(Cow::Borrowed("Market")),
            sl_order_type: Some(Cow::Borrowed("Market")),
            ..Self::default()
        }
    }
    /// Creates a futures limit order with limit-based take-profit and stop-loss.
    ///
    /// Constructs a perpetual futures limit order with TP/SL executed as limit orders.
    /// Ideal for precise exit price control, though execution is not guaranteed. Bots
    /// must set TP/SL prices realistically to ensure they are filled in volatile markets.
    pub fn futures_limit_with_limit_tpsl(
        symbol: &'a str,
        side: Side,
        qty: f64,
        price: f64,
        tp: f64,
        sl: f64,
    ) -> Self {
        Self {
            category: Category::Linear,
            symbol: Cow::Borrowed(symbol),
            side,
            order_type: OrderType::Limit,
            qty,
            price: Some(price),
            time_in_force: Some(Cow::Borrowed(TimeInForce::PostOnly.as_str())),
            reduce_only: Some(false),
            take_profit: Some(tp),
            stop_loss: Some(sl),
            tpsl_mode: Some(Cow::Borrowed("Partial")),
            tp_order_type: Some(Cow::Borrowed("Limit")),
            sl_order_type: Some(Cow::Borrowed("Limit")),
            tp_limit_price: Some(tp),
            sl_limit_price: Some(sl),
            ..Self::default()
        }
    }
    /// Creates a futures market order.
    ///
    /// Constructs a perpetual futures market order for immediate execution. Suitable
    /// for strategies needing fast entry or exit. Bots should account for slippage and
    /// taker fees, which can be significant in perpetual futures due to leverage and
    /// volatility.
    pub fn futures_market(symbol: &'a str, side: Side, qty: f64) -> Self {
        Self {
            category: Category::Linear,
            symbol: Cow::Borrowed(symbol),
            side,
            order_type: OrderType::Market,
            qty,
            time_in_force: Some(Cow::Borrowed(TimeInForce::IOC.as_str())),
            reduce_only: Some(false),
            ..Self::default()
        }
    }
    /// Creates a futures limit order to close a position.
    ///
    /// Constructs a perpetual futures limit order to close an existing position, with
    /// `reduce_only` set to `true`. Bots should use this to exit positions at specific
    /// prices, ensuring the `order_link_id` is unique for tracking. Validate quantity
    /// against the open position to avoid over-closing.
    pub fn futures_close_limit(
        symbol: &'a str,
        side: Side,
        qty: f64,
        price: f64,
        order_link_id: &'a str,
    ) -> Self {
        Self {
            category: Category::Linear,
            symbol: Cow::Borrowed(symbol),
            side,
            order_type: OrderType::Limit,
            qty,
            price: Some(price),
            time_in_force: Some(Cow::Borrowed(TimeInForce::GTC.as_str())),
            order_link_id: Some(Cow::Borrowed(order_link_id)),
            reduce_only: Some(true),
            ..Self::default()
        }
    }
    /// Creates a futures market order to close a position.
    ///
    /// Constructs a perpetual futures market order to immediately close an existing
    /// position. Ideal for rapid exits in volatile markets. Bots should ensure the
    /// quantity matches the open position and account for slippage and fees.
    pub fn futures_market_close(symbol: &'a str, side: Side, qty: f64) -> Self {
        Self {
            category: Category::Linear,
            symbol: Cow::Borrowed(symbol),
            side,
            order_type: OrderType::Market,
            qty,
            time_in_force: Some(Cow::Borrowed(TimeInForce::IOC.as_str())),
            reduce_only: Some(true),
            ..Self::default()
        }
    }
}
